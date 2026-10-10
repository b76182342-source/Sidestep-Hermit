#!/bin/bash
# 重命名构建产物，去除文件名中的空格

set -e

BUNDLE_DIR="target/release/bundle"

# 重命名 deb 文件
shopt -s nullglob
for file in "$BUNDLE_DIR/deb/Sidestep Hermit_"*".deb"; do
    new_name=$(echo "$file" | sed 's/Sidestep Hermit_/sidestep-hermit_/')
    mv "$file" "$new_name"
    echo "✓ 重命名: $(basename "$file") -> $(basename "$new_name")"
done

# 重命名 AppImage 文件
for file in "$BUNDLE_DIR/appimage/Sidestep Hermit_"*".AppImage"; do
    new_name=$(echo "$file" | sed 's/Sidestep Hermit_/sidestep-hermit_/')
    mv "$file" "$new_name"
    echo "✓ 重命名: $(basename "$file") -> $(basename "$new_name")"
done

echo "✓ 构建产物重命名完成"
