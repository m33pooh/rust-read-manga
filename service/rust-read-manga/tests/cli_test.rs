use clap::Parser;
use rust_read_manga::cli::{Cli, Commands};

#[test]
fn test_workflow_db_connection_string_arg() {
    let args = vec![
        "rust-read-manga",
        "workflow",
        "test.toml",
        "--db-connection-string",
        "postgres://user:pass@host:port/db",
    ];
    let cli = Cli::parse_from(args);
    match cli.command {
        Commands::Workflow {
            file,
            db_connection_string,
        } => {
            assert_eq!(file, "test.toml");
            assert_eq!(
                db_connection_string,
                Some("postgres://user:pass@host:port/db".to_string())
            );
        }
        _ => panic!("Expected Workflow command"),
    }
}
