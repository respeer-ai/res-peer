// Copyright (c) Zefchain Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use self::state::Copilot;
use copilot::InstantiationArgument;
use linera_sdk::{
    base::{CryptoHash, WithContractAbi},
    views::{View, ViewStorageContext},
    Contract, ContractRuntime,
};

pub struct CopilotContract {
    state: Copilot,
    runtime: ContractRuntime<Self>,
}

linera_sdk::contract!(CopilotContract);

impl WithContractAbi for CopilotContract {
    type Abi = copilot::CopilotAbi;
}

impl Contract for CopilotContract {
    type Message = ();
    type InstantiationArgument = InstantiationArgument;
    type Parameters = ();

    async fn load(runtime: ContractRuntime<Self>) -> Self {
        let state = Copilot::load(ViewStorageContext::from(runtime.key_value_store()))
            .await
            .expect("Failed to load state");
        CopilotContract { state, runtime }
    }

    async fn instantiate(&mut self, argument: InstantiationArgument) {
        let mut cp_registry_params: cp_registry::RegisterParameters = argument.into();
        cp_registry_params.payment_chain_id = self.runtime.chain_id();
        cp_registry_params.link = format!(
            "{}/chains/{:?}/applications/{:?}",
            cp_registry_params.link,
            self.runtime.chain_id(),
            self.runtime.application_id()
        );
        cp_registry_params.node_id = Some(CryptoHash::new(&cp_registry_params));
        log::info!("InstantiateArgument {:?}", cp_registry_params);
    }

    async fn execute_operation(&mut self, _operation: ()) -> Self::Response {}

    async fn execute_message(&mut self, _message: ()) {
        panic!("Copilot application doesn't support any cross-chain messages");
    }

    async fn store(self) {}
}
