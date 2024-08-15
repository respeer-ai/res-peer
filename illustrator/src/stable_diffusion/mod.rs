pub mod attention;
pub mod clip;
pub mod ddim;
pub mod ddpm;
pub mod embeddings;
pub mod euler_ancestral_discrete;
pub mod resnet;
pub mod schedulers;
pub mod unet_2d;
pub mod unet_2d_blocks;
pub mod utils;
pub mod vae;

use std::sync::Arc;

use candle_core as candle;
use candle::{DType, Device, Result};
use candle_nn as nn;

use self::schedulers::{Scheduler, SchedulerConfig};

#[derive(Clone, Debug)]
pub struct StableDiffusionConfig {
    pub width: usize,
    pub height: usize,
    pub clip: clip::Config,
    pub clip2: Option<clip::Config>,
    autoencoder: vae::AutoEncoderKLConfig,
    unet: unet_2d::UNet2DConditionModelConfig,
    scheduler: Arc<dyn SchedulerConfig>,
}

impl StableDiffusionConfig {
    pub fn tiny_sd(
        sliced_attention_size: Option<usize>,
        height: Option<usize>,
        width: Option<usize>,
    ) -> Self {
        let bc = |out_channels, use_cross_attn, attention_head_dim| unet_2d::BlockConfig {
            out_channels,
            use_cross_attn,
            attention_head_dim,
        };
        // https://huggingface.co/segmind/tiny-sd/blob/main/unet/config.json
        let unet = unet_2d::UNet2DConditionModelConfig {
            blocks: vec![
                bc(320, Some(1), 8),
                bc(640, Some(1), 8),
                bc(1280, Some(1), 8),
            ],
            center_input_sample: false,
            cross_attention_dim: 768,
            downsample_padding: 1,
            flip_sin_to_cos: true,
            freq_shift: 0.,
            layers_per_block: 1,
            mid_block_scale_factor: 1.,
            norm_eps: 1e-5,
            norm_num_groups: 32,
            sliced_attention_size,
            use_linear_projection: false,
        };
        let autoencoder = vae::AutoEncoderKLConfig {
            block_out_channels: vec![128, 256, 512, 512],
            layers_per_block: 2,
            latent_channels: 4,
            norm_num_groups: 32,
        };
        let height = if let Some(height) = height {
            assert_eq!(height % 8, 0, "height has to be divisible by 8");
            height
        } else {
            512
        };

        let width = if let Some(width) = width {
            assert_eq!(width % 8, 0, "width has to be divisible by 8");
            width
        } else {
            512
        };

        let scheduler = Arc::new(ddim::DDIMSchedulerConfig {
            prediction_type: schedulers::PredictionType::Epsilon,
            ..Default::default()
        });

        StableDiffusionConfig {
            width,
            height,
            clip: clip::Config::tiny_sd(),
            clip2: None,
            autoencoder,
            scheduler,
            unet,
        }
    }

    pub fn build_buffered_vae (
        &self,
        vae_weights: Vec<u8>,
        device: &Device,
        dtype: DType,
    ) -> Result<vae::AutoEncoderKL> {
        let vs_ae = nn::VarBuilder::from_buffered_safetensors(vae_weights, dtype, device)?;
        // https://huggingface.co/runwayml/stable-diffusion-v1-5/blob/main/vae/config.json
        let autoencoder = vae::AutoEncoderKL::new(vs_ae, 3, 3, self.autoencoder.clone())?;
        Ok(autoencoder)
    }

    pub fn build_buffered_unet(
        &self,
        unet_weights: Vec<u8>,
        device: &Device,
        in_channels: usize,
        use_flash_attn: bool,
        dtype: DType,
    ) -> Result<unet_2d::UNet2DConditionModel> {
        let vs_unet = nn::VarBuilder::from_buffered_safetensors(unet_weights, dtype, device)?;
        let unet = unet_2d::UNet2DConditionModel::new(
            vs_unet,
            in_channels,
            4,
            use_flash_attn,
            self.unet.clone(),
        )?;

        Ok(unet)
    }

    pub fn build_scheduler(&self, n_steps: usize) -> Result<Box<dyn Scheduler>> {
        self.scheduler.build(n_steps)
    }
}

pub fn build_buffered_clip_transformer(
    clip: &clip::Config,
    clip_weights: Vec<u8>,
    device: &Device,
    dtype: DType,
) -> Result<clip::ClipTextTransformer> {
    let vs = nn::VarBuilder::from_buffered_safetensors(clip_weights, dtype, device)?;
    let text_model = clip::ClipTextTransformer::new(vs, clip)?;
    Ok(text_model)
}
