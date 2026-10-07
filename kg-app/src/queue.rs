//! 统一播放队列。
//!
//! 本地文件与酷狗在线曲目共用同一套顺序播放逻辑：队列只保存「怎么找回这首歌」
//! 的信息（provider + id），真正取流时再按 provider 分派。

use serde::{Deserialize, Serialize};

/// 队列中的一条曲目。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueueEntry {
    /// `local` | `kugou`
    pub provider: String,
    /// 本地为文件路径，酷狗为资源 hash
    pub id: String,
    pub title: String,
    pub artist: String,
    pub album: String,
    /// 秒
    pub duration: u64,
    /// 是否通过「下一首播放」插队进来的（队列抽屉里会标记）
    #[serde(default)]
    pub inserted: bool,
}

impl QueueEntry {
    pub fn key(&self) -> String {
        format!("{}:{}", self.provider, self.id)
    }

    pub fn is_online(&self) -> bool {
        self.provider != "local"
    }
}

/// 给前端的队列快照。
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QueueSnapshot {
    pub items: Vec<QueueEntry>,
    pub index: Option<usize>,
    pub current: Option<QueueEntry>,
}

/// 顺序播放队列（不循环；到底即停，由上层决定后续行为）。
#[derive(Debug, Default)]
pub struct PlayQueue {
    items: Vec<QueueEntry>,
    index: Option<usize>,
}

impl PlayQueue {
    /// 用新列表替换队列，并把游标落在 `start` 上。
    pub fn replace(&mut self, items: Vec<QueueEntry>, start: usize) -> Option<QueueEntry> {
        self.items = items;

        if self.items.is_empty() {
            self.index = None;
            return None;
        }

        let start = start.min(self.items.len() - 1);
        self.index = Some(start);
        self.items.get(start).cloned()
    }

    pub fn current(&self) -> Option<QueueEntry> {
        self.index.and_then(|i| self.items.get(i)).cloned()
    }

    /// 追加到队尾。返回是否因此从「空队列」变为有内容。
    pub fn append(&mut self, entries: Vec<QueueEntry>) -> bool {
        let was_empty = self.items.is_empty();
        self.items.extend(entries);

        if was_empty && !self.items.is_empty() {
            self.index = Some(0);
            return true;
        }

        false
    }

    /// 插到当前曲目之后（「下一首播放」）。队列为空时等同追加。
    pub fn insert_next(&mut self, mut entry: QueueEntry) -> bool {
        entry.inserted = true;

        if self.items.is_empty() || self.index.is_none() {
            self.items.push(entry);
            self.index = Some(0);
            return true;
        }

        let at = self.index.unwrap_or(0) + 1;
        self.items.insert(at, entry);
        false
    }

    /// 下一首；已是最后一首返回 `None`。
    pub fn next(&mut self) -> Option<QueueEntry> {
        let next = self.index? + 1;
        if next >= self.items.len() {
            return None;
        }
        self.index = Some(next);
        self.items.get(next).cloned()
    }

    /// 上一首；已是第一首返回 `None`。
    pub fn previous(&mut self) -> Option<QueueEntry> {
        let prev = self.index?.checked_sub(1)?;
        self.index = Some(prev);
        self.items.get(prev).cloned()
    }

    /// 移出指定位置；返回移除后应当播放的曲目（若队列非空）。
    pub fn remove(&mut self, index: usize) -> Option<QueueEntry> {
        if index >= self.items.len() {
            return self.current();
        }

        let removed_playing = self.index == Some(index);
        self.items.remove(index);

        match self.index {
            Some(cur) if cur > index => self.index = Some(cur - 1),
            _ if removed_playing => {
                self.index = if self.items.is_empty() {
                    None
                } else {
                    Some(index.min(self.items.len() - 1))
                };
            }
            _ => {}
        }

        self.current()
    }

    pub fn clear(&mut self) {
        self.items.clear();
        self.index = None;
    }

    pub fn snapshot(&self) -> QueueSnapshot {
        QueueSnapshot {
            items: self.items.clone(),
            index: self.index,
            current: self.current(),
        }
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]

    use super::*;

    fn entry(id: &str) -> QueueEntry {
        QueueEntry {
            provider: "local".into(),
            id: id.into(),
            title: id.into(),
            artist: String::new(),
            album: String::new(),
            duration: 0,
            inserted: false,
        }
    }

    #[test]
    fn replace_clamps_start_index() {
        let mut q = PlayQueue::default();
        let cur = q.replace(vec![entry("a"), entry("b")], 99).unwrap();
        assert_eq!(cur.id, "b");
        assert_eq!(q.snapshot().index, Some(1));
    }

    #[test]
    fn next_stops_at_end() {
        let mut q = PlayQueue::default();
        q.replace(vec![entry("a"), entry("b")], 0);
        assert_eq!(q.next().unwrap().id, "b");
        assert!(q.next().is_none(), "最后一首之后不应再前进");
    }

    #[test]
    fn previous_stops_at_start() {
        let mut q = PlayQueue::default();
        q.replace(vec![entry("a"), entry("b")], 1);
        assert_eq!(q.previous().unwrap().id, "a");
        assert!(q.previous().is_none(), "第一首之前不应再后退");
    }

    #[test]
    fn removing_before_current_shifts_index() {
        let mut q = PlayQueue::default();
        q.replace(vec![entry("a"), entry("b"), entry("c")], 2);
        let cur = q.remove(0).unwrap();
        assert_eq!(cur.id, "c");
        assert_eq!(q.snapshot().index, Some(1));
    }

    #[test]
    fn removing_current_plays_following_track() {
        let mut q = PlayQueue::default();
        q.replace(vec![entry("a"), entry("b"), entry("c")], 1);
        let cur = q.remove(1).unwrap();
        assert_eq!(cur.id, "c", "删掉正在播的那条应顺延到下一首");
    }

    #[test]
    fn removing_last_track_falls_back_to_previous() {
        let mut q = PlayQueue::default();
        q.replace(vec![entry("a"), entry("b")], 1);
        let cur = q.remove(1).unwrap();
        assert_eq!(cur.id, "a");
    }

    #[test]
    fn clearing_empties_queue() {
        let mut q = PlayQueue::default();
        q.replace(vec![entry("a")], 0);
        q.clear();
        assert!(q.snapshot().items.is_empty());
        assert!(q.current().is_none());
    }

    #[test]
    fn appending_to_empty_queue_starts_playing() {
        let mut q = PlayQueue::default();
        assert!(q.append(vec![entry("a")]), "空队列追加后应当开始播放");
        assert_eq!(q.current().unwrap().id, "a");
    }

    #[test]
    fn appending_keeps_current_track() {
        let mut q = PlayQueue::default();
        q.replace(vec![entry("a"), entry("b")], 0);
        assert!(!q.append(vec![entry("c")]));
        assert_eq!(q.current().unwrap().id, "a", "追加不应打断当前播放");
        assert_eq!(q.snapshot().items.len(), 3);
    }

    #[test]
    fn insert_next_lands_right_after_current() {
        let mut q = PlayQueue::default();
        q.replace(vec![entry("a"), entry("b")], 0);
        q.insert_next(entry("x"));

        let ids: Vec<_> = q.snapshot().items.iter().map(|e| e.id.clone()).collect();
        assert_eq!(ids, vec!["a", "x", "b"]);
        assert!(q.snapshot().items[1].inserted, "插队项应带标记");
        assert_eq!(q.next().unwrap().id, "x", "下一首应播插队项");
    }
}
