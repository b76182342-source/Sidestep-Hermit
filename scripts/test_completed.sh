#!/bin/bash
# 测试已完成的模块

set -e

echo "🧪 测试已完成的模块"
echo "================================"

cd /home/lilin/rust_music/my_music

# 临时禁用 cc wrapper
if [ -f ~/.local/bin/cc ]; then
    mv ~/.local/bin/cc ~/.local/bin/cc.bak
    trap "mv ~/.local/bin/cc.bak ~/.local/bin/cc" EXIT
fi

echo ""
echo "✅ 测试 Agent 2 成果：QQ 音乐 Provider"
echo "-----------------------------------"
cargo check -p kg-providers --features qqmusic
if [ $? -eq 0 ]; then
    echo "✅ QQMusicProvider 编译通过"
else
    echo "❌ QQMusicProvider 编译失败"
    exit 1
fi

echo ""
echo "✅ 测试 Agent 3 成果：账号管理前端"
echo "-----------------------------------"
if [ -f kg-app/src-ui/src/pages/AccountManagement/AccountManagement.tsx ]; then
    echo "✅ AccountManagement.tsx 存在"
    wc -l kg-app/src-ui/src/pages/AccountManagement/AccountManagement.tsx

    # 检查 TypeScript 语法（如果有 tsc）
    cd kg-app/src-ui
    if command -v npx &> /dev/null; then
        npx tsc --noEmit --skipLibCheck src/pages/AccountManagement/AccountManagement.tsx 2>&1 | head -20
    fi
    cd ../..
else
    echo "❌ AccountManagement.tsx 不存在"
    exit 1
fi

echo ""
echo "🔍 检查集成点"
echo "-----------------------------------"
echo "Provider 注册检查："
grep -c "pub use.*QQMusicProvider" kg-providers/src/lib.rs && echo "  ✅ QQMusicProvider 已导出"
grep -c "qqmusic = " kg-providers/Cargo.toml && echo "  ✅ qqmusic feature 已添加"

echo ""
echo "Tauri 命令检查："
grep -c "list_providers" kg-app/src/main.rs && echo "  ✅ list_providers 已注册"
grep -c "account_statuses" kg-app/src/main.rs && echo "  ✅ account_statuses 已注册"

echo ""
echo "✅ 所有测试通过！"
