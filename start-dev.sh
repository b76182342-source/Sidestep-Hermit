#!/bin/bash
# KGPlayer 开发模式启动脚本

cd /home/lilin/rust_music/my_music/kg-app

echo "🎵 正在启动 KGPlayer 开发模式..."
echo "📁 当前目录: $(pwd)"
echo ""

# 检查 tauri.conf.json 是否存在
if [ ! -f "tauri.conf.json" ]; then
    echo "❌ 错误: 找不到 tauri.conf.json"
    exit 1
fi

echo "✅ 找到 tauri.conf.json"
echo ""

# 启动 Tauri 开发模式
echo "🚀 启动 Tauri..."
cd src-ui && npm run dev &
VITE_PID=$!

sleep 3

cd ..
cargo tauri dev

# 清理
kill $VITE_PID 2>/dev/null
