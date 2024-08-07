// Copyright (c) Zefchain Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use async_graphql::{Context, EmptyMutation, EmptySubscription, Object, Request, Response, Schema};
use linera_sdk::{base::WithServiceAbi, Service, ServiceRuntime};
use log::info;

pub struct BlobGatewayService {}

linera_sdk::service!(BlobGatewayService);

impl WithServiceAbi for BlobGatewayService {
    type Abi = blob_gateway::BlobGatewayAbi;
}

struct QueryRoot {}

#[Object]
impl QueryRoot {
    async fn prompt(&self, _ctx: &Context<'_>, _prompt: String) -> String {
        String::new()
    }
}

impl Service for BlobGatewayService {
    type Parameters = ();

    async fn new(_runtime: ServiceRuntime<Self>) -> Self {
        BlobGatewayService {}
    }

    async fn handle_query(&self, request: Request) -> Response {
        let query_string = &request.query;
        info!("query: {}", query_string);

        let schema = Schema::build(QueryRoot {}, EmptyMutation, EmptySubscription).finish();
        schema.execute(request).await
    }
}
