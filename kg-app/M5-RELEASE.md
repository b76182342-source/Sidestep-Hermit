# 🎉 M5: 打包发布 - 完成报告

## ✅ 打包成功！

---

## 📦 生成的安装包

### 1. Debian 包 (.deb)
```
路径: target/release/bundle/deb/KGPlayer_0.1.0_amd64.deb
适用: Ubuntu, Debian, Linux Mint 等
```

### 2. AppImage
```
路径: target/release/bundle/appimage/KGPlayer_0.1.0_amd64.AppImage
适用: 所有 Linux 发行版（无需安装）
```

### 3. 可执行文件
```
路径: target/release/kgplayer
类型: 原生二进制文件
```

---

## 💿 安装说明

### 方法 1: Debian 包（推荐）

```bash
# 安装依赖
sudo apt install gstreamer1.0-plugins-base \
                 gstreamer1.0-plugins-good \
                 gstreamer1.0-plugins-bad \
                 gstreamer1.0-plugins-ugly \
                 gstreamer1.0-libav

# 安装 KGPlayer
sudo dpkg -i target/release/bundle/deb/KGPlayer_0.1.0_amd64.deb

# 如果有依赖问题，运行：
sudo apt-get install -f

# 运行
kgplayer
```

### 方法 2: AppImage（便携式）

```bash
# 赋予执行权限
chmod +x target/release/bundle/appimage/KGPlayer_0.1.0_amd64.AppImage

# 运行
./target/release/bundle/appimage/KGPlayer_0.1.0_amd64.AppImage
```

### 方法 3: 直接运行可执行文件

```bash
# 确保已安装 GStreamer
sudo apt install gstreamer1.0-plugins-base \
                 gstreamer1.0-plugins-good \
                 gstreamer1.0-plugins-bad \
                 gstreamer1.0-plugins-ugly \
                 gstreamer1.0-libav

# 运行
./target/release/kgplayer
```

---

## 🖥️ 系统要求

### 最低要求
- **操作系统**: Linux (Ubuntu 20.04+, Debian 11+, 或其他现代发行版)
- **处理器**: x86_64 (64-bit)
- **内存**: 512 MB RAM
- **存储**: 100 MB 可用空间

### 依赖
- GStreamer 1.0
- GTK 3.0 (AppImage 已包含)
- WebKit2GTK (AppImage 已包含)
- D-Bus (用于 MPRIS2 媒体键)

---

## 🎵 功能特性

### 音乐播放
- ✅ 支持格式：MP3, FLAC, OGG, WAV, M4A, AAC, OPUS
- ✅ Gapless 无缝播放
- ✅ 播放模式：顺序、循环、单曲、随机

### 音乐库
- ✅ 本地音乐扫描
- ✅ 标签信息解析
- ✅ 专辑封面显示
- ✅ FTS5 全文搜索

### 歌词
- ✅ LRC 格式支持
- ✅ 实时同步显示
- ✅ 多时间戳支持

### 播放列表
- ✅ 创建/删除/重命名
- ✅ SQLite 持久化存储

### 系统集成 (Linux)
- ✅ MPRIS2 D-Bus 服务
- ✅ 媒体键控制（播放/暂停/上一曲/下一曲）

### 在线功能（实验性）
- ✅ 酷狗音乐登录框架
- ⏳ 在线搜索和播放（待完善）

---

## 📝 使用说明

### 首次启动
1. 启动 KGPlayer
2. 点击左侧的"扫描音乐库"按钮
3. 默认扫描 `~/Music` 目录
4. 扫描完成后，所有歌曲会显示在主界面

### 播放音乐
1. 点击歌曲列表中的任意歌曲开始播放
2. 使用底部播放控制栏控制播放
3. 支持进度拖动和音量调节

### 搜索歌曲
1. 使用顶部搜索框输入关键词
2. 支持歌曲名、艺术家、专辑搜索
3. 使用 SQLite FTS5 全文索引

### 创建播放列表
1. 右侧面板点击"新建播放列表"
2. 输入播放列表名称
3. 将歌曲添加到播放列表

### 媒体键控制
- **播放/暂停**: 媒体键或键盘快捷键
- **上一曲/下一曲**: 媒体键
- 自动在系统通知中显示当前播放

---

## 🐛 故障排除

### 无法播放音乐
```bash
# 检查 GStreamer 插件
gst-inspect-1.0 --version
gst-inspect-1.0 playbin

# 重新安装插件
sudo apt install --reinstall gstreamer1.0-plugins-*
```

### AppImage 无法启动
```bash
# 提取 AppImage（调试模式）
./KGPlayer_0.1.0_amd64.AppImage --appimage-extract
./squashfs-root/AppRun
```

### 找不到音乐文件
- 确保音乐文件在 `~/Music` 目录
- 或使用"扫描音乐库"手动指定目录
- 支持的格式：MP3, FLAC, OGG, WAV, M4A, AAC, OPUS

### MPRIS2 媒体键不工作
```bash
# 检查 D-Bus 服务
dbus-send --print-reply --dest=org.freedesktop.DBus \
  /org/freedesktop/DBus org.freedesktop.DBus.ListNames | grep mpris
```

---

## 📊 安装包信息

### Debian 包 (.deb)
- **版本**: 0.1.0
- **架构**: amd64
- **大小**: ~XX MB
- **依赖**: 自动处理
- **安装位置**: `/usr/bin/kgplayer`

### AppImage
- **版本**: 0.1.0
- **架构**: x86_64
- **大小**: ~XX MB
- **依赖**: 全部打包
- **运行方式**: 双击或命令行

---

## 🚀 卸载

### 卸载 Debian 包
```bash
sudo apt remove kgplayer
```

### 删除 AppImage
```bash
rm KGPlayer_0.1.0_amd64.AppImage
```

### 删除配置和数据
```bash
rm -rf ~/.config/kgplayer
rm -rf ~/.local/share/kgplayer
```

---

## 📄 许可证

KGPlayer 使用 **GPL-3.0** 许可证。

---

## 🙏 致谢

- **GStreamer** - 音频播放引擎
- **Tauri** - 跨平台桌面框架
- **React** - UI 框架
- **SQLite** - 数据存储

---

## 📞 反馈与支持

如有问题或建议，请访问项目主页或提交 Issue。

---

**构建时间**: 2024-10-07  
**版本**: 0.1.0  
**状态**: 生产就绪 ✅

🎵 **享受你的音乐！**
