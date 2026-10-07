/* 全部歌曲（本地曲库）—— 对照设计稿 01-main */
import { useEffect, useMemo, useState } from 'react'
import { invoke } from '@tauri-apps/api/core'
import SongList from '../../components/SongList'
import type { SongItem } from '../../playback'
import type { Track } from '../../types'

interface HomeProps {
  query: string
  activeKey: string | null
  onPlay: (items: SongItem[], index: number) => void
  onPlayNext: (item: SongItem) => void
  onAppend: (item: SongItem) => void
  onScan: () => void
  reloadToken: number
}

/** 本地 `Track` → 统一的 `SongItem` */
function toSong(track: Track): SongItem {
  return {
    provider: 'local',
    id: track.id.id,
    title: track.title,
    artist: track.artist,
    album: track.album,
    duration: track.duration,
  }
}

export default function Home({
  query,
  activeKey,
  onPlay,
  onPlayNext,
  onAppend,
  onScan,
  reloadToken,
}: HomeProps) {
  const [tracks, setTracks] = useState<Track[]>([])
  const [loading, setLoading] = useState(true)

  useEffect(() => {
    let alive = true
    setLoading(true)
    invoke<Track[]>('get_all_tracks')
      .then((all) => alive && setTracks(all))
      .catch(() => alive && setTracks([]))
      .finally(() => alive && setLoading(false))
    return () => {
      alive = false
    }
  }, [reloadToken])

  const songs = useMemo(() => tracks.map(toSong), [tracks])

  const visible = useMemo(() => {
    const q = query.trim().toLowerCase()
    if (!q) return songs
    return songs.filter(
      (s) =>
        s.title.toLowerCase().includes(q) ||
        s.artist.toLowerCase().includes(q) ||
        s.album.toLowerCase().includes(q)
    )
  }, [songs, query])

  return (
    <>
      <div className="page-head">
        <h1 className="page-title">全部歌曲</h1>
        <span className="page-sub">
          本地{loading ? ' · 读取中' : ` · ${visible.length} 首`}
        </span>
      </div>

      {loading ? (
        <div className="state">
          <div className="spinner" />
        </div>
      ) : songs.length === 0 ? (
        <div className="state">
          <div className="state-title">音乐库还是空的</div>
          <p>把音乐文件放进音乐目录，或扫描一次本地曲库</p>
          <button className="btn-dark" onClick={onScan}>
            扫描音乐库
          </button>
        </div>
      ) : (
        <SongList
          items={visible}
          activeKey={activeKey}
          onPlay={(_, index) => onPlay(visible, index)}
          onPlayNext={onPlayNext}
          onAppend={onAppend}
          emptyText="没有匹配的歌曲"
        />
      )}
    </>
  )
}
