#!/usr/bin/env python3
from PIL import Image
import os

# 源图片
source = "/home/lilin/rust_music/my_music/mascot-typing.png"
icon_dir = "/home/lilin/rust_music/my_music/kg-app/icons"

# 需要生成的尺寸
sizes = [16, 24, 32, 48, 64, 128, 256, 512]

# 打开源图片
img = Image.open(source)

# 生成各种尺寸的图标
for size in sizes:
    resized = img.resize((size, size), Image.NEAREST)  # 使用 NEAREST 保持像素风格
    output_path = os.path.join(icon_dir, f"{size}x{size}.png")
    resized.save(output_path)
    print(f"Generated {output_path}")

# 生成 icon.png (通常是最大尺寸)
img.resize((1024, 1024), Image.NEAREST).save(os.path.join(icon_dir, "icon.png"))
print("Generated icon.png")

print("All icons generated successfully!")
