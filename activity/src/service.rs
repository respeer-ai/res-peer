#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use self::state::{Activity, ActivityParameters};
use async_graphql::{EmptySubscription, Object, Request, Response, Schema};
use linera_sdk::{base::WithServiceAbi, Service, ServiceRuntime, ViewStateStorage};
use std::sync::Arc;

use activity::{ActivityError, AnnounceParams, CreateParams, Operation, UpdateParams};

pub struct ActivityService {
    state: Arc<Activity>,
}

linera_sdk::service!(ActivityService);

impl WithServiceAbi for ActivityService {
    type Abi = activity::ActivityAbi;
}

impl Service for ActivityService {
    type Error = ActivityError;
    type Storage = ViewStateStorage<Self>;
    type State = Activity;
    type Parameters = ActivityParameters;

    async fn new(state: Self::State, _runtime: ServiceRuntime<Self>) -> Result<Self, Self::Error> {
        Ok(ActivityService {
            state: Arc::new(state),
        })
    }

    async fn handle_query(&self, request: Request) -> Result<Response, Self::Error> {
        let schema = Schema::build(self.state.clone(), MutationRoot, EmptySubscription).finish();
        let response = schema.execute(request).await;
        Ok(response)
    }
}

struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn create(&self, params: CreateParams) -> Vec<u8> {
        bcs::to_bytes(&Operation::Create { params }).unwrap()
    }

    async fn update(&self, params: UpdateParams) -> Vec<u8> {
        bcs::to_bytes(&Operation::Update { params }).unwrap()
    }

    async fn register(&self, activity_id: u64, object_id: String) -> Vec<u8> {
        bcs::to_bytes(&Operation::Register {
            activity_id,
            object_id,
        })
        .unwrap()
    }

    async fn vote(&self, activity_id: u64, object_id: String) -> Vec<u8> {
        bcs::to_bytes(&Operation::Vote {
            activity_id,
            object_id,
        })
        .unwrap()
    }

    async fn announce(&self, params: AnnounceParams) -> Vec<u8> {
        bcs::to_bytes(&Operation::Announce { params }).unwrap()
    }

    async fn request_subscribe(&self) -> Vec<u8> {
        bcs::to_bytes(&Operation::RequestSubscribe).unwrap()
    }

    async fn finalize(&self, activity_id: u64) -> Vec<u8> {
        bcs::to_bytes(&Operation::Finalize { activity_id }).unwrap()
    }
}
