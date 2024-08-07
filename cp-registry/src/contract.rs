// Copyright (c) Zefchain Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use cp_registry::{Message, Operation};
use linera_sdk::{base::WithContractAbi, Contract, ContractRuntime, views::{View, ViewStorageContext}};
use self::state::CPRegistry;

pub struct CPRegistryContract {
    state: CPRegistry,
    runtime: ContractRuntime<Self>
}

linera_sdk::contract!(CPRegistryContract);

impl WithContractAbi for CPRegistryContract {
    type Abi = cp_registry::CPRegistryAbi;
}

impl Contract for CPRegistryContract {
    type Message = Message;
    type InstantiationArgument = ();
    type Parameters = ();

    async fn load(runtime: ContractRuntime<Self>) -> Self {
        let state = CPRegistry::load(ViewStorageContext::from(runtime.key_value_store()))
            .await
            .expect("Failed to load state");
        CPRegistryContract { state, runtime }
    }

    async fn instantiate(&mut self, _value: ()) {
    }

    async fn execute_operation(&mut self, operation: Operation) -> Self::Response {
        match operation {
            Operation::Register { params } => self.on_op_register(params).await?,
            Operation::Update { params } => self.on_op_update(params).await?,
            Operation::Deregister { node_id } => self.on_op_deregister(node_id).await?,
            Operation::RequestSubscribe => self.on_op_request_subscribe().await?,
        }
    }

    async fn execute_message(&mut self, _message: Message) {
        panic!("CPRegistry application doesn't support any cross-chain messages");
    }

    async fn store(self) {}
}
