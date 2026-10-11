# CPU 占用测量方法

## 工具要求

使用 `pidstat`（来自 sysstat 包）或 `top -b` 进行实时采样测量。

```bash
# Ubuntu/Debian 安装
sudo apt install sysstat
```

## 测量方法

### 方法 1：使用 pidstat（推荐）

```bash
# 1. 启动应用
sidestep-hermit &

# 2. 获取主进程 PID
MAIN_PID=$(pgrep -x sidestep-hermit)

# 3. 获取所有相关进程（包括 WebKit 子进程）
PIDS=$(pgrep -P $MAIN_PID | tr '\n' ',' | sed 's/,$//')$MAIN_PID

# 4. 测量 10 分钟，每 10 秒采样一次（共 60 次）
pidstat -u -p $PIDS 10 60 | tee cpu-measurement.log

# 5. 计算平均值
grep -E "sidestep-hermit|WebKitWebProcess|WebKitNetworkProcess" cpu-measurement.log | \
  grep -v "^Average" | \
  awk '{sum+=$8; count++} END {print "Average CPU: " sum/count "%"}'
```

### 方法 2：使用 top（备选）

```bash
# 1. 启动应用
sidestep-hermit &

# 2. 获取所有相关进程 PID
MAIN_PID=$(pgrep -x sidestep-hermit)
PIDS="$MAIN_PID $(pgrep -P $MAIN_PID | tr '\n' ' ')"

# 3. 使用 top 批处理模式采样
for i in {1..60}; do
  for pid in $PIDS; do
    if ps -p $pid > /dev/null 2>&1; then
      top -b -n 1 -p $pid | grep "^ *$pid" | awk '{print $9}'
    fi
  done
  sleep 10
done > cpu-samples.txt

# 4. 计算平均值
awk '{sum+=$1; count++} END {print "Average CPU: " sum/count "%"}' cpu-samples.txt
```

### 方法 3：使用 ps（简化版，精度较低）

```bash
# 每 10 秒采样一次，共 60 次
for i in {1..60}; do
  ps -C sidestep-hermit,WebKitWebProcess,WebKitNetworkProcess -o %cpu= 2>/dev/null | \
    awk '{sum+=$1} END {print sum}'
  sleep 10
done | awk '{sum+=$1; count++} END {print "Average CPU: " sum/count "%"}'
```

## 测试场景

### 场景 1：空闲状态
- 应用已启动
- 未播放音乐
- 无用户交互
- 测量时长：10 分钟

### 场景 2：播放状态
- 正在播放本地音乐（MP3 或 FLAC）
- 歌词显示开启
- 均衡器关闭
- 测量时长：10 分钟

### 场景 3：活跃使用
- 播放音乐
- 切换歌曲、搜索、浏览
- 模拟正常使用
- 测量时长：10 分钟

## 注意事项

1. **避免匹配无关进程**：
   - 不要使用 `grep sidestep` 或 `grep -i hermit`
   - 使用精确匹配：`pgrep -x sidestep-hermit`
   - 开发时避免 cargo/rust-analyzer 进程干扰

2. **包含所有子进程**：
   - WebKitWebProcess（页面渲染）
   - WebKitNetworkProcess（网络请求）
   - GStreamer 子进程（如有）

3. **采样间隔**：
   - 不要使用 `ps aux` 的 %CPU（累计平均值）
   - 使用按时间段计算的工具
   - 建议间隔：10 秒

4. **测试环境**：
   - 记录 OS 版本、CPU 型号
   - 关闭其他音乐播放器
   - 系统负载应正常（非高负载）

## 报告格式

```markdown
## 性能测试结果

- **CPU 占用**: 
  - 空闲：X.X% ± Y.Y%
  - 播放：X.X% ± Y.Y%
- **内存占用**: XXX MB
- **测试环境**:
  - 系统: Ubuntu XX.XX LTS
  - CPU: [型号] @ [频率]
  - 测试方法: pidstat 10秒采样，10分钟平均，包含所有子进程
  - 测试日期: YYYY-MM-DD
```

## 自动化脚本

```bash
#!/bin/bash
# scripts/measure-cpu.sh

set -e

DURATION=600  # 10 分钟
INTERVAL=10   # 10 秒采样

echo "启动 CPU 测量..."

# 检查应用是否运行
if ! pgrep -x sidestep-hermit > /dev/null; then
    echo "错误: sidestep-hermit 未运行"
    echo "请先启动应用，然后重新运行此脚本"
    exit 1
fi

# 获取进程 ID
MAIN_PID=$(pgrep -x sidestep-hermit)
echo "主进程 PID: $MAIN_PID"

# 获取所有子进程
CHILD_PIDS=$(pgrep -P $MAIN_PID | tr '\n' ',')
ALL_PIDS="${CHILD_PIDS}${MAIN_PID}"

echo "监控进程: $ALL_PIDS"
echo "测量时长: $DURATION 秒"
echo "采样间隔: $INTERVAL 秒"
echo ""

# 开始测量
pidstat -u -p $ALL_PIDS $INTERVAL $((DURATION / INTERVAL)) | tee cpu-measurement-$(date +%Y%m%d-%H%M%S).log

echo ""
echo "测量完成。使用以下命令计算平均值："
echo "grep -E 'sidestep-hermit|WebKit' cpu-measurement-*.log | grep -v '^Average' | awk '{sum+=\$8; count++} END {print \"Average CPU: \" sum/count \"%\"}'"
```
