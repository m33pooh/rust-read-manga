use crate::config::TimingConfig;
use crate::ocr::analysis::PageAnalysis;

pub fn evaluate_formula(page: &PageAnalysis, cfg: &TimingConfig) -> f32 {
    // Minimal stub, always returns min duration
    cfg.min_duration_sec
}
