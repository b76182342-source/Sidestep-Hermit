#!/bin/bash

# 停止所有测试服务器

echo "🛑 停止所有测试服务器..."

# 停止 vite 进程
pkill -f vite && echo "✅ 已停止 Vite 服务器" || echo "ℹ️  没有运行中的 Vite 进程"

# 释放 5173 端口
lsof -ti:5173 | xargs kill -9 2>/dev/null && echo "✅ 已释放端口 5173" || echo "ℹ️  端口 5173 未被占用"

echo ""
echo "✅ 清理完成！"
echo ""
echo "现在可以重新开始测试："
echo "  ./test-ui.sh styles"
echo "  ./start-test.sh"
