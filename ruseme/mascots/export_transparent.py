#!/usr/bin/env python3
"""
为应用内使用生成透明背景的吉祥物图片
从现有的 -32.png 文件读取像素数据，导出为透明背景的 1024x1024 PNG
"""
import sys
import os
from PIL import Image

def export_transparent(input_32px, output_path, size=1024):
    """
    读取 32x32 的像素图，转换为透明背景的大尺寸图片

    Args:
        input_32px: 输入的 32x32 PNG 文件路径
        output_path: 输出文件路径
        size: 输出尺寸（默认 1024x1024）
    """
    # 读取 32x32 原图
    img_32 = Image.open(input_32px).convert('RGBA')

    # 创建透明背景的新图片
    transparent = Image.new('RGBA', (32, 32), (0, 0, 0, 0))

    # 遍历每个像素
    for y in range(32):
        for x in range(32):
            pixel = img_32.getpixel((x, y))
            # 如果像素是底色 #F5EBE1 (245, 235, 225)，设为透明
            # 否则保留原色
            if isinstance(pixel, tuple) and len(pixel) >= 3:
                r, g, b = pixel[:3]
                if r == 245 and g == 235 and b == 225:
                    # 底色，设为完全透明
                    transparent.putpixel((x, y), (0, 0, 0, 0))
                else:
                    # 保留原色，完全不透明
                    transparent.putpixel((x, y), (r, g, b, 255))

    # 放大到目标尺寸，使用最近邻插值保持像素风格
    large = transparent.resize((size, size), Image.NEAREST)
    large.save(output_path)
    print(f"✓ 导出透明背景: {output_path}")

if __name__ == '__main__':
    # 处理所有需要的吉祥物
    mascots_dir = '/home/lilin/rust_music/my_music/ruseme/mascots'
    output_dir = '/home/lilin/rust_music/my_music/kg-app/src-ui/public/mascots'

    # 映射：源文件 -> 目标文件
    mappings = [
        ('sleepy-v2/sleepy-v2-32.png', 'mascot-mustache.png'),
        ('lyric-v2/lyric-v2-32.png', 'mascot-typing.png'),
        ('dj-v2/dj-v2-32.png', 'mascot-cool-v2.png'),
    ]

    for src, dst in mappings:
        input_path = os.path.join(mascots_dir, src)
        output_path = os.path.join(output_dir, dst)

        if os.path.exists(input_path):
            export_transparent(input_path, output_path, size=1024)
        else:
            print(f"✗ 文件不存在: {input_path}")
