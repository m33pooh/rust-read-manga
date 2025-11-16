use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PageAnalysis {
    pub words: usize,
    pub chars: usize,
}

pub fn analyze(text: &str) -> PageAnalysis {
    PageAnalysis {
        words: text.split_whitespace().count(),
        chars: text.chars().count(),
    }
}
