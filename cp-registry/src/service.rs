// Copyright (c) Zefchain Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use std::{
    str::FromStr,
    sync::{Arc, Mutex},
};

use async_graphql::{Context, EmptyMutation, EmptySubscription, Object, Request, Response, Schema};
use linera_sdk::{
    base::{Blob, BlobId, WithServiceAbi},
    Service, ServiceRuntime,
};

pub struct CPRegistryService {
    runtime: Arc<Mutex<ServiceRuntime<CPRegistryService>>>,
}

linera_sdk::service!(CPRegistryService);

impl WithServiceAbi for CPRegistryService {
    type Abi = cp_registry::CPRegistryAbi;
}

struct FetchContext {
    runtime: Arc<Mutex<ServiceRuntime<CPRegistryService>>>,
}

struct QueryRoot {}

#[Object]
impl QueryRoot {
    async fn fetch(&self, ctx: &Context<'_>, blob_id: String) -> Result<Blob, anyhow::Error> {
        let blob_id = BlobId::from_str(&blob_id)?;
        let ctx = ctx.data::<FetchContext>().unwrap();
        Ok(ctx.runtime.lock().unwrap().read_blob(blob_id).into_inner())
    }
}

impl Service for CPRegistryService {
    type Parameters = ();

    async fn new(runtime: ServiceRuntime<Self>) -> Self {
        CPRegistryService {
            runtime: Arc::new(Mutex::new(runtime)),
        }
    }

    async fn handle_query(&self, request: Request) -> Response {
        let fetch_context = FetchContext {
            runtime: self.runtime.clone(),
        };

        let schema = Schema::build(QueryRoot {}, EmptyMutation, EmptySubscription)
            .data(fetch_context)
            .finish();
        schema.execute(request).await
    }
}
