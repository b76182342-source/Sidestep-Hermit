import { create } from 'zustand'
import type { QueueEntry } from '../playback'

const STORAGE_KEY = 'kgplayer:history'
const MAX_ITEMS = 200

interface HistoryStore {
  items: QueueEntry[]
  /** 记一首歌；同一首重复播放会提到最前 */
  record: (entry: QueueEntry) => void
  clear: () => void
}

function load(): QueueEntry[] {
  try {
    const raw = localStorage.getItem(STORAGE_KEY)
    if (!raw) return []
    const parsed = JSON.parse(raw)
    return Array.isArray(parsed) ? (parsed as QueueEntry[]) : []
  } catch {
    return []
  }
}

function persist(items: QueueEntry[]) {
  try {
    localStorage.setItem(STORAGE_KEY, JSON.stringify(items))
  } catch {
    /* 存储不可用时静默降级：历史只是便利功能 */
  }
}

export const useHistoryStore = create<HistoryStore>((set, get) => ({
  items: load(),

  record: (entry) => {
    const key = `${entry.provider}:${entry.id}`
    const rest = get().items.filter((i) => `${i.provider}:${i.id}` !== key)
    const items = [entry, ...rest].slice(0, MAX_ITEMS)
    persist(items)
    set({ items })
  },

  clear: () => {
    persist([])
    set({ items: [] })
  },
}))
