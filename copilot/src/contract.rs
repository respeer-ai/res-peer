// Copyright (c) Zefchain Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use self::state::Copilot;
use copilot::{CopilotError, CopilotParameters, InstantiationArgument, Message, Operation};
use cp_registry::{CPRegistryAbi, RegisterParameters};
use linera_sdk::{
    base::{Account, Amount, ApplicationId, ChannelName, CryptoHash, MessageId, WithContractAbi},
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

const SUBSCRIPTION_CHANNEL: &[u8] = b"subscriptions";

impl Contract for CopilotContract {
    type Message = Message;
    type InstantiationArgument = InstantiationArgument;
    type Parameters = CopilotParameters;

    async fn load(runtime: ContractRuntime<Self>) -> Self {
        let state = Copilot::load(ViewStorageContext::from(runtime.key_value_store()))
            .await
            .expect("Failed to load state");
        CopilotContract { state, runtime }
    }

    // TODO: this will be run each time the chain is instantiate (e.g. load in a new wallet)
    async fn instantiate(&mut self, argument: InstantiationArgument) {
        if self.runtime.chain_id() != self.runtime.application_id().creation.chain_id {
            return;
        }

        self.state.instantiate(argument.clone()).await;

        let mut cp_registry_params: cp_registry::RegisterParameters = argument.into();
        cp_registry_params.payment_chain_id = self.runtime.chain_id();
        cp_registry_params.link = format!(
            "{}/chains/{}/applications/{}",
            cp_registry_params.link,
            self.runtime.chain_id(),
            serde_json::to_string(&self.runtime.application_id())
                .unwrap()
                .trim_matches('"'),
        );
        cp_registry_params.node_id = Some(CryptoHash::new(&cp_registry_params));

        let signer = match self.runtime.authenticated_signer() {
            Some(owner) => owner.to_string(),
            _ => "Invalid signer".to_string(),
        };

        log::info!(
            "Register computing node {:?} at chain {} by {} link {}",
            cp_registry_params.node_id,
            self.runtime.chain_id(),
            signer,
            cp_registry_params.link,
        );
        self.cp_registry_register(cp_registry_params)
            .expect("Failed register node");
    }

    async fn execute_operation(&mut self, operation: Self::Operation) -> () {
        match operation {
            Operation::Deposit { query_id } => self
                .on_op_deposit_query(query_id)
                .await
                .expect("Failed OP: deposit query"),
            Operation::RequestSubscribe => self
                .on_op_request_subscribe()
                .expect("Failed OP: request subscribe"),
        }
    }

    async fn execute_message(&mut self, message: Message) {
        match message {
            Message::Deposit { query_id } => self
                .on_msg_deposit_query(query_id)
                .await
                .expect("Failed MSG: deposit query"),
            Message::Pay { query_id, amount } => self
                .on_msg_pay(query_id, amount)
                .await
                .expect("Failed MSG: pay"),
            Message::Paid { query_id } => {
                self.on_msg_paid(query_id).await.expect("Failed MSG: paid")
            }
            Message::RequestSubscribe => self
                .on_msg_request_subscribe()
                .await
                .expect("Failed MSG: request subscribe"),
            Message::QuotaPrice { amount } => self.on_msg_quota_price(amount).await,
        }
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
        log::info!(
            "Deposit query at runtime chain {} message chain {} to chain {} by owner {}",
            self.runtime.chain_id(),
            self.runtime.message_id().unwrap().chain_id,
            self.runtime.application_id().creation.chain_id,
            self.runtime.authenticated_signer().unwrap()
        );
        let owner = self.runtime.authenticated_signer();
        if !self.state.free_query(owner.expect("Invalid owner")).await? {
            self.runtime
                .prepare_message(Message::Pay {
                    query_id,
                    amount: self.state._quota_price().await,
                })
                .with_authentication()
                .send_to(self.runtime.message_id().unwrap().chain_id);
            return Ok(());
        }
        Ok(self
            .state
            .deposit_query(owner.expect("Invalid owner"), query_id)
            .await?)
    }

    async fn on_op_deposit_query(&mut self, query_id: CryptoHash) -> Result<(), CopilotError> {
        if self
            .state
            .query_deposited(self.runtime.authenticated_signer().unwrap(), query_id)
            .await?
        {
            return Err(CopilotError::InvalidQuery);
        }
        let quota_price = self.state._quota_price().await;
        let owner = self.runtime.authenticated_signer().unwrap();
        if self.runtime.owner_balance(owner).le(&quota_price)
            || self.runtime.chain_balance().le(&quota_price)
        {
            return Err(CopilotError::InsufficientFunds);
        }
        self.state.deposit_query(owner, query_id).await?;
        self.runtime
            .prepare_message(Message::Deposit { query_id })
            .with_authentication()
            .send_to(self.runtime.application_id().creation.chain_id);
        Ok(())
    }

    fn on_op_request_subscribe(&mut self) -> Result<(), CopilotError> {
        self.runtime
            .prepare_message(Message::RequestSubscribe)
            .with_authentication()
            .send_to(self.runtime.application_id().creation.chain_id);
        Ok(())
    }

    // Only in creation chain
    async fn on_msg_deposit_query(&mut self, query_id: CryptoHash) -> Result<(), CopilotError> {
        self.deposit_query(query_id).await
    }

    // Only from creation chain
    async fn on_msg_pay(
        &mut self,
        query_id: CryptoHash,
        amount: Amount,
    ) -> Result<(), CopilotError> {
        log::info!(
            "Pay query at runtime chain {} message chain {} to chain {} by owner {}",
            self.runtime.chain_id(),
            self.runtime.message_id().unwrap().chain_id,
            self.runtime.application_id().creation.chain_id,
            self.runtime.authenticated_signer().unwrap()
        );
        if !self
            .state
            .query_deposited(self.runtime.authenticated_signer().unwrap(), query_id)
            .await?
        {
            return Err(CopilotError::InvalidQuery);
        }
        if self.runtime.message_id().unwrap().chain_id
            != self.runtime.application_id().creation.chain_id
        {
            return Err(CopilotError::InvalidPayChain);
        }
        let destination = Account {
            chain_id: self.runtime.application_id().creation.chain_id,
            owner: None,
        };
        let mut owner = self.runtime.authenticated_signer();
        if owner.is_some() {
            let owner_balance = self.runtime.owner_balance(owner.unwrap());
            if owner_balance.le(&amount) {
                owner = None
            }
        }
        if owner.is_none() {
            let chain_balance = self.runtime.chain_balance();
            if chain_balance.le(&amount) {
                return Err(CopilotError::InsufficientFunds);
            }
        }
        self.runtime.transfer(owner, destination, amount);
        self.runtime
            .prepare_message(Message::Paid { query_id })
            .with_authentication()
            .send_to(self.runtime.application_id().creation.chain_id);
        Ok(())
    }

    async fn on_msg_paid(&mut self, query_id: CryptoHash) -> Result<(), CopilotError> {
        let owner = self.runtime.authenticated_signer();
        Ok(self
            .state
            .deposit_query(owner.expect("Invalid owner"), query_id)
            .await?)
    }

    fn require_message_id(&mut self) -> Result<MessageId, CopilotError> {
        match self.runtime.message_id() {
            Some(message_id) => Ok(message_id),
            None => Err(CopilotError::InvalidMessageId),
        }
    }

    async fn on_msg_request_subscribe(&mut self) -> Result<(), CopilotError> {
        let message_id = self.require_message_id()?;
        // The subscribe message must be from another chain
        if message_id.chain_id == self.runtime.application_id().creation.chain_id {
            return Ok(());
        }
        self.runtime.subscribe(
            message_id.chain_id,
            ChannelName::from(SUBSCRIPTION_CHANNEL.to_vec()),
        );
        self.runtime
            .prepare_message(Message::QuotaPrice {
                amount: self.state._quota_price().await,
            })
            .with_authentication()
            .send_to(self.require_message_id()?.chain_id);
        Ok(())
    }

    async fn on_msg_quota_price(&mut self, amount: Amount) {
        self.state.set_quota_price(amount).await;
    }
}
