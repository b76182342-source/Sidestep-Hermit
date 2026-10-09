#!/bin/bash
# 测试搜索结果中的封面 URL

# 启动应用（后台）
kgplayer &
APP_PID=$!

# 等待应用启动
sleep 3

# 这里需要调用 Tauri 命令来测试搜索
# 但因为没有直接的 CLI 接口，我们需要通过前端调用

# 清理
kill $APP_PID 2>/dev/null

echo "需要通过前端界面测试搜索功能"
