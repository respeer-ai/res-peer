// Copyright (c) Zefchain Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use async_graphql::{Request, Response, SimpleObject};
use linera_sdk::{
    base::{
        Amount, ApplicationId, BcsHashable, ChainId, ContractAbi, CryptoError, CryptoHash,
        ServiceAbi,
    },
    graphql::GraphQLMutationRoot,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub struct IllustratorAbi;

impl ContractAbi for IllustratorAbi {
    type Operation = Operation;
    type Response = IllustratorResponse;
}

impl ServiceAbi for IllustratorAbi {
    type Query = Request;
    type QueryResponse = Response;
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct IllustratorParameters {
    pub cp_registry_app_id: ApplicationId<cp_registry::CPRegistryAbi>,
}

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct InstantiationArgument {
    pub brand_logo: String,
    pub brand_name: String,
    pub link_base: String,
    pub resource_type: cp_registry::ResourceType,
    pub device_model: String,
    pub cpu_model: String,
    pub storage_type: cp_registry::StorageType,
    pub storage_bytes: u64,
    pub memory_bytes: u64,
    pub free_quota: u32,
    pub price_quota: u16,
    pub quota_price: Amount,
    pub supported_task_types: Vec<cp_registry::TaskType>,
    pub fetch_server_url: Option<String>,
}

impl BcsHashable for InstantiationArgument {}

impl Into<cp_registry::RegisterParameters> for InstantiationArgument {
    fn into(self) -> cp_registry::RegisterParameters {
        cp_registry::RegisterParameters {
            node_id: Some(CryptoHash::new(&self)),
            brand_logo: self.brand_logo,
            brand_name: self.brand_name,
            link: self.link_base,
            application_id: ApplicationId::from_str(
                "1db1936dad0717597a7743a8353c9c0191c14c3a129b258e9743aec2b4f05d031800000000000000000000001db1936dad0717597a7743a8353c9c0191c14c3a129b258e9743aec2b4f05d031a0000000000000000000000",
            ).
            expect("Invalid applicationId"),
            resource_type: self.resource_type,
            device_model: self.device_model,
            cpu_model: self.cpu_model,
            storage_type: self.storage_type,
            storage_bytes: self.storage_bytes,
            memory_bytes: self.memory_bytes,
            free_quota: self.free_quota,
            price_quota: self.price_quota,
            quota_price: self.quota_price,
            supported_task_types: self.supported_task_types,
            payment_chain_id: ChainId::from_str(
                "1db1936dad0717597a7743a8353c9c0191c14c3a129b258e9743aec2b4f05d03",
            )
            .expect("Invalid chainId"),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, SimpleObject, Eq, PartialEq)]
pub struct DepositQuota {
    finished_quota: u64,
    deposit_quota: u64,
}

#[derive(Debug, Deserialize, Serialize, GraphQLMutationRoot)]
pub enum Operation {
    Deposit { query_id: CryptoHash },
    RequestSubscribe,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Message {
    Deposit {
        query_id: CryptoHash,
    },
    Pay {
        query_id: CryptoHash,
        amount: Amount,
    },
    Paid {
        query_id: CryptoHash,
    },
    RequestSubscribe,
    QuotaPrice {
        amount: Amount,
    },
}

#[derive(Debug, Error)]
#[allow(dead_code)]
pub enum IllustratorError {
    #[error("Invalid query")]
    InvalidQuery,

    #[error("Stale query")]
    StaleQuery,

    #[error("Unpaid query")]
    UnpaidQuery,

    #[error("Invalid pay chain")]
    InvalidPayChain,

    #[error("Insufficient funds")]
    InsufficientFunds,

    #[error("Invalid messageId")]
    InvalidMessageId,

    #[error(transparent)]
    CryptoError(#[from] CryptoError),

    #[error(transparent)]
    CandleError(#[from] candle_core::Error),

    #[error(transparent)]
    ViewError(#[from] linera_sdk::views::ViewError),

    #[error(transparent)]
    SignatureError(#[from] ed25519_dalek::SignatureError),

    #[error(transparent)]
    HexError(#[from] hex::FromHexError),
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub enum IllustratorResponse {
    #[default]
    Ok,
}
