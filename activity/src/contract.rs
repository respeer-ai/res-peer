#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use std::collections::HashSet;

use self::state::Activity;
use activity::{
    ActivityError, AnnounceParams, CreateParams, Message, Operation, UpdateParams, VoteType,
};
use async_trait::async_trait;
use feed::{FeedAbi, FeedResponse};
use foundation::FoundationAbi;
use linera_sdk::{
    base::{Amount, ApplicationId, ChannelName, Destination, MessageId, Owner, WithContractAbi},
    Contract, ContractRuntime, ViewStateStorage,
};
use review::ReviewAbi;

pub struct ActivityContract {
    state: Activity,
    runtime: ContractRuntime<Self>,
}

linera_sdk::contract!(ActivityContract);

impl WithContractAbi for ActivityContract {
    type Abi = activity::ActivityAbi;
}

const SUBSCRIPTION_CHANNEL: &[u8] = b"subscriptions";

#[async_trait]
impl Contract for ActivityContract {
    type Error = ActivityError;
    type Storage = ViewStateStorage<Self>;
    type State = Activity;
    type Message = Message;

    async fn new(state: Activity, runtime: ContractRuntime<Self>) -> Result<Self, Self::Error> {
        Ok(ActivityContract { state, runtime })
    }

    fn state_mut(&mut self) -> &mut Self::State {
        &mut self.state
    }

    async fn initialize(
        &mut self,
        _argument: Self::InitializationArgument,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    async fn execute_operation(&mut self, operation: Self::Operation) -> Result<(), Self::Error> {
        match operation {
            Operation::Create { params } => self.on_op_create(params),
            Operation::Update { params } => self.on_op_update(params),
            Operation::Register {
                activity_id,
                object_id,
            } => self.on_op_register(activity_id, object_id),
            Operation::Vote {
                activity_id,
                object_id,
            } => self.on_op_vote(activity_id, object_id),
            Operation::Announce { params } => self.on_op_announce(params),
            Operation::RequestSubscribe => self.on_op_request_subscribe(),
            Operation::Finalize { activity_id } => self.on_op_finalize(activity_id).await,
        }
    }

    async fn execute_message(&mut self, message: Self::Message) -> Result<(), Self::Error> {
        match message {
            Message::Create { params } => self.on_msg_create(params).await,
            Message::Update { params } => self.on_msg_update(params).await,
            Message::Register {
                activity_id,
                object_id,
            } => self.on_msg_register(activity_id, object_id).await,
            Message::Vote {
                activity_id,
                object_id,
            } => self.on_msg_vote(activity_id, object_id).await,
            Message::Announce { params } => self.on_msg_announce(params).await,
            Message::RequestSubscribe => self.on_msg_request_subscribe(),
            Message::Finalize { activity_id } => self.on_msg_finalize(activity_id).await,
        }
    }
}

impl ActivityContract {
    fn review_app_id(&mut self) -> ApplicationId<ReviewAbi> {
        self.runtime.application_parameters().review_app_id
    }

    fn foundation_app_id(&mut self) -> ApplicationId<FoundationAbi> {
        self.runtime.application_parameters().foundation_app_id
    }

    fn feed_app_id(&mut self) -> ApplicationId<FeedAbi> {
        self.runtime.application_parameters().feed_app_id
    }

    async fn create_announcement(&mut self, params: AnnounceParams) -> Result<(), ActivityError> {
        let call = review::Operation::SubmitContent {
            cid: params.cid,
            title: params.title,
            content: params.content,
        };
        let review_app_id = self.review_app_id();
        self.runtime.call_application(true, review_app_id, &call);
        Ok(())
    }

    async fn account_balance(&mut self, owner: Owner) -> Result<Amount, ActivityError> {
        let call = foundation::Operation::Balance { owner };
        let foundation_app_id = self.foundation_app_id();
        Ok(self
            .runtime
            .call_application(true, foundation_app_id, &call))
    }

    async fn _create_activity(
        &mut self,
        owner: Owner,
        params: CreateParams,
    ) -> Result<(), ActivityError> {
        let activity_id = self
            .state
            .create_activity(owner, params.clone(), self.runtime.system_time())
            .await?;
        let call = review::Operation::SubmitActivity {
            activity_id,
            activity_host: owner,
            budget_amount: params.budget_amount,
        };
        let review_app_id = self.review_app_id();
        let _ = self.runtime.call_application(true, review_app_id, &call);
        Ok(())
    }

    async fn activity_approved(&mut self, activity_id: u64) -> Result<bool, ActivityError> {
        let call = review::Operation::ActivityApproved { activity_id };
        let review_app_id = self.review_app_id();
        Ok(self.runtime.call_application(true, review_app_id, &call))
    }

    async fn content_author(&mut self, cid: String) -> Result<Owner, ActivityError> {
        let call = feed::Operation::ContentAuthor { cid };
        let feed_app_id = self.feed_app_id();
        match self.runtime.call_application(true, feed_app_id, &call) {
            FeedResponse::ContentAuthor(Some(author)) => Ok(author),
            _ => Err(ActivityError::InvalidContentAuthor),
        }
    }

    async fn activity_rewards(
        &mut self,
        activity_id: u64,
        winner_user: Owner,
        voter_users: HashSet<Owner>,
        reward_amount: Amount,
        voter_reward_percent: u8,
    ) -> Result<(), ActivityError> {
        let call = foundation::Operation::ActivityRewards {
            activity_id,
            winner_user,
            voter_users,
            reward_amount,
            voter_reward_percent,
        };
        let foundation_app_id = self.foundation_app_id();
        let _ = self
            .runtime
            .call_application(true, foundation_app_id, &call);
        Ok(())
    }

    async fn reward_activity_host(&mut self, activity_id: u64) -> Result<(), ActivityError> {
        let call = foundation::Operation::Reward {
            reward_user: None,
            reward_type: foundation::RewardType::Activity,
            activity_id: Some(activity_id),
        };
        let foundation_app_id = self.foundation_app_id();
        let _ = self
            .runtime
            .call_application(true, foundation_app_id, &call);
        Ok(())
    }

    async fn _finalize(&mut self, activity_id: u64) -> Result<(), ActivityError> {
        self.state.finalize(activity_id).await?;
        let activity = self.state.activity(activity_id).await?;
        for winner in activity.winners {
            let author = self.content_author(winner.clone().object_id).await?;
            let voter_users = activity.voters.get(&winner.object_id).unwrap().clone();
            let index = match activity
                .prize_configs
                .iter()
                .position(|r| r.place == winner.place)
            {
                Some(index) => index,
                _ => return Err(ActivityError::InvalidPrizeConfig),
            };
            let reward_amount = match activity.prize_configs[index].reward_amount {
                Some(amount) => amount,
                _ => Amount::ZERO,
            };
            self.activity_rewards(
                activity_id,
                author,
                voter_users,
                reward_amount,
                activity.voter_reward_percent,
            )
            .await?;
        }
        self.reward_activity_host(activity_id).await?;
        Ok(())
    }

    fn require_authenticated_signer(&mut self) -> Result<Owner, ActivityError> {
        match self.runtime.authenticated_signer() {
            Some(owner) => Ok(owner),
            None => Err(ActivityError::InvalidSigner),
        }
    }

    fn require_message_id(&mut self) -> Result<MessageId, ActivityError> {
        match self.runtime.message_id() {
            Some(message_id) => Ok(message_id),
            None => Err(ActivityError::InvalidMessageId),
        }
    }

    fn on_op_create(&mut self, params: CreateParams) -> Result<(), ActivityError> {
        self.runtime
            .prepare_message(Message::Create { params })
            .with_authentication()
            .send_to(self.runtime.application_id().creation.chain_id);
        Ok(())
    }

    fn on_op_update(&mut self, params: UpdateParams) -> Result<(), ActivityError> {
        self.runtime
            .prepare_message(Message::Update { params })
            .with_authentication()
            .send_to(self.runtime.application_id().creation.chain_id);
        Ok(())
    }

    fn on_op_register(&mut self, activity_id: u64, object_id: String) -> Result<(), ActivityError> {
        self.runtime
            .prepare_message(Message::Register {
                activity_id,
                object_id,
            })
            .with_authentication()
            .send_to(self.runtime.application_id().creation.chain_id);
        Ok(())
    }

    fn on_op_vote(&mut self, activity_id: u64, object_id: String) -> Result<(), ActivityError> {
        self.runtime
            .prepare_message(Message::Vote {
                activity_id,
                object_id,
            })
            .with_authentication()
            .send_to(self.runtime.application_id().creation.chain_id);
        Ok(())
    }

    fn on_op_announce(&mut self, params: AnnounceParams) -> Result<(), ActivityError> {
        self.runtime
            .prepare_message(Message::Announce { params })
            .with_authentication()
            .send_to(self.runtime.application_id().creation.chain_id);
        Ok(())
    }

    fn on_op_request_subscribe(&mut self) -> Result<(), ActivityError> {
        self.runtime
            .prepare_message(Message::RequestSubscribe)
            .with_authentication()
            .send_to(self.runtime.application_id().creation.chain_id);
        Ok(())
    }

    async fn on_op_finalize(&mut self, activity_id: u64) -> Result<(), ActivityError> {
        let activity = self.state.activity(activity_id).await?;
        if Some(activity.host) != self.runtime.authenticated_signer() {
            return Err(ActivityError::NotActivityHost);
        }
        self.runtime
            .prepare_message(Message::Finalize { activity_id })
            .with_authentication()
            .send_to(self.runtime.application_id().creation.chain_id);
        Ok(())
    }

    async fn on_msg_create(&mut self, params: CreateParams) -> Result<(), ActivityError> {
        let owner = self.require_authenticated_signer()?;
        self._create_activity(owner, params.clone()).await?;
        if self.runtime.chain_id() != self.runtime.application_id().creation.chain_id {
            return Ok(());
        }
        let dest = Destination::Subscribers(ChannelName::from(SUBSCRIPTION_CHANNEL.to_vec()));
        self.runtime
            .prepare_message(Message::Create { params })
            .with_authentication()
            .send_to(dest);
        Ok(())
    }

    async fn on_msg_update(&mut self, params: UpdateParams) -> Result<(), ActivityError> {
        if self.require_authenticated_signer()?
            != self.state.activity(params.activity_id).await?.host
        {
            return Err(ActivityError::InvalidSigner);
        }
        self.state.update_activity(params.clone()).await?;
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

    async fn on_msg_register(
        &mut self,
        activity_id: u64,
        object_id: String,
    ) -> Result<(), ActivityError> {
        self.state.register(activity_id, object_id.clone()).await?;
        if self.runtime.chain_id() != self.runtime.application_id().creation.chain_id {
            return Ok(());
        }
        let dest = Destination::Subscribers(ChannelName::from(SUBSCRIPTION_CHANNEL.to_vec()));
        self.runtime
            .prepare_message(Message::Register {
                activity_id,
                object_id,
            })
            .with_authentication()
            .send_to(dest);
        Ok(())
    }

    async fn on_msg_vote(
        &mut self,
        activity_id: u64,
        object_id: String,
    ) -> Result<(), ActivityError> {
        match self.activity_approved(activity_id).await {
            Ok(true) => {}
            Ok(false) => return Err(ActivityError::ActivityNotApproved),
            Err(err) => return Err(err),
        }
        match self.state.votable(activity_id).await {
            Ok(true) => {}
            Ok(false) => return Err(ActivityError::ActivityNotVotable),
            Err(err) => return Err(err),
        }
        let owner = self.require_authenticated_signer()?;
        let balance = self.account_balance(owner).await?;
        let activity = self.state.activity(activity_id).await?;
        let power = match activity.vote_type {
            VoteType::Power => balance,
            VoteType::Account => Amount::ONE,
        };
        if power.eq(&Amount::ZERO) {
            return Err(ActivityError::AccountBalanceRequired);
        }
        self.state
            .vote(owner, activity_id, object_id.clone(), power)
            .await?;
        if self.runtime.chain_id() != self.runtime.application_id().creation.chain_id {
            return Ok(());
        }
        let dest = Destination::Subscribers(ChannelName::from(SUBSCRIPTION_CHANNEL.to_vec()));
        self.runtime
            .prepare_message(Message::Register {
                activity_id,
                object_id,
            })
            .with_authentication()
            .send_to(dest);
        Ok(())
    }

    async fn on_msg_announce(&mut self, params: AnnounceParams) -> Result<(), ActivityError> {
        self.create_announcement(params.clone()).await?;
        self.state
            .announce(
                params.activity_id,
                params.cid.clone(),
                params.announce_prize,
            )
            .await?;
        if self.runtime.chain_id() != self.runtime.application_id().creation.chain_id {
            return Ok(());
        }
        let dest = Destination::Subscribers(ChannelName::from(SUBSCRIPTION_CHANNEL.to_vec()));
        self.runtime
            .prepare_message(Message::Announce { params })
            .with_authentication()
            .send_to(dest);
        Ok(())
    }

    fn on_msg_request_subscribe(&mut self) -> Result<(), ActivityError> {
        if self.require_message_id()?.chain_id != self.runtime.application_id().creation.chain_id {
            return Ok(());
        }
        let message_id = self.require_message_id()?;
        self.runtime.subscribe(
            message_id.chain_id,
            ChannelName::from(SUBSCRIPTION_CHANNEL.to_vec()),
        );
        Ok(())
    }

    async fn on_msg_finalize(&mut self, activity_id: u64) -> Result<(), ActivityError> {
        self._finalize(activity_id).await?;
        if self.runtime.chain_id() != self.runtime.application_id().creation.chain_id {
            return Ok(());
        }
        let dest = Destination::Subscribers(ChannelName::from(SUBSCRIPTION_CHANNEL.to_vec()));
        self.runtime
            .prepare_message(Message::Finalize { activity_id })
            .with_authentication()
            .send_to(dest);
        Ok(())
    }
}
