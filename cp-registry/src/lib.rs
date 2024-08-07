// Copyright (c) Zefchain Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use async_graphql::{Enum, InputObject, Request, Response, SimpleObject};
use linera_sdk::{base::{Amount, ChainId, ContractAbi, CryptoHash, ServiceAbi}, graphql::GraphQLMutationRoot};
use serde::{Deserialize, Serialize};

pub struct CPRegistryAbi;

impl ContractAbi for CPRegistryAbi {
    type Operation = Operation;
    type Response = CPRegistryResponse;
}

impl ServiceAbi for CPRegistryAbi {
    type Query = Request;
    type QueryResponse = Response;
}

#[derive(Debug, Deserialize, Serialize, Clone, Eq, PartialEq, Enum, Copy)]
pub enum ResourceType {
    CPU,
    GPU
}

#[derive(Debug, Deserialize, Serialize, Clone, Eq, PartialEq, Enum, Copy)]
pub enum StorageType {
    NVME,
    SSD,
    HDD
}

#[derive(Debug, Deserialize, Serialize, Clone, Eq, PartialEq, Enum, Copy)]
pub enum TaskType {
    FixGrammar,
    RewriteEasierUnderstand,
    Paraphrase,
    WriteFormally,
    WriteMoreNeutral,
    GenerateIllustrate
}

#[derive(Debug, Deserialize, Serialize, Clone, SimpleObject, InputObject)]
pub struct RegisterParameters {
    pub brand_logo: String,
    pub brand_name: String,
    pub link: String,
    pub resource_type: ResourceType,
    pub device_model: String,
    pub cpu_model: String,
    pub storage_type: StorageType,
    pub storage_bytes: u64,
    pub memory_bytes: u64,
    pub free_quota: u32,
    pub price_quota: u32,
    pub quota_price: Amount,
    pub supported_task_types: Vec<TaskType>,
    pub payment_chain_id: ChainId
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
    pub price_quota: Option<u32>,
    pub quota_price: Option<Amount>,
    pub supported_task_types: Option<Vec<TaskType>>,
    pub payment_chain_id: Option<ChainId>
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
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub enum CPRegistryResponse {
    #[default]
    Ok,
    NodeId(CryptoHash)
}
