use crate::config::TimingConfig;
use crate::ocr::analysis::PageAnalysis;
use rhai::{Engine, Scope};
use log;

pub fn evaluate_formula(page: &PageAnalysis, cfg: &TimingConfig) -> f32 {
    let formula = match &cfg.custom_formula {
        Some(f) if !f.is_empty() => f,
        _ => {
            // Fallback to min_duration_sec if no custom formula is provided
            return cfg.min_duration_sec;
        }
    };

    let engine = Engine::new();
    let mut scope = Scope::new();

    // Push variables into the script scope.
    // We cast to f32 to make scripting easier.
    scope.push("words", page.words as f32);
    scope.push("chars", page.chars as f32);
    scope.push("min", cfg.min_duration_sec);
    scope.push("max", cfg.max_duration_sec);
    scope.push("base", cfg.base_time_sec);

    match engine.eval_with_scope::<f32>(&mut scope, formula) {
        Ok(duration) => duration,
        Err(e) => {
            log::warn!("Failed to evaluate custom duration formula: {}. Error: {}", formula, e);
            // Fallback to min_duration_sec on script error
            cfg.min_duration_sec
        }
    }
}