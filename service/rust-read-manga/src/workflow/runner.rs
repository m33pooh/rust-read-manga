use crate::workflow::{
    context::Context,
    models::{NodeType, Workflow},
    node_type::Node,
};
use anyhow::Result;
use tokio_postgres::Client;
use crate::database::client::{save_step, get_step};
use serde_json::Value;

pub struct WorkflowRunner<'a> {
    workflow: &'a Workflow,
    workflow_id: &'a str,
    db_client: &'a Client,
}

impl<'a> WorkflowRunner<'a> {
    pub fn new(workflow: &'a Workflow, workflow_id: &'a str, db_client: &'a Client) -> Self {
        Self { workflow, workflow_id, db_client }
    }

    pub async fn run(&self) -> Result<()> {
        let mut context = Context::new();

        // TODO: Implement a proper node ordering mechanism
        for (name, node) in &self.workflow.nodes {
            // Check if step already completed
            match get_step(self.db_client, self.workflow_id, name, 0).await { // Assuming page_number 0 for now
                Ok(saved_data) => {
                    println!("Loading saved state for step: {}", name);
                    context = serde_json::from_value(saved_data)?;
                    continue;
                },
                Err(_) => {
                    println!("Executing step: {}", name);
                    context = self.execute_node(node, context).await?;
                    // Save state after execution
                    let current_state: Value = serde_json::to_value(&context)?; // Convert context to JSON
                    save_step(self.db_client, self.workflow_id, name, Some(0), current_state).await?;
                }
            }
        }

        Ok(())
    }

    async fn execute_node(&self, node: &NodeType, context: Context) -> Result<Context> {
        match node {
            NodeType::InputLoader(n) => n.run(context).await,
            NodeType::ImagePreprocess(n) => n.run(context).await,
            NodeType::Ocr(n) => n.run(context).await,
            NodeType::Duration(n) => n.run(context).await,
            NodeType::Transition(n) => n.run(context).await,
            NodeType::Audio(n) => n.run(context).await,
            NodeType::Encoder(n) => n.run(context).await,
            NodeType::OutputWriter(n) => n.run(context).await,
        }
    }
}
