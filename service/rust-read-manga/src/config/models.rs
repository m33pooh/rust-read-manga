use serde::Deserialize;

#[derive(Debug, Deserialize, Default)]
pub struct AppConfig {
    pub video: VideoConfig,
    pub timing: TimingConfig,
    #[serde(default)]
    pub plugins: PluginsConfig,
    #[serde(default)]
    pub scripting: ScriptingConfig,
}

#[derive(Debug, Deserialize, Default)]
pub struct PluginsConfig {
    pub directory: Option<String>,
    #[serde(default)]
    pub enabled: Vec<String>,
}

#[derive(Debug, Deserialize, Default)]
pub struct ScriptingConfig {
    pub directory: Option<String>,
    #[serde(default)]
    pub enabled: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            video: VideoConfig::default(),
            timing: TimingConfig::default(),
            plugins: PluginsConfig::default(),
            scripting: ScriptingConfig::default(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct VideoConfig {
    pub codec: VideoCodec,
    pub resolution: Resolution,
    pub fps: u32,
    pub bitrate_kbps: u32,
}

impl Default for VideoConfig {
    fn default() -> Self {
        Self {
            codec: VideoCodec::H264,
            resolution: Resolution {
                width: 1920,
                height: 1080,
            },
            fps: 30,
            bitrate_kbps: 5000,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum VideoCodec {
    H264,
    H265,
    Vp9,
    Theora,
}

impl ToString for VideoCodec {
    fn to_string(&self) -> String {
        match self {
            VideoCodec::H264 => "libx264".to_string(),
            VideoCodec::H265 => "libx265".to_string(),
            VideoCodec::Vp9 => "libvpx-vp9".to_string(),
            VideoCodec::Theora => "libtheora".to_string(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Resolution {
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct TimingConfig {
    pub mode: TimingMode,
    pub reading_speed_wpm: Option<u32>,
    pub reading_speed_cps: Option<u32>,
    pub min_duration_sec: f32,
    pub max_duration_sec: f32,
    pub base_time_sec: f32,
    pub custom_formula: Option<String>,
}

impl Default for TimingConfig {
    fn default() -> Self {
        Self {
            mode: TimingMode::Auto,
            reading_speed_wpm: Some(180),
            reading_speed_cps: Some(12),
            min_duration_sec: 1.0,
            max_duration_sec: 15.0,
            base_time_sec: 0.5,
            custom_formula: None,
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum TimingMode {
    Words,
    Chars,
    Auto,
    Custom,
}

