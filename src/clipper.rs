use crate::analyzer::Highlight;
use crate::error::{Result, VideoClipError};
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};
use tracing::{debug, info};

pub struct VideoClipper;

impl VideoClipper {
    pub fn new() -> Self {
        Self
    }

    /// Extract audio from video file to a temporary MP3 file
    pub fn extract_audio(&self, input: &Path, output: &Path) -> Result<()> {
        info!("📹 Extracting audio from video...");

        let output_str = output.to_str().ok_or_else(|| {
            VideoClipError::FFmpeg("Output path contains invalid UTF-8".to_string())
        })?;

        let result = Command::new("ffmpeg")
            .args([
                "-i",
                input.to_str().unwrap(),
                "-vn", // No video
                "-acodec",
                "libmp3lame",
                "-q:a",
                "2", // High quality
                "-y", // Overwrite
                output_str,
            ])
            .stdout(Stdio::null())
            .stderr(Stdio::piped())
            .output()?;

        if !result.status.success() {
            let stderr = String::from_utf8_lossy(&result.stderr);
            return Err(VideoClipError::FFmpeg(format!(
                "Failed to extract audio: {}",
                stderr
            )));
        }

        debug!("Audio extracted to: {}", output_str);
        Ok(())
    }

    /// Get video duration in seconds
    pub fn get_video_duration(&self, input: &Path) -> Result<f64> {
        let output = Command::new("ffprobe")
            .args([
                "-v",
                "error",
                "-show_entries",
                "format=duration",
                "-of",
                "default=noprint_wrappers=1:nokey=1",
                input.to_str().unwrap(),
            ])
            .output()?;

        if !output.status.success() {
            return Err(VideoClipError::FFmpeg(
                "Failed to get video duration".to_string(),
            ));
        }

        let duration_str = String::from_utf8_lossy(&output.stdout);
        let duration: f64 = duration_str
            .trim()
            .parse()
            .map_err(|_| VideoClipError::FFmpeg("Invalid duration format".to_string()))?;

        Ok(duration)
    }

    /// Check if video duration is within limits (max 4 hours)
    pub fn validate_video_duration(&self, input: &Path) -> Result<()> {
        let duration = self.get_video_duration(input)?;
        const MAX_DURATION: f64 = 4.0 * 60.0 * 60.0; // 4 hours

        if duration > MAX_DURATION {
            return Err(VideoClipError::VideoTooLong(format!(
                "{:.1} hours (max: 4 hours)",
                duration / 3600.0
            )));
        }

        Ok(())
    }

    /// Extract a clip from the video
    pub fn extract_clip(
        &self,
        input: &Path,
        output: &Path,
        start: f64,
        duration: f64,
    ) -> Result<()> {
        debug!("Extracting clip: start={:.2}s, duration={:.2}s", start, duration);

        let result = Command::new("ffmpeg")
            .args([
                "-ss",
                &format!("{:.3}", start),
                "-i",
                input.to_str().unwrap(),
                "-t",
                &format!("{:.3}", duration),
                "-c",
                "copy",
                "-avoid_negative_ts",
                "make_zero",
                "-y",
                output.to_str().unwrap(),
            ])
            .stdout(Stdio::null())
            .stderr(Stdio::piped())
            .output()?;

        if !result.status.success() {
            let stderr = String::from_utf8_lossy(&result.stderr);
            return Err(VideoClipError::FFmpeg(format!(
                "Failed to extract clip: {}",
                stderr
            )));
        }

        Ok(())
    }

    /// Get video resolution
    pub fn get_video_resolution(&self, input: &Path) -> Result<(u32, u32)> {
        let output = Command::new("ffprobe")
            .args([
                "-v",
                "error",
                "-select_streams",
                "v:0",
                "-show_entries",
                "stream=width,height",
                "-of",
                "csv=s=x:p=0",
                input.to_str().unwrap(),
            ])
            .output()?;

        if !output.status.success() {
            return Err(VideoClipError::FFmpeg(
                "Failed to get video resolution".to_string(),
            ));
        }

        let resolution_str = String::from_utf8_lossy(&output.stdout);
        let parts: Vec<&str> = resolution_str.trim().split('x').collect();

        if parts.len() != 2 {
            return Err(VideoClipError::FFmpeg(
                "Invalid resolution format".to_string(),
            ));
        }

        let width: u32 = parts[0]
            .parse()
            .map_err(|_| VideoClipError::FFmpeg("Invalid width".to_string()))?;
        let height: u32 = parts[1]
            .parse()
            .map_err(|_| VideoClipError::FFmpeg("Invalid height".to_string()))?;

        Ok((width, height))
    }

    /// Generate a safe filename from highlight title
    pub fn generate_filename(&self, highlight: &Highlight, index: usize) -> String {
        let safe_title = highlight
            .title
            .chars()
            .map(|c| {
                if c.is_alphanumeric() || c == ' ' {
                    c.to_ascii_lowercase()
                } else {
                    '_'
                }
            })
            .collect::<String>()
            .split_whitespace()
            .collect::<Vec<_>>()
            .join("_");

        let safe_title = if safe_title.len() > 50 {
            &safe_title[..50]
        } else {
            &safe_title
        };

        format!("{:02}_{}.mp4", index + 1, safe_title)
    }
}
