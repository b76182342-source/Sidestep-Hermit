#!/bin/bash

# 测试启动脚本 - 用于单独测试各个UI模块

show_usage() {
    echo "用法: ./test-ui.sh <模块名>"
    echo ""
    echo "可用模块:"
    echo "  styles     - 测试样式系统"
    echo "  components - 测试组件库 (新增)"
    echo "  sidebar    - 测试侧边栏组件"
    echo "  player     - 测试播放器控制栏"
    echo "  tracklist  - 测试歌曲列表组件"
    echo "  full       - 测试完整应用"
    echo ""
    echo "示例:"
    echo "  ./test-ui.sh components  # 查看所有基础组件"
    echo "  ./test-ui.sh styles"
    echo "  ./test-ui.sh player"
}

if [ $# -eq 0 ]; then
    show_usage
    exit 1
fi

MODULE=$1

case $MODULE in
    styles)
        echo "🎨 启动样式系统测试..."
        cd kg-app/src-ui && VITE_TEST_MODE=styles npm run dev
        ;;
    components)
        echo "🧩 启动组件库测试..."
        cd kg-app/src-ui && VITE_TEST_MODE=components npm run dev
        ;;
    sidebar)
        echo "📱 启动侧边栏测试..."
        cd kg-app/src-ui && VITE_TEST_MODE=sidebar npm run dev
        ;;
    player)
        echo "▶️  启动播放器测试..."
        cd kg-app/src-ui && VITE_TEST_MODE=player npm run dev
        ;;
    tracklist)
        echo "📋 启动歌曲列表测试..."
        cd kg-app/src-ui && VITE_TEST_MODE=tracklist npm run dev
        ;;
    full)
        echo "🚀 启动完整应用..."
        cd kg-app/src-ui && npm run dev
        ;;
    *)
        echo "❌ 未知模块: $MODULE"
        echo ""
        show_usage
        exit 1
        ;;
esac
