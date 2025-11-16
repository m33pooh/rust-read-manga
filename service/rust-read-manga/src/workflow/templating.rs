use crate::workflow::context::Context;
use anyhow::{anyhow, Result};
use regex::Regex;

pub fn resolve_string(s: &str, context: &Context) -> Result<String> {
    let re = Regex::new(r"\{\{(.*?)\}\}")?;
    let mut result = s.to_string();
    for cap in re.captures_iter(s) {
        let path = &cap[1].trim();
        let value = context
            .get::<String>(path)
            .ok_or_else(|| anyhow!("Value not found in context: {}", path))?;
        result = result.replace(&cap[0], value);
    }
    Ok(result)
}
