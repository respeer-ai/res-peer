// Copyright (c) Zefchain Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

#![cfg_attr(target_arch = "wasm32", no_main)]

mod random;
mod state;
mod token;

use std::{
    io::Read,
    sync::{Arc, Mutex},
};

use async_graphql::{
    Context, Description, EmptyMutation, EmptySubscription, Object, Request, Response, Schema,
};
use candle_core::{Device, Tensor};
use candle_transformers::{generation::LogitsProcessor, models::quantized_t5 as t5};
use copilot::Operation;
use linera_sdk::{
    base::{BcsHashable, CryptoHash, Timestamp, WithServiceAbi},
    graphql::GraphQLMutationRoot,
    Service, ServiceRuntime,
};
use log::info;
use serde::{Deserialize, Serialize};
use tokenizers::Tokenizer;

use anyhow::Result as AnyResult;

use crate::token::TokenOutputStream;

pub struct CopilotService {
    model_context: Arc<ModelContext>,
}

linera_sdk::service!(CopilotService);

impl WithServiceAbi for CopilotService {
    type Abi = copilot::CopilotAbi;
}

struct QueryRoot {}

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
struct HashInput {
    buf: [u8; 32],
}

impl BcsHashable for HashInput {}

#[Object]
impl QueryRoot {
    async fn prepare_query_id(&self) -> CryptoHash {
        let mut hash_input = HashInput { buf: [0u8; 32] };
        getrandom::getrandom(&mut hash_input.buf).unwrap();
        CryptoHash::new(&hash_input)
    }

    async fn prompt(&self, ctx: &Context<'_>, prompt: String) -> String {
        let model_context = ctx.data::<Arc<ModelContext>>().unwrap();
        model_context.t5_model_builder.run_model(&prompt).unwrap()
    }
}

struct T5ModelBuilder {
    device: Device,
    config: t5::Config,
    raw_weights: Vec<u8>,
    tokenizer: Vec<u8>,
}

struct ModelContext {
    t5_model_builder: T5ModelBuilder,
    runtime: Mutex<ServiceRuntime<CopilotService>>,
}

impl Service for CopilotService {
    type Parameters = ();

    async fn new(runtime: ServiceRuntime<Self>) -> Self {
        let device = Device::Cpu;
        info!("Downloading t5 model");
        let raw_weights = runtime.fetch_url("http://localhost:10001/t5_small/model.gguf");

        info!("Downloading tokenizer");
        let tokenizer_bytes = runtime.fetch_url("http://localhost:10001/t5_small/tokenizer.json");

        info!("Downloading config");
        let config = runtime.fetch_url("http://localhost:10001/t5_small/config.json");

        let config_format: Result<String, std::string::FromUtf8Error> = String::from_utf8(config);
        let mut config_str = String::new();
        match config_format {
            Ok(valid_string) => {
                config_str = valid_string;
            }
            Err(e) => info!("Error converting string: {:?}", e),
        }

        let mut config: t5::Config =
            serde_json::from_str(config_str.as_str()).expect("invalid load config");
        config.use_cache = true;

        let t5_model_builder = T5ModelBuilder {
            device: device,
            config: config,
            raw_weights: raw_weights,
            tokenizer: tokenizer_bytes,
        };

        CopilotService {
            model_context: Arc::new(ModelContext {
                t5_model_builder,
                runtime: Mutex::new(runtime),
            }),
        }
    }

    async fn handle_query(&self, request: Request) -> Response {
        let query_string = &request.query;
        info!("query: {}", query_string);

        let schema = Schema::build(QueryRoot {}, Operation::mutation_root(), EmptySubscription)
            .data(self.model_context.clone())
            .finish();
        schema.execute(request).await
    }
}

impl T5ModelBuilder {
    pub fn build_model(&self) -> AnyResult<t5::T5ForConditionalGeneration> {
        let device = Device::Cpu;
        let vb = t5::VarBuilder::from_gguf_buffer(&self.raw_weights, &device)?;
        Ok(t5::T5ForConditionalGeneration::load(vb, &self.config)?)
    }

    fn run_model(&self, prompt_string: &str) -> Result<String, candle_core::Error> {
        let device = &self.device;
        let tokenizer_bytes = &self.tokenizer;
        let tokenizer = Tokenizer::from_bytes(tokenizer_bytes).expect("failed to create tokenizer");
        let repeat_penalty = 1.1f32;
        let repeat_last_n = 64;
        let mut output = String::new();

        let tokens = tokenizer
            .encode(prompt_string, true)
            .unwrap()
            .get_ids()
            .to_vec();

        let mut tokenizer_stream = TokenOutputStream::new(tokenizer);

        let input_token_ids = Tensor::new(&tokens[..], device)?.unsqueeze(0)?;
        let mut model = self.build_model().expect("error to load model");
        let mut output_token_ids = [self
            .config
            .decoder_start_token_id
            .unwrap_or(self.config.pad_token_id) as u32]
        .to_vec();

        let mut logits_processor = LogitsProcessor::new(299792458, None, None);
        let encoder_output = model
            .encode(&input_token_ids)
            .expect("invalid to load encode");
        let index_pos = 0;

        for index in 0.. {
            if output_token_ids.len() > 512 {
                break;
            }
            let decoder_token_ids = if index == 0 || !self.config.use_cache {
                Tensor::new(output_token_ids.as_slice(), device)?.unsqueeze(0)?
            } else {
                let last_token = *output_token_ids.last().unwrap();
                Tensor::new(&[last_token], device)?.unsqueeze(0)?
            };
            if index_pos == 256 {
                return Ok(output
                    .rsplit_once('.')
                    .map(|(before, _)| format!("{}.", before))
                    .unwrap_or_else(|| output.to_string()));
            }
            let logits = model
                .decode(&decoder_token_ids, &encoder_output)
                .expect("invalid decode")
                .squeeze(0)
                .expect("invalid decode");
            let logits = if repeat_penalty == 1. {
                logits
            } else {
                let start_at = output_token_ids.len().saturating_sub(repeat_last_n);
                candle_transformers::utils::apply_repeat_penalty(
                    &logits,
                    repeat_penalty,
                    &output_token_ids[start_at..],
                )
                .expect("invalid code")
            };

            let next_token_id = logits_processor.sample(&logits)?;
            if next_token_id as usize == self.config.eos_token_id {
                break;
            }
            output_token_ids.push(next_token_id);

            if let Some(t) = tokenizer_stream.next_token(next_token_id)? {
                let text = t.replace('▁', " ").replace("<0x0A>", "\n");
                print!("{text}");
                output.push_str(&t);
            }
        }
        if let Some(rest) = tokenizer_stream.decode_rest().unwrap() {
            output.push_str(&rest);
        }
        info!("{} tokens generated", output_token_ids.len(),);
        Ok(output)
    }
}
