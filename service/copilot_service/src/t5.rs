use std::io::Write;
use std::path::PathBuf;

use candle_transformers::models::quantized_t5 as t5;

use anyhow::{Error as E, Result as AnyResult};
use candle_core::{Device, Tensor, Result as CandleResult, utils::{cuda_is_available, metal_is_available}};
use candle_transformers::generation::LogitsProcessor;
use clap::{Parser, ValueEnum};
use hf_hub::{api::sync::Api, api::sync::ApiRepo, Repo, RepoType};
use tokenizers::Tokenizer;

#[derive(Clone, Debug, Copy, ValueEnum)]
enum Which {
    T5Small,
    FlanT5Small,
    FlanT5Base,
    FlanT5Large,
    FlanT5Xl,
    FlanT5Xxl,
}

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Enable tracing (generates a trace-timestamp.json file).
    #[arg(long)]
    tracing: bool,

    /// The model repository to use on the HuggingFace hub.
    #[arg(long)]
    model_id: Option<String>,

    #[arg(long)]
    revision: Option<String>,

    #[arg(long)]
    weight_file: Option<String>,

    #[arg(long)]
    config_file: Option<String>,

    #[arg(long)]
    tokenizer: Option<String>,

    /// Run on CPU rather than on GPU.
    #[arg(long)]
    cpu: bool,

    // Enable/disable decoding.
    #[arg(long, default_value = "false")]
    disable_cache: bool,

    /// The temperature used to generate samples.
    #[arg(long, default_value_t = 0.8)]
    temperature: f64,

    /// Nucleus sampling probability cutoff.
    #[arg(long)]
    top_p: Option<f64>,

    /// Penalty to be applied for repeating tokens, 1. means no penalty.
    #[arg(long, default_value_t = 1.1)]
    repeat_penalty: f32,

    /// The context size to consider for the repeat penalty.
    #[arg(long, default_value_t = 64)]
    repeat_last_n: usize,

    /// The model size to use.
    #[arg(long, default_value = "t5-small")]
    which: Which,
}

struct T5ModelBuilder {
    // device: Device,
    config: t5::Config,
    weights_filename: PathBuf,
}

impl T5ModelBuilder {
    pub fn load(args: &Args) -> AnyResult<(Self, Tokenizer)> {
        // let device = Device::Cpu;
        let default_model = "jbochi/candle-coedit-quantized".to_string();
        let (model_id, revision) = match (args.model_id.to_owned(), args.revision.to_owned()) {
            (Some(model_id), Some(revision)) => (model_id, revision),
            (Some(model_id), None) => (model_id, "main".to_string()),
            (None, Some(revision)) => (default_model, revision),
            (None, None) => (default_model, "main".to_string()),
        };

        let repo = Repo::with_revision(model_id, RepoType::Model, revision);
        let api = Api::new()?;
        let api = api.repo(repo);
        let config_filename = match &args.config_file {
            Some(filename) => Self::get_local_or_remote_file(filename, &api)?,
            None => match args.which {
                Which::T5Small => api.get("config.json")?,
                Which::FlanT5Small => api.get("config-flan-t5-small.json")?,
                Which::FlanT5Base => api.get("config-flan-t5-base.json")?,
                Which::FlanT5Large => api.get("config-flan-t5-large.json")?,
                Which::FlanT5Xl => api.get("config-flan-t5-xl.json")?,
                Which::FlanT5Xxl => api.get("config-flan-t5-xxl.json")?,
            },
        };
        // let tokenizer_filename = api.get("tokenizer.json")?;
        let tokenizer_filename = match &args.tokenizer {
            Some(filename) => Self::get_local_or_remote_file(filename, &api)?,
            None => api.get("tokenizer.json")?,
        };

        let weights_filename = match &args.weight_file {
            Some(filename) => Self::get_local_or_remote_file(filename, &api)?,
            None => match args.which {
                Which::T5Small => api.get("model.gguf")?,
                Which::FlanT5Small => api.get("model-flan-t5-small.gguf")?,
                Which::FlanT5Base => api.get("model-flan-t5-base.gguf")?,
                Which::FlanT5Large => api.get("model-flan-t5-large.gguf")?,
                Which::FlanT5Xl => api.get("model-flan-t5-xl.gguf")?,
                Which::FlanT5Xxl => api.get("model-flan-t5-xxl.gguf")?,
            },
        };
        let config = std::fs::read_to_string(config_filename)?;
        let mut config: t5::Config = serde_json::from_str(&config)?;
        config.use_cache = !args.disable_cache;
        let tokenizer = Tokenizer::from_file(tokenizer_filename).map_err(E::msg)?;
        Ok((
            Self {
                // device,
                config,
                weights_filename,
            },
            tokenizer,
        ))
    }

    pub fn build_model(&self, device: Device) -> CandleResult<t5::T5ForConditionalGeneration> {
        // let device = Device::Cpu;
        let vb = t5::VarBuilder::from_gguf(&self.weights_filename, &device)?;
        Ok(t5::T5ForConditionalGeneration::load(vb, &self.config)?)
    }

    fn get_local_or_remote_file(filename: &str, api: &ApiRepo) -> AnyResult<PathBuf> {
        let local_filename = std::path::PathBuf::from(filename);
        if local_filename.exists() {
            Ok(local_filename)
        } else {
            Ok(api.get(filename)?)
        }
    }
}

pub fn build_device(cpu: bool) -> CandleResult<Device> {
  if cpu {
      Ok(Device::Cpu)
  } else if cuda_is_available() {
      Ok(Device::new_cuda(0)?)
  } else if metal_is_available() {
      Ok(Device::new_metal(0)?)
  } else {
      #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
      {
          println!(
              "Running on CPU, to run on GPU(metal), build this example with `--features metal`"
          );
      }
      #[cfg(not(all(target_os = "macos", target_arch = "aarch64")))]
      {
          println!("Running on CPU, to run on GPU, build this example with `--features cuda`");
      }
      Ok(Device::Cpu)
  }
}

pub fn run(args: Args, prompt: String) -> AnyResult<String> {
  if prompt.is_empty() || prompt == "" {
    return Ok("invalid prompt".to_string())
  }

  let mut output = String::new();

  let device = build_device(args.cpu)?;

  let (builder, mut tokenizer) = T5ModelBuilder::load(&args)?;
  // let device = &builder.device;
  let tokenizer = tokenizer
      .with_padding(None)
      .with_truncation(None)
      .map_err(E::msg)?;
  let tokens = tokenizer
      .encode(prompt, true)
      .map_err(E::msg)?
      .get_ids()
      .to_vec();
  let input_token_ids = Tensor::new(&tokens[..], &device)?.unsqueeze(0)?;
  let mut model = builder.build_model(device.clone())?;
  let mut output_token_ids = [builder
      .config
      .decoder_start_token_id
      .unwrap_or(builder.config.pad_token_id) as u32]
  .to_vec();
  let temperature = if args.temperature <= 0. {
      None
  } else {
      Some(args.temperature)
  };
  let mut logits_processor = LogitsProcessor::new(299792458, temperature, args.top_p);
  let encoder_output = model.encode(&input_token_ids)?;
  let start = std::time::Instant::now();

  for index in 0.. {
      if output_token_ids.len() > 512 {
          break;
      }
      let decoder_token_ids = if index == 0 || !builder.config.use_cache {
          Tensor::new(output_token_ids.as_slice(), &device.clone())?.unsqueeze(0)?
      } else {
          let last_token = *output_token_ids.last().unwrap();
          Tensor::new(&[last_token], &device.clone())?.unsqueeze(0)?
      };
      let logits = model
          .decode(&decoder_token_ids, &encoder_output)?
          .squeeze(0)?;
      let logits = if args.repeat_penalty == 1. {
          logits
      } else {
          let start_at = output_token_ids.len().saturating_sub(args.repeat_last_n);
          candle_transformers::utils::apply_repeat_penalty(
              &logits,
              args.repeat_penalty,
              &output_token_ids[start_at..],
          )?
      };

      let next_token_id = logits_processor.sample(&logits)?;
      if next_token_id as usize == builder.config.eos_token_id {
          break;
      }
      output_token_ids.push(next_token_id);
      if let Some(text) = tokenizer.id_to_token(next_token_id) {
          let text = text.replace('▁', " ").replace("<0x0A>", "\n");
          print!("{text}");
          output.push_str(&text);
          std::io::stdout().flush()?;
      }
  }
  let dt = start.elapsed();
  println!(
      "\n{} tokens generated ({:.2} token/s)\n",
      output_token_ids.len(),
      output_token_ids.len() as f64 / dt.as_secs_f64(),
  );
  Ok(output)
}
