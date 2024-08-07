// Copyright (c) Zefchain Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use async_graphql::SimpleObject;
use cp_registry::{CPNode, CPRegistryError, RegisterParameters};
use linera_sdk::{base::CryptoHash, views::{linera_views, MapView, RootView, SetView, ViewStorageContext}};

#[derive(RootView, SimpleObject)]
#[view(context = "ViewStorageContext")]
pub struct CPRegistry {
    // link -> node
    nodes: MapView<CryptoHash, CPNode>,
    links: SetView<String>
}

impl CPRegistry {
    pub(crate) async fn register_cp_node(&mut self, params: RegisterParameters) -> Result<CryptoHash, CPRegistryError> {
        if self.links.contains(&params.link).await? {
            return Err(CPRegistryError::AlreadyRegistered);
        }
        let node: CPNode = params.into();
        self.nodes.insert(&node.node_id, node.clone())?;
        self.links.insert(&node.link)?;
        Ok(node.node_id)
    }
}
