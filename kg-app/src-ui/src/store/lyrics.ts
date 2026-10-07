import { create } from 'zustand'
import type { LyricLine } from '../types'

interface LyricsStore {
  lyrics: LyricLine[]
  currentIndex: number | null
  setLyrics: (lines: LyricLine[]) => void
  setCurrentIndex: (index: number | null) => void
  clear: () => void
}

export const useLyricsStore = create<LyricsStore>((set) => ({
  lyrics: [],
  currentIndex: null,
  setLyrics: (lyrics) => set({ lyrics, currentIndex: null }),
  setCurrentIndex: (index) => set({ currentIndex: index }),
  clear: () => set({ lyrics: [], currentIndex: null }),
}))
