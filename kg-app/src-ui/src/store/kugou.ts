import { create } from 'zustand'
import { invoke } from '@tauri-apps/api/core'

/**
 * 酷狗登录态。
 *
 * 这里 **不保存 token** —— 凭据只留在 Rust 侧与系统密钥环中，
 * 前端只需要知道「是否已登录」和账号标识。
 */
interface KugouStore {
  isLoggedIn: boolean
  userid: string | null
  mid: string | null
  source: 'local' | 'online'

  /** 启动时从后端（必要时从密钥环）恢复登录态 */
  restore: () => Promise<void>
  setAccount: (userid: string, mid?: string) => void
  logout: () => Promise<void>
  setSource: (source: 'local' | 'online') => void
}

export const useKugouStore = create<KugouStore>((set) => ({
  isLoggedIn: false,
  userid: null,
  mid: null,
  source: 'local',

  restore: async () => {
    try {
      const s = await invoke<{ loggedIn: boolean; userid?: string; mid?: string }>('kugou_session')
      set({
        isLoggedIn: Boolean(s?.loggedIn),
        userid: s?.userid ?? null,
        mid: s?.mid ?? null,
      })
    } catch {
      set({ isLoggedIn: false, userid: null, mid: null })
    }
  },

  setAccount: (userid, mid) =>
    set({ isLoggedIn: true, userid, mid: mid ?? null }),

  logout: async () => {
    try {
      await invoke('kugou_logout')
    } finally {
      set({ isLoggedIn: false, userid: null, mid: null, source: 'local' })
    }
  },

  setSource: (source) => set({ source }),
}))
