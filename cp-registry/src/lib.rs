// Copyright (c) Zefchain Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use async_graphql::{Enum, InputObject, Request, Response, SimpleObject};
use linera_sdk::{
    base::{
        Amount, ApplicationId, BcsHashable, ChainId, ContractAbi, CryptoHash, ServiceAbi, Timestamp,
    },
    graphql::GraphQLMutationRoot,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub struct CPRegistryAbi;

impl ContractAbi for CPRegistryAbi {
    type Operation = Operation;
    type Response = CPRegistryResponse;
}

impl ServiceAbi for CPRegistryAbi {
    type Query = Request;
    type QueryResponse = Response;
}

#[derive(Debug, Deserialize, Serialize, Clone, Eq, PartialEq, Enum, Copy, Ord, PartialOrd)]
pub enum ResourceType {
    CPU,
    GPU,
}

#[derive(Debug, Deserialize, Serialize, Clone, Eq, PartialEq, Enum, Copy, Ord, PartialOrd)]
pub enum StorageType {
    NVME,
    SSD,
    HDD,
}

#[derive(Debug, Deserialize, Serialize, Clone, Eq, PartialEq, Enum, Copy, Ord, PartialOrd)]
pub enum TaskType {
    FixGrammar,
    RewriteEasierUnderstand,
    Paraphrase,
    WriteFormally,
    WriteMoreNeutral,
    GenerateIllustrate,
}

#[derive(Debug, Deserialize, Serialize, Clone, SimpleObject, Eq, PartialEq)]
pub struct CPNode {
    pub node_id: CryptoHash,
    pub brand_logo: String,
    pub brand_name: String,
    pub link: String,
    pub application_id: ApplicationId,
    pub resource_type: ResourceType,
    pub device_model: String,
    pub cpu_model: String,
    pub storage_type: StorageType,
    pub storage_bytes: u64,
    pub memory_bytes: u64,
    pub free_quota: u32,
    pub price_quota: u16,
    pub quota_price: Amount,
    pub supported_task_types: Vec<TaskType>,
    pub ai_model: String,
    pub ai_model_url: String,
    pub payment_chain_id: ChainId,
    pub available: bool,
    pub created_at: Timestamp,
}

#[derive(Debug, Deserialize, Serialize, Clone, SimpleObject, InputObject)]
pub struct RegisterParameters {
    pub node_id: Option<CryptoHash>,
    pub brand_logo: String,
    pub brand_name: String,
    pub link: String,
    pub application_id: ApplicationId,
    pub resource_type: ResourceType,
    pub device_model: String,
    pub cpu_model: String,
    pub storage_type: StorageType,
    pub storage_bytes: u64,
    pub memory_bytes: u64,
    pub free_quota: u32,
    pub price_quota: u16,
    pub quota_price: Amount,
    pub supported_task_types: Vec<TaskType>,
    pub ai_model: String,
    pub ai_model_url: String,
    pub payment_chain_id: ChainId,
}

impl BcsHashable for RegisterParameters {}

impl Into<CPNode> for RegisterParameters {
    fn into(self) -> CPNode {
        CPNode {
            node_id: self.node_id.unwrap_or(CryptoHash::new(&self)),
            brand_logo: self.brand_logo,
            brand_name: self.brand_name,
            link: self.link,
            application_id: self.application_id,
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
            ai_model: self.ai_model,
            ai_model_url: self.ai_model_url,
            payment_chain_id: self.payment_chain_id,
            available: true,
            created_at: 0.into(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, SimpleObject, InputObject)]
pub struct UpdateParameters {
    pub node_id: CryptoHash,
    pub brand_logo: Option<String>,
    pub brand_name: Option<String>,
    pub link: Option<String>,
    pub resource_type: Option<ResourceType>,
    pub device_model: Option<String>,
    pub cpu_model: Option<String>,
    pub storage_type: Option<StorageType>,
    pub storage_bytes: Option<u64>,
    pub memory_bytes: Option<u64>,
    pub free_quota: Option<u32>,
    pub price_quota: Option<u16>,
    pub quota_price: Option<Amount>,
    pub supported_task_types: Option<Vec<TaskType>>,
    pub ai_model: Option<String>,
    pub ai_model_url: Option<String>,
    pub payment_chain_id: Option<ChainId>,
    pub available: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize, GraphQLMutationRoot)]
pub enum Operation {
    Register { params: RegisterParameters },
    Update { params: UpdateParameters },
    Deregister { node_id: CryptoHash },
    RequestSubscribe,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Message {
    Register { params: RegisterParameters },
    Update { params: UpdateParameters },
    Deregister { node_id: CryptoHash },
    RequestSubscribe,
    ExistNode { node: CPNode },
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub enum CPRegistryResponse {
    #[default]
    Ok,
    NodeId(CryptoHash),
}

#[derive(Debug, Error)]
#[allow(dead_code)]
pub enum CPRegistryError {
    #[error("Link is already registered")]
    AlreadyRegistered,

    #[error("Invalid node")]
    InvalidNode,

    #[error("Invalid message id")]
    InvalidMessageId,

    #[error(transparent)]
    LowLevelError(#[from] anyhow::Error),

    #[error(transparent)]
    ViewError(#[from] linera_views::views::ViewError),
}
