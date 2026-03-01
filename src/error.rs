use thiserror::Error;

#[derive(Error, Debug)]
pub enum VideoClipError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("FFmpeg error: {0}")]
    FFmpeg(String),

    #[error("API error: {0}")]
    Api(String),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Invalid video format: {0}")]
    InvalidFormat(String),

    #[error("API authentication failed: {0}")]
    AuthenticationFailed(String),

    #[error("No engaging moments found in video")]
    NoHighlightsFound,

    #[error("Video too long (max 4 hours): {0}")]
    VideoTooLong(String),

    #[error("Disk space low: {0}")]
    DiskSpaceLow(String),

    #[error("JSON parsing error: {0}")]
    JsonParse(#[from] serde_json::Error),

    #[error("TOML parsing error: {0}")]
    TomlParse(#[from] toml::de::Error),

    #[error("HTTP request error: {0}")]
    Http(#[from] reqwest::Error),
}

pub type Result<T> = std::result::Result<T, VideoClipError>;
