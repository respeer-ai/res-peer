// Copyright (c) Zefchain Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use std::sync::Arc;

use self::state::CPRegistry;
use async_graphql::{EmptySubscription, Request, Response, Schema};
use cp_registry::Operation;
use linera_sdk::{
    base::WithServiceAbi, graphql::GraphQLMutationRoot, views::View, Service, ServiceRuntime,
};

pub struct CPRegistryService {
    state: Arc<CPRegistry>,
}

linera_sdk::service!(CPRegistryService);

impl WithServiceAbi for CPRegistryService {
    type Abi = cp_registry::CPRegistryAbi;
}

impl Service for CPRegistryService {
    type Parameters = ();

    async fn new(runtime: ServiceRuntime<Self>) -> Self {
        let state = CPRegistry::load(runtime.root_view_storage_context())
            .await
            .expect("Failed to load state");
        CPRegistryService {
            state: Arc::new(state),
        }
    }

    async fn handle_query(&self, request: Request) -> Response {
        let schema = Schema::build(
            self.state.clone(),
            Operation::mutation_root(),
            EmptySubscription,
        )
        .finish();
        schema.execute(request).await
    }
}
