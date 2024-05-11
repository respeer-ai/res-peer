#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use self::state::Foundation;
use async_graphql::{EmptySubscription, Request, Response, Schema};
use foundation::Operation;
use linera_sdk::{
    base::WithServiceAbi,
    graphql::GraphQLMutationRoot,
    views::{View, ViewStorageContext},
    Service, ServiceRuntime,
};
use std::sync::Arc;

pub struct FoundationService {
    state: Arc<Foundation>,
}

linera_sdk::service!(FoundationService);

impl WithServiceAbi for FoundationService {
    type Abi = foundation::FoundationAbi;
}

impl Service for FoundationService {
    type Parameters = ();

    async fn new(runtime: ServiceRuntime<Self>) -> Self {
        let state = Foundation::load(ViewStorageContext::from(runtime.key_value_store()))
            .await
            .expect("Failed to load state");
        FoundationService {
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
