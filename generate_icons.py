#!/usr/bin/env python3
"""
KGPlayer 图标生成器
设计理念：音乐播放器 + 现代化设计
- 主色：Spotify 绿色 (#1DB954)
- 图标：音符 + 播放按钮组合
"""

from PIL import Image, ImageDraw
import os

def create_music_icon(size, bg_color='#1e1e1e', fg_color='#1DB954'):
    """创建音乐播放器图标"""
    img = Image.new('RGBA', (size, size), (0, 0, 0, 0))  # 透明背景
    draw = ImageDraw.Draw(img)

    # 计算缩放比例
    scale = size / 128

    # 绘制圆角矩形背景
    margin = int(8 * scale)
    # 转换颜色
    bg_rgb = tuple(int(bg_color.lstrip('#')[i:i+2], 16) for i in (0, 2, 4))
    fg_rgb = tuple(int(fg_color.lstrip('#')[i:i+2], 16) for i in (0, 2, 4))

    draw.rounded_rectangle(
        [(margin, margin), (size - margin, size - margin)],
        radius=int(16 * scale),
        fill=fg_rgb + (255,)  # 添加 alpha 通道
    )

    # 绘制播放按钮（三角形）
    center_x = size // 2
    center_y = size // 2
    triangle_size = int(32 * scale)

    triangle = [
        (center_x - triangle_size//2, center_y - triangle_size//2),
        (center_x - triangle_size//2, center_y + triangle_size//2),
        (center_x + triangle_size//2, center_y)
    ]
    draw.polygon(triangle, fill=bg_rgb + (255,))

    # 绘制音符装饰
    note_x = int(center_x + 20 * scale)
    note_y = int(center_y - 25 * scale)
    note_size = int(8 * scale)

    # 音符圆点
    draw.ellipse(
        [(note_x, note_y), (note_x + note_size, note_y + note_size)],
        fill=bg_rgb + (255,)
    )
    # 音符竖线
    draw.line(
        [(note_x + note_size, note_y), (note_x + note_size, note_y - int(20 * scale))],
        fill=bg_rgb + (255,),
        width=int(2 * scale)
    )

    return img

def main():
    # 创建图标目录
    icon_dir = 'kg-app/icons'
    os.makedirs(icon_dir, exist_ok=True)
    os.chdir(icon_dir)

    print("🎨 正在生成 KGPlayer 图标...")

    # 生成不同尺寸的图标
    sizes = {
        '32x32.png': 32,
        '128x128.png': 128,
        '128x128@2x.png': 256,
        'icon.png': 512
    }

    for filename, size in sizes.items():
        img = create_music_icon(size)
        img.save(filename)
        print(f"✅ 已生成 {filename} ({size}x{size})")

    # Windows ICO（包含多个尺寸）
    icon_sizes = [(16, 16), (32, 32), (48, 48), (64, 64), (128, 128), (256, 256)]
    icons = [create_music_icon(size[0]) for size in icon_sizes]
    icons[0].save('icon.ico', format='ICO', sizes=icon_sizes)
    print(f"✅ 已生成 icon.ico (多尺寸)")

    # macOS ICNS（简化版，使用 512x512）
    img_512 = create_music_icon(512)
    img_512.save('icon.icns', format='PNG')  # 简化版
    print(f"✅ 已生成 icon.icns")

    print("\n🎉 所有图标已生成完成！")
    print("\n图标设计：")
    print("- 主色：Spotify 绿 (#1DB954)")
    print("- 背景：深色 (#1e1e1e)")
    print("- 元素：播放按钮 + 音符")
    print("- 风格：现代化、圆角设计")

if __name__ == '__main__':
    main()
