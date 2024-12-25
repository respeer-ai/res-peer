// Copyright (c) Zefchain Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use async_graphql::{Enum, Request, Response, SimpleObject};
use linera_sdk::{
    base::{ContractAbi, CryptoHash, Owner, ServiceAbi, Timestamp},
    graphql::GraphQLMutationRoot,
    DataBlobHash,
};
use linera_views::views::ViewError;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Deserialize, Serialize, Default)]
pub enum BlobGatewayResponse {
    #[default]
    Ok,
}

pub struct BlobGatewayAbi;

impl ContractAbi for BlobGatewayAbi {
    type Operation = Operation;
    type Response = BlobGatewayResponse;
}

impl ServiceAbi for BlobGatewayAbi {
    type Query = Request;
    type QueryResponse = Response;
}

#[derive(Debug, Deserialize, Serialize, Clone, Eq, PartialEq, Enum, Copy)]
pub enum BlobDataType {
    Image,
    Video,
    Html,
    Raw,
}

#[derive(Debug, Deserialize, Serialize, Clone, SimpleObject, Eq, PartialEq)]
pub struct BlobData {
    pub data_type: BlobDataType,
    pub blob_hash: CryptoHash,
    pub created_at: Timestamp,
    pub creator: Owner,
}

#[derive(Debug, Deserialize, Serialize, GraphQLMutationRoot)]
pub enum Operation {
    Register {
        data_type: BlobDataType,
        blob_hash: CryptoHash,
    },
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Message {
    Register {
        data_type: BlobDataType,
        blob_hash: CryptoHash,
    },
    AssertDataBlobExists {
        data_blob_hash: DataBlobHash,
    },
}

#[derive(Debug, Error)]
#[allow(dead_code)]
pub enum BlobGatewayError {
    #[error("Already exists")]
    AlreadyExists,

    #[error(transparent)]
    ViewError(#[from] ViewError),
}
