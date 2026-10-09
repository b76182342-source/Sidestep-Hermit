/**
 * 播放相关的前端 API 封装。
 *
 * 队列与取流都在 Rust 侧完成，这里只负责下发指令和订阅状态。
 */
import { invoke } from '@tauri-apps/api/core'

/** 可播放的一首歌（本地或在线） */
export interface SongItem {
  /** `local` | `kugou` */
  provider: string
  /** 本地为文件路径，酷狗为资源 hash */
  id: string
  title: string
  artist: string
  album: string
  /** 秒 */
  duration: number
  /** 封面图片 URL */
  artworkUrl?: string
}

export interface QueueEntry extends SongItem {
  /** 由「下一首播放」插队进来 */
  inserted?: boolean
}

export interface QueueSnapshot {
  items: QueueEntry[]
  index: number | null
  current: QueueEntry | null
}

/** 歌词一行 */
export interface LyricWord {
  startMs: number
  durationMs: number
  text: string
}

export interface LyricLine {
  timeMs: number
  durationMs: number
  text: string
  translation: string | null
  words?: LyricWord[]
}

export const playback = {
  /** 用给定列表替换队列并从 start 播放 */
  playAll: (entries: QueueEntry[], start = 0) =>
    invoke('queue_play', { entries, start }),

  /** 追加到队列末尾 */
  append: (entry: QueueEntry) => invoke('queue_append', { entries: [entry] }),

  /** 插到当前曲目之后 */
  playNext: (entry: QueueEntry) => invoke('queue_play_next', { entry }),

  /** 跳到队列第 index 首 */
  jump: (index: number) => invoke('queue_jump', { index }),

  remove: (index: number) => invoke('queue_remove', { index }),
  clear: () => invoke('clear_queue'),
  next: () => invoke('next_track'),
  previous: () => invoke('previous_track'),
  pause: () => invoke('pause'),
  resume: () => invoke('resume'),

  /** 定位：入参为 0..1 的比例 */
  seekRatio: (ratio: number) => invoke('seek', { position: ratio }),
  /** 音量：入参为 0..1 */
  setVolume: (volume: number) => invoke('set_volume', { volume }),
}

/** 把任意来源的曲目统一成队列条目 */
export function toEntry(item: SongItem): QueueEntry {
  return {
    provider: item.provider,
    id: item.id,
    title: item.title,
    artist: item.artist,
    album: item.album,
    duration: item.duration,
    artworkUrl: item.artworkUrl,
  }
}

export function entryKey(entry: SongItem): string {
  return `${entry.provider}:${entry.id}`
}
