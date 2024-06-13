#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use std::collections::HashMap;

use self::state::Review;
use credit::CreditAbi;
use feed::FeedAbi;
use foundation::FoundationAbi;
use linera_sdk::{
    base::{
        Amount, ApplicationId, ChainId, ChannelName, Destination, MessageId, Owner, WithContractAbi,
    },
    views::{RootView, View, ViewStorageContext},
    Contract, ContractRuntime,
};
// use linera_views::views::ViewError;
use market::MarketAbi;
use review::{
    Asset, Content, InstantiationArgument, Message, Operation, ReviewError, ReviewParameters,
    ReviewResponse, Reviewer,
};
use log::info;

pub struct ReviewContract {
    state: Review,
    runtime: ContractRuntime<Self>,
}

linera_sdk::contract!(ReviewContract);

impl WithContractAbi for ReviewContract {
    type Abi = review::ReviewAbi;
}

const SUBSCRIPTION_CHANNEL: &[u8] = b"subscriptions";

impl Contract for ReviewContract {
    type Message = Message;
    type InstantiationArgument = InstantiationArgument;
    type Parameters = ReviewParameters;

    async fn load(runtime: ContractRuntime<Self>) -> Self {
        let state = Review::load(ViewStorageContext::from(runtime.key_value_store()))
            .await
            .expect("Failed to load state");
        ReviewContract { state, runtime }
    }

    async fn instantiate(&mut self, argument: Self::InstantiationArgument) {
        self.runtime.application_parameters();
        self._instantiate(argument)
            .await
            .expect("Failed to instantiate review");
        // We should add creator here to be a reviewer, but we keep the message as an test case of application id
        self.runtime
            .prepare_message(Message::GenesisReviewer)
            .with_authentication()
            .send_to(self.runtime.application_id().creation.chain_id);
    }

    async fn execute_operation(&mut self, operation: Self::Operation) -> Self::Response {
        match operation {
            Operation::ApplyReviewer { resume } => self
                .on_op_apply_reviewer(resume)
                .expect("Failed OP: apply reviewer"),
            Operation::UpdateReviewerResume { resume } => self
                .on_op_upeate_reviewer_resume(resume)
                .expect("Failed OP: update reviewer resume"),
            Operation::ApproveReviewer { candidate, reason } => self
                .on_op_approve_reviewer(candidate, reason)
                .expect("Failed OP: approve reviewer"),
            Operation::RejectReviewer { candidate, reason } => self
                .on_op_reject_reviewer(candidate, reason)
                .expect("Failed OP: reject reviewer"),
            Operation::SubmitContent {
                cid,
                title,
                content,
            } => self
                .on_op_submit_content(cid, title, content)
                .expect("Failed OP: submit content"),
            Operation::ApproveContent {
                content_cid,
                reason_cid,
                reason,
            } => self
                .on_op_approve_content(content_cid, reason_cid, reason)
                .expect("Failed OP: approve content"),
            Operation::RejectContent {
                content_cid,
                reason,
            } => self
                .on_op_reject_content(content_cid, reason)
                .expect("Failed OP: reject content"),
            Operation::SubmitComment {
                cid,
                comment_cid,
                comment,
            } => self
                .on_op_submit_comment(cid, comment_cid, comment)
                .expect("Failed OP: submit comment"),
            Operation::ApproveAsset { cid, reason } => self
                .on_op_approve_asset(cid, reason)
                .expect("Failed OP: approve asset"),
            Operation::RejectAsset { cid, reason } => self
                .on_op_reject_asset(cid, reason)
                .expect("Failed OP: reject asset"),
            Operation::SubmitAsset {
                cid,
                base_uri,
                uris,
                price,
                name,
            } => self
                .on_op_submit_asset(cid, base_uri, uris, price, name)
                .expect("Failed OP: submit asset"),
            Operation::RequestSubscribe => self
                .on_op_request_subscribe()
                .expect("Failed OP: subscribe"),
            Operation::ApproveActivity {
                activity_id,
                reason,
            } => self
                .on_op_approve_activity(activity_id, reason)
                .expect("Failed OP: approve activity"),
            Operation::RejectActivity {
                activity_id,
                reason,
            } => self
                .on_op_reject_activity(activity_id, reason)
                .expect("Failed OP: reject activity"),
            Operation::SubmitActivity {
                activity_id,
                activity_host,
                budget_amount,
            } => self
                .on_op_submit_activity(activity_id, activity_host, budget_amount)
                .expect("Failed OP: submit activity"),
            Operation::ActivityApproved { activity_id } => self
                .on_op_activity_approved(activity_id)
                .await
                .expect("Failed OP: activity approved"),
        }
    }

    async fn execute_message(&mut self, message: Self::Message) {
        match message {
            Message::GenesisReviewer {} => self
                .on_msg_genesis_reviewer()
                .await
                .expect("Failed MSG: genesis reviewer"),
            Message::ExistReviewer { reviewer } => self
                .on_msg_exist_reviewer(reviewer)
                .await
                .expect("Failed MSG: exist reviewer"),
            Message::ApplyReviewer { resume } => self
                .on_msg_apply_reviewer(resume)
                .await
                .expect("Failed MSG: apply reviewer"),
            Message::UpdateReviewerResume { resume } => self
                .on_msg_upeate_reviewer_resume(resume)
                .await
                .expect("Failed MSG: update reviewer resume"),
            Message::ApproveReviewer { candidate, reason } => self
                .on_msg_approve_reviewer(candidate, reason)
                .await
                .expect("Failed MSG: approve reviewer"),
            Message::RejectReviewer { candidate, reason } => self
                .on_msg_reject_reviewer(candidate, reason)
                .await
                .expect("Failed MSG: reject reviewer"),
            Message::SubmitContent {
                cid,
                title,
                content,
            } => self
                .on_msg_submit_content(cid, title, content)
                .await
                .expect("Failed MSG: submit content"),
            Message::ApproveContent {
                content_cid,
                reason_cid,
                reason,
            } => self
                .on_msg_approve_content(content_cid, reason_cid, reason)
                .await
                .expect("Failed MSG: approve content"),
            Message::RejectContent {
                content_cid,
                reason,
            } => self
                .on_msg_reject_content(content_cid, reason)
                .await
                .expect("Failed MSG: reject content"),
            Message::SubmitComment {
                cid,
                comment_cid,
                comment,
            } => self
                .on_msg_submit_comment(cid, comment_cid, comment)
                .await
                .expect("Failed MSG: submit comment"),
            Message::ApproveAsset { cid, reason } => self
                .on_msg_approve_asset(cid, reason)
                .await
                .expect("Failed MSG: approve asset"),
            Message::RejectAsset { cid, reason } => self
                .on_msg_reject_asset(cid, reason)
                .await
                .expect("Failed MSG: reject asset"),
            Message::SubmitAsset {
                cid,
                base_uri,
                uris,
                price,
                name,
            } => self
                .on_msg_submit_asset(cid, base_uri, uris, price, name)
                .await
                .expect("Failed MSG: submit asset"),
            Message::RequestSubscribe => self
                .on_msg_request_subscribe()
                .await
                .expect("Failed MSG: subscribe"),
            Message::InstantiationArgument { argument } => self
                .on_msg_initialization_argument(argument)
                .await
                .expect("Failed MSG: instantiation argument"),
            Message::SubmitActivity {
                activity_id,
                activity_host,
                budget_amount,
            } => self
                .on_msg_submit_activity(activity_id, activity_host, budget_amount)
                .await
                .expect("Failed MSG: submit activity"),
            Message::ApproveActivity {
                activity_id,
                reason,
            } => self
                .on_msg_approve_activity(activity_id, reason)
                .await
                .expect("Failed MSG: approve activity"),
            Message::RejectActivity {
                activity_id,
                reason,
            } => self
                .on_msg_reject_activity(activity_id, reason)
                .await
                .expect("Failed MSG: reject activity"),
        }
    }

    async fn store(mut self) {
        self.state.save().await.expect("Failed to save state");
    }
}

impl ReviewContract {
    fn feed_app_id(&mut self) -> ApplicationId<FeedAbi> {
        self.runtime.application_parameters().feed_app_id
    }

    fn credit_app_id(&mut self) -> ApplicationId<CreditAbi> {
        self.runtime.application_parameters().credit_app_id
    }

    fn foundation_app_id(&mut self) -> ApplicationId<FoundationAbi> {
        self.runtime.application_parameters().foundation_app_id
    }

    fn market_app_id(&mut self) -> ApplicationId<MarketAbi> {
        self.runtime.application_parameters().market_app_id
    }

    async fn reward_credits(&mut self, owner: Owner, amount: Amount) -> Result<(), ReviewError> {
        let call = credit::Operation::Reward { owner, amount };
        let credit_app_id = self.credit_app_id();
        self.runtime.call_application(true, credit_app_id, &call);
        Ok(())
    }

    async fn reward_tokens(&mut self) -> Result<(), ReviewError> {
        let call = foundation::Operation::Reward {
            reward_user: None,
            reward_type: foundation::RewardType::Review,
            activity_id: None,
        };
        let foundation_app_id = self.foundation_app_id();
        self.runtime
            .call_application(true, foundation_app_id, &call);
        Ok(())
    }

    async fn publish_content(
        &mut self,
        cid: String,
        title: String,
        content: String,
        author: Owner,
    ) -> Result<(), ReviewError> {
        let call = feed::Operation::Publish {
            cid: cid.clone(),
            title,
            content,
            author,
        };
        let feed_app_id = self.feed_app_id();
        info!("Publish content {} author {}", cid, author);
        self.runtime.call_application(true, feed_app_id, &call);
        Ok(())
    }

    async fn recommend_content(
        &mut self,
        cid: String,
        reason_cid: String,
        reason: String,
    ) -> Result<(), ReviewError> {
        let call = feed::Operation::Recommend {
            cid: cid.clone(),
            reason_cid: reason_cid.clone(),
            reason,
        };
        let feed_app_id = self.feed_app_id();
        self.runtime.call_application(true, feed_app_id, &call);
        Ok(())
    }

    async fn comment_content(
        &mut self,
        cid: String,
        comment_cid: String,
        comment: String,
        commentor: Owner,
    ) -> Result<(), ReviewError> {
        let call = feed::Operation::Comment {
            cid: cid.clone(),
            comment_cid: comment_cid.clone(),
            comment,
            commentor,
        };
        let feed_app_id = self.feed_app_id();
        self.runtime.call_application(true, feed_app_id, &call);
        Ok(())
    }

    async fn create_collection(
        &mut self,
        base_uri: String,
        uris: Vec<String>,
        price: Option<Amount>,
        name: String,
        publisher: Owner,
    ) -> Result<(), ReviewError> {
        let call = market::Operation::CreateCollection {
            base_uri: base_uri.clone(),
            price,
            name,
            uris,
            publisher,
        };
        let market_app_id = self.market_app_id();
        self.runtime.call_application(true, market_app_id, &call);
        Ok(())
    }

    async fn lock_activity_funds(
        &mut self,
        activity_id: u64,
        budget_amount: Amount,
    ) -> Result<(), ReviewError> {
        let call = foundation::Operation::Lock {
            activity_id,
            amount: budget_amount,
        };
        let foundation_app_id = self.foundation_app_id();
        self.runtime
            .call_application(true, foundation_app_id, &call);
        Ok(())
    }

    async fn _instantiate(&mut self, argument: InstantiationArgument) -> Result<(), ReviewError> {
        self.state.instantiate_review(argument).await?;
        Ok(())
    }

    async fn _apply_reviewer(
        &mut self,
        chain_id: ChainId,
        candidate: Owner,
        resume: String,
    ) -> Result<(), ReviewError> {
        self.state
            .apply_reviewer(chain_id, candidate, resume, self.runtime.system_time())
            .await?;
        Ok(())
    }

    async fn _update_reviewer_resume(
        &mut self,
        reviewer: Owner,
        resume: String,
    ) -> Result<(), ReviewError> {
        self.state.update_reviewer_resume(reviewer, resume).await?;
        Ok(())
    }

    async fn _approve_reviewer(
        &mut self,
        reviewer: Owner,
        candidate: Owner,
        reason: Option<String>,
        creation_chain: bool,
    ) -> Result<(), ReviewError> {
        let _reviewer = self
            .state
            .approve_reviewer(
                reviewer,
                candidate,
                reason.unwrap_or_default(),
                self.runtime.system_time(),
            )
            .await?;
        if !creation_chain {
            return Ok(());
        }
        match _reviewer {
            Some(_reviewer) => {
                // TODO: notify candidate is reviewer
            }
            _ => {
                // TODO: notify candidate is approved
            }
        }
        self.reward_credits(reviewer, Amount::from_tokens(100))
            .await?;
        self.reward_tokens().await?;
        Ok(())
    }

    async fn _reject_reviewer(
        &mut self,
        reviewer: Owner,
        candidate: Owner,
        reason: Option<String>,
        creation_chain: bool,
    ) -> Result<(), ReviewError> {
        let _reviewer = self
            .state
            .reject_reviewer(
                reviewer,
                candidate,
                reason.unwrap_or_default(),
                self.runtime.system_time(),
            )
            .await?;
        if !creation_chain {
            return Ok(());
        }
        match _reviewer {
            Some(_reviewer) => {
                // TODO: notify candidate is reviewer
            }
            _ => {
                // TODO: notify candidate is approved
            }
        }
        self.reward_credits(reviewer, Amount::from_tokens(100))
            .await?;
        self.reward_tokens().await?;
        Ok(())
    }

    async fn _submit_content(
        &mut self,
        cid: String,
        title: String,
        content: String,
        author: Owner,
        creation_chain: bool,
    ) -> Result<(), ReviewError> {
        self.state
            .submit_content(Content {
                // TODO: notify author
                cid,
                comment_to_cid: None,
                title,
                content,
                author,
                reviewers: HashMap::default(),
                approved: 0,
                rejected: 0,
                created_at: self.runtime.system_time(),
            })
            .await?;
        if !creation_chain {
            return Ok(());
        }
        self.reward_credits(author, Amount::from_tokens(10)).await?;
        Ok(())
    }

    async fn _submit_comment(
        &mut self,
        cid: String,
        comment_to_cid: String,
        comment: String,
        author: Owner,
        creation_chain: bool,
    ) -> Result<(), ReviewError> {
        self.state
            .submit_content(Content {
                cid,
                comment_to_cid: Some(comment_to_cid),
                title: String::default(),
                content: comment,
                author,
                reviewers: HashMap::default(),
                approved: 0,
                rejected: 0,
                created_at: self.runtime.system_time(),
            })
            .await?;
        if !creation_chain {
            return Ok(());
        }
        self.reward_credits(author, Amount::from_tokens(10)).await?;
        Ok(())
    }

    async fn _approve_content(
        &mut self,
        reviewer: Owner,
        content_cid: String,
        reason_cid: Option<String>,
        reason: Option<String>,
        creation_chain: bool,
    ) -> Result<(), ReviewError> {
        let content = self
            .state
            .approve_content(
                reviewer,
                content_cid.clone(),
                reason.clone().unwrap_or_default(),
                self.runtime.system_time(),
            )
            .await?;
        if !creation_chain {
            return Ok(());
        }
        match content {
            Some(content) => {
                match content.comment_to_cid {
                    Some(comment_to_cid) => {
                        self.comment_content(
                            comment_to_cid,
                            content.cid.clone(),
                            content.content,
                            content.author,
                        )
                        .await?;
                    }
                    _ => {
                        self.publish_content(
                            content.cid,
                            content.title,
                            content.content,
                            content.author,
                        )
                        .await?
                    }
                }
                match reason_cid {
                    Some(cid) => {
                        self.recommend_content(content_cid.clone(), cid, reason.unwrap_or_default())
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
        self.reward_tokens().await?;
        Ok(())
    }

    async fn _reject_content(
        &mut self,
        reviewer: Owner,
        content_cid: String,
        reason: Option<String>,
        creation_chain: bool,
    ) -> Result<(), ReviewError> {
        let content = self
            .state
            .reject_content(
                reviewer,
                content_cid,
                reason.unwrap_or_default(),
                self.runtime.system_time(),
            )
            .await?;
        if !creation_chain {
            return Ok(());
        }
        match content {
            Some(_content) => {
                // TODO: notify author content is rejected
            }
            _ => {
                // TODO: notify author content is rejected
            }
        }
        self.reward_credits(reviewer, Amount::from_tokens(50))
            .await?;
        self.reward_tokens().await?;
        Ok(())
    }

    async fn _approve_asset(
        &mut self,
        reviewer: Owner,
        cid: String,
        reason: Option<String>,
        creation_chain: bool,
    ) -> Result<(), ReviewError> {
        let asset = self
            .state
            .approve_asset(
                reviewer,
                cid,
                reason.unwrap_or_default(),
                self.runtime.system_time(),
            )
            .await?;
        if !creation_chain {
            return Ok(());
        }
        match asset {
            Some(asset) => {
                self.create_collection(
                    asset.base_uri,
                    asset.uris,
                    asset.price,
                    asset.name,
                    asset.author,
                )
                .await?;
                // TODO: notify author is approved
            }
            _ => {
                // TODO: notify author
            }
        }
        self.reward_credits(reviewer, Amount::from_tokens(50))
            .await?;
        self.reward_tokens().await?;
        Ok(())
    }

    async fn _reject_asset(
        &mut self,
        reviewer: Owner,
        cid: String,
        reason: Option<String>,
        creation_chain: bool,
    ) -> Result<(), ReviewError> {
        let asset = self
            .state
            .reject_asset(
                reviewer,
                cid,
                reason.unwrap_or_default(),
                self.runtime.system_time(),
            )
            .await?;
        if !creation_chain {
            return Ok(());
        }
        match asset {
            Some(_asset) => {
                // TODO: notify author is approved
            }
            _ => {
                // TODO: notify author
            }
        }
        self.reward_credits(reviewer, Amount::from_tokens(50))
            .await?;
        self.reward_tokens().await?;
        Ok(())
    }

    async fn _submit_asset(
        &mut self,
        author: Owner,
        cid: String,
        base_uri: String,
        uris: Vec<String>,
        price: Option<Amount>,
        name: String,
    ) -> Result<(), ReviewError> {
        self.state
            .submit_asset(Asset {
                cid,
                author,
                base_uri,
                uris,
                price,
                name,
                reviewers: HashMap::default(),
                approved: 0,
                rejected: 0,
                created_at: self.runtime.system_time(),
            })
            .await?;
        Ok(())
    }

    async fn _submit_activity(
        &mut self,
        activity_id: u64,
        activity_host: Owner,
        budget_amount: Amount,
    ) -> Result<(), ReviewError> {
        self.state
            .submit_activity(
                activity_id,
                activity_host,
                budget_amount,
                self.runtime.system_time(),
            )
            .await?;
        Ok(())
    }

    async fn _approve_activity(
        &mut self,
        owner: Owner,
        activity_id: u64,
        reason: Option<String>,
        creation_chain: bool,
    ) -> Result<(), ReviewError> {
        let activity = self
            .state
            .approve_activity(
                owner,
                activity_id,
                reason.unwrap_or_default(),
                self.runtime.system_time(),
            )
            .await?;
        if !creation_chain {
            return Ok(());
        }
        self.reward_credits(owner, Amount::from_tokens(50)).await?;
        self.reward_tokens().await?;
        if let Some(activity) = activity {
            self.lock_activity_funds(activity_id, activity.budget_amount)
                .await?;
        }
        Ok(())
    }

    async fn _reject_activity(
        &mut self,
        owner: Owner,
        activity_id: u64,
        reason: String,
        creation_chain: bool,
    ) -> Result<(), ReviewError> {
        let _activity = self
            .state
            .reject_activity(owner, activity_id, reason, self.runtime.system_time())
            .await?;
        if !creation_chain {
            return Ok(());
        }
        self.reward_credits(owner, Amount::from_tokens(50)).await?;
        self.reward_tokens().await?;
        Ok(())
    }

    fn require_message_id(&mut self) -> Result<MessageId, ReviewError> {
        match self.runtime.message_id() {
            Some(message_id) => Ok(message_id),
            None => Err(ReviewError::InvalidMessageId),
        }
    }

    fn require_authenticated_signer(&mut self) -> Result<Owner, ReviewError> {
        match self.runtime.authenticated_signer() {
            Some(owner) => Ok(owner),
            None => Err(ReviewError::InvalidSigner),
        }
    }

    fn on_op_apply_reviewer(&mut self, resume: String) -> Result<ReviewResponse, ReviewError> {
        self.runtime
            .prepare_message(Message::ApplyReviewer { resume })
            .with_authentication()
            .send_to(self.runtime.application_id().creation.chain_id);
        Ok(ReviewResponse::Ok)
    }

    fn on_op_upeate_reviewer_resume(
        &mut self,
        resume: String,
    ) -> Result<ReviewResponse, ReviewError> {
        self.runtime
            .prepare_message(Message::UpdateReviewerResume { resume })
            .with_authentication()
            .send_to(self.runtime.application_id().creation.chain_id);
        Ok(ReviewResponse::Ok)
    }

    fn on_op_approve_reviewer(
        &mut self,
        candidate: Owner,
        reason: Option<String>,
    ) -> Result<ReviewResponse, ReviewError> {
        self.runtime
            .prepare_message(Message::ApproveReviewer { candidate, reason })
            .with_authentication()
            .send_to(self.runtime.application_id().creation.chain_id);
        Ok(ReviewResponse::Ok)
    }

    fn on_op_reject_reviewer(
        &mut self,
        candidate: Owner,
        reason: Option<String>,
    ) -> Result<ReviewResponse, ReviewError> {
        self.runtime
            .prepare_message(Message::RejectReviewer { candidate, reason })
            .with_authentication()
            .send_to(self.runtime.application_id().creation.chain_id);
        Ok(ReviewResponse::Ok)
    }

    fn on_op_submit_content(
        &mut self,
        cid: String,
        title: String,
        content: String,
    ) -> Result<ReviewResponse, ReviewError> {
        self.runtime
            .prepare_message(Message::SubmitContent {
                cid,
                title,
                content,
            })
            .with_authentication()
            .send_to(self.runtime.application_id().creation.chain_id);
        Ok(ReviewResponse::Ok)
    }

    fn on_op_approve_content(
        &mut self,
        content_cid: String,
        reason_cid: Option<String>,
        reason: Option<String>,
    ) -> Result<ReviewResponse, ReviewError> {
        self.runtime
            .prepare_message(Message::ApproveContent {
                content_cid,
                reason_cid,
                reason,
            })
            .with_authentication()
            .send_to(self.runtime.application_id().creation.chain_id);
        Ok(ReviewResponse::Ok)
    }

    fn on_op_reject_content(
        &mut self,
        content_cid: String,
        reason: Option<String>,
    ) -> Result<ReviewResponse, ReviewError> {
        self.runtime
            .prepare_message(Message::RejectContent {
                content_cid,
                reason,
            })
            .with_authentication()
            .send_to(self.runtime.application_id().creation.chain_id);
        Ok(ReviewResponse::Ok)
    }

    fn on_op_submit_comment(
        &mut self,
        cid: String,
        comment_cid: String,
        comment: String,
    ) -> Result<ReviewResponse, ReviewError> {
        self.runtime
            .prepare_message(Message::SubmitComment {
                cid,
                comment_cid,
                comment,
            })
            .with_authentication()
            .send_to(self.runtime.application_id().creation.chain_id);
        Ok(ReviewResponse::Ok)
    }

    fn on_op_approve_asset(
        &mut self,
        cid: String,
        reason: Option<String>,
    ) -> Result<ReviewResponse, ReviewError> {
        self.runtime
            .prepare_message(Message::ApproveAsset { cid, reason })
            .with_authentication()
            .send_to(self.runtime.application_id().creation.chain_id);
        Ok(ReviewResponse::Ok)
    }

    fn on_op_reject_asset(
        &mut self,
        cid: String,
        reason: Option<String>,
    ) -> Result<ReviewResponse, ReviewError> {
        self.runtime
            .prepare_message(Message::RejectAsset { cid, reason })
            .with_authentication()
            .send_to(self.runtime.application_id().creation.chain_id);
        Ok(ReviewResponse::Ok)
    }

    fn on_op_submit_asset(
        &mut self,
        cid: String,
        base_uri: String,
        uris: Vec<String>,
        price: Option<Amount>,
        name: String,
    ) -> Result<ReviewResponse, ReviewError> {
        self.runtime
            .prepare_message(Message::SubmitAsset {
                cid,
                base_uri,
                uris,
                price,
                name,
            })
            .with_authentication()
            .send_to(self.runtime.application_id().creation.chain_id);
        Ok(ReviewResponse::Ok)
    }

    fn on_op_request_subscribe(&mut self) -> Result<ReviewResponse, ReviewError> {
        self.runtime
            .prepare_message(Message::RequestSubscribe)
            .with_authentication()
            .send_to(self.runtime.application_id().creation.chain_id);
        log::info!(
            "operation subscribe from chain {} to chain {}",
            self.runtime.chain_id(),
            self.runtime.application_id().creation.chain_id,
        );
        Ok(ReviewResponse::Ok)
    }

    fn on_op_approve_activity(
        &mut self,
        activity_id: u64,
        reason: Option<String>,
    ) -> Result<ReviewResponse, ReviewError> {
        self.runtime
            .prepare_message(Message::ApproveActivity {
                activity_id,
                reason,
            })
            .with_authentication()
            .send_to(self.runtime.application_id().creation.chain_id);
        // TODO: send initialization argument to subscriber
        Ok(ReviewResponse::Ok)
    }

    fn on_op_reject_activity(
        &mut self,
        activity_id: u64,
        reason: String,
    ) -> Result<ReviewResponse, ReviewError> {
        self.runtime
            .prepare_message(Message::RejectActivity {
                activity_id,
                reason,
            })
            .with_authentication()
            .send_to(self.runtime.application_id().creation.chain_id);
        // TODO: send initialization argument to subscriber
        Ok(ReviewResponse::Ok)
    }

    fn on_op_submit_activity(
        &mut self,
        activity_id: u64,
        activity_host: Owner,
        budget_amount: Amount,
    ) -> Result<ReviewResponse, ReviewError> {
        self.runtime
            .prepare_message(Message::SubmitActivity {
                activity_id,
                activity_host,
                budget_amount,
            })
            .with_authentication()
            .send_to(self.runtime.application_id().creation.chain_id);
        // TODO: send initialization argument to subscriber
        Ok(ReviewResponse::Ok)
    }

    async fn on_op_activity_approved(
        &mut self,
        activity_id: u64,
    ) -> Result<ReviewResponse, ReviewError> {
        Ok(ReviewResponse::Approved(
            self.state.activity_approved(activity_id).await?,
        ))
    }

    async fn on_msg_initialization_argument(
        &mut self,
        argument: InstantiationArgument,
    ) -> Result<(), ReviewError> {
        self.state.instantiate_review(argument).await
    }

    async fn on_msg_genesis_reviewer(&mut self) -> Result<(), ReviewError> {
        let chain_id = self.runtime.chain_id();
        let reviewer = self.require_authenticated_signer()?;
        self.state
            .genesis_reviewer(chain_id, reviewer, self.runtime.system_time())
            .await?;
        if self.runtime.chain_id() != self.runtime.application_id().creation.chain_id {
            return Ok(());
        }
        let dest = Destination::Subscribers(ChannelName::from(SUBSCRIPTION_CHANNEL.to_vec()));
        self.runtime
            .prepare_message(Message::GenesisReviewer)
            .with_authentication()
            .send_to(dest);
        Ok(())
    }

    async fn on_msg_exist_reviewer(&mut self, reviewer: Reviewer) -> Result<(), ReviewError> {
        self.state.add_exist_reviewer(reviewer).await?;
        Ok(())
    }

    async fn on_msg_apply_reviewer(&mut self, resume: String) -> Result<(), ReviewError> {
        let candidate = self.require_authenticated_signer()?;
        let chain_id = self.runtime.chain_id();
        self._apply_reviewer(chain_id, candidate, resume.clone())
            .await?;
        if self.runtime.chain_id() != self.runtime.application_id().creation.chain_id {
            return Ok(());
        }
        let dest = Destination::Subscribers(ChannelName::from(SUBSCRIPTION_CHANNEL.to_vec()));
        self.runtime
            .prepare_message(Message::ApplyReviewer { resume })
            .with_authentication()
            .send_to(dest);
        Ok(())
    }

    async fn on_msg_upeate_reviewer_resume(&mut self, resume: String) -> Result<(), ReviewError> {
        let candidate = self.require_authenticated_signer()?;
        self._update_reviewer_resume(candidate, resume.clone())
            .await?;
        if self.runtime.chain_id() != self.runtime.application_id().creation.chain_id {
            return Ok(());
        }
        let dest = Destination::Subscribers(ChannelName::from(SUBSCRIPTION_CHANNEL.to_vec()));
        self.runtime
            .prepare_message(Message::UpdateReviewerResume { resume })
            .with_authentication()
            .send_to(dest);
        Ok(())
    }

    async fn on_msg_approve_reviewer(
        &mut self,
        candidate: Owner,
        reason: Option<String>,
    ) -> Result<(), ReviewError> {
        let reviewer = self.require_authenticated_signer()?;
        let creation_chain =
            self.runtime.chain_id() != self.runtime.application_id().creation.chain_id;
        self._approve_reviewer(reviewer, candidate, reason.clone(), creation_chain)
            .await?;
        if self.runtime.chain_id() != self.runtime.application_id().creation.chain_id {
            return Ok(());
        }
        let dest = Destination::Subscribers(ChannelName::from(SUBSCRIPTION_CHANNEL.to_vec()));
        self.runtime
            .prepare_message(Message::ApproveReviewer { candidate, reason })
            .with_authentication()
            .send_to(dest);
        Ok(())
    }

    async fn on_msg_reject_reviewer(
        &mut self,
        candidate: Owner,
        reason: Option<String>,
    ) -> Result<(), ReviewError> {
        let reviewer = self.require_authenticated_signer()?;
        let creation_chain =
            self.runtime.chain_id() != self.runtime.application_id().creation.chain_id;
        self._reject_reviewer(reviewer, candidate, reason.clone(), creation_chain)
            .await?;
        if self.runtime.chain_id() != self.runtime.application_id().creation.chain_id {
            return Ok(());
        }
        let dest = Destination::Subscribers(ChannelName::from(SUBSCRIPTION_CHANNEL.to_vec()));
        self.runtime
            .prepare_message(Message::RejectReviewer { candidate, reason })
            .with_authentication()
            .send_to(dest);
        Ok(())
    }

    async fn on_msg_submit_content(
        &mut self,
        cid: String,
        title: String,
        content: String,
    ) -> Result<(), ReviewError> {
        let author = self.require_authenticated_signer()?;
        let creation_chain =
            self.runtime.chain_id() != self.runtime.application_id().creation.chain_id;
        self._submit_content(
            cid.clone(),
            title.clone(),
            content.clone(),
            author,
            creation_chain,
        )
        .await?;
        if self.runtime.chain_id() != self.runtime.application_id().creation.chain_id {
            return Ok(());
        }
        let dest = Destination::Subscribers(ChannelName::from(SUBSCRIPTION_CHANNEL.to_vec()));
        self.runtime
            .prepare_message(Message::SubmitContent {
                cid,
                title,
                content,
            })
            .with_authentication()
            .send_to(dest);
        Ok(())
    }

    async fn on_msg_approve_content(
        &mut self,
        content_cid: String,
        reason_cid: Option<String>,
        reason: Option<String>,
    ) -> Result<(), ReviewError> {
        info!("Approve content {}", content_cid);
        let reviewer = self.require_authenticated_signer()?;
        let creation_chain =
            self.runtime.chain_id() != self.runtime.application_id().creation.chain_id;
        self._approve_content(
            reviewer,
            content_cid.clone(),
            reason_cid.clone(),
            reason.clone(),
            creation_chain,
        )
        .await?;
        if self.runtime.chain_id() != self.runtime.application_id().creation.chain_id {
            return Ok(());
        }
        let dest = Destination::Subscribers(ChannelName::from(SUBSCRIPTION_CHANNEL.to_vec()));
        self.runtime
            .prepare_message(Message::ApproveContent {
                content_cid,
                reason_cid,
                reason,
            })
            .with_authentication()
            .send_to(dest);
        Ok(())
    }

    async fn on_msg_reject_content(
        &mut self,
        content_cid: String,
        reason: Option<String>,
    ) -> Result<(), ReviewError> {
        let reviewer = self.require_authenticated_signer()?;
        let creation_chain =
            self.runtime.chain_id() != self.runtime.application_id().creation.chain_id;
        self._reject_content(
            reviewer,
            content_cid.clone(),
            reason.clone(),
            creation_chain,
        )
        .await?;
        if self.runtime.chain_id() != self.runtime.application_id().creation.chain_id {
            return Ok(());
        }
        let dest = Destination::Subscribers(ChannelName::from(SUBSCRIPTION_CHANNEL.to_vec()));
        self.runtime
            .prepare_message(Message::RejectContent {
                content_cid,
                reason,
            })
            .with_authentication()
            .send_to(dest);
        Ok(())
    }

    async fn on_msg_submit_comment(
        &mut self,
        cid: String,
        comment_cid: String,
        comment: String,
    ) -> Result<(), ReviewError> {
        let author = self.require_authenticated_signer()?;
        let creation_chain =
            self.runtime.chain_id() != self.runtime.application_id().creation.chain_id;
        self._submit_comment(
            comment_cid.clone(),
            cid.clone(),
            comment.clone(),
            author,
            creation_chain,
        )
        .await?;
        if self.runtime.chain_id() != self.runtime.application_id().creation.chain_id {
            return Ok(());
        }
        let dest = Destination::Subscribers(ChannelName::from(SUBSCRIPTION_CHANNEL.to_vec()));
        self.runtime
            .prepare_message(Message::SubmitComment {
                cid,
                comment_cid,
                comment,
            })
            .with_authentication()
            .send_to(dest);
        Ok(())
    }

    async fn on_msg_approve_asset(
        &mut self,
        cid: String,
        reason: Option<String>,
    ) -> Result<(), ReviewError> {
        let reviewer = self.require_authenticated_signer()?;
        let creation_chain =
            self.runtime.chain_id() != self.runtime.application_id().creation.chain_id;
        self._approve_asset(reviewer, cid.clone(), reason.clone(), creation_chain)
            .await?;
        if self.runtime.chain_id() != self.runtime.application_id().creation.chain_id {
            return Ok(());
        }
        let dest = Destination::Subscribers(ChannelName::from(SUBSCRIPTION_CHANNEL.to_vec()));
        self.runtime
            .prepare_message(Message::ApproveAsset { cid, reason })
            .with_authentication()
            .send_to(dest);
        Ok(())
    }

    async fn on_msg_reject_asset(
        &mut self,
        cid: String,
        reason: Option<String>,
    ) -> Result<(), ReviewError> {
        let reviewer = self.require_authenticated_signer()?;
        let creation_chain =
            self.runtime.chain_id() != self.runtime.application_id().creation.chain_id;
        self._reject_asset(reviewer, cid.clone(), reason.clone(), creation_chain)
            .await?;
        if self.runtime.chain_id() != self.runtime.application_id().creation.chain_id {
            return Ok(());
        }
        let dest = Destination::Subscribers(ChannelName::from(SUBSCRIPTION_CHANNEL.to_vec()));
        self.runtime
            .prepare_message(Message::RejectAsset { cid, reason })
            .with_authentication()
            .send_to(dest);
        Ok(())
    }

    async fn on_msg_submit_asset(
        &mut self,
        cid: String,
        base_uri: String,
        uris: Vec<String>,
        price: Option<Amount>,
        name: String,
    ) -> Result<(), ReviewError> {
        let author = self.require_authenticated_signer()?;
        self._submit_asset(
            author,
            cid.clone(),
            base_uri.clone(),
            uris.clone(),
            price.clone(),
            name.clone(),
        )
        .await?;
        if self.runtime.chain_id() != self.runtime.application_id().creation.chain_id {
            return Ok(());
        }
        let dest = Destination::Subscribers(ChannelName::from(SUBSCRIPTION_CHANNEL.to_vec()));
        self.runtime
            .prepare_message(Message::SubmitAsset {
                cid,
                base_uri,
                uris,
                price,
                name,
            })
            .with_authentication()
            .send_to(dest);
        Ok(())
    }

    async fn on_msg_request_subscribe(&mut self) -> Result<(), ReviewError> {
        let message_id = self.require_message_id()?;
        // The subscribe message must be from another chain
        if message_id.chain_id == self.runtime.application_id().creation.chain_id {
            return Ok(());
        }
        log::info!(
            "message subscribe from chain {} to chain {} on creation chain {}",
            message_id.chain_id,
            self.runtime.chain_id(),
            self.runtime.application_id().creation.chain_id,
        );
        self.runtime.subscribe(
            message_id.chain_id,
            ChannelName::from(SUBSCRIPTION_CHANNEL.to_vec()),
        );
        for reviewer in self.state.reviewers.indices().await? {
            let reviewer = self.state.reviewers.get(&reviewer).await?.unwrap();
            self.runtime
                .prepare_message(Message::ExistReviewer { reviewer })
                .with_authentication()
                .send_to(self.require_message_id()?.chain_id);
        }
        self.runtime
            .prepare_message(Message::InstantiationArgument {
                argument: self.state.instantiation_argument().await?,
            })
            .with_authentication()
            .send_to(self.require_message_id()?.chain_id);
        Ok(())
    }

    async fn on_msg_approve_activity(
        &mut self,
        activity_id: u64,
        reason: Option<String>,
    ) -> Result<(), ReviewError> {
        let reviewer = self.require_authenticated_signer()?;
        let creation_chain =
            self.runtime.chain_id() != self.runtime.application_id().creation.chain_id;
        self._approve_activity(reviewer, activity_id, reason.clone(), creation_chain)
            .await?;
        if self.runtime.chain_id() != self.runtime.application_id().creation.chain_id {
            return Ok(());
        }
        let dest = Destination::Subscribers(ChannelName::from(SUBSCRIPTION_CHANNEL.to_vec()));
        self.runtime
            .prepare_message(Message::ApproveActivity {
                activity_id,
                reason,
            })
            .with_authentication()
            .send_to(dest);
        Ok(())
    }

    async fn on_msg_reject_activity(
        &mut self,
        activity_id: u64,
        reason: String,
    ) -> Result<(), ReviewError> {
        let reviewer = self.require_authenticated_signer()?;
        let creation_chain =
            self.runtime.chain_id() != self.runtime.application_id().creation.chain_id;
        self._reject_activity(reviewer, activity_id, reason.clone(), creation_chain)
            .await?;
        if self.runtime.chain_id() != self.runtime.application_id().creation.chain_id {
            return Ok(());
        }
        let dest = Destination::Subscribers(ChannelName::from(SUBSCRIPTION_CHANNEL.to_vec()));
        self.runtime
            .prepare_message(Message::RejectActivity {
                activity_id,
                reason,
            })
            .with_authentication()
            .send_to(dest);
        Ok(())
    }

    async fn on_msg_submit_activity(
        &mut self,
        activity_id: u64,
        activity_host: Owner,
        budget_amount: Amount,
    ) -> Result<(), ReviewError> {
        self._submit_activity(activity_id, activity_host, budget_amount)
            .await?;
        if self.runtime.chain_id() != self.runtime.application_id().creation.chain_id {
            return Ok(());
        }
        let dest = Destination::Subscribers(ChannelName::from(SUBSCRIPTION_CHANNEL.to_vec()));
        self.runtime
            .prepare_message(Message::SubmitActivity {
                activity_id,
                activity_host,
                budget_amount,
            })
            .with_authentication()
            .send_to(dest);
        Ok(())
    }
}
