use crate::config::TimingConfig;
use crate::ocr::analysis::PageAnalysis;
use super::calculator::calculate_duration;

/// The `DurationEngine` is responsible for calculating the optimal
/// display duration for a manga page based on its text content.
pub struct DurationEngine {
    config: TimingConfig,
}

impl DurationEngine {
    /// Creates a new `DurationEngine` with the given timing configuration.
    pub fn new(config: TimingConfig) -> Self {
        Self { config }
    }

    /// Calculates the display duration for a single page.
    pub fn calculate(&self, page: &PageAnalysis) -> f32 {
        calculate_duration(page, &self.config)
    }
}
