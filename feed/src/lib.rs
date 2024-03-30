use std::collections::HashMap;

use async_graphql::{Request, Response, SimpleObject};
use linera_sdk::base::{Amount, ApplicationId, ContractAbi, Owner, ServiceAbi, Timestamp};
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub struct FeedAbi;

impl ContractAbi for FeedAbi {
    type Parameters = FeedParameters;
    type InitializationArgument = InitialState;
    type Operation = Operation;
    type Response = FeedResponse;
}

impl ServiceAbi for FeedAbi {
    type Parameters = FeedParameters;
    type Query = Request;
    type QueryResponse = Response;
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FeedParameters {
    pub credit_app_id: ApplicationId<credit::CreditAbi>,
    pub foundation_app_id: ApplicationId<foundation::FoundationAbi>,
}

#[derive(Debug, Deserialize, Serialize, Clone, SimpleObject, Eq, PartialEq)]
pub struct Content {
    /// Here cid is the content cid::Cid store in ipfs
    pub cid: String,
    pub comment_to_cid: Option<String>,
    pub author: Owner,
    pub title: String,
    pub content: String,
    pub likes: u64,
    pub dislikes: u64,
    pub accounts: HashMap<Owner, bool>,
    pub created_at: Timestamp,
}

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct InitialState {
    pub react_interval_ms: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Operation {
    Like {
        cid: String,
    },
    Dislike {
        cid: String,
    },
    Tip {
        cid: String,
        amount: Amount,
    },
    RequestSubscribe,
    Publish {
        cid: String,
        title: String,
        content: String,
        author: Owner,
    },
    Recommend {
        cid: String,
        reason_cid: String,
        reason: String,
    },
    Comment {
        cid: String,
        comment_cid: String,
        comment: String,
        commentor: Owner,
    },
    ContentAuthor {
        cid: String,
    },
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Message {
    Like {
        cid: String,
    },
    Dislike {
        cid: String,
    },
    Tip {
        cid: String,
        amount: Amount,
    },
    Publish {
        cid: String,
        title: String,
        content: String,
        author: Owner,
    },
    Recommend {
        cid: String,
        reason_cid: String,
        reason: String,
    },
    Comment {
        cid: String,
        comment_cid: String,
        comment: String,
        commentor: Owner,
    },
    RequestSubscribe,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub enum FeedResponse {
    #[default]
    Ok,
    ContentAuthor(Option<Owner>),
}

/// An error that can occur during the contract execution.
#[derive(Debug, Error)]
pub enum FeedError {
    /// Failed to deserialize BCS bytes
    #[error("Failed to deserialize BCS bytes")]
    BcsError(#[from] bcs::Error),

    /// Failed to deserialize JSON string
    #[error("Failed to deserialize JSON string")]
    JsonError(#[from] serde_json::Error),
    // Add more error variants here.
    #[error("Invalid publisher")]
    InvalidPublisher,

    #[error("Cross-application sessions not supported")]
    SessionsNotSupported,

    #[error("Content already exists")]
    AlreadyExists,

    #[error("Content not exist")]
    NotExist,

    #[error("Only 1 reaction is allowed within 1 minute")]
    TooFrequently,

    #[error("Only 1 like is allowed for each content")]
    TooManyLike,

    #[error("Invalid content")]
    InvalidContent,

    #[error("Invalid signer")]
    InvalidSigner,

    #[error("Invalid message id")]
    InvalidMessageId,

    #[error("View error")]
    ViewError(#[from] linera_views::views::ViewError),
}
