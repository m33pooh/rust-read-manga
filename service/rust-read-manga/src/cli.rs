use clap::Parser;
use std::path::PathBuf;

/// Converts a directory of manga/comic images into a single video file.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct CliArgs {
    /// Path to the input directory containing image files.
    #[arg(short = 'i', long, required_unless_present = "workflow")]
    pub input_dir: Option<PathBuf>,

    /// Path for the output video file.
    #[arg(short = 'o', long, required_unless_present = "workflow")]
    pub output_file: Option<PathBuf>,

    /// Optional: Path to a custom TOML configuration file.
    #[arg(short = 'c', long)]
    pub config: Option<PathBuf>,

    /// Optional: Path to a workflow file.
    #[arg(short = 'w', long)]
    pub workflow: Option<PathBuf>,

    /// Optional: Video codec (h264, h265, vp9, theora).
    #[arg(long)]
    pub codec: Option<String>,

    /// Optional: Video resolution (e.g., "1920x1080").
    #[arg(long)]
    pub resolution: Option<String>,

    /// Optional: Frames per second.
    #[arg(long)]
    pub fps: Option<u32>,

    /// Optional: Video bitrate in kbps.
    #[arg(long)]
    pub bitrate: Option<u32>,
}