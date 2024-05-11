use std::collections::HashMap;

use async_graphql::{Request, Response, SimpleObject};
use linera_sdk::{
    base::{Amount, ApplicationId, ArithmeticError, ContractAbi, Owner, ServiceAbi, Timestamp},
    graphql::GraphQLMutationRoot,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub struct MarketAbi;

impl ContractAbi for MarketAbi {
    type Operation = Operation;
    type Response = ();
}

impl ServiceAbi for MarketAbi {
    type Query = Request;
    type QueryResponse = Response;
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MarketParameters {
    pub credit_app_id: ApplicationId<credit::CreditAbi>,
    pub foundation_app_id: ApplicationId<foundation::FoundationAbi>,
}

#[derive(Debug, Deserialize, Serialize, Clone, SimpleObject, Eq, PartialEq)]
pub struct NFT {
    /// Sequence ID of NFT in collections
    pub token_id: u16,
    /// Storage location of http or ipfs
    pub uri_index: u16,
    /// Price in Linera Token
    pub price: Option<Amount>,
    pub on_sale: bool,
    pub minted_at: Timestamp,
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, SimpleObject, Eq, PartialEq)]
pub struct Collection {
    pub collection_id: u64,
    pub base_uri: String,
    pub uris: Vec<String>,
    pub nfts: HashMap<u16, NFT>,
    pub price: Option<Amount>,
    pub name: String,
    pub created_at: Timestamp,
    pub publisher: Owner,
}

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct InstantiationArgument {
    pub credits_per_linera: Amount,
    pub max_credits_percent: u8,
    pub trade_fee_percent: u8,
    pub collection_id: Option<u64>,
}

#[derive(Debug, Deserialize, Serialize, GraphQLMutationRoot)]
#[allow(clippy::large_enum_variant)]
pub enum Operation {
    MintNFT {
        collection_id: u64,
        uri_index: u16,
        price: Option<Amount>,
        name: String,
    },
    BuyNFT {
        collection_id: u64,
        token_id: u16,
        credits: Amount,
    },
    UpdateCreditsPerLinera {
        credits_per_linera: Amount,
    },
    UpdateNFTPrice {
        collection_id: u64,
        token_id: Option<u16>,
        price: Amount,
    },
    OnSaleNFT {
        collection_id: u64,
        token_id: u16,
    },
    OffSaleNFT {
        collection_id: u64,
        token_id: u16,
    },
    SetAvatar {
        collection_id: u64,
        token_id: u16,
    },
    RequestSubscribe,
    CreateCollection {
        base_uri: String,
        price: Option<Amount>,
        name: String,
        uris: Vec<String>,
        publisher: Owner,
    },
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[allow(clippy::large_enum_variant)]
pub enum Message {
    InstantiationArgument {
        argument: InstantiationArgument,
    },
    CreateCollection {
        base_uri: String,
        price: Option<Amount>,
        name: String,
        uris: Vec<String>,
        publisher: Owner,
    },
    MintNFT {
        collection_id: u64,
        uri_index: u16,
        price: Option<Amount>,
        name: String,
    },
    BuyNFT {
        collection_id: u64,
        token_id: u16,
        credits: Amount,
    },
    UpdateCreditsPerLinera {
        credits_per_linera: Amount,
    },
    UpdateNFTPrice {
        collection_id: u64,
        token_id: Option<u16>,
        price: Amount,
    },
    OnSaleNFT {
        collection_id: u64,
        token_id: u16,
    },
    OffSaleNFT {
        collection_id: u64,
        token_id: u16,
    },
    SetAvatar {
        collection_id: u64,
        token_id: u16,
    },
    RequestSubscribe,
}

/// An error that can occur during the contract execution.
#[derive(Debug, Error)]
pub enum MarketError {
    /// Failed to deserialize BCS bytes
    #[error("Failed to deserialize BCS bytes")]
    BcsError(#[from] bcs::Error),

    /// Failed to deserialize JSON string
    #[error("Failed to deserialize JSON string")]
    JsonError(#[from] serde_json::Error),
    // Add more error variants here.
    #[error("NOT IMPLEMENTED")]
    NotImplemented,

    #[error("Operation not allowed")]
    OperationNotAllowed,

    #[error("Invalid owner")]
    InvalidOwner,

    #[error("Cross-application sessions not supported")]
    SessionsNotSupported,

    #[error(transparent)]
    ViewError(#[from] linera_views::views::ViewError),

    #[error(transparent)]
    ArithmeticError(#[from] ArithmeticError),

    #[error("Owner is not collection owner")]
    NotCollectionOwner,

    #[error("Owner is not token owner")]
    NotTokenOwner,

    #[error("Base uri already exists")]
    BaseURIALreadyExists,

    #[error("Collection not exists")]
    CollectionNotExists,

    #[error("Token ID not exists")]
    TokenIDNotExists,

    #[error("NFT not on sale")]
    TokenNotOnSale,

    #[error("Invalid price")]
    InvalidPrice,

    #[error("Buyer is same as owner")]
    BuyerIsOwner,

    #[error("Invalid uri index")]
    InvalidUriIndex,

    #[error("Invalid signer")]
    InvalidSigner,

    #[error("Invalid message id")]
    InvalidMessageId,
}
