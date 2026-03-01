use crate::config::PlatformPreset;
use crate::error::{Result, VideoClipError};
use std::path::Path;
use std::process::{Command, Stdio};
use tracing::{debug, info};

pub struct VideoFormatter;

impl VideoFormatter {
    pub fn new() -> Self {
        Self
    }

    /// Format a video clip according to platform preset
    pub fn format_for_platform(
        &self,
        input: &Path,
        output: &Path,
        preset: &PlatformPreset,
        smart_crop: bool,
    ) -> Result<()> {
        info!("🎬 Formatting for {}...", preset.name);

        let (input_width, input_height) = self.get_video_resolution(input)?;
        let input_aspect = input_width as f64 / input_height as f64;
        let target_aspect = preset.aspect_ratio.0 as f64 / preset.aspect_ratio.1 as f64;

        let filter = if (input_aspect - target_aspect).abs() < 0.01 {
            // Aspect ratio is close enough, just scale
            format!(
                "scale={}:{}:force_original_aspect_ratio=decrease,pad={}:{}:(ow-iw)/2:(oh-ih)/2",
                preset.resolution.0, preset.resolution.1, preset.resolution.0, preset.resolution.1
            )
        } else if smart_crop {
            // Smart crop to target aspect ratio
            self.build_smart_crop_filter(
                input_width,
                input_height,
                preset.resolution.0,
                preset.resolution.1,
                preset.aspect_ratio,
            )
        } else {
            // Simple scale and pad
            format!(
                "scale={}:{}:force_original_aspect_ratio=decrease,pad={}:{}:(ow-iw)/2:(oh-ih)/2:black",
                preset.resolution.0, preset.resolution.1, preset.resolution.0, preset.resolution.1
            )
        };

        debug!("FFmpeg filter: {}", filter);

        let result = Command::new("ffmpeg")
            .args([
                "-i",
                input.to_str().unwrap(),
                "-vf",
                &filter,
                "-c:v",
                preset.video_codec,
                "-b:v",
                preset.video_bitrate,
                "-c:a",
                preset.audio_codec,
                "-b:a",
                "128k",
                "-ar",
                "44100",
                "-movflags",
                "+faststart",
                "-y",
                output.to_str().unwrap(),
            ])
            .stdout(Stdio::null())
            .stderr(Stdio::piped())
            .output()?;

        if !result.status.success() {
            let stderr = String::from_utf8_lossy(&result.stderr);
            return Err(VideoClipError::FFmpeg(format!(
                "Failed to format video: {}",
                stderr
            )));
        }

        debug!("Video formatted successfully");
        Ok(())
    }

    fn get_video_resolution(&self, input: &Path) -> Result<(u32, u32)> {
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

    fn build_smart_crop_filter(
        &self,
        input_width: u32,
        input_height: u32,
        target_width: u32,
        target_height: u32,
        aspect_ratio: (u32, u32),
    ) -> String {
        // Calculate crop dimensions to match target aspect ratio
        let input_aspect = input_width as f64 / input_height as f64;
        let target_aspect = aspect_ratio.0 as f64 / aspect_ratio.1 as f64;

        if input_aspect > target_aspect {
            // Input is wider, crop width
            let crop_width = (input_height as f64 * target_aspect) as u32;
            let x_offset = (input_width - crop_width) / 2;
            format!(
                "crop={}:{}:{}:0,scale={}:{}",
                crop_width, input_height, x_offset, target_width, target_height
            )
        } else {
            // Input is taller, crop height
            let crop_height = (input_width as f64 / target_aspect) as u32;
            let y_offset = (input_height - crop_height) / 2;
            format!(
                "crop={}:{}:0:{},scale={}:{}",
                input_width, crop_height, y_offset, target_width, target_height
            )
        }
    }

    /// Validate that clip duration doesn't exceed platform limit
    pub fn validate_duration(&self, duration: f64, preset: &PlatformPreset) -> Result<f64> {
        if duration > preset.max_duration as f64 {
            debug!(
                "Clip duration {:.1}s exceeds platform limit {}s, capping",
                duration, preset.max_duration
            );
            Ok(preset.max_duration as f64)
        } else {
            Ok(duration)
        }
    }
}
