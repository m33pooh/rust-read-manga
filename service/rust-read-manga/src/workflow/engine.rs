use crate::workflow::{parser::parse_workflow, runner::WorkflowRunner};
use anyhow::Result;
use tokio_postgres::Client;

pub struct WorkflowEngine<'a> {
    file_path: String,
    workflow_id: &'a str,
    db_client: Client,
}

impl<'a> WorkflowEngine<'a> {
    pub fn new(file_path: &str, workflow_id: &'a str, db_client: Client) -> Self {
        Self {
            file_path: file_path.to_string(),
            workflow_id,
            db_client,
        }
    }

    pub async fn run(&self) -> Result<()> {
        let workflow = parse_workflow(&self.file_path)?;
        let runner = WorkflowRunner::new(&workflow, self.workflow_id, &self.db_client);
        runner.run().await
    }
}
