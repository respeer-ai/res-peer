// Copyright (c) Zefchain Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use self::state::CPRegistry;
use cp_registry::{
    CPNode, CPRegistryError, CPRegistryResponse, Message, Operation, RegisterParameters,
    UpdateParameters,
};
use linera_sdk::{
    base::{ChannelName, CryptoHash, Destination, MessageId, WithContractAbi},
    views::{RootView, View, ViewStorageContext},
    Contract, ContractRuntime,
};

pub struct CPRegistryContract {
    state: CPRegistry,
    runtime: ContractRuntime<Self>,
}

linera_sdk::contract!(CPRegistryContract);

impl WithContractAbi for CPRegistryContract {
    type Abi = cp_registry::CPRegistryAbi;
}

const SUBSCRIPTION_CHANNEL: &[u8] = b"subscriptions";

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

    async fn instantiate(&mut self, _value: ()) {}

    async fn execute_operation(&mut self, operation: Operation) -> Self::Response {
        match operation {
            Operation::Register { params } => self
                .on_op_register(params)
                .await
                .expect("Failed OP: register"),
            Operation::Update { params } => {
                self.on_op_update(params).await.expect("Failed OP: update")
            }
            Operation::Deregister { node_id } => self
                .on_op_deregister(node_id)
                .await
                .expect("Failed OP: deregister"),
            Operation::RequestSubscribe => self
                .on_op_request_subscribe()
                .expect("Failed OP: request subscribe"),
        }
    }

    async fn execute_message(&mut self, message: Message) {
        match message {
            Message::Register { params } => self
                .on_msg_register(params)
                .await
                .expect("Failed MSG: register"),
            Message::Update { params } => self
                .on_msg_update(params)
                .await
                .expect("Failed MSG: update"),
            Message::Deregister { node_id } => self
                .on_msg_deregister(node_id)
                .await
                .expect("Failed MSG: deregister"),
            Message::RequestSubscribe => self
                .on_msg_request_subscribe()
                .await
                .expect("Failed MSG: request subscribe"),
            Message::ExistNode { node } => self
                .on_msg_exist_node(node)
                .await
                .expect("Failed MSG: exist node"),
        }
    }

    async fn store(mut self) {
        self.state.save().await.expect("Failed to save state");
    }
}

impl CPRegistryContract {
    async fn on_op_register(
        &mut self,
        mut params: RegisterParameters,
    ) -> Result<CPRegistryResponse, CPRegistryError> {
        if self.state.exist_node_with_link(params.clone().link).await? {
            return Err(CPRegistryError::AlreadyRegistered);
        }
        let node: CPNode = params.clone().into();
        params.application_id = self
            .runtime
            .authenticated_caller_id()
            .expect("Invalid applicationId");
        log::info!(
            "OP Register node {} from chain {} link {} application {}",
            node.node_id,
            self.runtime.chain_id(),
            node.link,
            node.application_id,
        );
        if self.state.exist_node_with_id(node.node_id).await? {
            return Err(CPRegistryError::AlreadyRegistered);
        }
        self.runtime
            .prepare_message(Message::Register { params })
            .with_authentication()
            .send_to(self.runtime.application_id().creation.chain_id);
        Ok(CPRegistryResponse::NodeId(node.node_id))
    }

    async fn on_op_update(
        &mut self,
        params: UpdateParameters,
    ) -> Result<CPRegistryResponse, CPRegistryError> {
        if !self.state.exist_node_with_id(params.node_id).await? {
            return Err(CPRegistryError::InvalidNode);
        }
        self.runtime
            .prepare_message(Message::Update { params })
            .with_authentication()
            .send_to(self.runtime.application_id().creation.chain_id);
        Ok(CPRegistryResponse::Ok)
    }

    async fn on_op_deregister(
        &mut self,
        node_id: CryptoHash,
    ) -> Result<CPRegistryResponse, CPRegistryError> {
        if !self.state.exist_node_with_id(node_id).await? {
            return Err(CPRegistryError::InvalidNode);
        }
        self.runtime
            .prepare_message(Message::Deregister { node_id })
            .with_authentication()
            .send_to(self.runtime.application_id().creation.chain_id);
        Ok(CPRegistryResponse::Ok)
    }

    fn on_op_request_subscribe(&mut self) -> Result<CPRegistryResponse, CPRegistryError> {
        self.runtime
            .prepare_message(Message::RequestSubscribe)
            .with_authentication()
            .send_to(self.runtime.application_id().creation.chain_id);
        Ok(CPRegistryResponse::Ok)
    }

    async fn on_msg_register(&mut self, params: RegisterParameters) -> Result<(), CPRegistryError> {
        let node: CPNode = params.clone().into();
        self.state
            .register_cp_node(node, self.runtime.system_time())
            .await?;
        if self.runtime.chain_id() != self.runtime.application_id().creation.chain_id {
            return Ok(());
        }
        let dest = Destination::Subscribers(ChannelName::from(SUBSCRIPTION_CHANNEL.to_vec()));
        self.runtime
            .prepare_message(Message::Register { params })
            .with_authentication()
            .send_to(dest);
        Ok(())
    }

    async fn on_msg_update(&mut self, params: UpdateParameters) -> Result<(), CPRegistryError> {
        self.state.update_cp_node(params.clone()).await?;
        if self.runtime.chain_id() != self.runtime.application_id().creation.chain_id {
            return Ok(());
        }
        let dest = Destination::Subscribers(ChannelName::from(SUBSCRIPTION_CHANNEL.to_vec()));
        self.runtime
            .prepare_message(Message::Update { params })
            .with_authentication()
            .send_to(dest);
        Ok(())
    }

    async fn on_msg_deregister(&mut self, node_id: CryptoHash) -> Result<(), CPRegistryError> {
        self.state.deregister_cp_node(node_id).await?;
        if self.runtime.chain_id() != self.runtime.application_id().creation.chain_id {
            return Ok(());
        }
        let dest = Destination::Subscribers(ChannelName::from(SUBSCRIPTION_CHANNEL.to_vec()));
        self.runtime
            .prepare_message(Message::Deregister { node_id })
            .with_authentication()
            .send_to(dest);
        Ok(())
    }

    fn require_message_id(&mut self) -> Result<MessageId, CPRegistryError> {
        match self.runtime.message_id() {
            Some(message_id) => Ok(message_id),
            None => Err(CPRegistryError::InvalidMessageId),
        }
    }

    async fn on_msg_request_subscribe(&mut self) -> Result<(), CPRegistryError> {
        let message_id = self.require_message_id()?;
        // The subscribe message must be from another chain
        if message_id.chain_id == self.runtime.application_id().creation.chain_id {
            return Ok(());
        }
        self.runtime.subscribe(
            message_id.chain_id,
            ChannelName::from(SUBSCRIPTION_CHANNEL.to_vec()),
        );
        for node in self.state._nodes().await? {
            self.runtime
                .prepare_message(Message::ExistNode { node })
                .with_authentication()
                .send_to(self.require_message_id()?.chain_id);
        }
        Ok(())
    }

    async fn on_msg_exist_node(&mut self, node: CPNode) -> Result<(), CPRegistryError> {
        self.state.insert_cp_node(node).await
    }
}
