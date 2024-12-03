// Copyright (c) Zefchain Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use std::sync::{Arc, Mutex};

use async_graphql::{Context, EmptyMutation, EmptySubscription, Object, Request, Response, Schema};
use linera_sdk::{base::WithServiceAbi, views::View, DataBlobHash, Service, ServiceRuntime};

use blob_gateway::BlobData;
use state::BlobGateway;

pub struct BlobGatewayService {
    state: Arc<BlobGateway>,
    runtime: Arc<Mutex<ServiceRuntime<BlobGatewayService>>>,
}

linera_sdk::service!(BlobGatewayService);

impl WithServiceAbi for BlobGatewayService {
    type Abi = blob_gateway::BlobGatewayAbi;
}

struct FetchContext {
    state: Arc<BlobGateway>,
    runtime: Arc<Mutex<ServiceRuntime<BlobGatewayService>>>,
}

struct QueryRoot {}

#[Object]
impl QueryRoot {
    async fn fetch(
        &self,
        ctx: &Context<'_>,
        blob_hash: DataBlobHash,
    ) -> Result<Vec<u8>, anyhow::Error> {
        let ctx = ctx.data::<FetchContext>().unwrap();
        Ok(ctx.runtime.lock().unwrap().read_data_blob(blob_hash))
    }

    async fn list(&self, ctx: &Context<'_>) -> Result<Vec<BlobData>, anyhow::Error> {
        let ctx = ctx.data::<FetchContext>().unwrap();
        let mut blobs = Vec::new();

        ctx.state
            .blobs
            .for_each_index_value(|_key, value| {
                blobs.push(value);
                Ok(())
            })
            .await?;

        Ok(blobs)
    }
}

impl Service for BlobGatewayService {
    type Parameters = ();

    async fn new(runtime: ServiceRuntime<Self>) -> Self {
        let state = BlobGateway::load(runtime.root_view_storage_context())
            .await
            .expect("Failed to load state");
        BlobGatewayService {
            state: Arc::new(state),
            runtime: Arc::new(Mutex::new(runtime)),
        }
    }

    async fn handle_query(&self, request: Request) -> Response {
        let fetch_context = FetchContext {
            state: self.state.clone(),
            runtime: self.runtime.clone(),
        };

        let schema = Schema::build(QueryRoot {}, EmptyMutation, EmptySubscription)
            .data(fetch_context)
            .finish();
        schema.execute(request).await
    }
}
