#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use self::state::Market;
use async_graphql::{EmptySubscription, Request, Response, Schema};
use linera_sdk::{
    base::WithServiceAbi, graphql::GraphQLMutationRoot, views::View, Service, ServiceRuntime,
};
use market::{MarketParameters, Operation};
use std::sync::Arc;

pub struct MarketService {
    state: Arc<Market>,
}

linera_sdk::service!(MarketService);

impl WithServiceAbi for MarketService {
    type Abi = market::MarketAbi;
}

impl Service for MarketService {
    type Parameters = MarketParameters;

    async fn new(runtime: ServiceRuntime<Self>) -> Self {
        let state = Market::load(runtime.root_view_storage_context())
            .await
            .expect("Failed to load state");
        MarketService {
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
