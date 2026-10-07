import { create } from 'zustand'
import type { LyricLine, QueueEntry } from '../playback'

export type PlayState = 'Stopped' | 'Playing' | 'Paused' | 'Buffering'

interface PlayerStore {
  state: PlayState
  /** 当前播放的曲目（由后端广播，队列前进时自动更新） */
  current: QueueEntry | null
  /** 播放队列 */
  items: QueueEntry[]
  /** 当前曲目在队列中的位置 */
  index: number | null
  /** 秒 */
  position: number
  /** 秒 */
  duration: number
  /** 0..1 */
  volume: number
  lyrics: LyricLine[]
  /** 最近一次播放类操作的失败原因，供界面提示 */
  error: string | null

  applyPlayback: (payload: {
    items: QueueEntry[]
    index: number | null
    current: QueueEntry | null
  }) => void
  setState: (state: PlayState) => void
  setPosition: (position: number) => void
  setDuration: (duration: number) => void
  setVolume: (volume: number) => void
  setLyrics: (lyrics: LyricLine[]) => void
  setError: (error: string | null) => void
  reset: () => void
}

export const usePlayerStore = create<PlayerStore>((set) => ({
  state: 'Stopped',
  current: null,
  items: [],
  index: null,
  position: 0,
  duration: 0,
  volume: 1,
  lyrics: [],
  error: null,

  applyPlayback: ({ items, index, current }) =>
    set((prev) => ({
      items,
      index,
      current,
      // 换歌时归零，避免进度条停留在上一首的位置
      position: current?.id === prev.current?.id ? prev.position : 0,
      duration: current?.duration || 0,
      lyrics: current?.id === prev.current?.id ? prev.lyrics : [],
      // 播放成功即清掉上一次的错误提示
      error: current ? null : prev.error,
    })),

  setState: (state) => set({ state }),
  setPosition: (position) => set({ position }),
  setDuration: (duration) => set({ duration }),
  setVolume: (volume) => set({ volume }),
  setLyrics: (lyrics) => set({ lyrics }),
  setError: (error) => set({ error }),
  reset: () =>
    set({
      state: 'Stopped',
      current: null,
      items: [],
      index: null,
      position: 0,
      duration: 0,
      lyrics: [],
      error: null,
    }),
}))
