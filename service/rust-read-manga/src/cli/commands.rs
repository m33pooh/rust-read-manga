use clap::{Command, Arg};

pub fn build_cli() -> Command {
    Command::new("rust-read-manga")
        .about("Convert manga images into video format")
        .subcommand(
            Command::new("convert")
                .about("Convert a manga folder into video")
                .arg(
                    Arg::new("input")
                        .help("Input folder containing manga pages")
                        .required(true)
                )
                .arg(
                    Arg::new("config")
                        .help("Path to config TOML file")
                        .required(false)
                )
        )
        .subcommand(
            Command::new("batch")
                .about("Process multiple manga folders")
                .arg(
                    Arg::new("root")
                        .help("Root folder containing multiple manga")
                        .required(true)
                )
        )
        .subcommand(
            Command::new("workflow")
                .about("Run a workflow file")
                .arg(
                    Arg::new("file")
                        .help("Path to the workflow file")
                        .required(true)
                )
        )
}
