// Copyright (c) Zefchain Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use async_graphql::SimpleObject;
use blob_gateway::BlobData;
use linera_sdk::{
    base::CryptoHash,
    views::{linera_views, MapView, RootView, ViewStorageContext},
};

#[derive(RootView, SimpleObject)]
#[view(context = "ViewStorageContext")]
pub struct BlobGateway {
    pub blobs: MapView<CryptoHash, BlobData>,
}
