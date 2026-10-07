# KGPlayer

一个现代化的 Linux 桌面音乐播放器，支持本地音乐库和在线音源（酷狗音乐）。

基于 **Tauri 2 + React + Rust + GStreamer** 构建。

## 特性

- 🎵 本地音乐播放（MP3、FLAC、OGG、WAV、M4A 等）
- 🎤 歌词同步显示（LRC 格式）
- 🔊 10段均衡器
- 🎨 React + CSS 现代化界面
- 🎹 MPRIS2 媒体键支持（Linux）
- 🔍 全文搜索（FTS5）
- 🌐 酷狗音乐支持（需要会员账号）
- 🌍 跨平台（Linux、Windows、macOS）

## 架构

采用前后端分离架构：

- **前端**: React + TypeScript + Vite
- **后端**: Rust + Tauri 2
- **音频**: GStreamer
- **数据库**: SQLite + FTS5
- **状态管理**: Zustand

详见 [docs/TAURI_ARCHITECTURE.md](docs/TAURI_ARCHITECTURE.md)

## 系统要求

### 运行时依赖（所有平台）

**Linux (Ubuntu 24.04+ / Debian)**

```bash
sudo apt install \
    gstreamer1.0-plugins-base \
    gstreamer1.0-plugins-good \
    gstreamer1.0-plugins-bad \
    gstreamer1.0-plugins-ugly \
    gstreamer1.0-libav \
    libwebkit2gtk-4.1-0
```

**Windows**: 下载安装 [GStreamer Runtime](https://gstreamer.freedesktop.org/download/)

**macOS**: `brew install gstreamer gst-plugins-{base,good,bad,ugly}`

### 开发依赖

```bash
# Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Node.js (推荐使用 nvm)
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.0/install.sh | bash
nvm install 20

# Tauri CLI
cargo install tauri-cli --version "^2.0.0"
```

## 开发

```bash
# 克隆仓库
git clone https://github.com/yourusername/kgplayer.git
cd kgplayer

# 安装前端依赖
cd kg-app/src-ui
npm install

# 启动开发服务器
npm run tauri dev
```

## 构建

```bash
cd kg-app/src-ui

# 构建生产版本
npm run tauri build

# 输出位置
# Linux: target/release/bundle/deb/ 和 target/release/bundle/appimage/
# Windows: target/release/bundle/msi/
# macOS: target/release/bundle/dmg/
```

## 使用

### 本地音乐

1. 点击侧边栏"扫描音乐库"
2. 默认扫描 `~/Music` 目录
3. 双击歌曲播放

### 酷狗音乐（可选）

启用 `kugou` feature 后：

```bash
cargo build --release --features kugou
```

首次使用需要扫码登录：

1. 打开应用，点击"登录酷狗"
2. 使用手机酷狗 App 的"扫一扫"功能
3. 在手机上确认登录
4. 凭据安全存储在系统密钥环中

**注意**: 使用酷狗音乐需要有效的会员账号。本应用不绕过任何付费限制。

## Cargo Workspace 结构

- **kg-core**: 领域模型与 trait 定义
- **kg-audio**: GStreamer 播放器封装
- **kg-providers**: 音源实现（本地、酷狗）
- **kg-lyrics**: LRC 歌词解析与同步
- **kg-store**: SQLite 数据库与持久化
- **kg-mpris**: MPRIS2 D-Bus 服务（Linux）
- **kg-app**: Tauri 主应用（前端 + 后端）

## 开发进度

- [x] M0: 基础架构搭建
- [x] M1: 本地音乐播放 + React UI
- [ ] M2: 歌词同步显示
- [ ] M3: 播放列表与搜索
- [ ] M4: 酷狗音乐集成
- [ ] M5: 打包与发布

## 许可证

GPL-3.0-or-later

本项目依赖 GStreamer 的某些插件采用 GPL 许可证，因此整个应用也必须使用 GPL。

## 贡献

欢迎贡献！请确保：

- Rust 代码通过 `cargo fmt` 和 `cargo clippy`
- React 代码通过 `npm run lint`
- 添加必要的测试

## 致谢

- [Tauri](https://tauri.app/) - 跨平台桌面应用框架
- [React](https://react.dev/) - UI 框架
- [GStreamer](https://gstreamer.freedesktop.org/) - 多媒体框架
- 社区提供的酷狗 API 文档
