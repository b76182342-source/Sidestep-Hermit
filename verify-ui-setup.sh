#!/bin/bash

# UI 集成验证脚本 - 在开始测试前运行此脚本

# 切换到脚本所在目录
cd "$(dirname "$0")"

echo "🎵 KGPlayer UI 集成验证"
echo "================================"
echo "📍 工作目录: $(pwd)"
echo ""

# 检查目录结构
echo "📁 检查目录结构..."
errors=0

dirs=(
    "kg-app/src-ui/src/styles"
    "kg-app/src-ui/src/components/__tests__"
)

for dir in "${dirs[@]}"; do
    if [ -d "$dir" ]; then
        echo "  ✅ $dir"
    else
        echo "  ❌ $dir 不存在"
        ((errors++))
    fi
done

# 检查样式文件
echo ""
echo "🎨 检查样式文件..."
style_files=(
    "kg-app/src-ui/src/styles/theme.css"
    "kg-app/src-ui/src/styles/globals.css"
    "kg-app/src-ui/src/styles/layout.css"
    "kg-app/src-ui/src/styles/components.css"
    "kg-app/src-ui/src/styles/player.css"
)

for file in "${style_files[@]}"; do
    if [ -f "$file" ]; then
        size=$(wc -l < "$file")
        echo "  ✅ $file ($size 行)"
    else
        echo "  ❌ $file 不存在"
        ((errors++))
    fi
done

# 检查测试文件
echo ""
echo "🧪 检查测试文件..."
test_files=(
    "kg-app/src-ui/src/components/__tests__/StylePreview.tsx"
    "kg-app/src-ui/src/components/__tests__/SidebarTest.tsx"
    "kg-app/src-ui/src/components/__tests__/PlayerTest.tsx"
    "kg-app/src-ui/src/components/__tests__/TrackListTest.tsx"
)

for file in "${test_files[@]}"; do
    if [ -f "$file" ]; then
        size=$(wc -l < "$file")
        echo "  ✅ $file ($size 行)"
    else
        echo "  ❌ $file 不存在"
        ((errors++))
    fi
done

# 检查脚本文件
echo ""
echo "🔧 检查脚本文件..."
scripts=(
    "test-ui.sh"
    "start-test.sh"
)

for script in "${scripts[@]}"; do
    if [ -f "$script" ]; then
        if [ -x "$script" ]; then
            echo "  ✅ $script (可执行)"
        else
            echo "  ⚠️  $script (不可执行，正在修复...)"
            chmod +x "$script"
            echo "     已添加执行权限"
        fi
    else
        echo "  ❌ $script 不存在"
        ((errors++))
    fi
done

# 检查 package.json
echo ""
echo "📦 检查 package.json..."
if [ -f "kg-app/src-ui/package.json" ]; then
    if grep -q "test:styles" kg-app/src-ui/package.json; then
        echo "  ✅ package.json 已更新（包含测试脚本）"
    else
        echo "  ⚠️  package.json 可能缺少测试脚本"
    fi
else
    echo "  ❌ package.json 不存在"
    ((errors++))
fi

# 检查 node_modules
echo ""
echo "📚 检查依赖..."
if [ -d "kg-app/src-ui/node_modules" ]; then
    echo "  ✅ node_modules 已存在"
    need_install=0
else
    echo "  ⚠️  node_modules 不存在，需要运行 npm install"
    need_install=1
fi

# 检查文档
echo ""
echo "📄 检查文档..."
docs=(
    "QUICKSTART_TEST.md"
    "README_UI_INTEGRATION.md"
    "UI_INTEGRATION_COMPLETE.md"
    "UI_INTEGRATION_STATUS.md"
)

for doc in "${docs[@]}"; do
    if [ -f "$doc" ]; then
        echo "  ✅ $doc"
    else
        echo "  ❌ $doc 不存在"
        ((errors++))
    fi
done

# 总结
echo ""
echo "================================"
if [ $errors -eq 0 ]; then
    echo "✅ 所有检查通过！"
    echo ""
    if [ $need_install -eq 1 ]; then
        echo "⚠️  但是需要先安装依赖："
        echo ""
        echo "    cd kg-app/src-ui && npm install && cd ../.."
        echo ""
    else
        echo "🚀 可以开始测试了："
        echo ""
        echo "    ./test-ui.sh styles    # 开始测试样式系统"
        echo "    或"
        echo "    ./start-test.sh        # 使用交互式菜单"
        echo ""
    fi
    exit 0
else
    echo "❌ 发现 $errors 个错误，请检查后重试"
    exit 1
fi
