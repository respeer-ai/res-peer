use std::collections::HashMap;

use async_graphql::{Request, Response, SimpleObject};
use linera_sdk::{
    base::{
        Amount, ApplicationId, ArithmeticError, ChainId, ContractAbi, Owner, ServiceAbi, Timestamp,
    },
    graphql::GraphQLMutationRoot,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub struct ReviewAbi;

impl ContractAbi for ReviewAbi {
    type Parameters = ReviewParameters;
    type InitializationArgument = InitializationArgument;
    type Operation = Operation;
    type Response = ReviewResponse;
}

impl ServiceAbi for ReviewAbi {
    type Parameters = ReviewParameters;
    type Query = Request;
    type QueryResponse = Response;
}

#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize, Default)]
pub enum ReviewResponse {
    #[default]
    Ok,
    Approved(bool),
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ReviewParameters {
    pub feed_app_id: ApplicationId<feed::FeedAbi>,
    pub credit_app_id: ApplicationId<credit::CreditAbi>,
    pub foundation_app_id: ApplicationId<foundation::FoundationAbi>,
    pub market_app_id: ApplicationId<market::MarketAbi>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct InitializationArgument {
    pub content_approved_threshold: u16,
    pub content_rejected_threshold: u16,
    pub asset_approved_threshold: u16,
    pub asset_rejected_threshold: u16,
    pub reviewer_approved_threshold: u16,
    pub reviewer_rejected_threshold: u16,
    pub activity_approved_threshold: u16,
    pub activity_rejected_threshold: u16,
}

#[derive(Debug, Deserialize, Serialize, Clone, SimpleObject, Eq, PartialEq)]
pub struct Review {
    pub reviewer: Owner,
    pub approved: bool,
    pub reason: String,
    pub created_at: Timestamp,
}

#[derive(Debug, Deserialize, Serialize, Clone, SimpleObject, Eq, PartialEq)]
pub struct Reviewer {
    pub chain_id: ChainId,
    pub reviewer: Owner,
    pub resume: Option<String>,
    pub reviewers: HashMap<Owner, Review>,
    pub approved: u16,
    pub rejected: u16,
    pub created_at: Timestamp,
}

#[derive(Debug, Deserialize, Serialize, Clone, SimpleObject, Eq, PartialEq)]
pub struct Content {
    /// Here cid is the content cid::Cid store in ipfs
    pub cid: String,
    pub comment_to_cid: Option<String>,
    pub author: Owner,
    pub title: String,
    pub content: String,
    pub reviewers: HashMap<Owner, Review>,
    pub approved: u16,
    pub rejected: u16,
    pub created_at: Timestamp,
}

#[derive(Debug, Deserialize, Serialize, Clone, SimpleObject, Eq, PartialEq)]
pub struct Asset {
    pub cid: String,
    pub base_uri: String,
    pub uris: Vec<String>,
    pub author: Owner,
    pub price: Option<Amount>,
    pub name: String,
    pub reviewers: HashMap<Owner, Review>,
    pub approved: u16,
    pub rejected: u16,
    pub created_at: Timestamp,
}

#[derive(Debug, Deserialize, Serialize, Clone, SimpleObject, Eq, PartialEq)]
pub struct Activity {
    pub activity_id: u64,
    pub activity_host: Owner,
    pub budget_amount: Amount,
    pub approved: u16,
    pub rejected: u16,
    pub created_at: Timestamp,
    pub reviewers: HashMap<Owner, Review>,
}

#[derive(Debug, Deserialize, Serialize, GraphQLMutationRoot)]
pub enum Operation {
    ApplyReviewer {
        resume: String,
    },
    UpdateReviewerResume {
        resume: String,
    },
    ApproveReviewer {
        candidate: Owner,
        reason: Option<String>,
    },
    RejectReviewer {
        candidate: Owner,
        reason: Option<String>,
    },
    SubmitContent {
        cid: String,
        title: String,
        content: String,
    },
    ApproveContent {
        content_cid: String,
        reason_cid: Option<String>,
        reason: Option<String>,
    },
    RejectContent {
        content_cid: String,
        reason: Option<String>,
    },
    SubmitComment {
        cid: String,
        comment_cid: String,
        comment: String,
    },
    ApproveAsset {
        cid: String,
        reason: Option<String>,
    },
    RejectAsset {
        cid: String,
        reason: Option<String>,
    },
    SubmitAsset {
        cid: String,
        base_uri: String,
        uris: Vec<String>,
        price: Option<Amount>,
        name: String,
    },
    RequestSubscribe,
    SubmitActivity {
        activity_id: u64,
        activity_host: Owner,
        budget_amount: Amount,
    },
    ApproveActivity {
        activity_id: u64,
        reason: Option<String>,
    },
    RejectActivity {
        activity_id: u64,
        reason: String,
    },
    ActivityApproved {
        activity_id: u64,
    },
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Message {
    GenesisReviewer,
    ExistReviewer {
        reviewer: Reviewer,
    },
    ApplyReviewer {
        resume: String,
    },
    UpdateReviewerResume {
        resume: String,
    },
    ApproveReviewer {
        candidate: Owner,
        reason: Option<String>,
    },
    RejectReviewer {
        candidate: Owner,
        reason: Option<String>,
    },
    SubmitContent {
        cid: String,
        title: String,
        content: String,
    },
    ApproveContent {
        content_cid: String,
        reason_cid: Option<String>,
        reason: Option<String>,
    },
    RejectContent {
        content_cid: String,
        reason: Option<String>,
    },
    SubmitComment {
        cid: String,
        comment_cid: String,
        comment: String,
    },
    ApproveAsset {
        cid: String,
        reason: Option<String>,
    },
    RejectAsset {
        cid: String,
        reason: Option<String>,
    },
    SubmitAsset {
        cid: String,
        base_uri: String,
        uris: Vec<String>,
        price: Option<Amount>,
        name: String,
    },
    RequestSubscribe,
    InitializationArgument {
        argument: InitializationArgument,
    },
    SubmitActivity {
        activity_id: u64,
        activity_host: Owner,
        budget_amount: Amount,
    },
    ApproveActivity {
        activity_id: u64,
        reason: Option<String>,
    },
    RejectActivity {
        activity_id: u64,
        reason: String,
    },
}

#[derive(Debug, Error)]
pub enum ReviewError {
    #[error("View error")]
    ViewError(#[from] linera_views::views::ViewError),

    #[error("Arithmetic error")]
    ArithmeticError(#[from] ArithmeticError),

    #[error("Invalid reviewer")]
    InvalidReviewer,

    #[error("Already reviewed")]
    AlreadyReviewed,

    #[error("Invalid content")]
    InvalidContent,

    #[error("Already exists")]
    AlreadyExists,

    #[error("Invalid activity")]
    InvalidActivity,

    #[error("Failed to deserialize BCS bytes")]
    BcsError(#[from] bcs::Error),

    /// Failed to deserialize JSON string
    #[error("Failed to deserialize JSON string")]
    JsonError(#[from] serde_json::Error),

    #[error("Invalid user")]
    InvalidUser,

    #[error("Cross-application sessions not supported")]
    SessionsNotSupported,

    #[error("Invalid signer")]
    InvalidSigner,

    #[error("Invalid message id")]
    InvalidMessageId,
}
