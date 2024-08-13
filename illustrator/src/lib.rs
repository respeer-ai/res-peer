// Copyright (c) Zefchain Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use async_graphql::{Request, Response};
use linera_sdk::base::{ContractAbi, ServiceAbi};

pub struct IllustratorAbi;

impl ContractAbi for IllustratorAbi {
    type Operation = ();
    type Response = ();
}

impl ServiceAbi for IllustratorAbi {
    type Query = Request;
    type QueryResponse = Response;
}
