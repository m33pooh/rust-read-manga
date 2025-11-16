use clap::Parser;
use std::path::PathBuf;

/// Converts a directory of manga/comic images into a single video file.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct CliArgs {
    /// Path to the input directory containing image files.
    #[arg(short = 'i', long, required = true)]
    pub input_dir: PathBuf,

    /// Path for the output video file.
    #[arg(short = 'o', long, required = true)]
    pub output_file: PathBuf,

    /// Duration (in seconds) to display each image.
    #[arg(short = 'd', long, default_value_t = 5.0)]
    pub duration: f64,

    /// Optional: Video codec to use (e.g., "libx264").
    #[arg(short = 'c', long, default_value = "libx264")]
    pub codec: String,

    /// Optional: Output resolution (e.g., "1920x1080").
    /// If not set, uses the resolution of the first image.
    #[arg(short = 'r', long)]
    pub resolution: Option<String>,
}