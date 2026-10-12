#!/bin/bash
# 监控并行 Agent 的进度

echo "🔍 多会话开发监控面板"
echo "================================"
echo "启动时间: $(date '+%Y-%m-%d %H:%M:%S')"
echo ""

while true; do
    clear
    echo "🔍 多会话开发监控面板"
    echo "================================"
    echo "当前时间: $(date '+%H:%M:%S')"
    echo ""

    echo "📊 Agent 状态"
    echo "-----------------------------------"
    echo "✅ Agent 2 (QQ 音乐)      - 已完成"
    echo "✅ Agent 3 (账号管理)     - 已完成"
    echo "🟡 Agent 1 (网易云 API)   - 进行中"
    echo "🟡 Agent 4 (AI 架构)      - 进行中"
    echo ""

    echo "💾 系统资源"
    echo "-----------------------------------"
    free -h | grep "内存："
    echo ""

    echo "📦 文件变化"
    echo "-----------------------------------"
    if [ -f kg-providers/src/qqmusic.rs ]; then
        echo "✅ kg-providers/src/qqmusic.rs ($(wc -l < kg-providers/src/qqmusic.rs) 行)"
    fi

    if [ -f kg-app/src-ui/src/pages/AccountManagement/AccountManagement.tsx ]; then
        echo "✅ AccountManagement.tsx ($(wc -l < kg-app/src-ui/src/pages/AccountManagement/AccountManagement.tsx) 行)"
    fi

    if [ -f kg-providers/src/netease.rs ]; then
        echo "🔄 kg-providers/src/netease.rs ($(wc -l < kg-providers/src/netease.rs) 行)"
    fi

    if [ -d kg-ai ]; then
        echo "🔄 kg-ai/ ($(find kg-ai -name '*.rs' | wc -l) 个文件)"
    fi

    echo ""
    echo "按 Ctrl+C 退出监控"

    sleep 10
done
