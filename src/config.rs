use crate::error::{Result, VideoClipError};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Config {
    #[serde(default = "default_platform")]
    pub platform: String,
    
    #[serde(default)]
    pub groq_api_key: Option<String>,
    
    #[serde(default = "default_min_duration")]
    pub min_duration: u32,
    
    #[serde(default = "default_max_duration")]
    pub max_duration: u32,
    
    #[serde(default = "default_num_clips")]
    pub num_clips: u32,
    
    #[serde(default = "default_output_dir")]
    pub output_dir: String,
    
    #[serde(default = "default_smart_crop")]
    pub smart_crop: bool,
    
    #[serde(default)]
    pub burn_captions: bool,
    
    #[serde(default)]
    pub thumbnail: bool,
}

fn default_platform() -> String {
    "tiktok".to_string()
}

fn default_min_duration() -> u32 {
    15
}

fn default_max_duration() -> u32 {
    60
}

fn default_num_clips() -> u32 {
    3
}

fn default_output_dir() -> String {
    "./clips".to_string()
}

fn default_smart_crop() -> bool {
    true
}

impl Default for Config {
    fn default() -> Self {
        Self {
            platform: default_platform(),
            groq_api_key: None,
            min_duration: default_min_duration(),
            max_duration: default_max_duration(),
            num_clips: default_num_clips(),
            output_dir: default_output_dir(),
            smart_crop: default_smart_crop(),
            burn_captions: false,
            thumbnail: false,
        }
    }
}

impl Config {
    pub fn load(config_path: Option<&Path>) -> Result<Self> {
        let mut config = if let Some(path) = config_path {
            if path.exists() {
                let content = fs::read_to_string(path)?;
                toml::from_str(&content)?
            } else {
                Config::default()
            }
        } else {
            // Try to load from default location
            let default_path = PathBuf::from(".videoclip.toml");
            if default_path.exists() {
                let content = fs::read_to_string(default_path)?;
                toml::from_str(&content)?
            } else {
                Config::default()
            }
        };

        // Override with environment variable if present
        if config.groq_api_key.is_none() {
            config.groq_api_key = std::env::var("GROQ_API_KEY").ok();
        }

        Ok(config)
    }

    pub fn get_api_key(&self) -> Result<String> {
        self.groq_api_key
            .clone()
            .ok_or_else(|| {
                VideoClipError::Config(
                    "Groq API key not found. Set GROQ_API_KEY environment variable or add to config file".to_string()
                )
            })
    }
}

#[derive(Debug, Clone)]
pub struct PlatformPreset {
    pub name: &'static str,
    pub aspect_ratio: (u32, u32),
    pub max_duration: u32,
    pub resolution: (u32, u32),
    pub video_codec: &'static str,
    pub audio_codec: &'static str,
    pub video_bitrate: &'static str,
}

impl PlatformPreset {
    pub fn from_name(name: &str) -> Result<Self> {
        match name.to_lowercase().as_str() {
            "tiktok" => Ok(Self::tiktok()),
            "shorts" | "youtube" => Ok(Self::shorts()),
            "reels" | "instagram" => Ok(Self::reels()),
            "twitter" | "x" => Ok(Self::twitter()),
            _ => Err(VideoClipError::Config(format!(
                "Unknown platform: {}. Supported: tiktok, shorts, reels, twitter",
                name
            ))),
        }
    }

    pub fn tiktok() -> Self {
        Self {
            name: "TikTok",
            aspect_ratio: (9, 16),
            max_duration: 60,
            resolution: (1080, 1920),
            video_codec: "libx264",
            audio_codec: "aac",
            video_bitrate: "2500k",
        }
    }

    pub fn shorts() -> Self {
        Self {
            name: "YouTube Shorts",
            aspect_ratio: (9, 16),
            max_duration: 60,
            resolution: (1080, 1920),
            video_codec: "libx264",
            audio_codec: "aac",
            video_bitrate: "2500k",
        }
    }

    pub fn reels() -> Self {
        Self {
            name: "Instagram Reels",
            aspect_ratio: (9, 16),
            max_duration: 90,
            resolution: (1080, 1920),
            video_codec: "libx264",
            audio_codec: "aac",
            video_bitrate: "2500k",
        }
    }

    pub fn twitter() -> Self {
        Self {
            name: "Twitter/X",
            aspect_ratio: (16, 9),
            max_duration: 140,
            resolution: (1280, 720),
            video_codec: "libx264",
            audio_codec: "aac",
            video_bitrate: "2000k",
        }
    }
}
