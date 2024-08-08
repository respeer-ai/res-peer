// Copyright (c) Zefchain Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use async_graphql::SimpleObject;
use cp_registry::{CPNode, CPRegistryError, RegisterParameters, UpdateParameters};
use linera_sdk::{
    base::CryptoHash,
    views::{linera_views, MapView, RootView, SetView, ViewStorageContext},
};

#[derive(RootView, SimpleObject)]
#[view(context = "ViewStorageContext")]
pub struct CPRegistry {
    // link -> node
    nodes: MapView<CryptoHash, CPNode>,
    links: SetView<String>,
}

#[allow(dead_code)]
impl CPRegistry {
    pub(crate) async fn exist_node_with_link(&self, link: String) -> Result<bool, CPRegistryError> {
        Ok(self.links.contains(&link).await?)
    }

    pub(crate) async fn exist_node_with_id(
        &self,
        node_id: CryptoHash,
    ) -> Result<bool, CPRegistryError> {
        Ok(self.nodes.contains_key(&node_id).await?)
    }

    pub(crate) async fn register_cp_node(
        &mut self,
        params: RegisterParameters,
    ) -> Result<CryptoHash, CPRegistryError> {
        if self.links.contains(&params.link).await? {
            return Err(CPRegistryError::AlreadyRegistered);
        }
        let node: CPNode = params.into();
        self.nodes.insert(&node.node_id, node.clone())?;
        self.links.insert(&node.link)?;
        Ok(node.node_id)
    }

    pub(crate) async fn update_cp_node(
        &mut self,
        params: UpdateParameters,
    ) -> Result<(), CPRegistryError> {
        let mut node = self
            .nodes
            .get(&params.node_id)
            .await?
            .expect("Invalid node");
        if let Some(brand_logo) = params.brand_logo {
            node.brand_logo = brand_logo;
        }
        if let Some(brand_name) = params.brand_name {
            node.brand_name = brand_name;
        }
        if let Some(link) = params.link {
            node.link = link;
        }
        if let Some(resource_type) = params.resource_type {
            node.resource_type = resource_type;
        }
        if let Some(device_mode) = params.device_model {
            node.device_model = device_mode;
        }
        if let Some(cpu_model) = params.cpu_model {
            node.cpu_model = cpu_model;
        }
        if let Some(storage_type) = params.storage_type {
            node.storage_type = storage_type
        }
        if let Some(storage_bytes) = params.storage_bytes {
            node.storage_bytes = storage_bytes;
        }
        if let Some(memory_bytes) = params.memory_bytes {
            node.memory_bytes = memory_bytes;
        }
        if let Some(free_quota) = params.free_quota {
            node.free_quota = free_quota;
        }
        if let Some(price_quota) = params.price_quota {
            node.price_quota = price_quota;
        }
        if let Some(quota_price) = params.quota_price {
            node.quota_price = quota_price;
        }
        if let Some(supported_task_types) = params.supported_task_types {
            node.supported_task_types = supported_task_types;
        }
        if let Some(payment_chain_id) = params.payment_chain_id {
            node.payment_chain_id = payment_chain_id;
        }
        if let Some(available) = params.available {
            node.available = available;
        }
        self.nodes.insert(&params.node_id, node)?;
        Ok(())
    }

    pub(crate) async fn deregister_cp_node(
        &mut self,
        node_id: CryptoHash,
    ) -> Result<(), CPRegistryError> {
        Ok(self.nodes.remove(&node_id)?)
    }
}
