#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use self::state::Credit;
use async_trait::async_trait;
use credit::{CreditAbi, CreditError, InitializationArgument, Message, Operation};
use linera_sdk::{
    base::{Amount, ApplicationId, ChannelName, Destination, MessageId, Owner, WithContractAbi},
    Contract, ContractRuntime, ViewStateStorage,
};

const SUBSCRIPTION_CHANNEL: &[u8] = b"subscriptions";

pub struct CreditContract {
    state: Credit,
    runtime: ContractRuntime<Self>,
}

linera_sdk::contract!(CreditContract);

impl WithContractAbi for CreditContract {
    type Abi = CreditAbi;
}

#[async_trait]
impl Contract for CreditContract {
    type Error = CreditError;
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
        self.state.initialize_credit(state).await;
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
            Operation::Reward { owner, amount } => self.on_op_reward(owner, amount),
        }
    }

    async fn execute_message(&mut self, message: Message) -> Result<(), Self::Error> {
        match message {
            Message::InitializationArgument { argument } => {
                self.on_msg_initialization_argument(argument).await
            }
            Message::Liquidate => self.on_msg_liquidate().await,
            Message::Reward { owner, amount } => self.on_msg_reward(owner, amount).await,
            Message::SetRewardCallers { application_ids } => {
                self.on_msg_set_reward_callers(application_ids).await
            }
            Message::SetTransferCallers { application_ids } => {
                self.on_msg_set_transfer_callers(application_ids).await
            }
            Message::Transfer { from, to, amount } => self.on_msg_transfer(from, to, amount).await,
            Message::TransferExt { to, amount } => self.on_msg_transfer_ext(to, amount).await,
            Message::RequestSubscribe => self.on_msg_request_subscribe().await,
        }
    }
}

impl CreditContract {
    fn require_message_id(&mut self) -> Result<MessageId, CreditError> {
        match self.runtime.message_id() {
            Some(message_id) => Ok(message_id),
            None => Err(CreditError::InvalidMessageId),
        }
    }

    fn require_authenticated_signer(&mut self) -> Result<Owner, CreditError> {
        match self.runtime.authenticated_signer() {
            Some(owner) => Ok(owner),
            None => Err(CreditError::InvalidSigner),
        }
    }

    fn on_op_liquidate(&mut self) -> Result<(), CreditError> {
        self.runtime
            .prepare_message(Message::Liquidate)
            .with_authentication()
            .send_to(self.runtime.application_id().creation.chain_id);
        Ok(())
    }

    fn on_op_set_reward_callers(
        &mut self,
        application_ids: Vec<ApplicationId>,
    ) -> Result<(), CreditError> {
        if self.runtime.chain_id() != self.runtime.application_id().creation.chain_id {
            return Err(CreditError::OperationNotAllowed);
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
    ) -> Result<(), CreditError> {
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
    ) -> Result<(), CreditError> {
        self.runtime
            .prepare_message(Message::Transfer { from, to, amount })
            .with_authentication()
            .send_to(self.runtime.application_id().creation.chain_id);
        Ok(())
    }

    fn on_op_transfer_ext(&mut self, to: Owner, amount: Amount) -> Result<(), CreditError> {
        self.runtime
            .prepare_message(Message::TransferExt { to, amount })
            .with_authentication()
            .send_to(self.runtime.application_id().creation.chain_id);
        Ok(())
    }

    fn on_op_request_subscribe(&mut self) -> Result<(), CreditError> {
        self.runtime
            .prepare_message(Message::RequestSubscribe)
            .with_authentication()
            .send_to(self.runtime.application_id().creation.chain_id);
        Ok(())
    }

    fn on_op_reward(&mut self, owner: Owner, amount: Amount) -> Result<(), CreditError> {
        self.runtime
            .prepare_message(Message::Reward { owner, amount })
            .with_authentication()
            .send_to(self.runtime.application_id().creation.chain_id);
        Ok(())
    }

    async fn on_msg_initialization_argument(
        &mut self,
        arg: InitializationArgument,
    ) -> Result<(), CreditError> {
        self.state.initialize_credit(arg).await;
        Ok(())
    }

    async fn on_msg_liquidate(&mut self) -> Result<(), CreditError> {
        self.state.liquidate(self.runtime.system_time()).await;
        if self.runtime.chain_id() != self.runtime.application_id().creation.chain_id {
            return Ok(());
        }
        let dest = Destination::Subscribers(ChannelName::from(SUBSCRIPTION_CHANNEL.to_vec()));
        self.runtime
            .prepare_message(Message::Liquidate)
            .with_authentication()
            .send_to(dest);
        Ok(())
    }

    async fn on_msg_reward(&mut self, owner: Owner, amount: Amount) -> Result<(), CreditError> {
        self.state
            .reward(owner, amount, self.runtime.system_time())
            .await?;
        if self.runtime.chain_id() != self.runtime.application_id().creation.chain_id {
            return Ok(());
        }
        let dest = Destination::Subscribers(ChannelName::from(SUBSCRIPTION_CHANNEL.to_vec()));
        self.runtime
            .prepare_message(Message::Reward { owner, amount })
            .with_authentication()
            .send_to(dest);
        Ok(())
    }

    async fn on_msg_set_reward_callers(
        &mut self,
        application_ids: Vec<ApplicationId>,
    ) -> Result<(), CreditError> {
        if self.require_message_id()?.chain_id != self.runtime.application_id().creation.chain_id {
            return Err(CreditError::OperationNotAllowed);
        }
        self.state.set_reward_callers(application_ids.clone()).await;
        if self.runtime.chain_id() != self.runtime.application_id().creation.chain_id {
            return Ok(());
        }
        let dest = Destination::Subscribers(ChannelName::from(SUBSCRIPTION_CHANNEL.to_vec()));
        self.runtime
            .prepare_message(Message::SetRewardCallers { application_ids })
            .with_authentication()
            .send_to(dest);
        Ok(())
    }

    async fn on_msg_set_transfer_callers(
        &mut self,
        application_ids: Vec<ApplicationId>,
    ) -> Result<(), CreditError> {
        if self.require_message_id()?.chain_id != self.runtime.application_id().creation.chain_id {
            return Err(CreditError::OperationNotAllowed);
        }
        self.state
            .set_transfer_callers(application_ids.clone())
            .await;
        if self.runtime.chain_id() != self.runtime.application_id().creation.chain_id {
            return Ok(());
        }
        let dest = Destination::Subscribers(ChannelName::from(SUBSCRIPTION_CHANNEL.to_vec()));
        self.runtime
            .prepare_message(Message::SetTransferCallers { application_ids })
            .with_authentication()
            .send_to(dest);
        Ok(())
    }

    async fn on_msg_transfer(
        &mut self,
        from: Owner,
        to: Owner,
        amount: Amount,
    ) -> Result<(), CreditError> {
        self.state
            .transfer(from, to, amount, self.runtime.system_time())
            .await?;
        if self.runtime.chain_id() != self.runtime.application_id().creation.chain_id {
            return Ok(());
        }
        let dest = Destination::Subscribers(ChannelName::from(SUBSCRIPTION_CHANNEL.to_vec()));
        self.runtime
            .prepare_message(Message::Transfer { from, to, amount })
            .with_authentication()
            .send_to(dest);
        Ok(())
    }

    async fn on_msg_transfer_ext(&mut self, to: Owner, amount: Amount) -> Result<(), CreditError> {
        let from = self.require_authenticated_signer()?;
        self.state
            .transfer(from, to, amount, self.runtime.system_time())
            .await?;
        if self.runtime.chain_id() != self.runtime.application_id().creation.chain_id {
            return Ok(());
        }
        let dest = Destination::Subscribers(ChannelName::from(SUBSCRIPTION_CHANNEL.to_vec()));
        self.runtime
            .prepare_message(Message::TransferExt { to, amount })
            .with_authentication()
            .send_to(dest);
        Ok(())
    }

    async fn on_msg_request_subscribe(&mut self) -> Result<(), CreditError> {
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
            .prepare_message(Message::InitializationArgument {
                argument: self.state.initialization_argument().await?,
            })
            .with_authentication()
            .send_to(self.require_message_id()?.chain_id);
        Ok(())
    }
}
