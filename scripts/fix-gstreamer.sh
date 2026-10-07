#!/bin/bash
set -e

echo "开始安装缺失的 GStreamer 依赖..."
echo ""

# 安装 gstreamer-plugins-bad 开发库
sudo apt-get install -y \
    libgstreamer-plugins-bad1.0-dev \
    gstreamer1.0-plugins-bad

echo ""
echo "✅ 依赖安装完成！"
echo ""
echo "现在可以编译了："
echo "  cargo check --all-features"
echo ""
echo "或直接运行："
echo "  cd kg-app/src-ui && npm run tauri dev"
