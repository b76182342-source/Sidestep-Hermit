#!/bin/bash
# 创建简单的占位图标

cd kg-app/icons

# 创建一个简单的 PNG 文件头（1x1 蓝色像素）
printf '\x89PNG\r\n\x1a\n\x00\x00\x00\rIHDR\x00\x00\x00\x01\x00\x00\x00\x01\x08\x02\x00\x00\x00\x90wS\xde\x00\x00\x00\x0cIDATx\x9cc\x00\x01\x00\x00\x05\x00\x01\r\n-\xb4\x00\x00\x00\x00IEND\xaeB`\x82' > 32x32.png
cp 32x32.png 128x128.png
cp 32x32.png 128x128@2x.png
cp 32x32.png icon.ico
cp 32x32.png icon.icns

echo "✅ 图标文件已创建"
ls -lh
