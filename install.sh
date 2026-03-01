#!/bin/bash

# VideoClip Installation Script
# This script helps set up VideoClip on your system

set -e

echo "🎬 VideoClip Installation Script"
echo "================================"
echo ""

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "❌ Rust is not installed."
    echo "Please install Rust from: https://rustup.rs/"
    exit 1
fi
echo "✅ Rust detected: $(rustc --version)"

# Check if FFmpeg is installed
if ! command -v ffmpeg &> /dev/null; then
    echo "⚠️  FFmpeg is not installed."
    echo ""
    echo "Installing FFmpeg..."
    
    if command -v dnf &> /dev/null; then
        sudo dnf install -y ffmpeg
    elif command -v apt &> /dev/null; then
        sudo apt install -y ffmpeg
    elif command -v brew &> /dev/null; then
        brew install ffmpeg
    elif command -v pacman &> /dev/null; then
        sudo pacman -S ffmpeg
    else
        echo "❌ Cannot detect package manager."
        echo "Please install FFmpeg manually: https://ffmpeg.org/download.html"
        exit 1
    fi
else
    echo "✅ FFmpeg detected: $(ffmpeg -version | head -1)"
fi

echo ""
echo "📦 Building VideoClip..."
cargo build --release

echo ""
echo "✅ Build successful!"
echo ""
echo "🎯 Installation Options:"
echo ""
echo "Option 1 - Install globally:"
echo "  cargo install --path ."
echo "  Then use: videoclip <video.mp4>"
echo ""
echo "Option 2 - Use from current directory:"
echo "  ./target/release/videoclip <video.mp4>"
echo ""
echo "Option 3 - Add to PATH:"
echo "  sudo ln -s $(pwd)/target/release/videoclip /usr/local/bin/videoclip"
echo "  Then use: videoclip <video.mp4>"
echo ""

# Check for API key
if [ -z "$GROQ_API_KEY" ]; then
    echo "⚠️  GROQ_API_KEY environment variable not set"
    echo ""
    echo "To set your API key:"
    echo "  export GROQ_API_KEY='your-api-key-here'"
    echo ""
    echo "Get your free API key at: https://console.groq.com/"
    echo ""
    echo "To make it permanent, add to your ~/.bashrc or ~/.zshrc:"
    echo "  echo 'export GROQ_API_KEY=\"your-key\"' >> ~/.bashrc"
else
    echo "✅ GROQ_API_KEY is set"
fi

echo ""
echo "📚 Next Steps:"
echo "  1. Set your Groq API key (if not done)"
echo "  2. Read QUICKSTART.md for usage examples"
echo "  3. Try: ./target/release/videoclip --help"
echo ""
echo "🎉 VideoClip is ready to use!"
