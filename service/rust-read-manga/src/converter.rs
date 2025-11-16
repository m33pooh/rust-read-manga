use crate::config::models::AppConfig;
use crate::duration;
use crate::error::AppError;
use crate::ocr;
use crate::plugins::PluginManager;
use crate::video;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const ALLOWED_EXTENSIONS: [&str; 3] = ["png", "jpg", "jpeg"];

pub struct VideoConverter {
    config: AppConfig,
    plugins: PluginManager,
}

impl VideoConverter {
    pub fn new(config: AppConfig, plugins: PluginManager) -> Self {
        Self { config, plugins }
    }

    pub fn run(&self, input_dir: &Path, output_file: &Path) -> Result<(), AppError> {
        let image_paths = self.scan_images(input_dir)?;
        let timeline = self.generate_timeline(&image_paths)?;
        self.encode_video(&timeline, output_file)?;
        Ok(())
    }

    fn scan_images(&self, input_dir: &Path) -> Result<Vec<PathBuf>, AppError> {
        println!("Scanning input directory: {:?}", input_dir);

        let mut image_paths: Vec<PathBuf> = fs::read_dir(input_dir)?
            .filter_map(Result::ok)
            .map(|entry| entry.path())
            .filter(|path| {
                if path.is_file() {
                    if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
                        return ALLOWED_EXTENSIONS.contains(&ext.to_lowercase().as_str());
                    }
                }
                false
            })
            .collect();

        if image_paths.is_empty() {
            return Err(AppError::NoImagesFound(input_dir.to_path_buf()));
        }

        image_paths.sort();
        println!("Found and sorted {} images.", image_paths.len());
        Ok(image_paths)
    }

    fn generate_timeline(
        &self,
        image_paths: &[PathBuf],
    ) -> Result<video::timeline::Timeline, AppError> {
        let ocr_engine = ocr::engine::OcrEngine::new();
        let duration_engine = duration::engine::DurationEngine::new(self.config.timing.clone());
        let mut timeline = video::timeline::Timeline::new();

        for image_path in image_paths {
            let text = ocr_engine.extract_text(image_path.to_str().unwrap_or_default())?;
            let analysis = ocr::analysis::analyze(&text);
            let duration = duration_engine.calculate(&analysis);
            timeline.add_frame(image_path.clone(), duration);
        }

        println!("\nGenerated timeline:");
        println!("{:#?}", timeline);
        Ok(timeline)
    }

    fn encode_video(
        &self,
        timeline: &video::timeline::Timeline,
        output_file: &Path,
    ) -> Result<(), AppError> {
        println!("\nStarting video encoding...");

        let video_config = &self.config.video;
        let mut command = Command::new("ffmpeg");

        // Prepare a temporary file for ffmpeg's concat demuxer
        let concat_file_content = timeline
            .frames
            .iter()
            .map(|frame| {
                format!(
                    "file '{}'\nduration {}",
                    frame.image_path.to_str().unwrap(),
                    frame.duration_sec
                )
            })
            .collect::<Vec<String>>()
            .join("\n");

        let concat_file_path = "ffmpeg_concat.txt";
        fs::write(concat_file_path, concat_file_content)?;

        command
            .arg("-f")
            .arg("concat")
            .arg("-safe")
            .arg("0")
            .arg("-i")
            .arg(concat_file_path)
            .arg("-c:v")
            .arg(video_config.codec.to_string())
            .arg("-r")
            .arg(video_config.fps.to_string())
            .arg("-pix_fmt")
            .arg("yuv420p") // Common pixel format for compatibility
            .arg("-vf")
            .arg(format!(
                "scale={}:{}",
                video_config.resolution.width, video_config.resolution.height
            ))
            .arg("-b:v")
            .arg(format!("{}k", video_config.bitrate_kbps))
            .arg("-y") // Overwrite output file if it exists
            .arg(output_file);

        println!("Executing FFmpeg command: {:?}", command);

        let output = command.output()?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(AppError::FfmpegError(stderr.to_string()));
        }

        // Clean up the temporary file
        fs::remove_file(concat_file_path)?;

        println!("\nVideo encoding finished successfully!");
        Ok(())
    }
}

pub fn run_conversion(
    input_dir: &Path,
    output_file: &Path,
    config: AppConfig,
    plugins: PluginManager,
) -> Result<(), AppError> {
    let converter = VideoConverter::new(config, plugins);
    converter.run(input_dir, output_file)
}