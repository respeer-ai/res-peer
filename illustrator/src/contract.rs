// Copyright (c) Zefchain Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use linera_sdk::{base::WithContractAbi, Contract, ContractRuntime};

pub struct IllustratorContract;

linera_sdk::contract!(IllustratorContract);

impl WithContractAbi for IllustratorContract {
    type Abi = illustrator::IllustratorAbi;
}

impl Contract for IllustratorContract {
    type Message = ();
    type InstantiationArgument = ();
    type Parameters = ();

    async fn load(_runtime: ContractRuntime<Self>) -> Self {
        IllustratorContract
    }

    async fn instantiate(&mut self, _value: ()) {}

    async fn execute_operation(&mut self, _operation: ()) -> Self::Response {}

    async fn execute_message(&mut self, _message: ()) {
        panic!("Illustrator application doesn't support any cross-chain messages");
    }

    async fn store(self) {}
}
