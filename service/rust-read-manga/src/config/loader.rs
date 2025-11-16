use super::AppConfig;
use std::fs;

pub fn load_config(path: &str) -> anyhow::Result<AppConfig> {
    let text = fs::read_to_string(path)?;
    let config: AppConfig = toml::from_str(&text)?;
    Ok(config)
}
