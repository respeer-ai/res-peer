#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use self::state::Feed;
use async_graphql::{EmptySubscription, Request, Response, Schema};
use feed::{FeedParameters, Operation};
use linera_sdk::{
    base::WithServiceAbi, graphql::GraphQLMutationRoot, views::View, Service, ServiceRuntime,
};
use std::sync::Arc;

pub struct FeedService {
    state: Arc<Feed>,
}

linera_sdk::service!(FeedService);

impl WithServiceAbi for FeedService {
    type Abi = feed::FeedAbi;
}

impl Service for FeedService {
    type Parameters = FeedParameters;

    async fn new(runtime: ServiceRuntime<Self>) -> Self {
        let state = Feed::load(runtime.root_view_storage_context())
            .await
            .expect("Failed to load state");
        FeedService {
            state: Arc::new(state),
        }
    }

    async fn handle_query(&self, request: Request) -> Response {
        // TODO: we need to filter content according to requester and review state here
        let schema = Schema::build(
            self.state.clone(),
            Operation::mutation_root(),
            EmptySubscription,
        )
        .finish();
        schema.execute(request).await
    }
}
