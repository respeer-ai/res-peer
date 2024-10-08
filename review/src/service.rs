#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use self::state::Review;
use async_graphql::{EmptySubscription, Request, Response, Schema};
use linera_sdk::{
    base::WithServiceAbi, graphql::GraphQLMutationRoot, views::View, Service, ServiceRuntime,
};
use review::{Operation, ReviewParameters};
use std::sync::Arc;

pub struct ReviewService {
    state: Arc<Review>,
}

linera_sdk::service!(ReviewService);

impl WithServiceAbi for ReviewService {
    type Abi = review::ReviewAbi;
}

impl Service for ReviewService {
    type Parameters = ReviewParameters;

    async fn new(runtime: ServiceRuntime<Self>) -> Self {
        let state = Review::load(runtime.root_view_storage_context())
            .await
            .expect("Failed to load state");
        ReviewService {
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
