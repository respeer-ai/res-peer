#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use std::collections::HashMap;

use self::state::Review;
use async_trait::async_trait;
use credit::CreditAbi;
use feed::FeedAbi;
use foundation::FoundationAbi;
use linera_sdk::{
    base::{
        Amount, ApplicationId, ChainId, ChannelName, Destination, Owner, SessionId, WithContractAbi,
    },
    contract::system_api,
    ApplicationCallResult, CalleeContext, Contract, ExecutionResult, MessageContext,
    OperationContext, SessionCallResult, ViewStateStorage,
};
use linera_views::views::ViewError;
use review::{Content, InitialState, Message, Operation};
use thiserror::Error;

linera_sdk::contract!(Review);

impl WithContractAbi for Review {
    type Abi = review::ReviewAbi;
}

const SUBSCRIPTION_CHANNEL: &[u8] = b"subscriptions";

#[async_trait]
impl Contract for Review {
    type Error = ContractError;
    type Storage = ViewStateStorage<Self>;

    async fn initialize(
        &mut self,
        _context: &OperationContext,
        state: Self::InitializationArgument,
    ) -> Result<ExecutionResult<Self::Message>, Self::Error> {
        self._initialize(state).await?;
        return Ok(ExecutionResult::default().with_authenticated_message(
            system_api::current_application_id().creation.chain_id,
            Message::GenesisReviewer,
        ));
    }

    async fn execute_operation(
        &mut self,
        context: &OperationContext,
        operation: Self::Operation,
    ) -> Result<ExecutionResult<Self::Message>, Self::Error> {
        match operation {
            Operation::ApplyReviewer { resume } => {
                return Ok(ExecutionResult::default().with_authenticated_message(
                    system_api::current_application_id().creation.chain_id,
                    Message::ApplyReviewer { resume },
                ));
            }
            Operation::UpdateReviewerResume { resume } => {
                return Ok(ExecutionResult::default().with_authenticated_message(
                    system_api::current_application_id().creation.chain_id,
                    Message::ApplyReviewer { resume },
                ));
            }
            Operation::ApproveReviewer { candidate } => {
                return Ok(ExecutionResult::default().with_authenticated_message(
                    system_api::current_application_id().creation.chain_id,
                    Message::ApproveReviewer { candidate },
                ));
            }
            Operation::RejectReviewer { candidate } => {
                return Ok(ExecutionResult::default().with_authenticated_message(
                    system_api::current_application_id().creation.chain_id,
                    Message::RejectReviewer { candidate },
                ));
            }
            Operation::SubmitContent {
                cid,
                title,
                content,
            } => {
                return Ok(ExecutionResult::default().with_authenticated_message(
                    system_api::current_application_id().creation.chain_id,
                    Message::SubmitContent {
                        cid,
                        title,
                        content,
                    },
                ));
            }
            Operation::ApproveContent {
                content_cid,
                reason_cid,
                reason,
            } => {
                return Ok(ExecutionResult::default().with_authenticated_message(
                    system_api::current_application_id().creation.chain_id,
                    Message::ApproveContent {
                        content_cid,
                        reason_cid,
                        reason,
                    },
                ));
            }
            Operation::RejectContent {
                content_cid,
                reason,
            } => {
                return Ok(ExecutionResult::default().with_authenticated_message(
                    system_api::current_application_id().creation.chain_id,
                    Message::RejectContent {
                        content_cid,
                        reason,
                    },
                ));
            }
            Operation::ApproveAsset {
                collection_id,
                reason_cid,
                reason,
            } => {
                self._approve_asset(
                    context.authenticated_signer.unwrap(),
                    collection_id,
                    reason_cid,
                    reason,
                )
                .await?;
            }
            Operation::RejectAsset {
                collection_id,
                reason,
            } => {
                self._reject_asset(context.authenticated_signer.unwrap(), collection_id, reason)
                    .await?;
            }
            Operation::RequestSubscribe => {
                return Ok(ExecutionResult::default().with_message(
                    system_api::current_application_id().creation.chain_id,
                    Message::RequestSubscribe,
                ));
            }
        }
        Ok(ExecutionResult::default())
    }

    async fn execute_message(
        &mut self,
        context: &MessageContext,
        message: Self::Message,
    ) -> Result<ExecutionResult<Self::Message>, Self::Error> {
        match message {
            Message::GenesisReviewer {} => {
                let chain_id = context.chain_id;
                let reviewer = context.authenticated_signer.unwrap();
                self.genesis_reviewer(chain_id, reviewer).await?;
                let dest =
                    Destination::Subscribers(ChannelName::from(SUBSCRIPTION_CHANNEL.to_vec()));
                return Ok(ExecutionResult::default()
                    .with_authenticated_message(dest, Message::GenesisReviewer));
            }
            Message::ExistReviewer { reviewer } => {
                self.add_exist_reviewer(reviewer).await?;
            }
            Message::ApplyReviewer { resume } => {
                let candidate = context.authenticated_signer.unwrap();
                self._apply_reviewer(context.chain_id, candidate, resume.clone())
                    .await?;
                // TODO: broadcast to other chains
                let dest =
                    Destination::Subscribers(ChannelName::from(SUBSCRIPTION_CHANNEL.to_vec()));
                return Ok(ExecutionResult::default()
                    .with_authenticated_message(dest, Message::ApplyReviewer { resume }));
            }
            Message::UpdateReviewerResume { resume } => {
                let candidate = context.authenticated_signer.unwrap();
                self._update_reviewer_resume(candidate, resume.clone())
                    .await?;
                // TODO: broadcast to other chains
                let dest =
                    Destination::Subscribers(ChannelName::from(SUBSCRIPTION_CHANNEL.to_vec()));
                return Ok(ExecutionResult::default()
                    .with_authenticated_message(dest, Message::UpdateReviewerResume { resume }));
            }
            Message::ApproveReviewer { candidate } => {
                self._approve_reviewer(context.authenticated_signer.unwrap(), candidate)
                    .await?;
                // TODO: broadcast to other chains
                let dest =
                    Destination::Subscribers(ChannelName::from(SUBSCRIPTION_CHANNEL.to_vec()));
                return Ok(ExecutionResult::default()
                    .with_authenticated_message(dest, Message::ApproveReviewer { candidate }));
            }
            Message::RejectReviewer { candidate } => {
                self._reject_reviewer(context.authenticated_signer.unwrap(), candidate)
                    .await?;
                // TODO: broadcast to other chains
                let dest =
                    Destination::Subscribers(ChannelName::from(SUBSCRIPTION_CHANNEL.to_vec()));
                return Ok(ExecutionResult::default()
                    .with_authenticated_message(dest, Message::RejectReviewer { candidate }));
            }
            Message::SubmitContent {
                cid,
                title,
                content,
            } => {
                let author = context.authenticated_signer.unwrap();
                log::info!(
                    "Message submit content cid {:?} by {:?} at {:?}",
                    cid,
                    author,
                    context.chain_id,
                );
                self._submit_content(cid.clone(), title.clone(), content.clone(), author)
                    .await?;
                let dest =
                    Destination::Subscribers(ChannelName::from(SUBSCRIPTION_CHANNEL.to_vec()));
                return Ok(ExecutionResult::default().with_authenticated_message(
                    dest,
                    Message::SubmitContent {
                        cid,
                        title,
                        content,
                    },
                ));
            }
            Message::ApproveContent {
                content_cid,
                reason_cid,
                reason,
            } => {
                let reviewer = context.authenticated_signer.unwrap();
                log::info!(
                    "Message approve content cid {:?} by {:?} with reason {:?} at {:?}",
                    content_cid,
                    reviewer,
                    reason_cid,
                    context.chain_id,
                );
                self._approve_content(
                    reviewer,
                    content_cid.clone(),
                    reason_cid.clone(),
                    reason.clone(),
                )
                .await?;
                let dest =
                    Destination::Subscribers(ChannelName::from(SUBSCRIPTION_CHANNEL.to_vec()));
                return Ok(ExecutionResult::default().with_authenticated_message(
                    dest,
                    Message::ApproveContent {
                        content_cid,
                        reason_cid,
                        reason,
                    },
                ));
            }
            Message::RejectContent {
                content_cid,
                reason,
            } => {
                let reviewer = context.authenticated_signer.unwrap();
                log::info!(
                    "Message reject content cid {:?} by {:?} with reason {:?}",
                    content_cid,
                    reviewer,
                    reason,
                );
                self._reject_content(reviewer, content_cid.clone(), reason.clone())
                    .await?;
                let dest =
                    Destination::Subscribers(ChannelName::from(SUBSCRIPTION_CHANNEL.to_vec()));
                return Ok(ExecutionResult::default().with_authenticated_message(
                    dest,
                    Message::RejectContent {
                        content_cid,
                        reason,
                    },
                ));
            }
            Message::RequestSubscribe => {
                let mut result = ExecutionResult::default();
                log::info!(
                    "Subscribe to {} at {} creation {}",
                    context.message_id.chain_id,
                    context.chain_id,
                    system_api::current_application_id().creation.chain_id
                );
                if context.message_id.chain_id
                    == system_api::current_application_id().creation.chain_id
                {
                    return Ok(result);
                }
                result.subscribe.push((
                    ChannelName::from(SUBSCRIPTION_CHANNEL.to_vec()),
                    context.message_id.chain_id,
                ));
                let mut reviewers = Vec::new();
                self.reviewers
                    .for_each_index_value(|_index, reviewer| -> Result<(), ViewError> {
                        reviewers.push(reviewer);
                        Ok(())
                    })
                    .await
                    .unwrap();
                for reviewer in reviewers {
                    result = result.with_authenticated_message(
                        context.message_id.chain_id,
                        Message::ExistReviewer { reviewer },
                    );
                }
                return Ok(result);
            }
        }
        Ok(ExecutionResult::default())
    }

    async fn handle_application_call(
        &mut self,
        _context: &CalleeContext,
        _call: Self::ApplicationCall,
        _forwarded_sessions: Vec<SessionId>,
    ) -> Result<ApplicationCallResult<Self::Message, Self::Response, Self::SessionState>, Self::Error>
    {
        Ok(ApplicationCallResult::default())
    }

    async fn handle_session_call(
        &mut self,
        _context: &CalleeContext,
        _session: Self::SessionState,
        _call: Self::SessionCall,
        _forwarded_sessions: Vec<SessionId>,
    ) -> Result<SessionCallResult<Self::Message, Self::Response, Self::SessionState>, Self::Error>
    {
        Err(ContractError::SessionsNotSupported)
    }
}

impl Review {
    fn feed_app_id() -> Result<ApplicationId<FeedAbi>, ContractError> {
        Ok(Self::parameters()?.feed_app_id)
    }

    fn credit_app_id() -> Result<ApplicationId<CreditAbi>, ContractError> {
        Ok(Self::parameters()?.credit_app_id)
    }

    fn foundation_app_id() -> Result<ApplicationId<FoundationAbi>, ContractError> {
        Ok(Self::parameters()?.foundation_app_id)
    }

    async fn reward_credits(&mut self, owner: Owner, amount: Amount) -> Result<(), ContractError> {
        log::info!("Reward credits owner {:?} amount {:?}", owner, amount);
        let call = credit::ApplicationCall::Reward { owner, amount };
        self.call_application(true, Self::credit_app_id()?, &call, vec![])
            .await?;
        log::info!("Rewarded credits owner {:?} amount {:?}", owner, amount);
        Ok(())
    }

    async fn reward_tokens(&mut self, reviewer: Owner) -> Result<(), ContractError> {
        log::info!("Reward tokens owner {:?}", reviewer);
        let call = foundation::ApplicationCall::Reward {
            reward_user: None,
            reward_type: foundation::RewardType::Review,
            amount: None,
            activity_id: None,
        };
        self.call_application(true, Self::foundation_app_id()?, &call, vec![])
            .await?;
        log::info!("Rewarded tokens owner {:?}", reviewer);
        Ok(())
    }

    async fn publish_content(
        &mut self,
        cid: String,
        title: String,
        content: String,
        author: Owner,
    ) -> Result<(), ContractError> {
        log::info!("Publish cid {:?}", cid);
        let call = feed::ApplicationCall::Publish {
            cid: cid.clone(),
            title,
            content,
            author,
        };
        self.call_application(true, Self::feed_app_id()?, &call, vec![])
            .await?;
        log::info!("Published cid {:?}", cid);
        Ok(())
    }

    async fn recommend_content(
        &mut self,
        cid: String,
        reason_cid: String,
        reason: String,
    ) -> Result<(), ContractError> {
        log::info!("Recommend content {:?} reason {:?}", cid, reason_cid);
        let call = feed::ApplicationCall::Recommend {
            cid: cid.clone(),
            reason_cid: reason_cid.clone(),
            reason,
        };
        self.call_application(true, Self::feed_app_id()?, &call, vec![])
            .await?;
        log::info!("Recommended content {:?} reason {:?}", cid, reason_cid);
        Ok(())
    }

    async fn _initialize(&mut self, state: InitialState) -> Result<(), ContractError> {
        self.initialize(state).await?;
        Ok(())
    }

    async fn _apply_reviewer(
        &mut self,
        chain_id: ChainId,
        candidate: Owner,
        resume: String,
    ) -> Result<(), ContractError> {
        self.apply_reviewer(chain_id, candidate, resume).await?;
        Ok(())
    }

    async fn _update_reviewer_resume(
        &mut self,
        reviewer: Owner,
        resume: String,
    ) -> Result<(), ContractError> {
        self.update_reviewer_resume(reviewer, resume).await?;
        Ok(())
    }

    async fn _approve_reviewer(
        &mut self,
        reviewer: Owner,
        candidate: Owner,
    ) -> Result<(), ContractError> {
        match self.approve_reviewer(reviewer, candidate).await? {
            Some(_reviewer) => {
                // TODO: notify candidate is reviewer
            }
            _ => {
                // TODO: notify candidate is approved
            }
        }
        self.reward_credits(reviewer, Amount::from_tokens(100))
            .await?;
        self.reward_tokens(reviewer).await?;
        Ok(())
    }

    async fn _reject_reviewer(
        &mut self,
        reviewer: Owner,
        candidate: Owner,
    ) -> Result<(), ContractError> {
        match self.reject_reviewer(reviewer, candidate).await? {
            Some(_reviewer) => {
                // TODO: notify candidate is reviewer
            }
            _ => {
                // TODO: notify candidate is approved
            }
        }
        self.reward_credits(reviewer, Amount::from_tokens(100))
            .await?;
        self.reward_tokens(reviewer).await?;
        Ok(())
    }

    async fn _submit_content(
        &mut self,
        cid: String,
        title: String,
        content: String,
        author: Owner,
    ) -> Result<(), ContractError> {
        self.submit_content(Content {
            // TODO: notify author
            cid,
            title,
            content,
            author,
            reviewers: HashMap::default(),
            approved: 0,
            rejected: 0,
            created_at: system_api::current_system_time(),
        })
        .await?;
        self.reward_credits(author, Amount::from_tokens(10)).await?;
        Ok(())
    }

    async fn _approve_content(
        &mut self,
        reviewer: Owner,
        content_cid: String,
        reason_cid: Option<String>,
        reason: Option<String>,
    ) -> Result<(), ContractError> {
        match self
            .approve_content(
                reviewer,
                content_cid.clone(),
                reason.clone().unwrap_or_default(),
            )
            .await?
        {
            Some(content) => {
                self.publish_content(content.cid, content.title, content.content, content.author)
                    .await?;
                match reason_cid {
                    Some(cid) => {
                        self.recommend_content(content_cid, cid, reason.unwrap_or_default())
                            .await?
                    }
                    _ => {}
                }
                // TODO: notify author content is published
            }
            _ => {
                // TODO: notify author content is approved
            }
        }
        self.reward_credits(reviewer, Amount::from_tokens(50))
            .await?;
        self.reward_tokens(reviewer).await?;
        Ok(())
    }

    async fn _reject_content(
        &mut self,
        reviewer: Owner,
        content_cid: String,
        reason: Option<String>,
    ) -> Result<(), ContractError> {
        match self
            .reject_content(reviewer, content_cid, reason.unwrap_or_default())
            .await?
        {
            Some(_content) => {
                // TODO: notify author content is rejected
            }
            _ => {
                // TODO: notify author content is rejected
            }
        }
        self.reward_credits(reviewer, Amount::from_tokens(50))
            .await?;
        self.reward_tokens(reviewer).await?;
        Ok(())
    }

    async fn _approve_asset(
        &mut self,
        reviewer: Owner,
        collection_id: u64,
        _reason_cid: Option<String>,
        _reason: Option<String>,
    ) -> Result<(), ContractError> {
        self.approve_asset(reviewer, collection_id).await?;
        // TODO: add reason
        // TODO: if already approved, publish asset
        // TODO: notify author
        Ok(())
    }

    async fn _reject_asset(
        &mut self,
        reviewer: Owner,
        collection_id: u64,
        _reason: Option<String>,
    ) -> Result<(), ContractError> {
        self.reject_asset(reviewer, collection_id).await?;
        // TODO: add reason
        // TODO: notify author
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

    #[error("Invalid user")]
    InvalidUser,

    #[error("Cross-application sessions not supported")]
    SessionsNotSupported,
}
