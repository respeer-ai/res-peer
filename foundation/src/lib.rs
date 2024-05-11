use std::collections::HashSet;

use async_graphql::{Enum, Request, Response};
use linera_sdk::{
    base::{Amount, ArithmeticError, ContractAbi, Owner, ServiceAbi},
    graphql::GraphQLMutationRoot,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;
pub struct FoundationAbi;

impl ContractAbi for FoundationAbi {
    type Operation = Operation;
    type Response = FoundationResponse;
}

impl ServiceAbi for FoundationAbi {
    type Query = Request;
    type QueryResponse = Response;
}

#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct InstantiationArgument {
    pub review_reward_percent: u8,
    pub review_reward_factor: u8,
    pub author_reward_percent: u8,
    pub author_reward_factor: u8,
    pub activity_reward_percent: u8,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub enum FoundationResponse {
    #[default]
    Ok,
    Balance(Amount),
}

#[derive(Debug, Deserialize, Serialize, Copy, Clone, Enum, Eq, PartialEq)]
pub enum RewardType {
    Review,
    Publish,
    Activity,
}

#[derive(Debug, Deserialize, Serialize, GraphQLMutationRoot)]
pub enum Operation {
    UserDeposit {
        amount: Amount,
    },
    RequestSubscribe,
    Deposit {
        from: Owner,
        amount: Amount,
    },
    Transfer {
        from: Owner,
        to: Owner,
        amount: Amount,
    },
    Reward {
        // Review: sender is the reward user (the reviewer)
        // Author: sender is not the reward user but the reviewer
        // Activity: sender must be the activity host
        reward_user: Option<Owner>,
        reward_type: RewardType,
        activity_id: Option<u64>,
    },
    ActivityRewards {
        activity_id: u64,
        winner_user: Owner,
        voter_users: HashSet<Owner>,
        reward_amount: Amount,
        voter_reward_percent: u8,
    },
    Lock {
        activity_id: u64,
        amount: Amount,
    },
    Balance {
        owner: Owner,
    },
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Message {
    UserDeposit {
        amount: Amount,
    },
    RequestSubscribe,
    InstantiationArgument {
        argument: InstantiationArgument,
    },
    Deposit {
        from: Owner,
        amount: Amount,
    },
    Transfer {
        from: Owner,
        to: Owner,
        amount: Amount,
    },
    Reward {
        // Review: sender is the reward user (the reviewer)
        // Author: sender is not the reward user but the reviewer
        // Activity: sender must be the activity host
        // TODO: for activity host reward we should review it
        reward_user: Option<Owner>,
        reward_type: RewardType,
        activity_id: Option<u64>,
    },
    ActivityRewards {
        activity_id: u64,
        winner_user: Owner,
        voter_users: HashSet<Owner>,
        reward_amount: Amount,
        voter_reward_percent: u8,
    },
    Lock {
        activity_id: u64,
        amount: Amount,
    },
}

/// An error that can occur during the contract execution.
#[derive(Debug, Error)]
pub enum FoundationError {
    /// Failed to deserialize BCS bytes
    #[error("Failed to deserialize BCS bytes")]
    BcsError(#[from] bcs::Error),

    /// Failed to deserialize JSON string
    #[error("Failed to deserialize JSON string")]
    JsonError(#[from] serde_json::Error),

    #[error("View error")]
    ViewError(#[from] linera_views::views::ViewError),

    #[error("Invalid user")]
    InvalidUser,

    #[error("Cross-application sessions not supported")]
    SessionsNotSupported,

    #[error("Insufficient funds")]
    InsufficientFunds,

    #[error("Invalid percent")]
    InvalidPercent,

    #[error("Insufficient balance")]
    InsufficientBalance,

    #[error("Arithmetic error")]
    ArithmeticError(#[from] ArithmeticError),

    #[error("Invalid account")]
    InvalidAccount,

    #[error("Invalid activity funds")]
    InvalidActivityFunds,

    #[error("Invalid signer")]
    InvalidSigner,

    #[error("Invalid message id")]
    InvalidMessageId,
}
