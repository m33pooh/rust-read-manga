use anyhow::Result;
use clap::Parser;
use rust_read_manga::{
    cli::CliArgs,
    config::{
        load_config,
        models::{AppConfig, Resolution, VideoCodec},
    },
    converter::run_conversion,
    plugins,
    utils::logging::init_logging,
};

fn main() -> Result<()> {
    init_logging();
    let args = CliArgs::parse();
    run(args)
}

fn run(args: CliArgs) -> Result<()> {
    let mut config = if let Some(config_path) = args.config.as_ref() {
        load_config(config_path.to_str().unwrap())?
    } else {
        AppConfig::default()
    };

    if let Some(codec_str) = &args.codec {
        config.video.codec = match codec_str.to_lowercase().as_str() {
            "h264" => VideoCodec::H264,
            "h265" => VideoCodec::H265,
            "vp9" => VideoCodec::Vp9,
            "theora" => VideoCodec::Theora,
            _ => return Err(anyhow::anyhow!("Invalid codec")),
        };
    }

    if let Some(resolution_str) = &args.resolution {
        let parts: Vec<&str> = resolution_str.split('x').collect();
        if parts.len() != 2 {
            return Err(anyhow::anyhow!(
                "Invalid resolution format. Use 'widthxheight'"
            ));
        }
        config.video.resolution = Resolution {
            width: parts[0].parse()?,
            height: parts[1].parse()?,
        };
    }

    if let Some(fps) = args.fps {
        config.video.fps = fps;
    }

    if let Some(bitrate) = args.bitrate {
        config.video.bitrate_kbps = bitrate;
    }

    let plugins = plugins::load_plugins(&config.plugins)?;

    run_conversion(args, config, plugins)?;

    Ok(())
}
