# VideoClip - Quick Start Guide

Welcome to VideoClip! This guide will get you up and running in 5 minutes.

## 🚀 Prerequisites

Before you start, ensure you have:

1. **Rust** (1.75 or later) - Already installed ✅
2. **FFmpeg** - Required for video processing
3. **Groq API Key** - Free at https://console.groq.com/

### Installing FFmpeg

**Linux (Fedora/RHEL):**
```bash
sudo dnf install -y ffmpeg
```

**Linux (Ubuntu/Debian):**
```bash
sudo apt install -y ffmpeg
```

**macOS:**
```bash
brew install ffmpeg
```

Check installation:
```bash
ffmpeg -version
```

## 🔑 Get Your Groq API Key

1. Go to https://console.groq.com/
2. Sign up or log in
3. Navigate to API Keys
4. Create a new API key
5. Copy the key (you'll need it next)

## ⚙️ Setup

### Set your API key (choose one method):

**Method 1: Environment Variable (Recommended)**
```bash
export GROQ_API_KEY='your-api-key-here'
```

Add to your shell profile for persistence:
```bash
echo 'export GROQ_API_KEY="your-api-key-here"' >> ~/.bashrc
source ~/.bashrc
```

**Method 2: Config File**
```bash
# Copy the example config
cp .videoclip.toml.example .videoclip.toml

# Edit and add your API key
nano .videoclip.toml
```

## 🎬 Build and Use

### Build the Application

```bash
# Build release version (already built!)
cargo build --release

# The binary is at: target/release/videoclip

# Optional: Install globally
cargo install --path .
```

### Your First Clip

```bash
# Process a video with default settings
./target/release/videoclip your-video.mp4

# Or if installed globally:
videoclip your-video.mp4
```

This will:
- Transcribe your video using Groq Whisper
- Analyze it for engaging moments using Llama
- Generate 3 TikTok-ready clips in `./clips/`

### Common Use Cases

**Generate YouTube Shorts:**
```bash
videoclip podcast.mp4 --platform shorts --num-clips 5
```

**Preview highlights before clipping:**
```bash
videoclip video.mp4 --preview
```

**Instagram Reels with custom duration:**
```bash
videoclip stream.mp4 --platform reels --min-duration 30 --max-duration 90
```

**Custom output directory:**
```bash
videoclip video.mp4 --output ~/Desktop/clips
```

## 📊 What to Expect

**Processing Time:** Approximately 5-10 minutes per hour of video

**Cost:** ~$0.15 per hour of video (Groq's free tier is generous!)

**Output:** Platform-optimized MP4 files ready to upload

## 🎯 Platform Support

| Platform | Format | Resolution | Duration |
|----------|--------|------------|----------|
| TikTok | 9:16 | 1080x1920 | ≤60s |
| YouTube Shorts | 9:16 | 1080x1920 | ≤60s |
| Instagram Reels | 9:16 | 1080x1920 | ≤90s |
| Twitter/X | 16:9 | 1280x720 | ≤140s |

## 💡 Pro Tips

1. **Use --preview** to review highlights before generating clips
2. **Adjust duration range** to get more or fewer clips
3. **Process trailers first** (5-10 min videos) to test settings
4. **Check clips directory** after each run
5. **Start with short videos** (<30 min) to test

## 🐛 Common Issues

### "FFmpeg not found"
Install FFmpeg (see prerequisites above)

### "API authentication failed"
- Double-check your Groq API key
- Ensure it's properly set in environment or config
- Try generating a new key

### "No engaging moments found"
- Try longer video (minimum 5 minutes recommended)
- Adjust `--min-duration` and `--max-duration`
- Increase `--num-clips`

## 📚 More Help

- Full documentation: [README.md](README.md)
- Groq API docs: https://console.groq.com/docs
- FFmpeg guide: https://ffmpeg.org/documentation.html

## 🎉 Example Workflow

```bash
# 1. Set API key (one time)
export GROQ_API_KEY='your-key-here'

# 2. Process a podcast
videoclip my-podcast.mp4 --platform shorts --preview

# 3. Review the suggested clips in terminal

# 4. Press Enter to generate clips

# 5. Find clips in ./clips/ directory

# 6. Upload to YouTube Shorts!
```

## 🚀 Next Steps

- Try different platforms: `--platform tiktok|shorts|reels|twitter`
- Experiment with clip duration: `--min-duration` and `--max-duration`
- Process multiple videos in a batch
- Check out advanced features in [README.md](README.md)

---

**Questions?** Open an issue on GitHub or check the full README.md

**Happy Clipping! 🎬✨**
