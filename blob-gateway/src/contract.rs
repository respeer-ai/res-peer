// Copyright (c) Zefchain Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use linera_sdk::{base::WithContractAbi, Contract, ContractRuntime};

pub struct BlobGatewayContract;

linera_sdk::contract!(BlobGatewayContract);

impl WithContractAbi for BlobGatewayContract {
    type Abi = blob_gateway::BlobGatewayAbi;
}

impl Contract for BlobGatewayContract {
    type Message = ();
    type InstantiationArgument = ();
    type Parameters = ();

    async fn load(_runtime: ContractRuntime<Self>) -> Self {
        BlobGatewayContract
    }

    async fn instantiate(&mut self, _value: ()) {}

    async fn execute_operation(&mut self, _operation: ()) -> Self::Response {}

    async fn execute_message(&mut self, _message: ()) {
        panic!("BlobGateway application doesn't support any cross-chain messages");
    }

    async fn store(self) {}
}
