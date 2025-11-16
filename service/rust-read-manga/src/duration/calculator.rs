use crate::config::{TimingConfig, TimingMode};
use crate::ocr::analysis::PageAnalysis;
use super::formula::evaluate_formula;

pub fn calculate_duration(page: &PageAnalysis, cfg: &TimingConfig) -> f32 {
    let mut dur = match cfg.mode {
        TimingMode::Words => {
            let wps = cfg.reading_speed_wpm.unwrap_or(180) as f32 / 60.0;
            (page.words as f32 / wps) + cfg.base_time_sec
        }
        TimingMode::Chars => {
            let cps = cfg.reading_speed_cps.unwrap_or(12) as f32;
            (page.chars as f32 / cps) + cfg.base_time_sec
        }
        TimingMode::Auto => {
            if page.words > 0 {
                (page.words as f32 / (cfg.reading_speed_wpm.unwrap_or(180) as f32 / 60.0))
                    + cfg.base_time_sec
            } else {
                (page.chars as f32 / cfg.reading_speed_cps.unwrap_or(12) as f32)
                    + cfg.base_time_sec
            }
        }
        TimingMode::Custom => evaluate_formula(page, cfg),
    };

    dur = dur.clamp(cfg.min_duration_sec, cfg.max_duration_sec);
    dur
}
