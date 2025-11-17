use anyhow::Result;
use clap::Parser;
use rust_read_manga::{
    cli::{Cli, Commands},
    database,
    utils::logging::init_logging,
    workflow::engine::WorkflowEngine,
};
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<()> {
    init_logging();
    let cli = Cli::parse();

    match cli.command {
        Commands::Workflow {
            file,
            db_connection_string,
        } => {
            let workflow_id = Uuid::new_v4().to_string();
            let connection_string = db_connection_string
                .as_deref()
                .unwrap_or("host=localhost user=postgres password=secret dbname=manga_workflow");
            let db_client = database::client::connect(connection_string).await?;
            let engine = WorkflowEngine::new(&file, &workflow_id, db_client);
            engine.run().await?;
        }
        Commands::Convert { .. } => {
            println!("'convert' command is not implemented yet");
        }
        Commands::Batch { .. } => {
            println!("'batch' command is not implemented yet");
        }
    }

    Ok(())
}
