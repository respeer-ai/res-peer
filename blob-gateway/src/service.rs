// Copyright (c) Zefchain Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use std::sync::{Arc, Mutex};

use async_graphql::{Context, EmptyMutation, EmptySubscription, Object, Request, Response, Schema};
use linera_sdk::{base::{WithServiceAbi, Timestamp}, views::View, DataBlobHash, Service, ServiceRuntime};

use blob_gateway::{BlobData, BlobDataType};
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

    async fn list(
        &self, 
        ctx: &Context<'_>,
        created_before: Option<Timestamp>,
        created_after: Option<Timestamp>,
        data_type: Option<BlobDataType>,
        limit: usize
    ) -> Result<Vec<BlobData>, anyhow::Error> {
        let ctx = ctx.data::<FetchContext>().unwrap();
        let mut blobs = Vec::new();

        ctx.state
            .blobs
            .for_each_index_value(|_key, value| {
                blobs.push(value);
                Ok(())
            })
            .await?;
        blobs.sort_by(|a, b| a.created_at.cmp(&b.created_at));
        
        let filtered: Vec<BlobData> = blobs
        .into_iter()
        .filter(|item| {
            let before_condition = created_before.map_or(true, |before| item.created_at < before);
            let after_condition = created_after.map_or(true, |after| item.created_at > after);
            let application_type_condition = data_type.map_or(true, |data| item.data_type == data);
            before_condition && after_condition && application_type_condition
        })
        .collect();
        let results = filtered.into_iter().take(limit).collect();
        Ok(results)
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
