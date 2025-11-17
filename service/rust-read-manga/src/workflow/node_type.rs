
use crate::workflow::context::Context;
use anyhow::{anyhow, Result};
use ffmpeg_sidecar::command::{ffmpeg_is_installed, FfmpegCommand};
use glob::glob;
use image::{imageops, DynamicImage};
use serde::Deserialize;
use std::io::Write;
use tempfile::Builder;
use tesseract::Tesseract;
use async_trait::async_trait;

#[async_trait]
pub trait Node {
    async fn run(&self, context: Context) -> Result<Context>;
}

#[derive(Debug, Deserialize, Default, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SortOrder {
    #[default]
    None,
    Natural,
    Asc,
    Desc,
}

#[derive(Debug, Deserialize, Default)]
pub struct InputLoaderNode {
    pub path: String,
    #[serde(default)]
    pub recursive: bool,
    #[serde(default)]
    pub sort_order: SortOrder,
    #[serde(default)]
    pub extensions: Vec<String>,
}

impl Node for InputLoaderNode {
    async fn run(&self, mut context: Context) -> Result<Context> {
        let mut files: Vec<String> = Vec::new();

        if self.path == "test://dummy" {
            let temp_dir = Builder::new().prefix("rust-read-manga-").tempdir()?;
            for i in 0..2 {
                let file_path = temp_dir.path().join(format!("page{}.png", i));
                let img = image::RgbImage::new(100, 100);
                img.save(&file_path)?;
                files.push(file_path.to_str().unwrap().to_string());
            }
        } else {
            let path = if self.recursive {
                format!("{}/**/*.{{{}}}", self.path, self.extensions.join(","))
            } else {
                format!("{}/*.{{{}}}", self.path, self.extensions.join(","))
            };

            for entry in glob(&path)? {
                files.push(entry?.to_str().map(|s| s.to_string()).ok_or_else(|| anyhow!("Invalid UTF-8 path"))?);
            }
        }
        context.set("files", files)?;
        Ok(context)
    }
}

#[derive(Debug, Deserialize, Default)]
pub struct ImagePreprocessNode {
    #[serde(default)]
    pub auto_contrast: bool,
    #[serde(default)]
    pub normalize: bool,
    #[serde(default)]
    pub resize: String,
    #[serde(default)]
    pub grayscale: bool,
    #[serde(default)]
    pub denoise_level: u8,
}

impl Node for ImagePreprocessNode {
    async fn run(&self, mut context: Context) -> Result<Context> {
        let files = context
            .get::<Vec<String>>("files")
            .ok_or_else(|| anyhow!("No files in context"))?
            .clone();
        let temp_dir = Builder::new().prefix("rust-read-manga-").tempdir()?;
        let mut processed_files: Vec<String> = Vec::new();

        for file in files {
            let mut img = image::open(&file)?;
            if self.grayscale {
                img = DynamicImage::ImageLuma8(imageops::grayscale(&img));
            }
            // if self.auto_contrast {
            //     img = DynamicImage::ImageLuma8(imageops::equalize_histogram(&img.to_luma8()));
            // }
            // if self.normalize {
            //     let luma_img = img.to_luma8();
            //     let normalized = imageops::normalize(&luma_img);
            //     img = DynamicImage::ImageLuma8(normalized);
            // }
            if self.denoise_level > 0 {
                img = DynamicImage::ImageRgba8(imageops::blur(&img, self.denoise_level as f32));
            }
            if !self.resize.is_empty() {
                let dims: Vec<u32> = self
                    .resize
                    .split('x')
                    .filter_map(|s| s.parse().ok())
                    .collect();
                if dims.len() == 2 {
                    img = img.resize_exact(dims[0], dims[1], imageops::FilterType::Lanczos3);
                } else {
                    return Err(anyhow!("Invalid resize dimensions: {}", self.resize));
                }
            }

            let new_path = temp_dir.path().join(
                std::path::Path::new(&file)
                    .file_name()
                    .ok_or_else(|| anyhow!("Failed to get file name"))?,
            );
            img.save(&new_path)?;
            processed_files.push(new_path.to_str().unwrap().to_string());
        }

        context.set("files", processed_files)?;
        Ok(context)
    }
}

#[derive(Debug, Deserialize, Default, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum OcrEngine {
    #[default]
    Tesseract,
}

#[derive(Debug, Deserialize, Default)]
pub struct OcrNode {
    #[serde(default)]
    pub engine: OcrEngine,
    #[serde(default)]
    pub language: String,
    #[serde(default)]
    pub min_confidence: f32,
    #[serde(default)]
    pub scale_before_ocr: bool,
    #[serde(default)]
    pub detect_bubbles: bool,
}

impl Node for OcrNode {
    async fn run(&self, mut context: Context) -> Result<Context> {
        let files = context
            .get::<Vec<String>>("files")
            .ok_or_else(|| anyhow!("No files in context"))?
            .clone();
        let mut ocr_results: Vec<String> = Vec::new();

        let mut tesseract = Tesseract::new(None, Some(&self.language))?;

        for file in files {
            tesseract = tesseract.set_image(&file)?;
            let text = tesseract.get_text()?;
            ocr_results.push(text);
        }

        context.set("ocr_results", ocr_results)?;
        Ok(context)
    }
}

#[derive(Debug, Deserialize, Default, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum DurationCurve {
    #[default]
    Linear,
    Sqrt,
    Log,
}

#[derive(Debug, Deserialize, Default)]
pub struct DurationNode {
    #[serde(default)]
    pub reading_speed_wpm: u32,
    #[serde(default)]
    pub min_seconds: f32,
    #[serde(default)]
    pub max_seconds: f32,
    #[serde(default)]
    pub base_time: f32,
    #[serde(default)]
    pub curve: DurationCurve,
}

impl Node for DurationNode {
    async fn run(&self, mut context: Context) -> Result<Context> {
        let ocr_results = context
            .get::<Vec<String>>("ocr_results")
            .ok_or_else(|| anyhow!("No ocr_results in context"))?
            .clone();
        let mut durations: Vec<f32> = Vec::new();

        for text in ocr_results {
            let word_count = text.split_whitespace().count() as f32;
            let mut duration = self.base_time;

            if self.reading_speed_wpm == 0 {
                return Err(anyhow!("Reading speed (wpm) cannot be zero."));
            }
            duration += (word_count / self.reading_speed_wpm as f32) * 60.0;

            duration = match self.curve {
                DurationCurve::Linear => duration,
                DurationCurve::Sqrt => duration.sqrt(),
                DurationCurve::Log => duration.log10(),
            };

            duration = duration.max(self.min_seconds).min(self.max_seconds);
            durations.push(duration);
        }

        context.set("durations", durations)?;
        Ok(context)
    }
}

#[derive(Debug, Deserialize, Default, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum VideoCodec {
    #[default]
    H264,
    H265,
}

#[derive(Debug, Deserialize, Default)]
pub struct EncoderNode {
    #[serde(default)]
    pub codec: VideoCodec,
    #[serde(default)]
    pub fps: u32,
    #[serde(default)]
    pub resolution: String,
    #[serde(default)]
    pub bitrate: u32,
    #[serde(default)]
    pub crf: Option<u8>,
}

impl Node for EncoderNode {
    async fn run(&self, context: Context) -> Result<Context> {
        if !ffmpeg_is_installed() {
            return Err(anyhow!("FFmpeg is not installed. Please install FFmpeg and ensure it is in your system's PATH."));
        }

        let files = context
            .get::<Vec<String>>("files")
            .ok_or_else(|| anyhow!("No files in context"))?
            .clone();
        let durations = context
            .get::<Vec<f32>>("durations")
            .ok_or_else(|| anyhow!("No durations in context"))?
            .clone();
        let output_path = context
            .get::<String>("output_path")
            .ok_or_else(|| anyhow!("No output_path in context"))?
            .clone();

        let mut command = FfmpegCommand::new();

        if let Some(transition_type) = context.get::<TransitionType>("transition_type") {
            let transition_duration = context.get::<f32>("transition_duration").unwrap_or(0.5);
            let transition_ease = context.get::<Ease>("transition_ease");

            let mut filter_complex = String::new();
            let mut inputs = String::new();

            for (i, file) in files.iter().enumerate() {
                inputs.push_str(&format!("-i {} ", file));
                let d = durations[i] - if i > 0 { transition_duration } else { 0.0 };
                filter_complex.push_str(&format!(
                    "[{i}:v]scale={},setsar=1,fade=t=in:st=0:d={}:alpha=1[v{i}];",
                    self.resolution, transition_duration
                ));
            }

            let mut last_v = "v0".to_string();
            for i in 0..files.len() - 1 {
                let offset = durations[0..i + 1].iter().sum::<f32>() - transition_duration;
                let transition = match transition_type {
                    TransitionType::Fade => "fade",
                    TransitionType::SlideLeft => "slideleft",
                    TransitionType::SlideRight => "slideright",
                    TransitionType::SlideUp => "slideup",
                    TransitionType::SlideDown => "slidedown",
                };
                let ease = match transition_ease {
                    Some(Ease::Linear) => "linear",
                    Some(Ease::EaseIn) => "qsin",
                    Some(Ease::EaseOut) => "hsin",
                    Some(Ease::EaseInOut) => "esin",
                    _ => "linear",
                };

                filter_complex.push_str(&format!(
                    "[{last_v}][v{next}]xfade=transition={transition}:duration={duration}:offset={offset}:expr='{ease}'[v{next}_t];",
                    last_v = last_v,
                    next = i + 1,
                    transition = transition,
                    duration = transition_duration,
                    offset = offset,
                    ease = ease
                ));
                last_v = format!("v{}_t", i + 1);
            }

            command.args(inputs.split_whitespace());
            command.arg("-filter_complex").arg(&filter_complex);
            command.arg("-map").arg(format!("[{}]", last_v));
        } else {
            let mut input_file = Builder::new().suffix(".txt").tempfile()?;
            for (file, duration) in files.iter().zip(durations.iter()) {
                writeln!(input_file, "file '{}'", file)?;
                writeln!(input_file, "duration {}", duration)?;
            }
            command
                .arg("-f")
                .arg("concat")
                .arg("-safe")
                .arg("0")
                .arg("-i")
                .arg(input_file.path().to_str().ok_or_else(|| anyhow!("Invalid UTF-8 path for temporary file"))?);
        }

        if let Some(audio_source) = context.get::<String>("audio_source") {
            command.arg("-i").arg(audio_source);
            if let Some(loop_audio) = context.get::<bool>("loop_audio") {
                if loop_audio {
                    command.arg("-stream_loop").arg("-1");
                }
            }
            if let Some(sync_to_pages) = context.get::<bool>("sync_to_pages") {
                if sync_to_pages {
                    command.arg("-shortest");
                }
            }
            if let Some(volume) = context.get::<f32>("volume") {
                command.arg("-filter:a").arg(format!("volume={}", volume));
            }
        }

        command
            .arg("-vsync")
            .arg("vfr")
            .arg("-pix_fmt")
            .arg("yuv420p")
            .arg("-y")
            .arg(&output_path);

        match self.codec {
            VideoCodec::H264 => {
                command.arg("-c:v").arg("libx264");
            }
            VideoCodec::H265 => {
                command.arg("-c:v").arg("libx265");
            }
        }

        if self.fps > 0 {
            command.arg("-r").arg(self.fps.to_string());
        }
        if !self.resolution.is_empty() {
            command.arg("-s").arg(&self.resolution);
        }
        if self.bitrate > 0 {
            command.arg("-b:v").arg(format!("{}k", self.bitrate));
        }
        if let Some(crf) = self.crf {
            command.arg("-crf").arg(crf.to_string());
        }

        println!("ffmpeg command: {:?}", command);

        command.spawn()?.wait()?;

        Ok(context)
    }
}

#[derive(Debug, Deserialize, serde::Serialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum TransitionType {
    Fade,
    SlideLeft,
    SlideRight,
    SlideUp,
    SlideDown,
}

impl Default for TransitionType {
    fn default() -> Self {
        TransitionType::Fade
    }
}

#[derive(Debug, Deserialize, serde::Serialize, Default, PartialEq, Eq, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum Ease {
    #[default]
    Linear,
    EaseInOut,
    EaseIn,
    EaseOut,
}

#[derive(Debug, Deserialize, Default)]
pub struct TransitionNode {
    #[serde(rename = "type")]
    #[serde(default)]
    pub transition_type: TransitionType,
    #[serde(default)]
    pub duration: f32,
    #[serde(default)]
    pub ease: Ease,
}

impl Node for TransitionNode {
    async fn run(&self, mut context: Context) -> Result<Context> {
        context.set("transition_type", self.transition_type)?;
        context.set("transition_duration", self.duration)?;
        context.set("transition_ease", self.ease)?;
        Ok(context)
    }
}

#[derive(Debug, Deserialize, Default)]
pub struct AudioNode {
    pub source: String,
    #[serde(default)]
    pub volume: f32,
    #[serde(default)]
    pub sync_to_pages: bool,
    #[serde(rename = "loop")]
    #[serde(default)]
    pub loop_audio: bool,
}

impl Node for AudioNode {
    async fn run(&self, mut context: Context) -> Result<Context> {
        context.set("audio_source", self.source.clone())?;
        context.set("volume", self.volume)?;
        context.set("sync_to_pages", self.sync_to_pages)?;
        context.set("loop_audio", self.loop_audio)?;
        Ok(context)
    }
}

#[derive(Debug, Deserialize, Default)]
pub struct OutputWriterNode {
    pub output_path: String,
    #[serde(default)]
    pub overwrite: bool,
    #[serde(default)]
    pub create_dirs: bool,
}

impl Node for OutputWriterNode {
    async fn run(&self, mut context: Context) -> Result<Context> {
        context.set("output_path", self.output_path.clone())?;
        if self.create_dirs {
            let path = std::path::Path::new(&self.output_path);
            if let Some(parent) = path.parent() {
                std::fs::create_dir_all(parent)?;
            }
        }
        Ok(context)
    }
}
