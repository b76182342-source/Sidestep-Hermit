#!/bin/bash

# UI 集成快速启动脚本

echo "🎵 KGPlayer UI 模块化测试系统"
echo "================================"
echo ""
echo "首次使用？请先安装依赖："
echo "  cd kg-app/src-ui && npm install"
echo ""
echo "测试模式："
echo "  1. 样式系统测试"
echo "  2. 侧边栏测试"
echo "  3. 播放器测试"
echo "  4. 歌曲列表测试"
echo "  5. 完整应用"
echo "  0. 退出"
echo ""
read -p "请选择 (0-5): " choice

case $choice in
    1)
        echo "🎨 启动样式系统测试..."
        ./test-ui.sh styles
        ;;
    2)
        echo "📱 启动侧边栏测试..."
        ./test-ui.sh sidebar
        ;;
    3)
        echo "▶️  启动播放器测试..."
        ./test-ui.sh player
        ;;
    4)
        echo "📋 启动歌曲列表测试..."
        ./test-ui.sh tracklist
        ;;
    5)
        echo "🚀 启动完整应用..."
        ./test-ui.sh full
        ;;
    0)
        echo "👋 再见！"
        exit 0
        ;;
    *)
        echo "❌ 无效选择"
        exit 1
        ;;
esac
