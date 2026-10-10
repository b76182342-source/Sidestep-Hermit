# 快速开始

## 前置条件

- Ubuntu 24.04+ / Debian 12+（或其他支持 GStreamer 的 Linux 发行版）
- 互联网连接

## 一键安装（推荐）

```bash
# 运行安装脚本（会提示输入 sudo 密码）
./scripts/install-deps-ubuntu.sh
```

该脚本会安装：
- GStreamer 运行时和开发库
- WebKit2GTK（Tauri 依赖）
- Rust 工具链（如果未安装）
- Node.js 20（如果未安装）

## 手动安装

### 1. 系统依赖

```bash
sudo apt-get update
sudo apt-get install -y \
    gstreamer1.0-plugins-{base,good,bad,ugly} \
    gstreamer1.0-libav \
    libgstreamer1.0-dev \
    libgstreamer-plugins-base1.0-dev \
    libwebkit2gtk-4.1-dev \
    libsoup-3.0-dev \
    pkg-config
```

### 2. Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### 3. Node.js 20

```bash
curl -fsSL https://deb.nodesource.com/setup_20.x | sudo -E bash -
sudo apt-get install -y nodejs
```

## 运行开发版本

```bash
# 安装前端依赖
cd kg-app/src-ui
npm install

# 启动开发服务器（会自动编译 Rust 后端并打开窗口）
npm run tauri dev
```

首次运行会编译 Rust 依赖，需要 5-10 分钟。

## 构建生产版本

```bash
cd kg-app/src-ui
npm run tauri build
```

生成的安装包：
- `../../target/release/bundle/deb/sidestep-hermit_0.1.0_amd64.deb`

## 使用

1. 点击侧边栏"扫描音乐库"（默认扫描 `~/Music`）
2. 双击歌曲播放
3. 使用底部播放器控制栏

## 常见问题

### Q: 提示找不到 GStreamer

A: 确保安装了运行时库：
```bash
sudo apt install gstreamer1.0-plugins-{base,good,bad,ugly} gstreamer1.0-libav
```

### Q: Tauri dev 窗口打不开

A: 检查 WebKit 是否安装：
```bash
sudo apt install libwebkit2gtk-4.1-0
```

### Q: 编译报错 "package not found"

A: 确保安装了开发库：
```bash
sudo apt install libgstreamer1.0-dev libgstreamer-plugins-base1.0-dev libwebkit2gtk-4.1-dev pkg-config
```

### Q: 中文输入法无法使用

A: Tauri 使用 WebView，中文输入开箱即用，无需额外配置。

---

**更多文档**: 见 [docs/TAURI_ARCHITECTURE.md](docs/TAURI_ARCHITECTURE.md)
