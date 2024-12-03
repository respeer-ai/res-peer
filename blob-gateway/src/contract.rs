// Copyright (c) Zefchain Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use blob_gateway::{BlobData, BlobDataType, BlobGatewayError, Operation};
use linera_sdk::{
    base::{CryptoHash, WithContractAbi},
    views::{RootView, View},
    Contract, ContractRuntime,
};

use self::state::BlobGateway;

pub struct BlobGatewayContract {
    state: BlobGateway,
    runtime: ContractRuntime<Self>,
}

linera_sdk::contract!(BlobGatewayContract);

impl WithContractAbi for BlobGatewayContract {
    type Abi = blob_gateway::BlobGatewayAbi;
}

impl Contract for BlobGatewayContract {
    type Message = ();
    type InstantiationArgument = ();
    type Parameters = ();

    async fn load(runtime: ContractRuntime<Self>) -> Self {
        let state = BlobGateway::load(runtime.root_view_storage_context())
            .await
            .expect("Failed to load state");
        BlobGatewayContract { state, runtime }
    }

    async fn instantiate(&mut self, _value: ()) {}

    async fn execute_operation(&mut self, operation: Operation) -> Self::Response {
        match operation {
            Operation::Register {
                data_type,
                blob_hash,
            } => self
                .on_op_register(data_type, blob_hash)
                .await
                .expect("Failed OP: Register"),
        }
    }

    async fn execute_message(&mut self, _message: ()) {
        panic!("BlobGateway application doesn't support any cross-chain messages");
    }

    async fn store(mut self) {
        self.state.save().await.expect("Failed to save state");
    }
}

impl BlobGatewayContract {
    async fn on_op_register(
        &mut self,
        data_type: BlobDataType,
        blob_hash: CryptoHash,
    ) -> Result<(), BlobGatewayError> {
        let creator = self.runtime.authenticated_signer().unwrap();

        // TODO: call exists api to fetch blob to current chain

        match self.state.blobs.get(&blob_hash).await? {
            Some(blob) => {
                if blob.creator == creator {
                    Ok(())
                } else {
                    Err(BlobGatewayError::AlreadyExists)
                }
            }
            _ => Ok(self.state.blobs.insert(
                &blob_hash,
                BlobData {
                    data_type,
                    blob_hash,
                    creator,
                    created_at: self.runtime.system_time(),
                },
            )?),
        }
    }
}
