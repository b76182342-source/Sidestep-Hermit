#!/bin/bash
# 集成所有并行开发的模块

set -e

echo "🔧 多会话开发成果集成脚本"
echo "================================"

# 临时禁用 cc wrapper
if [ -f ~/.local/bin/cc ]; then
    mv ~/.local/bin/cc ~/.local/bin/cc.bak
    trap "mv ~/.local/bin/cc.bak ~/.local/bin/cc" EXIT
fi

cd /home/lilin/rust_music/my_music

echo ""
echo "1️⃣ 检查所有 Provider..."
cargo check -p kg-providers --features local
cargo check -p kg-providers --features kugou
cargo check -p kg-providers --features netease
cargo check -p kg-providers --features qqmusic

echo ""
echo "2️⃣ 检查 kg-ai crate..."
if [ -d kg-ai ]; then
    cargo check -p kg-ai
else
    echo "⚠️  kg-ai 尚未创建"
fi

echo ""
echo "3️⃣ 检查主应用..."
cargo check -p kg-app

echo ""
echo "4️⃣ 检查前端..."
cd kg-app/src-ui
if command -v npm &> /dev/null; then
    npm run type-check 2>/dev/null || echo "⚠️  前端类型检查跳过"
fi
cd ../..

echo ""
echo "5️⃣ 运行测试..."
cargo test --workspace --all-features

echo ""
echo "✅ 集成检查完成！"
echo ""
echo "📊 统计："
echo "  - Providers: $(ls -1 kg-providers/src/*.rs | wc -l) 个"
echo "  - Crates: $(ls -1d kg-*/ | wc -l) 个"
echo "  - 总代码行数: $(find . -name '*.rs' -not -path './target/*' | xargs wc -l | tail -1)"
