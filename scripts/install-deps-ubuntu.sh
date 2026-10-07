#!/bin/bash
set -e

echo "========================================"
echo "  KGPlayer 系统依赖安装脚本"
echo "  适用于 Ubuntu 24.04+ / Debian"
echo "========================================"
echo ""

# 检测发行版
if [ -f /etc/os-release ]; then
    . /etc/os-release
    OS=$ID
    VER=$VERSION_ID
else
    echo "❌ 无法检测系统版本"
    exit 1
fi

echo "检测到系统: $OS $VER"
echo ""

# 安装 GStreamer
echo "📦 安装 GStreamer 运行时..."
sudo apt-get update
sudo apt-get install -y \
    gstreamer1.0-plugins-base \
    gstreamer1.0-plugins-good \
    gstreamer1.0-plugins-bad \
    gstreamer1.0-plugins-ugly \
    gstreamer1.0-libav

echo "✅ GStreamer 安装完成"
echo ""

# 安装 WebKit（Tauri 需要）
echo "📦 安装 WebKit2GTK..."
sudo apt-get install -y \
    libwebkit2gtk-4.1-0 \
    libwebkit2gtk-4.1-dev \
    libsoup-3.0-dev \
    libjavascriptcoregtk-4.1-dev

echo "✅ WebKit 安装完成"
echo ""

# 安装 GStreamer 开发库（编译需要）
echo "📦 安装 GStreamer 开发库..."
sudo apt-get install -y \
    libgstreamer1.0-dev \
    libgstreamer-plugins-base1.0-dev \
    libgstreamer-plugins-bad1.0-dev \
    pkg-config

echo "✅ GStreamer 开发库安装完成"
echo ""

# 检查 Rust
if ! command -v cargo &> /dev/null; then
    echo "📦 安装 Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
    echo "✅ Rust 安装完成"
else
    echo "✅ Rust 已安装: $(rustc --version)"
fi
echo ""

# 检查 Node.js
if ! command -v node &> /dev/null; then
    echo "📦 安装 Node.js 20..."
    curl -fsSL https://deb.nodesource.com/setup_20.x | sudo -E bash -
    sudo apt-get install -y nodejs
    echo "✅ Node.js 安装完成"
else
    echo "✅ Node.js 已安装: $(node --version)"
fi
echo ""

echo "========================================"
echo "  🎉 所有依赖安装完成！"
echo "========================================"
echo ""
echo "下一步："
echo "  1. cd kg-app/src-ui"
echo "  2. npm install"
echo "  3. npm run tauri dev"
echo ""
