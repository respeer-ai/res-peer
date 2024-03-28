#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use self::state::Credit;
use async_trait::async_trait;
use credit::{CreditAbi, InitializationArgument, Message, Operation};
use linera_sdk::{
    base::{Amount, ApplicationId, ChannelName, Destination, Owner, WithContractAbi},
    Contract, ContractRuntime, ViewStateStorage,
};
use thiserror::Error;

const SUBSCRIPTION_CHANNEL: &[u8] = b"subscriptions";

pub struct CreditContract {
    state: Credit,
    runtime: ContractRuntime<Self>,
}

linera_sdk::contract!(CreditContract);

impl WithContractAbi for Credit {
    type Abi = credit::CreditAbi;
}

#[async_trait]
impl Contract for Credit {
    type Error = ContractError;
    type Storage = ViewStateStorage<Self>;
    type State = Credit;
    type Message = Message;

    async fn new(state: Credit, runtime: ContractRuntime<Self>) -> Result<Self, Self::Error> {
        Ok(CreditContract { state, runtime })
    }

    fn state_mut(&mut self) -> &mut Self::State {
        &mut self.state
    }

    async fn initialize(&mut self, state: InitializationArgument) -> Result<(), Self::Error> {
        self.initialize_credit(state).await;
        Ok(())
    }

    async fn execute_operation(&mut self, operation: Operation) -> Result<(), Self::Error> {
        match operation {
            Operation::Liquidate => self.on_op_liquidate(),
            Operation::SetRewardCallers { application_ids } => {
                self.on_op_set_reward_callers(application_ids)
            }
            Operation::SetTransferCallers { application_ids } => {
                self.on_op_set_transfer_callers(application_ids)
            }
            Operation::Transfer { from, to, amount } => self.on_op_transfer(from, to, amount),
            Operation::TransferExt { to, amount } => self.on_op_transfer_ext(to, amount),
            Operation::RequestSubscribe => self.on_op_request_subscribe(),
        }
    }

    async fn execute_message(&mut self, message: Message) -> Result<(), Self::Error> {
        match message {
            Message::InitializationArgument { argument } => {
                self.on_msg_initialization_argument(argument)
            }
            Message::Liquidate => self.on_msg_liquidate(),
            Message::Reward { owner, amount } => self.on_msg_reward(owner, amount),
            Message::SetRewardCallers { application_ids } => {
                self.on_msg_set_reward_callers(application_ids)
            }
            Message::SetTransferCallers { application_ids } => {
                self.on_msg_set_transfer_callers(application_ids)
            }
            Message::Transfer { from, to, amount } => self.on_msg_transfer(from, to, amount),
            Message::TransferExt { to, amount } => self.on_msg_transfer_ext(to, amount),
            Message::RequestSubscribe => self.on_msg_request_subscribe(),
        }
    }
}

impl CreditContract {
    fn on_op_liquidate(&mut self) -> Result<(), ContractError> {
        self.runtime
            .prepare_message(Message::Liquidate)
            .with_authentication()
            .send_to(self.runtime.application_id().creation.chain_id);
        Ok(())
    }

    fn on_op_set_reward_callers(
        &mut self,
        application_ids: Vec<ApplicationId>,
    ) -> Result<(), ContractError> {
        if self.runtime.chain_id() != self.runtime.application_id().creation.chain_id {
            return Err(ContractError::OperationNotAllowed);
        }
        self.runtime
            .prepare_message(Message::SetRewardCallers { application_ids })
            .with_authentication()
            .send_to(self.runtime.application_id().creation.chain_id);
        Ok(())
    }

    fn on_op_set_transfer_callers(
        &mut self,
        application_ids: Vec<ApplicationId>,
    ) -> Result<(), ContractError> {
        self.runtime
            .prepare_message(Message::SetTransferCallers { application_ids })
            .with_authentication()
            .send_to(self.runtime.application_id().creation.chain_id);
        Ok(())
    }

    fn on_op_transfer(
        &mut self,
        from: Owner,
        to: Owner,
        amount: Amount,
    ) -> Result<(), ContractError> {
        self.runtime
            .prepare_message(Message::Transfer { from, to, amount })
            .with_authentication()
            .send_to(self.runtime.application_id().creation.chain_id);
        Ok(())
    }

    fn on_op_transfer_ext(&mut self, to: Owner, amount: Amount) -> Result<(), ContractError> {
        self.runtime
            .prepare_message(Message::TransferExt { to, amount })
            .with_authentication()
            .send_to(self.runtime.application_id().creation.chain_id);
        Ok(())
    }

    fn on_op_request_subscribe(&mut self) -> Result<(), ContractError> {
        self.runtime
            .prepare_message(Message::RequestSubscribe)
            .with_authentication()
            .send_to(self.runtime.application_id().creation.chain_id);
        Ok(())
    }

    fn on_msg_initialization_argument(
        &mut self,
        arg: InitializationArgument,
    ) -> Result<(), ContractError> {
        self.initialize_credit(arg).await;
        Ok(())
    }

    fn on_msg_liquidate(&mut self) -> Result<(), ContractError> {
        self.liquidate().await;
        if self.runtime.chain_id() != self.runtime.application_id().creation.chain_id {
            Ok(())
        }
        let dest = Destination::Subscribers(ChannelName::from(SUBSCRIPTION_CHANNEL.to_vec()));
        self.runtime
            .prepare_message(Message::Liquidate)
            .with_authentication()
            .send_to(dest);
        Ok(())
    }

    fn on_msg_reward(&mut self, owner: Owner, amount: Amount) -> Result<(), ContractError> {
        self.reward(owner, amount).await?;
        if self.runtime.chain_id() != self.runtime.application_id().creation.chain_id {
            Ok(())
        }
        let dest = Destination::Subscribers(ChannelName::from(SUBSCRIPTION_CHANNEL.to_vec()));
        self.runtime
            .prepare_message(Message::Reward { owner, amount })
            .with_authentication()
            .send_to(dest);
        Ok(())
    }

    fn on_msg_set_reward_callers(
        &mut self,
        application_ids: Vec<ApplicationId>,
    ) -> Result<(), ContractError> {
        if self.runtime.message_id()?.chain_id != self.runtime.application_id().creation.chain_id {
            Err(ContractError::OperationNotAllowed)
        }
        self.set_reward_callers(application_ids.clone()).await;
        if self.runtime.chain_id() != self.runtime.application_id().creation.chain_id {
            Ok(())
        }
        let dest = Destination::Subscribers(ChannelName::from(SUBSCRIPTION_CHANNEL.to_vec()));
        self.runtime
            .prepare_message(Message::SetRewardCallers { application_ids })
            .with_authentication()
            .send_to(dest);
        Ok(())
    }

    fn on_msg_set_transfer_callers(
        &mut self,
        application_ids: Vec<ApplicationId>,
    ) -> Result<(), ContractError> {
        if self.runtime.message_id()?.chain_id != self.runtime.application_id().creation.chain_id {
            Err(ContractError::OperationNotAllowed)
        }
        self.set_transfer_callers(application_ids.clone()).await;
        if self.runtime.chain_id() != self.runtime.application_id().creation.chain_id {
            Ok(())
        }
        let dest = Destination::Subscribers(ChannelName::from(SUBSCRIPTION_CHANNEL.to_vec()));
        self.runtime
            .prepare_message(Message::SetTransferCallers { application_ids })
            .with_authentication()
            .send_to(dest);
        Ok(())
    }

    fn on_msg_transfer(
        &mut self,
        from: Owner,
        to: Owner,
        amount: Amount,
    ) -> Result<(), ContractError> {
        self.transfer(from, to, amount).await?;
        if self.runtime.chain_id() != self.runtime.application_id().creation.chain_id {
            Ok(())
        }
        let dest = Destination::Subscribers(ChannelName::from(SUBSCRIPTION_CHANNEL.to_vec()));
        self.runtime
            .prepare_message(Message::Transfer { from, to, amount })
            .with_authentication()
            .send_to(dest);
        Ok(())
    }

    fn on_msg_transfer_ext(&mut self, to: Owner, amount: Amount) -> Result<(), ContractError> {
        self.transfer(context.authenticated_signer.unwrap(), to, amount)
            .await?;
        if self.runtime.chain_id() != self.runtime.application_id().creation.chain_id {
            Ok(())
        }
        let dest = Destination::Subscribers(ChannelName::from(SUBSCRIPTION_CHANNEL.to_vec()));
        self.runtime
            .prepare_message(Message::TransferExt { to, amount })
            .with_authentication()
            .send_to(dest);
        Ok(())
    }

    fn on_msg_request_subscribe(&mut self) -> Result<(), ContractError> {
        if self.runtime.message_id()?.chain_id != self.runtime.application_id().creation.chain_id {
            Ok(())
        }
        self.runtime.subscribe(
            self.runtime.message_id()?.chain_id,
            ChannelName::from(SUBSCRIPTION_CHANNEL.to_vec()),
        );
        self.runtime
            .prepare_message(Message::InitializationArgument {
                argument: self.state.initialization_argument().await?,
            })
            .with_authentication()
            .send_to(self.runtime.message_id()?.chain_id);
        Ok(())
    }
}

/// An error that can occur during the contract execution.
#[derive(Debug, Error)]
pub enum ContractError {
    /// Failed to deserialize BCS bytes
    #[error("Failed to deserialize BCS bytes")]
    BcsError(#[from] bcs::Error),

    /// Failed to deserialize JSON string
    #[error("Failed to deserialize JSON string")]
    JsonError(#[from] serde_json::Error),
    // Add more error variants here.
    #[error(transparent)]
    StateError(#[from] state::StateError),

    #[error("NOT IMPLEMENTED")]
    NotImplemented,

    #[error("Caller not allowed")]
    CallerNotAllowed,

    #[error("Operation not allowed")]
    OperationNotAllowed,

    #[error("Cross-application sessions not supported")]
    SessionsNotSupported,
}
