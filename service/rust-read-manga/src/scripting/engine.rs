use crate::config::ScriptingConfig;
use anyhow::Context; // <-- Add this use statement
use rhai::{Engine, Scope};
use std::fs;

pub fn run_script(config: &ScriptingConfig, script_name: &str, context: &mut Scope) -> anyhow::Result<()> {
    if !config.enabled {
        return Ok(());
    }

    let script_dir = match &config.directory {
        Some(dir) => dir,
        None => return Ok(()),
    };

    let script_path = format!("{}/{}", script_dir, script_name);
    if fs::metadata(&script_path).is_err() {
        return Ok(());
    }

    let engine = Engine::new();
    let script = fs::read_to_string(&script_path)
        .with_context(|| format!("Failed to read script file: {}", script_path))?;

    // This is the fix:
    engine
        .run_with_scope(context, &script)
        .map_err(|e| anyhow::anyhow!("Rhai script execution failed for {}: {}", script_name, e))?;

    Ok(())
}