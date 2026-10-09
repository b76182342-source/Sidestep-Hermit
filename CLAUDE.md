# 合规红线（必须遵守）

1. **不得绕过付费、VIP 或试听限制**；不得破解、解密任何 DRM 或厂商私有加密音频格式。

2. **不得编造酷狗的 API 端点、参数或签名算法**，也不得实现对私有协议的逆向代码。酷狗 Provider 只能基于官方文档或授权的开放接口。

3. **用户凭据只存系统密钥环**（Secret Service，使用 oo7 crate）；日志中不得出现 token、cookie、密码。

4. **只播放用户有权播放的内容**。

# Git 工作流

**重要：本项目只使用主分支（main），不创建任何其他分支。**

所有开发、修复和功能更新都直接提交到 main 分支：

```bash
# 提交更改
git add .
git commit -m "feat: 描述更改内容"
git push origin main
```

不要使用 `git checkout -b` 创建新分支，也不要使用 worktree。所有工作都在主分支上进行。

# 打包与系统集成

## 构建发布包

```bash
cd kg-app
cargo tauri build --features kugou
```

生成的安装包位置：
- **Debian/Ubuntu**: `target/release/bundle/deb/Sidestep Hermit_VERSION_amd64.deb`
- **AppImage**: `target/release/bundle/appimage/` (如果启用)

## 系统集成

安装 `.deb` 包后，应用会自动注册到 Linux 系统：

1. **应用菜单** - 可从应用启动器搜索 "Sidestep Hermit"
2. **桌面文件** - 安装到 `/usr/share/applications/`
3. **可执行文件** - 安装到 `/usr/bin/` 或 `/opt/`
4. **图标** - 安装到 `/usr/share/icons/`

### 验证安装

```bash
# 检查是否已安装
dpkg -l | grep -i hermit

# 查找可执行文件
which kgplayer

# 从终端启动
kgplayer  # 或 sidestep-hermit
```

### 卸载

```bash
sudo dpkg -r sidestep-hermit
# 或
sudo apt remove sidestep-hermit
```

Tauri 的 bundler 会自动创建 `.desktop` 文件并注册到系统，无需手动配置。

# 架构决策

## 技术栈

- 语言：Rust stable edition 2021
- UI：Tauri 2 + React 18 + TypeScript（前端在 `kg-app/src-ui/`）
  - 原计划的 gtk4-rs + libadwaita 已放弃：中文输入法（fcitx5/ibus）在
    GTK4 下问题较多，且 Web 技术栈可同时覆盖 Linux/Windows/macOS。
- 音频：gstreamer-rs (gstreamer-player / playbin3)
- 异步：tokio + reqwest (rustls)
  - 注意：Tauri 的 `setup()` 运行在主线程且**不在** Tokio runtime 上下文中，
    后台任务必须从 `kg-app/src/main.rs` 的全局 `RUNTIME` 派生，不能用裸 `tokio::spawn`。
- 数据库：rusqlite + FTS5
- 打包：Tauri bundler（.deb + AppImage）
- i18n：尚未落地（原计划的 gettext-rs 未接入）

## Cargo Workspace 结构

7 个 crate：kg-core、kg-audio、kg-providers、kg-lyrics、kg-store、kg-mpris、kg-app

## MusicProvider Trait

抽象音源接口，支持本地文件和在线服务。`stream_source()` 只返回合法授权的播放来源。

## 非功能约束

- UI 线程不阻塞：扫描/网络/解码在后台线程
- 不使用 unwrap/expect（测试除外）
- 冷启动 < 1s，空闲 CPU < 1%
- 1 万首曲库内存 < 200MB

# 开发指引

## 代码风格

- 遵守 rustfmt.toml 和 clippy 配置
- 业务代码不得 panic
- unsafe 须注释说明必要性

## 测试要求

- LRC 解析
- 队列逻辑（随机/循环/插队）
- Provider mock 实现

## CI 流程

GitHub Actions 检查：fmt、clippy、test、deny

## 里程碑

按顺序执行，每个里程碑独立可运行：

- M0: 骨架（workspace、CI、README、LICENSE）✅
- M1: 本地播放 + Tauri/React UI ✅
- M2: 歌词与 MPRIS2 ✅
- M3: 播放列表与搜索 ✅
- M4: 酷狗 Provider（登录框架已就绪，搜索/播放待接入）🟡
- M5: 打包（.deb + AppImage）✅
- M6: 歌词滚动优化 ✅

## UI 组件设计

### 歌词滚动实现

项目使用标准的 `scrollIntoView()` API 实现歌词滚动同步，参考业界成熟方案：

**核心实现**（`NowPlayingPanel.tsx` 和 `NowPlaying.tsx`）：

```typescript
const times = useMemo(() => lyrics.map((l) => l.timeMs), [lyrics])
const active = activeLyricIndex(times, position)
const activeLineRef = useRef<HTMLDivElement>(null)

// 使用浏览器原生 API 实现平滑滚动
useEffect(() => {
  if (activeLineRef.current) {
    activeLineRef.current.scrollIntoView({
      behavior: 'smooth',
      block: 'center',      // 当前歌词居中显示
      inline: 'nearest'     // 使用最近的可滚动容器
    })
  }
}, [active])
```

**设计原则**：
- 统一状态驱动：`active` 索引变化时立即触发滚动
- 浏览器原生优化：滚动计算由浏览器引擎处理，无需手动计算 offsetTop
- 居中对齐：当前播放的歌词始终显示在视口中心
- 避免"两个滑块错开"问题：高亮切换和视觉滚动完全同步

**参考资料**：
- [Sync Audio with Text in React](https://www.cyishere.dev/blog/sync-audio-with-text-in-react)
- [BlurLyric - 网易云第三方播放器](https://github.com/sider2vf/BlurLyric)

### 页面结构

1. **右侧面板** (`src/components/NowPlayingPanel.tsx`)
   - 上方：专辑封面（有封面时显示封面，无封面时显示吉祥物）+ 歌曲信息
   - 下方：可滚动的歌词列表，当前播放行高亮显示

2. **全屏歌词页面** (`src/pages/NowPlaying/NowPlaying.tsx`)
   - 左侧：专辑封面 + 播放控制
   - 右侧：全屏歌词显示，支持逐字高亮
   - 使用相同的 `scrollIntoView()` 滚动逻辑

## 已知技术债

- LRC 内嵌歌词（ID3 USLT/SYLT）未实现，lofty 0.22 不直接支持。
- 酷狗 Provider 的 `search` / `stream_source` 仍是占位实现。
- 内存占用约 240 MB RSS（空曲库），超出 200 MB 目标；
  主要是 WebKit 的固定开销，需用 1 万首曲库实测才能判定是否真超标。
- `Queue` 的随机播放不记播放历史（`fastrand` 直接随机，可能短期内重复）。

## 构建与运行

```bash
# 开发：必须带 --features kugou
cargo tauri dev --features kugou

# 发布：不用带任何 feature
cargo tauri build --bundles deb,appimage
```

**开发命令为什么要显式带 `--features kugou`**：`cargo tauri dev` 会传
`--no-default-features`，`default` 里的 kugou 因此不生效；而 `custom-protocol`
（开发者不启用）蕴含的 kugou 也不会生效。漏掉就得到一个没有在线功能的
dev 构建，界面上只会在调用时冒出一句「Kugou feature not enabled」。

发布构建反过来：`custom-protocol` 必开且蕴含 `kugou`，所以不需要额外参数。

开发构建的窗口标题会显示 `Sidestep Hermit (dev · 需要 Vite)`，发布构建才是干净的
`Sidestep Hermit`；两者界面差异极大（dev 需要 Vite 在跑，否则白屏报错），
认标题可以少走弯路。

## 坑：`tauri` 依赖不能走 workspace 继承

`tauri` 必须在 `kg-app/Cargo.toml` 里**直接声明**，不能写
`tauri.workspace = true`。原因：`cargo tauri build` 需要往 `tauri` 的
feature 集合里补 `custom-protocol`，而 workspace 继承的依赖它改不了，
只会在日志里提示一句

```
Info `tauri` dependency has workspace inheritance enabled.
     The features array won't be automatically rewritten.
```

后果很隐蔽：构建照样成功，但 `custom-protocol` 没启用，打出来的
release 包仍然指向 `devUrl`。装到系统里打开就是
`Could not connect to localhost: Connection refused`。

排查手法（比看界面可靠）：

```bash
# 在 5173 起监听，然后启动应用；若它来连，说明还在用 devUrl
python3 -c "
import socket
s=socket.socket(); s.setsockopt(socket.SOL_SOCKET,socket.SO_REUSEADDR,1)
s.bind(('127.0.0.1',5173)); s.listen(5); s.settimeout(12)
try: s.accept(); print('仍依赖 devUrl')
except socket.timeout: print('使用内嵌资源')
" &
target/release/kgplayer
```

## CI 检查

以下三条必须全绿（`--all-features` 也要过）：

```
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
```

测试代码里的 `unwrap` / `expect` 通过测试模块内的
`#![allow(clippy::unwrap_used, clippy::expect_used)]` 豁免。
