//! 播放调度：把队列条目解析成可播放来源、驱动队列前进，并广播状态。

use std::sync::{Arc, Mutex};

use tauri::{AppHandle, Emitter};
use tokio::sync::mpsc;
use tracing::{debug, info, warn};

use kg_audio::PlayerCommand;
use kg_core::{ProviderRegistry, StreamSource, TrackId};
use kg_store::Database;

use crate::queue::{PlayQueue, QueueEntry};

/// 播放状态变化事件名（前端订阅后刷新当前曲目与队列）
pub const EVENT_PLAYBACK: &str = "playback-changed";

#[cfg(feature = "kugou")]
use crate::commands_kugou;

/// 播放所需的共享上下文。命令层与 MPRIS 任务各持一份 `Arc`。
pub struct PlaybackCtx {
    registry: Arc<ProviderRegistry>,
    db: Arc<Mutex<Database>>,
    queue: Mutex<PlayQueue>,
    cmd_tx: mpsc::UnboundedSender<PlayerCommand>,
    app: AppHandle,
}

impl PlaybackCtx {
    pub fn new(
        registry: Arc<ProviderRegistry>,
        db: Arc<Mutex<Database>>,
        cmd_tx: mpsc::UnboundedSender<PlayerCommand>,
        app: AppHandle,
    ) -> Self {
        Self {
            registry,
            db,
            queue: Mutex::new(PlayQueue::default()),
            cmd_tx,
            app,
        }
    }

    /// 解析某条队列曲目的播放来源。
    ///
    /// 只解析账号有权播放的内容；受限曲目由取流环节直接报错，不做任何绕过。
    async fn resolve(&self, entry: &QueueEntry) -> Result<StreamSource, String> {
        // 从 registry 获取对应的 Provider
        let provider = self
            .registry
            .get(&entry.provider)
            .ok_or_else(|| format!("未知音源：{}", entry.provider))?;

        // 特殊处理酷狗（保持向后兼容）
        #[cfg(feature = "kugou")]
        if entry.provider == "kugou" {
            let outcome = commands_kugou::resolve_kugou_url(&entry.id).await?;
            if outcome.preview {
                debug!("该曲目仅获授权试听片段");
            }
            return Ok(StreamSource::Url(outcome.url));
        }

        // 通用 Provider 处理
        provider
            .stream_source(&TrackId::new(&entry.provider, &entry.id))
            .await
            .map_err(|e| format!("曲目无法播放：{e}"))
    }

    /// 用新列表替换队列并从 `start` 开始播放。
    pub async fn play_from(&self, items: Vec<QueueEntry>, start: usize) -> Result<(), String> {
        let entry = {
            let mut q = self.queue.lock().map_err(|e| e.to_string())?;
            q.replace(items, start)
        };

        let Some(entry) = entry else {
            return Err("队列为空".into());
        };

        self.start(&entry).await
    }

    /// 播放当前游标处的曲目（不移动游标）。
    pub async fn play_current(&self) -> Result<(), String> {
        let entry = {
            let q = self.queue.lock().map_err(|e| e.to_string())?;
            q.current()
        };

        match entry {
            Some(e) => self.start(&e).await,
            None => Err("队列为空".into()),
        }
    }

    /// 下一首。到队尾时返回 `Err`，由调用方决定是否提示。
    pub async fn next(&self) -> Result<(), String> {
        let entry = {
            let mut q = self.queue.lock().map_err(|e| e.to_string())?;
            q.next()
        };

        match entry {
            Some(e) => self.start(&e).await,
            None => Err("已经是最后一首".into()),
        }
    }

    /// 上一首。
    pub async fn previous(&self) -> Result<(), String> {
        let entry = {
            let mut q = self.queue.lock().map_err(|e| e.to_string())?;
            q.previous()
        };

        match entry {
            Some(e) => self.start(&e).await,
            None => Err("已经是第一首".into()),
        }
    }

    /// 追加到队尾；队列原本为空时直接开始播放。
    pub async fn append(&self, entries: Vec<QueueEntry>) -> Result<(), String> {
        let became_playable = {
            let mut q = self.queue.lock().map_err(|e| e.to_string())?;
            q.append(entries)
        };

        if became_playable {
            return self.play_current().await;
        }

        self.broadcast();
        Ok(())
    }

    /// 插到当前曲目之后（「下一首播放」）。
    pub async fn play_next(&self, entry: QueueEntry) -> Result<(), String> {
        let became_playable = {
            let mut q = self.queue.lock().map_err(|e| e.to_string())?;
            q.insert_next(entry)
        };

        if became_playable {
            return self.play_current().await;
        }

        self.broadcast();
        Ok(())
    }

    /// 跳转到队列中的指定位置并播放。
    pub async fn jump_to(&self, index: usize) -> Result<(), String> {
        let entry = {
            let mut q = self.queue.lock().map_err(|e| e.to_string())?;
            let items = q.snapshot().items;
            q.replace(items, index)
        };

        match entry {
            Some(e) => self.start(&e).await,
            None => Err("队列为空".into()),
        }
    }

    /// 移出队列中的某一项；若移出的正是当前曲目，则顺延播放。
    pub async fn remove(&self, index: usize) -> Result<(), String> {
        let removed_current = {
            let mut q = self.queue.lock().map_err(|e| e.to_string())?;
            let was_current = q.snapshot().index == Some(index);
            q.remove(index);
            was_current
        };

        if removed_current {
            if let Err(e) = self.play_current().await {
                debug!("移出当前曲目后没有可播放项：{e}");
            }
        } else {
            self.broadcast();
        }

        Ok(())
    }

    pub fn clear(&self) -> Result<(), String> {
        {
            let mut q = self.queue.lock().map_err(|e| e.to_string())?;
            q.clear();
        }
        self.broadcast();
        Ok(())
    }

    pub fn snapshot(&self) -> Result<crate::queue::QueueSnapshot, String> {
        let q = self.queue.lock().map_err(|e| e.to_string())?;
        Ok(q.snapshot())
    }

    /// 真正发指令给播放器并广播新状态。
    async fn start(&self, entry: &QueueEntry) -> Result<(), String> {
        // 对于酷狗歌曲，尝试获取真实的封面 URL
        #[cfg(feature = "kugou")]
        if entry.provider == "kugou" && entry.artwork_url.is_none() {
            if let Ok(artwork_url) = commands_kugou::get_artwork_url(&entry.id).await {
                // 更新队列中的封面 URL
                if let Ok(mut q) = self.queue.lock() {
                    q.update_artwork(&entry.id, artwork_url);
                }
            }
        }

        let source = match self.resolve(entry).await {
            Ok(source) => source,
            Err(e) => {
                // 取流失败是用户能感知的状态，必须留下日志而不是静默失败
                warn!("无法播放《{}》（{}）：{e}", entry.title, entry.provider);
                return Err(e);
            }
        };

        self.cmd_tx
            .send(PlayerCommand::Play(source))
            .map_err(|e| format!("下发播放指令失败：{e}"))?;

        info!("开始播放《{}》— {}", entry.title, entry.artist);
        self.broadcast();
        Ok(())
    }

    /// 把当前队列状态广播给前端。
    pub fn broadcast(&self) {
        let snapshot = match self.queue.lock() {
            Ok(q) => q.snapshot(),
            Err(e) => {
                warn!("读取播放队列失败：{e}");
                return;
            }
        };

        let payload = serde_json::json!({
            "items": snapshot.items,
            "index": snapshot.index,
            "current": snapshot.current,
        });

        if let Err(e) = self.app.emit(EVENT_PLAYBACK, payload) {
            warn!("广播播放状态失败：{e}");
        }
    }
}
