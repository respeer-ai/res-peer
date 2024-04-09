#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use std::collections::HashSet;

use self::state::Foundation;
use async_trait::async_trait;
use foundation::{
    FoundationError, FoundationResponse, InitializationArgument, Message, Operation, RewardType,
};
use linera_sdk::{
    base::{Amount, ChannelName, Destination, MessageId, Owner, WithContractAbi},
    Contract, ContractRuntime, ViewStateStorage,
};

pub struct FoundationContract {
    state: Foundation,
    runtime: ContractRuntime<Self>,
}

linera_sdk::contract!(FoundationContract);

const SUBSCRIPTION_CHANNEL: &[u8] = b"subscriptions";

impl WithContractAbi for FoundationContract {
    type Abi = foundation::FoundationAbi;
}

#[async_trait]
impl Contract for FoundationContract {
    type Error = FoundationError;
    type Storage = ViewStateStorage<Self>;
    type State = Foundation;
    type Message = Message;

    async fn new(state: Foundation, runtime: ContractRuntime<Self>) -> Result<Self, Self::Error> {
        Ok(FoundationContract { state, runtime })
    }

    fn state_mut(&mut self) -> &mut Self::State {
        &mut self.state
    }

    async fn initialize(&mut self, state: Self::InitializationArgument) -> Result<(), Self::Error> {
        self.state.initialize_foundation(state).await?;
        Ok(())
    }

    async fn execute_operation(
        &mut self,
        operation: Self::Operation,
    ) -> Result<FoundationResponse, Self::Error> {
        match operation {
            Operation::UserDeposit { amount } => self.on_op_user_deposit(amount),
            Operation::RequestSubscribe => self.on_op_request_subscribe(),
            Operation::ActivityRewards {
                activity_id,
                winner_user,
                voter_users,
                reward_amount,
                voter_reward_percent,
            } => self.on_op_activity_rewards(
                activity_id,
                winner_user,
                voter_users,
                reward_amount,
                voter_reward_percent,
            ),
            Operation::Balance { owner } => self.on_op_balance(owner).await,
            Operation::Deposit { from, amount } => self.on_op_deposit(from, amount),
            Operation::Lock {
                activity_id,
                amount,
            } => self.on_op_lock(activity_id, amount),
            Operation::Reward {
                reward_user,
                reward_type,
                activity_id,
            } => self.on_op_reward(reward_user, reward_type, activity_id),
            Operation::Transfer { from, to, amount } => self.on_op_transfer(from, to, amount),
        }
    }

    async fn execute_message(&mut self, message: Self::Message) -> Result<(), Self::Error> {
        match message {
            Message::InitializationArgument { argument } => {
                self.on_msg_initialization_argument(argument).await
            }
            Message::UserDeposit { amount } => self.on_msg_user_deposit(amount).await,
            Message::RequestSubscribe => self.on_msg_request_subscribe(),
            Message::Deposit { from, amount } => self.on_msg_deposit(from, amount).await,
            Message::Lock {
                activity_id,
                amount,
            } => self.on_msg_lock(activity_id, amount).await,
            Message::Reward {
                reward_user,
                reward_type,
                activity_id,
            } => {
                self.on_msg_reward(reward_user, reward_type, activity_id)
                    .await
            }
            Message::Transfer { from, to, amount } => self.on_msg_transfer(from, to, amount).await,
            Message::ActivityRewards {
                activity_id,
                winner_user,
                voter_users,
                reward_amount,
                voter_reward_percent,
            } => {
                self.on_msg_activity_rewards(
                    activity_id,
                    winner_user,
                    voter_users,
                    reward_amount,
                    voter_reward_percent,
                )
                .await
            }
        }
    }
}

impl FoundationContract {
    async fn _activity_rewards(
        &mut self,
        activity_id: u64,
        winner_user: Owner,
        voter_users: HashSet<Owner>,
        reward_amount: Amount,
        voter_reward_percent: u8,
    ) -> Result<(), FoundationError> {
        self.state
            .spend_activity_funds(activity_id, reward_amount)
            .await?;
        self.state
            .distribute_activity_rewards(
                winner_user,
                voter_users,
                reward_amount,
                voter_reward_percent,
            )
            .await?;
        Ok(())
    }

    fn require_message_id(&mut self) -> Result<MessageId, FoundationError> {
        match self.runtime.message_id() {
            Some(message_id) => Ok(message_id),
            None => Err(FoundationError::InvalidMessageId),
        }
    }

    fn require_authenticated_signer(&mut self) -> Result<Owner, FoundationError> {
        match self.runtime.authenticated_signer() {
            Some(owner) => Ok(owner),
            None => Err(FoundationError::InvalidSigner),
        }
    }

    fn on_op_request_subscribe(&mut self) -> Result<FoundationResponse, FoundationError> {
        self.runtime
            .prepare_message(Message::RequestSubscribe)
            .with_authentication()
            .send_to(self.runtime.application_id().creation.chain_id);
        // TODO: send initialization argument to subscriber
        Ok(FoundationResponse::Ok)
    }

    fn on_op_user_deposit(
        &mut self,
        amount: Amount,
    ) -> Result<FoundationResponse, FoundationError> {
        self.runtime
            .prepare_message(Message::UserDeposit { amount })
            .with_authentication()
            .send_to(self.runtime.application_id().creation.chain_id);
        Ok(FoundationResponse::Ok)
    }

    fn on_op_activity_rewards(
        &mut self,
        activity_id: u64,
        winner_user: Owner,
        voter_users: HashSet<Owner>,
        reward_amount: Amount,
        voter_reward_percent: u8,
    ) -> Result<FoundationResponse, FoundationError> {
        self.runtime
            .prepare_message(Message::ActivityRewards {
                activity_id,
                winner_user,
                voter_users,
                reward_amount,
                voter_reward_percent,
            })
            .with_authentication()
            .send_to(self.runtime.application_id().creation.chain_id);
        Ok(FoundationResponse::Ok)
    }

    async fn on_op_balance(&mut self, owner: Owner) -> Result<FoundationResponse, FoundationError> {
        Ok(FoundationResponse::Balance(
            self.state.balance(owner).await?,
        ))
    }

    fn on_op_deposit(
        &mut self,
        from: Owner,
        amount: Amount,
    ) -> Result<FoundationResponse, FoundationError> {
        self.runtime
            .prepare_message(Message::Deposit { from, amount })
            .with_authentication()
            .send_to(self.runtime.application_id().creation.chain_id);
        Ok(FoundationResponse::Ok)
    }

    fn on_op_lock(
        &mut self,
        activity_id: u64,
        amount: Amount,
    ) -> Result<FoundationResponse, FoundationError> {
        self.runtime
            .prepare_message(Message::Lock {
                activity_id,
                amount,
            })
            .with_authentication()
            .send_to(self.runtime.application_id().creation.chain_id);
        Ok(FoundationResponse::Ok)
    }

    fn on_op_reward(
        &mut self,
        reward_user: Option<Owner>,
        reward_type: RewardType,
        activity_id: Option<u64>,
    ) -> Result<FoundationResponse, FoundationError> {
        self.runtime
            .prepare_message(Message::Reward {
                reward_user,
                reward_type,
                activity_id,
            })
            .with_authentication()
            .send_to(self.runtime.application_id().creation.chain_id);
        Ok(FoundationResponse::Ok)
    }

    fn on_op_transfer(
        &mut self,
        from: Owner,
        to: Owner,
        amount: Amount,
    ) -> Result<FoundationResponse, FoundationError> {
        self.runtime
            .prepare_message(Message::Transfer { from, to, amount })
            .with_authentication()
            .send_to(self.runtime.application_id().creation.chain_id);
        Ok(FoundationResponse::Ok)
    }

    async fn on_msg_initialization_argument(
        &mut self,
        argument: InitializationArgument,
    ) -> Result<(), FoundationError> {
        self.state.initialize_foundation(argument).await
    }

    async fn on_msg_user_deposit(&mut self, amount: Amount) -> Result<(), FoundationError> {
        let user = self.require_authenticated_signer()?;
        self.state.user_deposit(user, amount).await?;
        if self.runtime.chain_id() != self.runtime.application_id().creation.chain_id {
            return Ok(());
        }
        let dest = Destination::Subscribers(ChannelName::from(SUBSCRIPTION_CHANNEL.to_vec()));
        self.runtime
            .prepare_message(Message::UserDeposit { amount })
            .with_authentication()
            .send_to(dest);
        Ok(())
    }

    fn on_msg_request_subscribe(&mut self) -> Result<(), FoundationError> {
        let message_id = self.require_message_id()?;
        // The subscribe message must be from another chain
        if message_id.chain_id == self.runtime.application_id().creation.chain_id {
            return Ok(());
        }
        self.runtime.subscribe(
            message_id.chain_id,
            ChannelName::from(SUBSCRIPTION_CHANNEL.to_vec()),
        );
        Ok(())
    }

    async fn on_msg_deposit(&mut self, from: Owner, amount: Amount) -> Result<(), FoundationError> {
        self.state.deposit(from, amount).await?;
        if self.runtime.chain_id() != self.runtime.application_id().creation.chain_id {
            return Ok(());
        }
        let dest = Destination::Subscribers(ChannelName::from(SUBSCRIPTION_CHANNEL.to_vec()));
        self.runtime
            .prepare_message(Message::Deposit { from, amount })
            .with_authentication()
            .send_to(dest);
        Ok(())
    }

    async fn on_msg_lock(
        &mut self,
        activity_id: u64,
        amount: Amount,
    ) -> Result<(), FoundationError> {
        self.state.lock(activity_id, amount).await?;
        if self.runtime.chain_id() != self.runtime.application_id().creation.chain_id {
            return Ok(());
        }
        let dest = Destination::Subscribers(ChannelName::from(SUBSCRIPTION_CHANNEL.to_vec()));
        self.runtime
            .prepare_message(Message::Lock {
                activity_id,
                amount,
            })
            .with_authentication()
            .send_to(dest);
        Ok(())
    }

    async fn on_msg_reward(
        &mut self,
        reward_user: Option<Owner>,
        reward_type: RewardType,
        activity_id: Option<u64>,
    ) -> Result<(), FoundationError> {
        let reward_user = match reward_type {
            RewardType::Review => self.runtime.authenticated_signer(),
            // TODO: activity reward should be reviewed then here will removed
            RewardType::Activity => self.runtime.authenticated_signer(),
            _ => reward_user,
        };
        let _reward_user = match reward_user {
            Some(user) => user,
            None => return Err(FoundationError::InvalidUser),
        };
        self.state
            .reward(_reward_user, reward_type, activity_id)
            .await?;
        if self.runtime.chain_id() != self.runtime.application_id().creation.chain_id {
            return Ok(());
        }
        let dest = Destination::Subscribers(ChannelName::from(SUBSCRIPTION_CHANNEL.to_vec()));
        self.runtime
            .prepare_message(Message::Reward {
                reward_user,
                reward_type,
                activity_id,
            })
            .with_authentication()
            .send_to(dest);
        Ok(())
    }

    async fn on_msg_transfer(
        &mut self,
        from: Owner,
        to: Owner,
        amount: Amount,
    ) -> Result<(), FoundationError> {
        self.state.transfer(from, to, amount).await?;
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

    async fn on_msg_activity_rewards(
        &mut self,
        activity_id: u64,
        winner_user: Owner,
        voter_users: HashSet<Owner>,
        reward_amount: Amount,
        voter_reward_percent: u8,
    ) -> Result<(), FoundationError> {
        self._activity_rewards(
            activity_id,
            winner_user,
            voter_users.clone(),
            reward_amount,
            voter_reward_percent,
        )
        .await?;
        let dest = Destination::Subscribers(ChannelName::from(SUBSCRIPTION_CHANNEL.to_vec()));
        self.runtime
            .prepare_message(Message::ActivityRewards {
                activity_id,
                winner_user,
                voter_users,
                reward_amount,
                voter_reward_percent,
            })
            .with_authentication()
            .send_to(dest);
        Ok(())
    }
}
