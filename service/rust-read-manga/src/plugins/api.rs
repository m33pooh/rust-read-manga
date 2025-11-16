use anyhow::Result;

pub struct ExecutionContext {
    pub image_path: Option<String>,
}

pub type PluginResult = Result<()>;

pub trait Plugin {
    fn name(&self) -> &str;
    fn execute(&self, context: &ExecutionContext) -> PluginResult;
}
