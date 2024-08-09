// Copyright (c) Zefchain Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use async_graphql::SimpleObject;
use copilot::{CopilotError, DepositQuota, InstantiationArgument};
use linera_sdk::{
    base::{Amount, CryptoHash, Owner},
    views::{MapView, RegisterView, RootView, SetView, ViewStorageContext},
};

#[derive(RootView, SimpleObject)]
#[view(context = "ViewStorageContext")]
pub struct Copilot {
    price_quota: RegisterView<u16>,
    quota_price: RegisterView<Amount>,
    deposit_quotas: MapView<Owner, DepositQuota>, // TODO: in future we'll meter fee with quota, but now we can just work with task
    query_deposits: SetView<CryptoHash>,
}

#[allow(dead_code)]
impl Copilot {
    pub(crate) async fn instantiate(&mut self, argument: InstantiationArgument) {
        self.price_quota.set(argument.price_quota);
        self.quota_price.set(argument.quota_price);
    }

    pub(crate) async fn _quota_price(&mut self) -> Amount {
        *self.quota_price.get()
    }

    pub(crate) async fn query_deposited(&self, query_id: CryptoHash) -> Result<bool, CopilotError> {
        Ok(self.query_deposits.contains(&query_id).await?)
    }

    pub(crate) async fn deposit_query(&mut self, query_id: CryptoHash) -> Result<(), CopilotError> {
        Ok(self.query_deposits.insert(&query_id)?)
    }
}
