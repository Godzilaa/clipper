mod analyzer;
mod clipper;
mod config;
mod error;
mod formatter;

use analyzer::AIAnalyzer;
use clipper::VideoClipper;
use clap::Parser;
use config::{Config, PlatformPreset};
use error::{Result, VideoClipError};
use formatter::VideoFormatter;
use indicatif::{ProgressBar, ProgressStyle};
use std::fs;
use std::path::{Path, PathBuf};
use tracing::{error, info};

#[derive(Parser, Debug)]
#[command(
    name = "videoclip",
    version = "1.0.0",
    about = "AI-Powered Auto Video Clipper using Groq API",
    author = "PrAtIk"
)]
struct Args {
    /// Input video file
    #[arg(value_name = "VIDEO")]
    input: PathBuf,

    /// Target platform: tiktok, shorts, reels, twitter
    #[arg(short, long, default_value = "tiktok")]
    platform: String,

    /// Output directory
    #[arg(short, long, default_value = "./clips")]
    output: PathBuf,

    /// Number of clips to generate
    #[arg(short, long, default_value = "3")]
    num_clips: u32,

    /// Config file path
    #[arg(short, long)]
    config: Option<PathBuf>,

    /// Groq API key (or use GROQ_API_KEY env var)
    #[arg(short = 'k', long)]
    api_key: Option<String>,

    /// Minimum clip duration in seconds
    #[arg(long, default_value = "15")]
    min_duration: u32,

    /// Maximum clip duration in seconds
    #[arg(long, default_value = "60")]
    max_duration: u32,

    /// Show detected moments before clipping
    #[arg(long)]
    preview: bool,

    /// Disable smart crop
    #[arg(long)]
    no_smart_crop: bool,
}

#[tokio::main]
async fn main() {
    // Initialize logging
    tracing_subscriber::fmt::init();

    let args = Args::parse();

    match run(args).await {
        Ok(_) => {
            info!("✅ All done!");
        }
        Err(e) => {
            error!("❌ Error: {}", e);
            eprintln!("\n❌ Error: {}", e);
            std::process::exit(1);
        }
    }
}

async fn run(args: Args) -> Result<()> {
    println!("\n🎬 VideoClip - AI-Powered Auto Video Clipper\n");

    // Load configuration
    let mut config = Config::load(args.config.as_deref())?;

    // Override config with CLI arguments
    if let Some(api_key) = args.api_key {
        config.groq_api_key = Some(api_key);
    }
    config.platform = args.platform;
    config.output_dir = args.output.to_string_lossy().to_string();
    config.num_clips = args.num_clips;
    config.min_duration = args.min_duration;
    config.max_duration = args.max_duration;
    if args.no_smart_crop {
        config.smart_crop = false;
    }

    // Validate input file
    if !args.input.exists() {
        return Err(VideoClipError::Io(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("Input file not found: {}", args.input.display()),
        )));
    }

    // Get API key
    let api_key = config.get_api_key()?;

    // Get platform preset
    let preset = PlatformPreset::from_name(&config.platform)?;
    println!("🎯 Target platform: {}", preset.name);

    // Create components
    let clipper = VideoClipper::new();
    let formatter = VideoFormatter::new();
    let analyzer = AIAnalyzer::new(api_key);

    // Validate video duration
    clipper.validate_video_duration(&args.input)?;

    // Create output directory
    fs::create_dir_all(&args.output)?;

    // Create temp directory for intermediate files
    let temp_dir = std::env::temp_dir().join(format!("videoclip_{}", chrono::Utc::now().timestamp()));
    fs::create_dir_all(&temp_dir)?;

    // Step 1: Extract audio
    println!("\n📹 Step 1/4: Extracting audio...");
    let audio_path = temp_dir.join("audio.mp3");
    let pb = create_progress_bar("Extracting audio");
    clipper.extract_audio(&args.input, &audio_path)?;
    pb.finish_with_message("✓ Audio extracted");

    // Step 2: Transcribe with Whisper
    println!("\n🎤 Step 2/4: Transcribing with Groq Whisper...");
    let pb = create_progress_bar("Transcribing");
    let transcript = analyzer.transcribe(&audio_path).await?;
    pb.finish_with_message(format!("✓ Transcribed {} characters", transcript.text.len()));

    // Step 3: Analyze with AI
    println!("\n🤖 Step 3/4: AI analyzing for highlights...");
    let pb = create_progress_bar("Analyzing");
    let highlights = analyzer
        .find_highlights(
            &transcript,
            config.min_duration,
            config.max_duration,
            config.num_clips,
        )
        .await?;
    pb.finish_with_message(format!("✓ Found {} highlights", highlights.len()));

    // Show preview if requested
    if args.preview {
        println!("\n📋 Detected Highlights:\n");
        for (i, highlight) in highlights.iter().enumerate() {
            let duration = highlight.end - highlight.start;
            println!(
                "  {}. {} ({:.1}s - {:.1}s, duration: {:.1}s)",
                i + 1,
                highlight.title,
                highlight.start,
                highlight.end,
                duration
            );
            println!("     Reason: {}", highlight.reason);
            println!();
        }

        println!("Press Enter to continue with clipping...");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).ok();
    }

    // Step 4: Generate clips
    println!("\n✂️  Step 4/4: Generating clips...");
    let total_clips = highlights.len();

    for (i, highlight) in highlights.iter().enumerate() {
        let duration = highlight.end - highlight.start;
        let capped_duration = formatter.validate_duration(duration, &preset)?;

        println!(
            "\n  [{}/{}] {} ({:.1}s)",
            i + 1,
            total_clips,
            highlight.title,
            capped_duration
        );

        // Extract raw clip
        let raw_clip = temp_dir.join(format!("clip_{}.mp4", i));
        clipper.extract_clip(&args.input, &raw_clip, highlight.start, capped_duration)?;

        // Format for platform
        let filename = clipper.generate_filename(highlight, i);
        let output_path = args.output.join(&filename);

        formatter.format_for_platform(&raw_clip, &output_path, &preset, config.smart_crop)?;

        println!("     ✓ Saved: {}", filename);
    }

    // Cleanup temp directory
    fs::remove_dir_all(&temp_dir).ok();

    println!("\n✅ Success! Generated {} clips in: {}", total_clips, args.output.display());
    println!("\n💡 Tip: Use --preview flag to review highlights before clipping");

    Ok(())
}

fn create_progress_bar(message: &str) -> ProgressBar {
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} {msg}")
            .unwrap(),
    );
    pb.set_message(message.to_string());
    pb.enable_steady_tick(std::time::Duration::from_millis(100));
    pb
}
