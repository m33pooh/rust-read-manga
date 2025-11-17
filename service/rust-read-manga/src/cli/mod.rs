use clap::Parser;



#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Parser)]
pub enum Commands {
    /// Convert a manga folder into video
    Convert {
        /// Input folder containing manga pages
        input: String,
        /// Path to config TOML file
        config: Option<String>,
    },
    /// Process multiple manga folders
    Batch {
        /// Root folder containing multiple manga
        root: String,
    },
    /// Run a workflow file
    Workflow {
        /// Path to the workflow file
        file: String,
    },
}
