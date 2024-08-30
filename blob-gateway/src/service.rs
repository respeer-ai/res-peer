// Copyright (c) Zefchain Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use std::{
    sync::{Arc, Mutex},
};

use async_graphql::{Context, EmptyMutation, EmptySubscription, Object, Request, Response, Schema};
use linera_sdk::{
    base::{WithServiceAbi},
    Service, ServiceRuntime,
    DataBlobHash,
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
    async fn fetch(&self, ctx: &Context<'_>, blob_hash: DataBlobHash) -> Result<Vec<u8>, anyhow::Error> {
        let ctx = ctx.data::<FetchContext>().unwrap();
        Ok(ctx.runtime.lock().unwrap().read_data_blob(blob_hash))
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
