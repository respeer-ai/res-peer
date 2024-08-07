// Copyright (c) Zefchain Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use linera_sdk::{base::WithContractAbi, Contract, ContractRuntime};

pub struct CPRegistryContract;

linera_sdk::contract!(CPRegistryContract);

impl WithContractAbi for CPRegistryContract {
    type Abi = cp_registry::CPRegistryAbi;
}

impl Contract for CPRegistryContract {
    type Message = ();
    type InstantiationArgument = ();
    type Parameters = ();

    async fn load(_runtime: ContractRuntime<Self>) -> Self {
        CPRegistryContract
    }

    async fn instantiate(&mut self, _value: ()) {}

    async fn execute_operation(&mut self, _operation: ()) -> Self::Response {}

    async fn execute_message(&mut self, _message: ()) {
        panic!("CPRegistry application doesn't support any cross-chain messages");
    }

    async fn store(self) {}
}
