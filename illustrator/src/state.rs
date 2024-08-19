// Copyright (c) Zefchain Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use async_graphql::SimpleObject;
use illustrator::{DepositQuota, IllustratorError, InstantiationArgument};
use linera_sdk::{
    base::{Amount, CryptoHash, Owner},
    views::{MapView, RegisterView, RootView, ViewStorageContext},
};

#[derive(RootView, SimpleObject)]
#[view(context = "ViewStorageContext")]
pub struct Illustrator {
    free_quota: RegisterView<u32>,
    price_quota: RegisterView<u16>,
    quota_price: RegisterView<Amount>,
    deposit_quotas: MapView<Owner, DepositQuota>, // TODO: in future we'll meter fee with quota, but now we can just work with task
    query_deposits: MapView<Owner, Vec<CryptoHash>>,
    fetch_server_url: RegisterView<Option<String>>,
}

#[allow(dead_code)]
impl Illustrator {
    pub(crate) async fn instantiate(&mut self, argument: InstantiationArgument) {
        self.price_quota.set(argument.price_quota);
        self.quota_price.set(argument.quota_price);
        self.free_quota.set(argument.free_quota);
        self.fetch_server_url.set(argument.fetch_server_url);
    }

    pub(crate) async fn _quota_price(&self) -> Amount {
        *self.quota_price.get()
    }

    pub(crate) async fn set_quota_price(&mut self, amount: Amount) {
        self.quota_price.set(amount);
    }

    pub(crate) async fn free_query(&mut self, owner: Owner) -> Result<bool, IllustratorError> {
        match self.query_deposits.get(&owner).await? {
            Some(queries) => Ok((queries.len() as u32) < *self.free_quota.get()),
            _ => Ok(true),
        }
    }

    pub(crate) async fn query_deposited(
        &self,
        owner: Owner,
        query_id: CryptoHash,
    ) -> Result<bool, IllustratorError> {
        match self.query_deposits.get(&owner).await? {
            Some(queries) => Ok(queries.contains(&query_id)),
            _ => Ok(false),
        }
    }

    pub(crate) async fn deposit_query(
        &mut self,
        owner: Owner,
        query_id: CryptoHash,
    ) -> Result<(), IllustratorError> {
        let mut queries = if let Some(queries) = self.query_deposits.get(&owner).await? {
            queries
        } else {
            Vec::new()
        };
        queries.push(query_id);
        Ok(self.query_deposits.insert(&owner, queries)?)
    }

    pub(crate) async fn query_fetch_server_url(&self) -> String {
        match &self.fetch_server_url.get() {
            None => String::new(),
            Some(url) => {
                let fetch_url = url.to_string();
                fetch_url
            }
        }
    }
}
