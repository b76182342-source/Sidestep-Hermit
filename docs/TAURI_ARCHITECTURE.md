# Tauri + React 架构说明

## 技术栈变更

**原计划**: GTK4 + libadwaita（原生 Linux UI）  
**新方案**: Tauri 2 + React + TypeScript（跨平台 Web UI）

### 为什么选择 Tauri

1. **现代化 UI**：使用 React 构建，支持丰富的 Web 生态（CSS、动画、组件库）
2. **跨平台**：Linux、Windows、macOS 统一代码库
3. **轻量级**：比 Electron 更小的体积和内存占用
4. **Rust 后端**：完全复用现有的 kg-* crates
5. **中文输入无忧**：Web 技术栈天然支持 IME

## 项目结构

```
kg-app/
├── src/                    # Rust 后端 (Tauri)
│   ├── main.rs            # Tauri 入口，状态管理
│   └── commands.rs        # Tauri 命令（前端调用的 API）
├── src-ui/                # React 前端
│   ├── src/
│   │   ├── main.tsx       # React 入口
│   │   ├── App.tsx        # 主应用组件
│   │   ├── components/    # UI 组件
│   │   │   ├── Player.tsx
│   │   │   ├── TrackList.tsx
│   │   │   └── Sidebar.tsx
│   │   ├── store/         # Zustand 状态管理
│   │   │   └── player.ts
│   │   ├── types.ts       # TypeScript 类型定义
│   │   └── styles.css     # 全局样式
│   ├── index.html
│   ├── package.json
│   └── vite.config.ts
├── tauri.conf.json        # Tauri 配置
├── build.rs
└── Cargo.toml
```

## 前后端通信

### Rust → React (事件)

```rust
// Rust 后端发送事件
app.emit("player-event", PlayerEventPayload {
    event_type: "StateChanged",
    state: Some("Playing"),
    ...
});
```

```typescript
// React 前端监听事件
listen<PlayerEvent>('player-event', (event) => {
  const { payload } = event
  // 处理事件
})
```

### React → Rust (命令)

```typescript
// React 调用 Rust 命令
import { invoke } from '@tauri-apps/api/core'

await invoke('play_track', { trackId: { provider: 'local', id: '/path/to/song.mp3' } })
```

```rust
// Rust 命令处理
#[tauri::command]
pub async fn play_track(state: State<'_, AppState>, track_id: TrackId) -> Result<(), String> {
    // 处理播放逻辑
}
```

## 运行方式

### 开发模式

```bash
cd kg-app/src-ui
npm install
npm run tauri dev
```

### 生产构建

```bash
npm run tauri build
```

生成的包：
- `target/release/bundle/deb/` - Debian 包
- `target/release/bundle/appimage/` - AppImage

## 系统依赖

仅需 GStreamer 运行时库（不再需要 GTK4 开发库）：

```bash
sudo apt install \
    gstreamer1.0-plugins-base \
    gstreamer1.0-plugins-good \
    gstreamer1.0-plugins-bad \
    gstreamer1.0-plugins-ugly \
    gstreamer1.0-libav
```

## 与原架构的对比

| 维度 | GTK4 方案 | Tauri 方案 |
|------|----------|-----------|
| UI 框架 | GTK4 + libadwaita | React + CSS |
| 跨平台 | 仅 Linux | Linux/Win/macOS |
| 打包大小 | ~15MB | ~8MB |
| 中文输入 | 依赖 fcitx5/ibus | 开箱即用 |
| 开发体验 | Rust UI 代码 | 熟悉的 Web 技术 |
| 系统集成 | 原生深浅色主题 | 需手动适配 |
| 构建依赖 | 需大量 -dev 包 | 仅需 GStreamer 运行时 |

## 已实现功能（M1）

- ✅ 本地音乐扫描（walkdir + lofty）
- ✅ 曲目列表显示
- ✅ 播放控制（播放/暂停/上一曲/下一曲）
- ✅ 进度条拖动
- ✅ 音量控制
- ✅ 实时状态同步（Rust → React）
- ✅ 播放队列基础逻辑
