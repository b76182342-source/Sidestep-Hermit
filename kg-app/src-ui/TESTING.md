# UI 模块测试指南

本指南说明如何单独测试各个 UI 模块，而不需要编译整个应用。

## 快速开始

使用测试脚本启动特定模块：

```bash
./test-ui.sh <模块名>
```

## 可用测试模块

### 1. 样式系统测试
测试所有基础样式、主题变量和组件样式。

```bash
./test-ui.sh styles
```

**测试内容：**
- 颜色系统（深色/浅色主题）
- 按钮样式
- 专辑卡片
- 歌曲列表
- 输入框
- 空状态
- 加载动画

### 2. 侧边栏组件测试
测试侧边栏导航功能。

```bash
./test-ui.sh sidebar
```

**测试内容：**
- 导航项点击
- 激活状态显示
- Hover 效果
- 酷狗登录按钮回调

### 3. 播放器控制栏测试
测试播放器控制功能。

```bash
./test-ui.sh player
```

**测试内容：**
- 播放/暂停控制
- 进度条拖动
- 音量控制
- 播放状态显示
- 响应式布局

### 4. 歌曲列表组件测试
测试歌曲列表显示和交互。

```bash
./test-ui.sh tracklist
```

**测试内容：**
- 歌曲列表显示
- 点击选中
- Hover 效果
- 文本溢出处理
- 滚动效果

### 5. 完整应用测试
运行完整的应用。

```bash
./test-ui.sh full
```

## 手动运行（不使用脚本）

如果不想使用脚本，可以直接进入前端目录运行：

```bash
cd kg-app/src-ui

# 测试样式系统
VITE_TEST_MODE=styles npm run dev

# 测试侧边栏
VITE_TEST_MODE=sidebar npm run dev

# 测试播放器
VITE_TEST_MODE=player npm run dev

# 测试歌曲列表
VITE_TEST_MODE=tracklist npm run dev

# 运行完整应用
npm run dev
```

## 测试流程建议

按以下顺序进行测试，确保每个模块正常后再进行下一步：

1. **Phase 1: 样式系统** ✅
   ```bash
   ./test-ui.sh styles
   ```
   - 检查主题切换是否正常
   - 检查所有组件样式是否显示正确
   - 确认深色/浅色主题都正常

2. **Phase 2: 基础组件**
   
   a. 测试侧边栏
   ```bash
   ./test-ui.sh sidebar
   ```
   
   b. 测试歌曲列表
   ```bash
   ./test-ui.sh tracklist
   ```
   
   c. 测试播放器
   ```bash
   ./test-ui.sh player
   ```

3. **Phase 3: 完整应用**
   ```bash
   ./test-ui.sh full
   ```
   - 测试所有功能集成
   - 测试页面导航
   - 测试数据流

## 访问地址

所有测试模块默认运行在：
```
http://localhost:5173
```

## 常见问题

### 端口被占用
如果 5173 端口被占用，Vite 会自动选择下一个可用端口（5174, 5175...）

### 依赖未安装
首次运行前需要安装依赖：
```bash
cd kg-app/src-ui
npm install
```

### 热更新不生效
重启开发服务器：
```bash
Ctrl+C  # 停止服务器
./test-ui.sh <模块名>  # 重新启动
```

## 下一步

当所有模块测试通过后，才进行完整应用的编译和 Tauri 集成：

```bash
cd kg-app/src-ui
npm run build

cd ..
cargo build
```

## 开发建议

- 修改组件代码后，测试页面会自动热更新
- 每次修改后都应该在对应的测试页面验证
- 发现问题立即修复，不要堆积问题
- 确保每个组件独立测试通过后再集成
