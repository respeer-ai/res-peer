// Copyright (c) Zefchain Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use self::state::Copilot;
use copilot::{CopilotError, CopilotParameters, InstantiationArgument, Operation};
use cp_registry::{CPRegistryAbi, RegisterParameters};
use linera_sdk::{
    base::{Account, ApplicationId, CryptoHash, WithContractAbi},
    views::{RootView, View, ViewStorageContext},
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
    type Parameters = CopilotParameters;

    async fn load(runtime: ContractRuntime<Self>) -> Self {
        let state = Copilot::load(ViewStorageContext::from(runtime.key_value_store()))
            .await
            .expect("Failed to load state");
        CopilotContract { state, runtime }
    }

    async fn instantiate(&mut self, argument: InstantiationArgument) {
        self.state.instantiate(argument.clone()).await;

        let mut cp_registry_params: cp_registry::RegisterParameters = argument.into();
        cp_registry_params.payment_chain_id = self.runtime.chain_id();
        cp_registry_params.link = format!(
            "{}/chains/{:?}/applications/{:?}",
            cp_registry_params.link,
            self.runtime.chain_id(),
            self.runtime.application_id()
        );
        cp_registry_params.node_id = Some(CryptoHash::new(&cp_registry_params));

        self.cp_registry_register(cp_registry_params)
            .expect("Failed register node");
    }

    async fn execute_operation(&mut self, operation: Self::Operation) -> () {
        match operation {
            Operation::Deposit { query_id } => self
                .on_op_deposit_query(query_id)
                .await
                .expect("Failed OP: deposit query"),
        }
    }

    async fn execute_message(&mut self, _message: ()) {
        panic!("Copilot contract always must be run with its node service");
    }

    async fn store(mut self) {
        self.state.save().await.expect("Failed to save state");
    }
}

impl CopilotContract {
    fn cp_registry_app_id(&mut self) -> ApplicationId<CPRegistryAbi> {
        self.runtime.application_parameters().cp_registry_app_id
    }

    fn cp_registry_register(&mut self, params: RegisterParameters) -> Result<(), CopilotError> {
        let call = cp_registry::Operation::Register { params };
        let cp_registry_app_id = self.cp_registry_app_id();
        self.runtime
            .call_application(true, cp_registry_app_id, &call);
        Ok(())
    }

    async fn deposit_query(&mut self, query_id: CryptoHash) -> Result<(), CopilotError> {
        let destination = Account {
            chain_id: self.runtime.application_id().creation.chain_id,
            owner: None,
        };
        let owner = self.runtime.authenticated_signer();
        if !self.state.free_query(owner.expect("Invalid owner")).await? {
            self.runtime
                .transfer(owner, destination, self.state._quota_price().await);
        }

        Ok(self
            .state
            .deposit_query(owner.expect("Invalid owner"), query_id)
            .await?)
    }

    async fn on_op_deposit_query(&mut self, query_id: CryptoHash) -> Result<(), CopilotError> {
        self.deposit_query(query_id).await
    }
}
