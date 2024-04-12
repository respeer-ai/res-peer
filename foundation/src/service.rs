#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use self::state::Foundation;
use async_graphql::{EmptySubscription, Object, Request, Response, Schema};
use foundation::Operation;
use linera_sdk::{
    base::{Amount, WithServiceAbi},
    Service, ServiceRuntime, ViewStateStorage,
};
use std::sync::Arc;
use thiserror::Error;

pub struct FoundationService {
    state: Arc<Foundation>,
}

linera_sdk::service!(FoundationService);

impl WithServiceAbi for FoundationService {
    type Abi = foundation::FoundationAbi;
}

impl Service for FoundationService {
    type Error = ServiceError;
    type Storage = ViewStateStorage<Self>;
    type State = Foundation;
    type Parameters = ();

    async fn new(state: Self::State, _runtime: ServiceRuntime<Self>) -> Result<Self, Self::Error> {
        Ok(FoundationService {
            state: Arc::new(state),
        })
    }

    async fn handle_query(&self, request: Request) -> Result<Response, Self::Error> {
        let schema = Schema::build(self.state.clone(), MutationRoot {}, EmptySubscription).finish();
        let response = schema.execute(request).await;
        Ok(response)
    }
}

struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn request_subscribe(&self) -> Vec<u8> {
        bcs::to_bytes(&Operation::RequestSubscribe).unwrap()
    }

    async fn user_deposit(&self, amount: Amount) -> Vec<u8> {
        bcs::to_bytes(&Operation::UserDeposit { amount }).unwrap()
    }
}

/// An error that can occur while querying the service.
#[derive(Debug, Error)]
pub enum ServiceError {
    /// Query not supported by the application.
    #[error("Queries not supported by application")]
    QueriesNotSupported,

    /// Invalid query argument; could not deserialize request.
    #[error("Invalid query argument; could not deserialize request")]
    InvalidQuery(#[from] serde_json::Error),
    // Add error variants here.
}
