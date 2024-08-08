// Copyright (c) Zefchain Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use linera_sdk::{base::WithContractAbi, Contract, ContractRuntime};

pub struct CopilotContract;

linera_sdk::contract!(CopilotContract);

impl WithContractAbi for CopilotContract {
    type Abi = copilot::CopilotAbi;
}

impl Contract for CopilotContract {
    type Message = ();
    type InstantiationArgument = cp_registry::RegisterParameters;
    type Parameters = ();

    async fn load(_runtime: ContractRuntime<Self>) -> Self {
        CopilotContract
    }

    async fn instantiate(&mut self, argument: InstantiationArgument) {
        log::info!("InstantiateArgument {:?}", argument);
    }

    async fn execute_operation(&mut self, _operation: ()) -> Self::Response {}

    async fn execute_message(&mut self, _message: ()) {
        panic!("Copilot application doesn't support any cross-chain messages");
    }

    async fn store(self) {}
}
