use crate::config::{CountMode, TimingConfig, TimingCurve};
use crate::ocr::analysis::PageAnalysis;
use super::formula::evaluate_formula;

pub fn calculate_duration(page: &PageAnalysis, cfg: &TimingConfig) -> f32 {
    let mut dur = match cfg.mode {
        CountMode::Words => {
            let wpm = cfg.reading_speed_wpm.unwrap_or(180) as f32;
            if wpm > 0.0 {
                (page.words as f32 / wpm) * 60.0
            } else {
                0.0
            }
        }
        CountMode::Chars => {
            let cpm = cfg.reading_speed_cpm.unwrap_or(180 * 5) as f32;
            if cpm > 0.0 {
                (page.chars as f32 / cpm) * 60.0
            } else {
                0.0
            }
        }
        CountMode::Mixed => {
            let wpm = cfg.reading_speed_wpm.unwrap_or(180) as f32;
            let cpm = cfg.reading_speed_cpm.unwrap_or(180 * 5) as f32;

            let word_duration = if wpm > 0.0 {
                (page.words as f32 / wpm) * 60.0
            } else {
                0.0
            };
            let char_duration = if cpm > 0.0 {
                (page.chars as f32 / cpm) * 60.0
            } else {
                0.0
            };

            let weight_words = cfg.weight_words.unwrap_or(0.5);
            let weight_chars = cfg.weight_chars.unwrap_or(0.5);

            (word_duration * weight_words) + (char_duration * weight_chars)
        }
        CountMode::Custom => evaluate_formula(page, cfg),
    };

    dur += cfg.base_time_sec;

    dur = apply_curve(dur, &cfg.curve);

    dur.clamp(cfg.min_duration_sec, cfg.max_duration_sec)
}

fn apply_curve(duration: f32, curve: &TimingCurve) -> f32 {
    match curve {
        TimingCurve::Linear => duration,
        TimingCurve::Log => (duration + 1.0).ln(),
        TimingCurve::Sqrt => duration.sqrt(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{CountMode, TimingConfig, TimingCurve};
    use crate::ocr::analysis::PageAnalysis;

    fn create_test_page_analysis(words: usize, chars: usize) -> PageAnalysis {
        PageAnalysis {
            words,
            chars,
        }
    }

    #[test]
    fn test_calculate_duration_words_mode() {
        let page = create_test_page_analysis(180, 900);
        let cfg = TimingConfig {
            mode: CountMode::Words,
            reading_speed_wpm: Some(180),
            ..TimingConfig::default()
        };
        let duration = calculate_duration(&page, &cfg);
        // (180 words / 180 wpm) * 60s/min = 60s.
        // 60s + 0.5s base_time = 60.5s
        // Clamped to max_duration_sec (10.0)
        assert_eq!(duration, 10.0);

        let cfg_fast = TimingConfig {
            mode: CountMode::Words,
            reading_speed_wpm: Some(360),
            ..TimingConfig::default()
        };
        let duration_fast = calculate_duration(&page, &cfg_fast);
        // (180 words / 360 wpm) * 60s/min = 30s.
        // 30s + 0.5s base_time = 30.5s
        // Clamped to max_duration_sec (10.0)
        assert_eq!(duration_fast, 10.0);
    }

    #[test]
    fn test_calculate_duration_chars_mode() {
        let page = create_test_page_analysis(180, 900);
        let cfg = TimingConfig {
            mode: CountMode::Chars,
            reading_speed_cpm: Some(900),
            ..TimingConfig::default()
        };
        let duration = calculate_duration(&page, &cfg);
        // (900 chars / 900 cpm) * 60s/min = 60s.
        // 60s + 0.5s base_time = 60.5s
        // Clamped to max_duration_sec (10.0)
        assert_eq!(duration, 10.0);
    }

    #[test]
    fn test_calculate_duration_mixed_mode() {
        let page = create_test_page_analysis(90, 450);
        let cfg = TimingConfig {
            mode: CountMode::Mixed,
            reading_speed_wpm: Some(180),
            reading_speed_cpm: Some(900),
            weight_words: Some(0.5),
            weight_chars: Some(0.5),
            ..TimingConfig::default()
        };
        let duration = calculate_duration(&page, &cfg);
        // word_duration = (90 / 180) * 60 = 30s
        // char_duration = (450 / 900) * 60 = 30s
        // mixed = (30 * 0.5) + (30 * 0.5) = 30s
        // 30s + 0.5s base_time = 30.5s
        // Clamped to max_duration_sec (10.0)
        assert_eq!(duration, 10.0);
    }

    #[test]
    fn test_min_max_clamp() {
        let page = create_test_page_analysis(10, 50);
        let cfg = TimingConfig {
            mode: CountMode::Words,
            reading_speed_wpm: Some(180),
            min_duration_sec: 3.0,
            ..TimingConfig::default()
        };
        let duration = calculate_duration(&page, &cfg);
        // (10 words / 180 wpm) * 60 = 3.333s
        // 3.333s + 0.5s base_time = 3.833s
        // Not clamped, as it's between 3.0 and 10.0
        assert!(duration > 3.8 && duration < 3.9);

        let page_min = create_test_page_analysis(1, 5);
        let duration_min = calculate_duration(&page_min, &cfg);
        // (1 / 180) * 60 = 0.333s
        // 0.333s + 0.5s = 0.833s
        // Clamped to min_duration_sec (3.0)
        assert_eq!(duration_min, 3.0);
    }

    #[test]
    fn test_apply_curve() {
        assert_eq!(apply_curve(10.0, &TimingCurve::Linear), 10.0);
        assert_eq!(apply_curve(10.0, &TimingCurve::Sqrt), 10.0_f32.sqrt());
        assert_eq!(apply_curve(10.0, &TimingCurve::Log), (10.0_f32 + 1.0).ln());
    }
}
