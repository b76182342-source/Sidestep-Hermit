---
name: verify-build
description: 验证 Rust 项目编译，包括后端和前端构建
tags: [rust, build, verification]
---

# 编译验证流程

## 步骤 1: 检查后端核心库

```bash
cd /home/lilin/rust_music/my_music
cargo check --package kg-providers --features kugou
```

**关注输出**：
- `Finished` 表示成功
- 记录所有 `error[E****]` 错误码和错误位置

## 步骤 2: 检查应用主程序

```bash
cd /home/lilin/rust_music/my_music
cargo check --package kg-app --features kugou
```

**关注输出**：
- 依赖的所有 crate 是否都通过
- 特别注意 `kg-audio`, `kg-lyrics`, `kg-store` 等依赖

## 步骤 3: 完整构建测试（可选）

仅在重大改动后执行：

```bash
cd /home/lilin/rust_music/my_music/kg-app
cargo build --features kugou
```

## 常见错误模式

### E0599: 方法不存在
- **原因**: SDK API 签名与预期不符
- **解决**: 查看 SDK 源码 `~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/kugou_sdk-*/src/`
- **工具**: `grep -r "pub async fn" <SDK路径>/src/<模块>/`

### E0609: 字段不存在
- **原因**: 结构体字段名错误
- **解决**: 读取类型定义，查看 `pub struct` 的实际字段
- **示例**: `PlaylistTracks.items` 而不是 `.songs`

### E0061: 参数数量不匹配
- **原因**: 函数签名变更或误用
- **解决**: 查看函数定义 `pub fn/async fn` 的参数列表

### 编译器分类器超时
- **现象**: `claude-opus-5-5[1m] is temporarily unavailable (timed out)`
- **应对**: 切换到非阻塞工具（Read, Bash with grep）继续工作
- **后续**: 等待分类器恢复后再执行需要权限的命令

## SDK 源码快速定位

```bash
SDK_PATH="/home/lilin/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/kugou_sdk-0.2.9"

# 查找模块的公开方法
grep -E "pub async fn|pub fn" "$SDK_PATH/src/<模块>/api.rs" | head -20

# 查找结构体定义
grep -n "pub struct <类型名>" "$SDK_PATH/src/<模块>/types.rs"

# 查看结构体字段（从定义行往后读 50 行）
sed -n '<行号>,+50p' "$SDK_PATH/src/<模块>/types.rs"
```

## 增量验证原则

1. **改一处，验一处**: 修改后立即 `cargo check --package <受影响的包>`
2. **优先验证最小单元**: 先 `kg-providers`，再 `kg-app`
3. **错误收敛**: 同一类型错误修复一处后，重新编译看剩余错误
4. **保存编译输出**: 完整错误信息比摘要更有价值

## 时间预估

- `cargo check --package kg-providers`: ~3-5 秒
- `cargo check --package kg-app`: ~5-10 秒  
- `cargo build`: ~30-120 秒（首次更长）

## 注意事项

- 编译缓存依赖 `Cargo.lock` 和 `target/` 目录完整性
- 修改依赖版本后需 `cargo update` 或删除 `Cargo.lock`
- 交叉编译（Linux → macOS/Windows）需额外工具链设置
