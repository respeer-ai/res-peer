// Copyright (c) Zefchain Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

#![cfg_attr(target_arch = "wasm32", no_main)]

mod random;
mod state;

mod stable_diffusion;

use std::{io::Cursor, sync::Arc};

use async_graphql::{Context, EmptyMutation, EmptySubscription, Object, Request, Response, Schema};
use candle_core::{DType, Device, Tensor, D, Module, IndexOp, Result as CandleResult, utils::{cuda_is_available, metal_is_available}};
use linera_sdk::{base::WithServiceAbi, Service, ServiceRuntime};
use log::{info, error};
use tokenizers::Tokenizer;

use anyhow::{Error as E, Result as AnyResult};

use image::{ImageBuffer, Rgb};

pub struct IllustratorService {
    t5_model_builder: Arc<T5ModelBuilder>,
}

linera_sdk::service!(IllustratorService);

impl WithServiceAbi for IllustratorService {
    type Abi = illustrator::IllustratorAbi;
}

struct QueryRoot {}

#[Object]
impl QueryRoot {
    async fn prompt(&self, ctx: &Context<'_>, prompt: String) -> String {
        let t5_model_builder = ctx.data::<Arc<T5ModelBuilder>>().unwrap();
        info!("run model");
        t5_model_builder.run_model(&prompt).unwrap()
    }
}

struct T5ModelBuilder {
    sd_tokenizer: Vec<u8>,
    clip_weights: Vec<u8>,
    vae_weights: Vec<u8>,
    unet_weights: Vec<u8>,
}

impl Service for IllustratorService {
    type Parameters = ();

    async fn new(runtime: ServiceRuntime<Self>) -> Self {
        info!("Downloading sd tokenizer");
        let sd_tokenizer_bytes = runtime.fetch_url("http://localhost:10001/stable_diffusion/tiny_sd/tokenizer.json");
        info!("sd_tokenizer_bytes len {:?}", sd_tokenizer_bytes.len());

        info!("Downloading sd clip_weights");
        let clip_weights_bytes = runtime.fetch_url("http://localhost:10001/stable_diffusion/tiny_sd/text_encoder/model.safetensors");
        info!("clip_weights_bytes len {:?}", clip_weights_bytes.len());

        info!("Downloading sd vae_weights");
        let vae_weights_bytes = runtime.fetch_url("http://localhost:10001/stable_diffusion/tiny_sd/vae/diffusion_pytorch_model.safetensors");
        info!("vae_weights_bytes len {:?}", vae_weights_bytes.len());

        info!("Downloading sd unet_weights");
        let unet_weights_bytes = runtime.fetch_url("http://localhost:10001/stable_diffusion/tiny_sd/unet/diffusion_pytorch_model_contiguous.safetensors");
        info!("unet_weights_bytes len {:?}", unet_weights_bytes.len());

        let t5_model_builder = Arc::new(T5ModelBuilder {
            sd_tokenizer: sd_tokenizer_bytes,
            clip_weights: clip_weights_bytes,
            vae_weights: vae_weights_bytes,
            unet_weights: unet_weights_bytes,
        });

        IllustratorService { t5_model_builder }
    }

    async fn handle_query(&self, request: Request) -> Response {
        let query_string = &request.query;
        info!("query: {}", query_string);

        let schema = Schema::build(QueryRoot {}, EmptyMutation, EmptySubscription)
            .data(self.t5_model_builder.clone())
            .finish();
        schema.execute(request).await
    }
}

impl T5ModelBuilder {
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
                info!(
                    "Running on CPU, to run on GPU(metal), build this example with `--features metal`"
                );
            }
            #[cfg(not(all(target_os = "macos", target_arch = "aarch64")))]
            {
                info!("Running on CPU, to run on GPU, build this example with `--features cuda`");
            }
            Ok(Device::Cpu)
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn text_embeddings(
        &self,
        prompt: &str,
        uncond_prompt: &str,
        sd_config: &stable_diffusion::StableDiffusionConfig,
        device: &Device,
        dtype: DType,
        use_guide_scale: bool,
        first: bool,
    ) -> AnyResult<Tensor> {
        let tokenizer_bytes = &self.sd_tokenizer;
        let tokenizer = Tokenizer::from_bytes(tokenizer_bytes).expect("failed to create tokenizer");

        let pad_id = match &sd_config.clip.pad_with {
            Some(padding) => *tokenizer.get_vocab(true).get(padding.as_str()).unwrap(),
            None => *tokenizer.get_vocab(true).get("<|endoftext|>").unwrap(),
        };
        let mut tokens = tokenizer
            .encode(prompt, true)
            .map_err(E::msg)?
            .get_ids()
            .to_vec();
        if tokens.len() > sd_config.clip.max_position_embeddings {
            anyhow::bail!(
                "the prompt is too long, {} > max-tokens ({})",
                tokens.len(),
                sd_config.clip.max_position_embeddings
            )
        }
        while tokens.len() < sd_config.clip.max_position_embeddings {
            tokens.push(pad_id)
        }
        let tokens = Tensor::new(tokens.as_slice(), device)?.unsqueeze(0)?;

        let clip_config = if first {
            &sd_config.clip
        } else {
            sd_config.clip2.as_ref().unwrap()
        };
        let clip_weights = &self.clip_weights;
        info!("Building the Clip transformer.");
        let text_model = stable_diffusion::build_buffered_clip_transformer(clip_config, clip_weights.to_vec(), device, dtype).expect("invalid clip");
        info!("Building the Clip transformer done.");
       
        let text_embeddings = text_model.forward(&tokens)?;

        let text_embeddings = if use_guide_scale {
            let mut uncond_tokens = tokenizer
                .encode(uncond_prompt, true)
                .map_err(E::msg)?
                .get_ids()
                .to_vec();
            if uncond_tokens.len() > sd_config.clip.max_position_embeddings {
                anyhow::bail!(
                    "the negative prompt is too long, {} > max-tokens ({})",
                    uncond_tokens.len(),
                    sd_config.clip.max_position_embeddings
                )
            }
            while uncond_tokens.len() < sd_config.clip.max_position_embeddings {
                uncond_tokens.push(pad_id)
            }

            let uncond_tokens = Tensor::new(uncond_tokens.as_slice(), device)?.unsqueeze(0)?;
            let uncond_embeddings = text_model.forward(&uncond_tokens)?;

            Tensor::cat(&[uncond_embeddings, text_embeddings], 0)?.to_dtype(dtype)?
        } else {
            text_embeddings.to_dtype(dtype)?
        };
        Ok(text_embeddings)
    }

    fn image_preprocess<T: AsRef<std::path::Path>>(path: T) -> anyhow::Result<Tensor> {
        let img = image::ImageReader::open(path)?.decode()?;
        let (height, width) = (img.height() as usize, img.width() as usize);
        let height = height - height % 32;
        let width = width - width % 32;
        let img = img.resize_to_fill(
            width as u32,
            height as u32,
            image::imageops::FilterType::CatmullRom,
        );
        let img = img.to_rgb8();
        let img = img.into_raw();
        let img = Tensor::from_vec(img, (height, width, 3), &Device::Cpu)?
            .permute((2, 0, 1))?
            .to_dtype(DType::F32)?
            .affine(2. / 255., -1.)?
            .unsqueeze(0)?;
        Ok(img)
    }
        
    #[allow(clippy::too_many_arguments)]
    fn generate_image(
        vae: &stable_diffusion::vae::AutoEncoderKL,
        latents: &Tensor,
        vae_scale: f64,
        bsize: usize,
    ) -> AnyResult<String> {
        info!("generate image");
        let images = vae.decode(&(latents / vae_scale)?)?;
        let images = ((images / 2.)? + 0.5)?.to_device(&Device::Cpu)?;
        let images = (images.clamp(0f32, 1.)? * 255.)?.to_dtype(DType::U8)?;
        let mut image_base64_str = String::new();
        for batch in 0..bsize {
            let image = images.i(batch)?;
            image_base64_str = T5ModelBuilder::generate_image_data(&image)?;
        }
        Ok(image_base64_str)
    }

    pub fn generate_image_data(img: &Tensor) -> CandleResult<String> {
        let (channel, height, width) = img.dims3()?;
        if channel != 3 {
            candle_core::bail!("save_image expects an input of shape (3, height, width)")
        }
        let img = img.permute((1, 2, 0))?.flatten_all()?;
        let pixels = img.to_vec1::<u8>()?;
        let image: image::ImageBuffer<image::Rgb<u8>, Vec<u8>> =
            match image::ImageBuffer::from_raw(width as u32, height as u32, pixels) {
                Some(image) => image,
                None => candle_core::bail!("error saving image"),
            };
        let base64_string = T5ModelBuilder::convert_image_to_base64(image.clone());
        info!("Base64 Encoded Image successful");
        let image_head = "data:image/png;base64,".to_string();
        let base64_img_string = image_head + &base64_string;
        Ok(base64_img_string)
    }

    fn convert_image_to_base64(image: ImageBuffer<Rgb<u8>, Vec<u8>>) -> String {
        let mut buffer = Vec::new();
        {
            let mut cursor = Cursor::new(&mut buffer);
            image.write_to(&mut cursor, image::ImageFormat::Png).unwrap();
        }
        base64::encode(&buffer)
    }
    
    fn run_model(&self, prompt_string: &str) -> Result<String, candle_core::Error> {
        let prompt = prompt_string;
        let cpu = true;
        let height: Option<usize> = Some(24);
        let width: Option<usize> = Some(24);
        
        let use_flash_attn = false;
        let sliced_attention_size: Option<usize> = None;
        let num_samples: usize = 1;
        let img2img: Option<String> = None;
        let img2img_strength: f64 = 0.8;
        let bsize: usize = 1;
        let seed: Option<u64> = None;
        let uncond_prompt = "";

        if !(0. ..=1.).contains(&img2img_strength) {
            error!("img2img-strength should be between 0 and 1, got {img2img_strength}")
        }

        let guidance_scale = 7.5;
        let n_steps = 30;
        let dtype = DType::F16;
        let sd_config = stable_diffusion::StableDiffusionConfig::vtiny_sd(sliced_attention_size, height, width);

        let scheduler = sd_config.build_scheduler(n_steps).expect("invalid build_scheduler");
        let device = T5ModelBuilder::build_device(cpu).expect("invalid build_device");
        if let Some(seed) = seed {
            let _ = device.set_seed(seed);
        }
        let use_guide_scale = guidance_scale > 1.0;
        let which = vec![true];
        let text_embeddings = which
        .iter()
        .map(|first| {
            T5ModelBuilder::text_embeddings(
                self,
                &prompt,
                &uncond_prompt,
                &sd_config,
                &device,
                dtype,
                use_guide_scale,
                *first,
            )
        })
        .collect::<AnyResult<Vec<_>>>().expect("invalid text_embeddings");

        let text_embeddings = Tensor::cat(&text_embeddings, D::Minus1)?;
        let text_embeddings = text_embeddings.repeat((bsize, 1, 1))?;
        // info!("{text_embeddings:?}");
    
        info!("Building the vae_weights autoencoder.");
        let vae_weights = &self.vae_weights;
        let vae = sd_config.build_buffered_vae(vae_weights.to_vec(), &device, dtype)?;
        info!("Building the vae_weights autoencoder done.");

        let init_latent_dist = match &img2img {
            None => None,
            Some(image) => {
                let image = T5ModelBuilder::image_preprocess(image).expect("invalid image_preprocess").to_device(&device).expect("innvalid to_device");
                Some(vae.encode(&image)?)
            }
        };

        info!("Building the unet.");
        let unet_weights = &self.unet_weights;
        let unet = sd_config.build_buffered_unet(unet_weights.to_vec(), &device, 4, use_flash_attn, dtype).expect("invalid build unet");
        info!("Building the unet done.");

        let t_start = if img2img.is_some() {
            n_steps - (n_steps as f64 * img2img_strength) as usize
        } else {
            0
        };
    
        let vae_scale = 0.18215;
        let mut image_base64_str = String::new();

        for idx in 0..num_samples {
            let timesteps = scheduler.timesteps();
            let latents = match &init_latent_dist {
                Some(init_latent_dist) => {
                    let latents = (init_latent_dist.sample()? * vae_scale)?.to_device(&device)?;
                    if t_start < timesteps.len() {
                        let noise = latents.randn_like(0f64, 1f64)?;
                        scheduler.add_noise(&latents, noise, timesteps[t_start])?
                    } else {
                        latents
                    }
                }
                None => {
                    let latents = Tensor::randn(
                        0f32,
                        1f32,
                        (bsize, 4, sd_config.height / 8, sd_config.width / 8),
                        &device,
                    )?;
                    (latents * scheduler.init_noise_sigma())?
                }
            };
            let mut latents = latents.to_dtype(dtype)?;
    
            for (timestep_index, &timestep) in timesteps.iter().enumerate() {
                if timestep_index < t_start {
                    continue;
                }
                // let start_time = std::time::Instant::now();
                let latent_model_input = if use_guide_scale {
                    Tensor::cat(&[&latents, &latents], 0)?
                } else {
                    latents.clone()
                };
    
                let latent_model_input = scheduler.scale_model_input(latent_model_input, timestep)?;
                let noise_pred =
                    unet.forward(&latent_model_input, timestep as f64, &text_embeddings)?;
                
                let noise_pred = if use_guide_scale {
                    let noise_pred = noise_pred.chunk(2, 0)?;
                    let (noise_pred_uncond, noise_pred_text) = (&noise_pred[0], &noise_pred[1]);
    
                    (noise_pred_uncond + ((noise_pred_text - noise_pred_uncond)? * guidance_scale)?)?
                } else {
                    noise_pred
                };
    
                latents = scheduler.step(&noise_pred, timestep, &latents)?;
                // let dt = start_time.elapsed().as_secs_f32();
                // info!("step {}/{n_steps} done, {:.2}s", timestep_index + 1, dt);
                info!("step {}/{n_steps} done", timestep_index + 1);
            }
    
            info!(
                "Generating the final image for sample {}/{}.",
                idx + 1,
                num_samples
            );
     
            image_base64_str = T5ModelBuilder::generate_image(
                &vae,
                &latents,
                vae_scale,
                bsize,
            ).expect("invalid generate image");
        }

        let output = image_base64_str;
        Ok(output)
    }

}
