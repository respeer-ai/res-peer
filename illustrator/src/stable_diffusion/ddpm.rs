use super::schedulers::{BetaSchedule, PredictionType};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DDPMVarianceType {
    FixedSmall,
}

impl Default for DDPMVarianceType {
    fn default() -> Self {
        Self::FixedSmall
    }
}

#[derive(Debug, Clone)]
pub struct DDPMSchedulerConfig {
    /// The value of beta at the beginning of training.
    pub beta_start: f64,
    /// The value of beta at the end of training.
    pub beta_end: f64,
    /// How beta evolved during training.
    pub beta_schedule: BetaSchedule,
    /// Option to predicted sample between -1 and 1 for numerical stability.
    pub clip_sample: bool,
    /// Option to clip the variance used when adding noise to the denoised sample.
    pub variance_type: DDPMVarianceType,
    /// prediction type of the scheduler function
    pub prediction_type: PredictionType,
    /// number of diffusion steps used to train the model.
    pub train_timesteps: usize,
}

impl Default for DDPMSchedulerConfig {
    fn default() -> Self {
        Self {
            beta_start: 0.00085,
            beta_end: 0.012,
            beta_schedule: BetaSchedule::ScaledLinear,
            clip_sample: false,
            variance_type: DDPMVarianceType::FixedSmall,
            prediction_type: PredictionType::Epsilon,
            train_timesteps: 1000,
        }
    }
}
