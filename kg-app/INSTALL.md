# KGPlayer 安装指南

## 安装

### Ubuntu / Debian（推荐）

```bash
sudo dpkg -i kgplayer_0.1.0_amd64.deb
sudo apt-get install -f   # 补齐缺失依赖
```

安装后会写入：

| 路径 | 内容 |
|---|---|
| `/usr/bin/kgplayer` | 可执行文件 |
| `/usr/share/applications/kgplayer.desktop` | 桌面项（出现在「显示应用程序」） |
| `/usr/share/icons/hicolor/*/apps/kgplayer.png` | 图标 |

卸载：`sudo apt remove kgplayer`

### 所有 Linux 发行版（AppImage，免安装）

```bash
chmod +x kgplayer_0.1.0_amd64.AppImage
./kgplayer_0.1.0_amd64.AppImage
```

### 开发态

裸二进制不会自动出现在应用列表里，需要手动装一次桌面项与图标：

```bash
./scripts/install-desktop-entry.sh
```

## 系统要求

- Linux x86_64
- GStreamer 1.0（含 `plugins-good` 的 `souphttpsrc`，在线播放需要）
- WebKitGTK 4.1
- 512 MB RAM
- 100 MB 存储空间

## 首次使用

1. 启动 KGPlayer
2. 本地音乐：把文件放进音乐目录后点「扫描音乐库」
3. 在线音乐：点「登录」扫码登录酷狗，切到「酷狗」标签即可搜索与查看每日推荐

凭据只写入系统密钥环，不上传、不写日志。本应用为非官方客户端，与酷狗无隶属关系。

## 从源码构建

```bash
cargo tauri build --bundles deb,appimage
```

产物在 `target/release/bundle/`。

开发模式：

```bash
cargo tauri dev --features kugou
```

**`--features kugou` 不能漏**：开发者模式下 `default` feature 不生效，
漏掉会得到一个没有在线功能的构建，调用时才报「Kugou feature not enabled」。
详见根目录 `CLAUDE.md` 的「构建与运行」。
