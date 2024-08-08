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

pub struct BlobGatewayService {
    runtime: Arc<Mutex<ServiceRuntime<BlobGatewayService>>>,
}

linera_sdk::service!(BlobGatewayService);

impl WithServiceAbi for BlobGatewayService {
    type Abi = blob_gateway::BlobGatewayAbi;
}

struct FetchContext {
    runtime: Arc<Mutex<ServiceRuntime<BlobGatewayService>>>,
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

impl Service for BlobGatewayService {
    type Parameters = ();

    async fn new(runtime: ServiceRuntime<Self>) -> Self {
        BlobGatewayService {
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
