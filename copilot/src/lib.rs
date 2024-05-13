// Copyright (c) Zefchain Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use async_graphql::{Request, Response};
use linera_sdk::base::{ContractAbi, ServiceAbi};

pub struct CopilotAbi;

impl ContractAbi for CopilotAbi {
    type Operation = ();
    type Response = ();
}

impl ServiceAbi for CopilotAbi {
    type Query = Request;
    type QueryResponse = Response;
}
