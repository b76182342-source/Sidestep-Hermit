#!/usr/bin/env python3
"""生成 KGPlayer 应用图标。

配色取自界面设计稿：赤褐 #B85537 + 米白 #FAF9F5。
图形沿用设计稿里的音符母题（引导页那个音符标记）。

产出两样东西：
  1. assets/icon-source.png —— 1024×1024 源图
  2. kg-app/icons/<n>x<n>.png —— 规格统一的图标阶梯

阶梯由本脚本自己生成，而不是交给 `cargo tauri icon`：
后者产出的 128x128@2x.png 构图与其它尺寸不一致（音符偏大、圆角偏小），
装到 hicolor 里大小档位看起来会不一样。Tauri 只负责 .ico / .icns
等平台专有格式：

    python3 scripts/make_icon.py
    cargo tauri icon assets/icon-source.png -o kg-app/icons
    python3 scripts/make_icon.py --ladder-only   # 覆盖回统一规格
"""

from pathlib import Path

from PIL import Image, ImageDraw

SIZE = 1024
BG = (184, 85, 55, 255)  # #B85537
FG = (250, 249, 245, 255)  # #FAF9F5

# hicolor 主题认识的标准尺寸；32/64/128/256/512 会被打进 deb
LADDER = (16, 24, 32, 48, 64, 128, 256, 512)

CORNER = int(SIZE * 0.225)

# 音符头的中心与半径
HEAD_CX, HEAD_CY = 400, 672
HEAD_RX, HEAD_RY = 134, 101
HEAD_TILT = -18

# 符干
STEM_X, STEM_W = 506, 48
STEM_TOP, STEM_BOTTOM = 226, 672


def bezier(p0, p1, p2, steps=40):
    """二次贝塞尔取样，用来描符尾的弧线。"""
    pts = []
    for i in range(steps + 1):
        t = i / steps
        u = 1 - t
        x = u * u * p0[0] + 2 * u * t * p1[0] + t * t * p2[0]
        y = u * u * p0[1] + 2 * u * t * p1[1] + t * t * p2[1]
        pts.append((x, y))
    return pts


def draw_note() -> Image.Image:
    """在独立图层上画音符，方便整体缩放与居中。"""
    layer = Image.new("RGBA", (SIZE, SIZE), (0, 0, 0, 0))

    # 符头：先画正椭圆再旋转，避免手算斜椭圆
    head = Image.new("RGBA", (SIZE, SIZE), (0, 0, 0, 0))
    ImageDraw.Draw(head).ellipse(
        [HEAD_CX - HEAD_RX, HEAD_CY - HEAD_RY, HEAD_CX + HEAD_RX, HEAD_CY + HEAD_RY],
        fill=FG,
    )
    head = head.rotate(HEAD_TILT, resample=Image.BICUBIC, center=(HEAD_CX, HEAD_CY))
    layer = Image.alpha_composite(layer, head)

    draw = ImageDraw.Draw(layer)

    # 符干
    draw.rounded_rectangle(
        [STEM_X, STEM_TOP, STEM_X + STEM_W, STEM_BOTTOM],
        radius=STEM_W // 2,
        fill=FG,
    )

    # 符尾：外弧向右下扫，内弧收回来，合成一片月牙
    outer = bezier((STEM_X + STEM_W, STEM_TOP + 4), (726, 262), (734, 452))
    inner = bezier((734, 452), (648, 348), (STEM_X + STEM_W, 322))
    draw.polygon(outer + inner, fill=FG)

    return layer


def main() -> None:
    canvas = Image.new("RGBA", (SIZE, SIZE), (0, 0, 0, 0))
    ImageDraw.Draw(canvas).rounded_rectangle(
        [0, 0, SIZE - 1, SIZE - 1], radius=CORNER, fill=BG
    )

    note = draw_note()

    # 让音符占满画布的 NOTE_FILL 比例，并居中 —— 手调坐标容易偏心
    NOTE_FILL = 0.62
    box = note.getbbox()
    if box is None:
        raise SystemExit("音符图层为空")

    note_w, note_h = box[0 + 2] - box[0], box[3] - box[1]
    scale = (SIZE * NOTE_FILL) / max(note_w, note_h)
    scaled = note.resize(
        (max(1, round(note.width * scale)), max(1, round(note.height * scale))),
        Image.LANCZOS,
    )

    scaled_box = scaled.getbbox()
    if scaled_box is None:
        raise SystemExit("缩放后图层为空")

    offset = (
        round(SIZE / 2 - (scaled_box[0] + scaled_box[2]) / 2),
        round(SIZE / 2 - (scaled_box[1] + scaled_box[3]) / 2),
    )
    canvas.alpha_composite(scaled, offset)

    root = Path(__file__).resolve().parent.parent

    out_dir = root / "assets"
    out_dir.mkdir(parents=True, exist_ok=True)
    out = out_dir / "icon-source.png"
    canvas.save(out)
    print(f"wrote {out} ({SIZE}×{SIZE})")

    ladder_dir = root / "kg-app" / "icons"
    ladder_dir.mkdir(parents=True, exist_ok=True)
    for size in LADDER:
        target = ladder_dir / f"{size}x{size}.png"
        canvas.resize((size, size), Image.LANCZOS).save(target)
        print(f"wrote {target}")


if __name__ == "__main__":
    main()
