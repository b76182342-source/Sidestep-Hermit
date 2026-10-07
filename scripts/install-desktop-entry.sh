#!/usr/bin/env bash
#
# 为开发态安装桌面项与图标。
#
# 打包成 .deb / AppImage 时 Tauri bundler 会自动带上这些；但 `cargo tauri dev`
# 跑的是裸二进制，窗口的 app_id 为 `kgplayer`，桌面环境找不到对应的 .desktop，
# 任务栏就只能回退成通用齿轮图标。
#
# 用法：./scripts/install-desktop-entry.sh [可执行文件路径]

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
APP_ID="kgplayer"
BIN="${1:-$ROOT/target/debug/kgplayer}"

SOURCE_PNG="$ROOT/assets/icon-source.png"
[[ -f "$SOURCE_PNG" ]] || { echo "缺少 $SOURCE_PNG，请先运行 scripts/make_icon.py" >&2; exit 1; }

APPS_DIR="$HOME/.local/share/applications"
ICON_ROOT="$HOME/.local/share/icons/hicolor"
mkdir -p "$APPS_DIR"

# 按 hicolor 主题尺寸生成图标
python3 - "$SOURCE_PNG" "$ICON_ROOT" "$APP_ID" <<'PY'
import sys
from pathlib import Path
from PIL import Image

src, root, app_id = Path(sys.argv[1]), Path(sys.argv[2]), sys.argv[3]
img = Image.open(src).convert("RGBA")

for size in (16, 24, 32, 48, 64, 128, 256, 512):
    out_dir = root / f"{size}x{size}" / "apps"
    out_dir.mkdir(parents=True, exist_ok=True)
    img.resize((size, size), Image.LANCZOS).save(out_dir / f"{app_id}.png")
    print(f"  icon {size}x{size} -> {out_dir / (app_id + '.png')}")
PY

# 桌面项
cat > "$APPS_DIR/$APP_ID.desktop" <<EOF
[Desktop Entry]
Type=Application
Version=1.0
Name=KGPlayer
Comment=本地与酷狗在线音乐播放器
Exec=$BIN
Icon=$APP_ID
Terminal=false
Categories=AudioVideo;Audio;Player;
Keywords=music;audio;player;kugou;
StartupWMClass=$APP_ID
StartupNotify=true
EOF

chmod +x "$APPS_DIR/$APP_ID.desktop"

# 刷新桌面数据库与图标缓存（命令缺失时忽略）
command -v update-desktop-database >/dev/null && update-desktop-database "$APPS_DIR" >/dev/null 2>&1 || true
command -v gtk-update-icon-cache >/dev/null && gtk-update-icon-cache -f -t "$ICON_ROOT" >/dev/null 2>&1 || true

echo
echo "已安装："
echo "  桌面项  $APPS_DIR/$APP_ID.desktop"
echo "  图标    $ICON_ROOT/*/apps/$APP_ID.png"
echo "  可执行  $BIN"
echo
echo "若任务栏仍显示旧图标，注销重登或重启桌面 Shell 后即可生效。"
