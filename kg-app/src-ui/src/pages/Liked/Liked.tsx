/* 我喜欢 —— 酷狗系统默认收藏歌单 */
import { useCallback, useEffect, useState } from 'react'
import { invoke } from '@tauri-apps/api/core'
import SongList from '../../components/SongList'
import { coverColor } from '../../utils/format'
import type { SongItem } from '../../playback'

interface LikedProps {
  query: string
  activeKey: string | null
  isLoggedIn: boolean
  onOpenLogin: () => void
  onPlay: (items: SongItem[], index: number) => void
  onPlayNext: (item: SongItem) => void
  onAppend: (item: SongItem) => void
}

/** 酷狗搜索结果 / 推荐项 → 统一的 `SongItem` */
interface KugouHit {
  hash: string
  title: string
  artist: string
  album: string
  durationSecs: number
  artworkUrl: string | null
  privilege: number | null
}

function toSong(hit: KugouHit): SongItem {
  return {
    provider: 'kugou',
    id: hit.hash,
    title: hit.title,
    artist: hit.artist,
    album: hit.album,
    duration: hit.durationSecs,
    artworkUrl: hit.artworkUrl || undefined,
  }
}

export default function Liked({
  query,
  activeKey,
  isLoggedIn,
  onOpenLogin,
  onPlay,
  onPlayNext,
  onAppend,
}: LikedProps) {
  const [hits, setHits] = useState<KugouHit[]>([])
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState('')

  const loadLiked = useCallback(async () => {
    if (!isLoggedIn) {
      setLoading(false)
      setError('请先登录酷狗账号')
      return
    }

    setLoading(true)
    setError('')
    try {
      const list = await invoke<KugouHit[]>('kugou_liked_songs')
      setHits(list)
    } catch (e) {
      setHits([])
      setError(String(e))
    } finally {
      setLoading(false)
    }
  }, [isLoggedIn])

  useEffect(() => {
    void loadLiked()
  }, [loadLiked])

  const songs = hits.map(toSong)
  const filtered = query.trim()
    ? songs.filter(
        (s) =>
          s.title.toLowerCase().includes(query.toLowerCase()) ||
          s.artist.toLowerCase().includes(query.toLowerCase())
      )
    : songs

  if (!isLoggedIn) {
    return (
      <>
        <div className="page-head">
          <h1 className="page-title">我喜欢</h1>
          <span className="page-sub">酷狗云端歌单</span>
        </div>
        <div className="state">
          <div className="state-title">需要登录</div>
          <p>
            查看收藏的歌曲需要登录酷狗账号
            <button type="button" onClick={onOpenLogin} className="inline-link">
              立即登录
            </button>
          </p>
        </div>
      </>
    )
  }

  if (loading) {
    return (
      <>
        <div className="page-head">
          <h1 className="page-title">我喜欢</h1>
          <span className="page-sub">酷狗云端歌单</span>
        </div>
        <div className="state">
          <div className="state-title">加载中…</div>
        </div>
      </>
    )
  }

  if (error) {
    return (
      <>
        <div className="page-head">
          <h1 className="page-title">我喜欢</h1>
          <span className="page-sub">酷狗云端歌单</span>
        </div>
        <div className="state">
          <div className="state-title">加载失败</div>
          <p>{error}</p>
          <button type="button" onClick={() => void loadLiked()}>
            重试
          </button>
        </div>
      </>
    )
  }

  if (filtered.length === 0) {
    return (
      <>
        <div className="page-head">
          <h1 className="page-title">我喜欢</h1>
          <span className="page-sub">酷狗云端歌单 · {hits.length} 首</span>
        </div>
        <div className="state">
          <img
            src="/mascots/mascot-typing.png"
            alt="Mascot"
            style={{ width: 120, height: 120, marginBottom: 16, imageRendering: 'pixelated' }}
          />
          <div className="state-title">
            {query.trim() ? '未找到匹配歌曲' : '还没有收藏歌曲'}
          </div>
          {query.trim() && <p>试试其他关键词</p>}
        </div>
      </>
    )
  }

  return (
    <>
      <div className="page-head">
        <h1 className="page-title">我喜欢</h1>
        <span className="page-sub">酷狗云端歌单 · {hits.length} 首</span>
      </div>
      <SongList
        items={filtered}
        activeKey={activeKey}
        onPlay={(item, index) => onPlay(filtered, index)}
        onPlayNext={onPlayNext}
        onAppend={onAppend}
      />
    </>
  )
}
