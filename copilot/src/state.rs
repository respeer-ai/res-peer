// Copyright (c) Zefchain Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use async_graphql::SimpleObject;
use copilot::DepositQuota;
use linera_sdk::{
    base::{Amount, Owner},
    views::{MapView, RegisterView, RootView, ViewStorageContext},
};

#[derive(RootView, SimpleObject)]
#[view(context = "ViewStorageContext")]
pub struct Copilot {
    price_quota: RegisterView<u16>,
    quota_price: RegisterView<Amount>,
    deposit_quotas: MapView<Owner, DepositQuota>,
}
