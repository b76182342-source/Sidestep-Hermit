use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// 音轨唯一标识
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TrackId {
    /// 提供方 ID (如 "local", "kugou")
    pub provider: String,
    /// 提供方内部 ID (本地文件路径或酷狗 hash)
    pub id: String,
}

impl TrackId {
    pub fn new(provider: impl Into<String>, id: impl Into<String>) -> Self {
        Self {
            provider: provider.into(),
            id: id.into(),
        }
    }

    pub fn local(path: impl Into<String>) -> Self {
        Self::new("local", path)
    }

    pub fn kugou(hash: impl Into<String>) -> Self {
        Self::new("kugou", hash)
    }
}

/// 音轨
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Track {
    pub id: TrackId,
    pub title: String,
    pub artist: String,
    pub album: String,
    pub duration_secs: u64,
    pub album_art_url: Option<String>,
    pub year: Option<u32>,
    pub track_number: Option<u32>,
    pub genre: Option<String>,
}

/// 专辑
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Album {
    pub id: String,
    pub title: String,
    pub artist: String,
    pub year: Option<u32>,
    pub cover_url: Option<String>,
    pub track_count: u32,
}

/// 艺术家
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Artist {
    pub id: String,
    pub name: String,
    pub album_count: u32,
    pub track_count: u32,
}

/// 播放列表
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Playlist {
    pub id: Uuid,
    pub name: String,
    pub tracks: Vec<TrackId>,
    pub created_at: i64,
    pub updated_at: i64,
}

/// 播放队列
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Queue {
    pub tracks: Vec<TrackId>,
    pub current_index: Option<usize>,
    pub mode: PlayMode,
}

impl Queue {
    pub fn new() -> Self {
        Self {
            tracks: Vec::new(),
            current_index: None,
            mode: PlayMode::Sequential,
        }
    }

    pub fn current(&self) -> Option<&TrackId> {
        self.current_index.and_then(|idx| self.tracks.get(idx))
    }

    /// 前进到下一首。
    ///
    /// 刻意不叫 `next()`：那会与 `Iterator::next` 混淆——语义上「推进播放位置」
    /// 和「取下一个元素」不是一回事，`Queue` 也不实现 `Iterator`。
    pub fn next_track(&mut self) -> Option<&TrackId> {
        if self.tracks.is_empty() {
            return None;
        }

        self.current_index = match (self.mode, self.current_index) {
            (PlayMode::Sequential, Some(idx)) if idx + 1 < self.tracks.len() => Some(idx + 1),
            (PlayMode::Sequential, _) => None,
            (PlayMode::Loop, Some(idx)) => Some((idx + 1) % self.tracks.len()),
            (PlayMode::Loop, None) => Some(0),
            (PlayMode::LoopOne, idx) => idx,
            (PlayMode::Shuffle, _) => {
                // TODO: track history properly
                Some(fastrand::usize(0..self.tracks.len()))
            }
        };

        self.current()
    }

    /// 回退到上一首。命名理由同 [`Queue::next_track`]。
    pub fn previous_track(&mut self) -> Option<&TrackId> {
        if self.tracks.is_empty() {
            return None;
        }

        self.current_index = match (self.mode, self.current_index) {
            (PlayMode::Sequential, Some(idx)) if idx > 0 => Some(idx - 1),
            (PlayMode::Sequential, _) => None,
            (PlayMode::Loop, Some(0)) => Some(self.tracks.len() - 1),
            (PlayMode::Loop, Some(idx)) => Some(idx - 1),
            (PlayMode::Loop, None) => Some(self.tracks.len() - 1),
            (PlayMode::LoopOne, idx) => idx,
            (PlayMode::Shuffle, _) => Some(fastrand::usize(0..self.tracks.len())),
        };

        self.current()
    }

    pub fn add(&mut self, track_id: TrackId) {
        self.tracks.push(track_id);
    }

    pub fn insert_next(&mut self, track_id: TrackId) {
        match self.current_index {
            Some(idx) => self.tracks.insert(idx + 1, track_id),
            None => self.tracks.insert(0, track_id),
        }
    }

    pub fn remove(&mut self, index: usize) {
        if index < self.tracks.len() {
            self.tracks.remove(index);
            if let Some(current) = self.current_index {
                if index < current {
                    self.current_index = Some(current - 1);
                } else if index == current {
                    self.current_index = None;
                }
            }
        }
    }

    pub fn clear(&mut self) {
        self.tracks.clear();
        self.current_index = None;
    }
}

impl Default for Queue {
    fn default() -> Self {
        Self::new()
    }
}

/// 播放模式
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PlayMode {
    Sequential,
    Loop,
    LoopOne,
    Shuffle,
}

/// 播放状态
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PlayState {
    Stopped,
    Playing,
    Paused,
    Buffering,
}

/// 搜索类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SearchKind {
    Track,
    Album,
    Artist,
}

/// 分页参数
#[derive(Debug, Clone, Copy)]
pub struct Page {
    pub number: u32,
    pub size: u32,
}

impl Default for Page {
    fn default() -> Self {
        Self {
            number: 1,
            size: 20,
        }
    }
}

/// 分页结果
#[derive(Debug, Clone)]
pub struct Paged<T> {
    pub items: Vec<T>,
    pub total: u32,
    pub page: Page,
}

/// 搜索结果项
#[derive(Debug, Clone)]
pub enum SearchItem {
    Track(Track),
    Album(Album),
    Artist(Artist),
}

/// 流媒体来源
#[derive(Debug, Clone)]
pub enum StreamSource {
    /// 本地文件路径
    LocalPath(String),
    /// HTTP(S) URL
    Url(String),
}

/// 歌词
#[derive(Debug, Clone)]
pub struct Lyrics {
    pub format: LyricsFormat,
    pub content: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LyricsFormat {
    Lrc,
    PlainText,
}
