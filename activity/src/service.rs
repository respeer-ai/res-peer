#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use self::state::Activity;
use activity::{ActivityParameters, Operation};
use async_graphql::{EmptySubscription, Request, Response, Schema};
use linera_sdk::{
    base::WithServiceAbi, graphql::GraphQLMutationRoot, views::View, Service, ServiceRuntime,
};
use std::sync::Arc;

pub struct ActivityService {
    state: Arc<Activity>,
}

linera_sdk::service!(ActivityService);

impl WithServiceAbi for ActivityService {
    type Abi = activity::ActivityAbi;
}

impl Service for ActivityService {
    type Parameters = ActivityParameters;

    async fn new(runtime: ServiceRuntime<Self>) -> Self {
        let state = Activity::load(runtime.root_view_storage_context())
            .await
            .expect("Failed to load state");
        ActivityService {
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
