/**
 * 播放动作的统一入口。
 *
 * 取流可能因为版权 / 登录状态失败，这类失败必须让用户看见，
 * 所以统一在这里兜底并写进 store，而不是只丢进控制台。
 */
import { playback } from './playback'
import type { QueueEntry, SongItem } from './playback'
import { usePlayerStore } from './store/player'

function guard<T>(promise: Promise<T>): Promise<T | undefined> {
  return promise.catch((err: unknown) => {
    const message = err instanceof Error ? err.message : String(err)
    usePlayerStore.getState().setError(message)
    console.error('[playback]', err)
    return undefined
  })
}

/**
 * 双击会连发两次 click，若不加去重会把同一首歌重新起播两次
 * （一次取流就够，两次纯属浪费）。这里吞掉紧邻的重复请求。
 */
const DEDUPE_MS = 600
let lastPlay: { key: string; at: number } | null = null

export const actions = {
  playAll: (items: SongItem[], start: number) => {
    const target = items[start]
    const key = `${target?.provider}:${target?.id}#${items.length}`
    const now = Date.now()

    if (lastPlay && lastPlay.key === key && now - lastPlay.at < DEDUPE_MS) {
      return Promise.resolve(undefined)
    }
    lastPlay = { key, at: now }

    return guard(playback.playAll(items.map(toEntryLocal), start))
  },

  playNext: (item: SongItem) => guard(playback.playNext(toEntryLocal(item))),

  append: (item: SongItem) => guard(playback.append(toEntryLocal(item))),

  jump: (index: number) => guard(playback.jump(index)),
  remove: (index: number) => guard(playback.remove(index)),
  clear: () => guard(playback.clear()),
  next: () => guard(playback.next()),
  previous: () => guard(playback.previous()),
  pause: () => guard(playback.pause()),
  resume: () => guard(playback.resume()),
  togglePlay: () => {
    const isPlaying = usePlayerStore.getState().state === 'Playing'
    return guard(isPlaying ? playback.pause() : playback.resume())
  },
  seekRatio: (ratio: number) => guard(playback.seekRatio(ratio)),
  setVolume: (volume: number) => guard(playback.setVolume(volume)),
}

function toEntryLocal(item: SongItem): QueueEntry {
  return {
    provider: item.provider,
    id: item.id,
    title: item.title,
    artist: item.artist,
    album: item.album,
    duration: item.duration,
  }
}
