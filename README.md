# VideoClip - AI-Powered Auto Video Clipper

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

VideoClip is a Rust-based CLI tool that automatically extracts engaging highlight clips from long-form videos using Groq AI API. Perfect for content creators who need to repurpose podcasts, streams, and tutorials into platform-optimized short-form content.

## ✨ Features

- 🤖 **AI-Powered Analysis** - Uses Groq's Whisper for transcription and Llama for intelligent highlight detection
- 🎬 **Platform Optimization** - Auto-formats for TikTok, YouTube Shorts, Instagram Reels, and Twitter/X
- ✂️ **Smart Clipping** - Extracts self-contained, engaging moments from long videos
- 🎯 **Smart Crop** - Automatically crops to vertical format for mobile platforms
- ⚡ **Fast Processing** - Built in Rust for maximum performance
- 🔧 **Configurable** - CLI arguments and config file support

## 📋 Requirements

- **Rust 1.75+** - [Install Rust](https://rustup.rs/)
- **FFmpeg** - Required for video processing
- **Groq API Key** - [Get your free API key](https://console.groq.com/)

### Installing FFmpeg

**Linux (Ubuntu/Debian):**
```bash
sudo apt update
sudo apt install ffmpeg
```

**macOS:**
```bash
brew install ffmpeg
```

**Windows:**
Download from [ffmpeg.org](https://ffmpeg.org/download.html) and add to PATH.

## 🚀 Installation

### From Source

```bash
# Clone the repository
git clone https://github.com/yourusername/videoclip.git
cd videoclip

# Build release binary
cargo build --release

# The binary will be at target/release/videoclip
# Optionally, install globally:
cargo install --path .
```

## 🔑 Setup

1. Get your Groq API key from https://console.groq.com/

2. Set the API key as an environment variable:

```bash
export GROQ_API_KEY='your-api-key-here'
```

Or add it to your shell profile (~/.bashrc, ~/.zshrc, etc.):

```bash
echo 'export GROQ_API_KEY="your-api-key-here"' >> ~/.bashrc
source ~/.bashrc
```

Alternatively, create a config file (see Configuration section).

## 📖 Usage

### Basic Usage

```bash
# Process a video with default settings (TikTok format, 3 clips)
videoclip input.mp4

# Specify platform and number of clips
videoclip input.mp4 --platform shorts --num-clips 5

# Custom output directory
videoclip input.mp4 --output ./my-clips

# Preview highlights before generating clips
videoclip input.mp4 --preview
```

### CLI Options

```
videoclip <VIDEO> [OPTIONS]

Arguments:
  <VIDEO>  Input video file

Options:
  -p, --platform <PLATFORM>      Target platform: tiktok, shorts, reels, twitter [default: tiktok]
  -o, --output <DIR>             Output directory [default: ./clips]
  -n, --num-clips <N>            Number of clips to generate [default: 3]
  -c, --config <FILE>            Config file path
  -k, --api-key <KEY>            Groq API key (or use GROQ_API_KEY env var)
      --min-duration <SECS>      Minimum clip duration [default: 15]
      --max-duration <SECS>      Maximum clip duration [default: 60]
      --preview                  Show detected moments before clipping
      --no-smart-crop            Disable smart crop
  -h, --help                     Print help
  -V, --version                  Print version
```

### Platform Formats

| Platform | Aspect Ratio | Resolution | Max Duration |
|----------|-------------|------------|--------------|
| TikTok | 9:16 | 1080x1920 | 60s |
| YouTube Shorts | 9:16 | 1080x1920 | 60s |
| Instagram Reels | 9:16 | 1080x1920 | 90s |
| Twitter/X | 16:9 | 1280x720 | 140s |

## ⚙️ Configuration

Create a `.videoclip.toml` file in your project directory:

```toml
# Default platform
platform = "tiktok"

# API configuration
groq_api_key = "your-api-key-here"  # Or use GROQ_API_KEY env var

# Clip settings
min_duration = 15
max_duration = 60
num_clips = 3

# Output
output_dir = "./clips"

# Advanced
smart_crop = true      # Auto-crop to active area
burn_captions = false  # Add subtitles (future feature)
thumbnail = false      # Generate thumbnails (future feature)
```

## 📊 How It Works

1. **Audio Extraction** - Extracts audio track from input video using FFmpeg
2. **Transcription** - Sends audio to Groq's Whisper API for accurate transcription with timestamps
3. **AI Analysis** - Groq's Llama model analyzes the transcript to identify engaging moments
4. **Clip Generation** - Extracts identified segments using FFmpeg
5. **Platform Formatting** - Applies platform-specific formatting (resolution, aspect ratio, codecs)

## 💡 Examples

### Process a podcast for TikTok
```bash
videoclip podcast-episode-42.mp4 --platform tiktok --num-clips 5
```

### Generate YouTube Shorts with preview
```bash
videoclip stream-vod.mp4 --platform shorts --preview
```

### Custom duration range
```bash
videoclip tutorial.mp4 --min-duration 30 --max-duration 90 --num-clips 3
```

### Use specific config file
```bash
videoclip video.mp4 --config ./my-config.toml
```

## 🐛 Troubleshooting

### "FFmpeg not found"
Make sure FFmpeg is installed and in your PATH. Test with: `ffmpeg -version`

### "API authentication failed"
- Check that your Groq API key is correct
- Ensure it's properly set in environment variable or config file
- Get a new key at https://console.groq.com/

### "No engaging moments found"
- Try increasing `--num-clips` value
- Adjust `--min-duration` and `--max-duration` ranges
- The AI might not find suitable clips in very short videos

### "Video too long"
Maximum supported video length is 4 hours. For longer videos, consider splitting them first.

## 📈 Cost Estimation

Groq offers generous free tier. Approximate costs per hour of video:
- Whisper transcription: ~$0.10
- Llama analysis: ~$0.05
- **Total: ~$0.15 per hour of video**

Always check current pricing at https://groq.com/pricing/

## 🗺️ Roadmap

### v1.0 (Current)
- ✅ CLI processing
- ✅ Groq Whisper + Llama integration
- ✅ Multi-platform formatting
- ✅ Config file support
- ✅ Progress indicators

### v2.0 (Planned)
- [ ] Batch processing
- [ ] Watch folder automation
- [ ] Caption burning
- [ ] Thumbnail generation
- [ ] TUI for clip selection
- [ ] Custom AI prompts

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## 📄 License

This project is licensed under the MIT License - see the LICENSE file for details.

## 🙏 Acknowledgments

- [Groq](https://groq.com/) for fast AI inference
- [FFmpeg](https://ffmpeg.org/) for video processing
- Rust community for excellent tooling

## 📞 Support

- 🐛 [Report bugs](https://github.com/yourusername/videoclip/issues)
- 💡 [Request features](https://github.com/yourusername/videoclip/issues)
- 📧 Email: support@example.com

---

**Made with ❤️ in Rust**
