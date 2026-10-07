/* 每日推荐 —— 对照设计稿 03-kugou-home */
import { useCallback, useEffect, useState } from 'react'
import { invoke } from '@tauri-apps/api/core'
import SongList from '../../components/SongList'
import { coverColor, greeting } from '../../utils/format'
import type { SongItem } from '../../playback'

interface DailyProps {
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
  duration_secs: number
  artwork_url: string | null
  privilege: number | null
}

function toSong(hit: KugouHit): SongItem {
  return {
    provider: 'kugou',
    id: hit.hash,
    title: hit.title,
    artist: hit.artist,
    album: hit.album,
    duration: hit.duration_secs,
  }
}

export default function Daily({
  query,
  activeKey,
  isLoggedIn,
  onOpenLogin,
  onPlay,
  onPlayNext,
  onAppend,
}: DailyProps) {
  const [hits, setHits] = useState<KugouHit[]>([])
  const [loading, setLoading] = useState(true)
  const [searching, setSearching] = useState(false)
  const [error, setError] = useState('')

  const loadDaily = useCallback(async () => {
    setLoading(true)
    setError('')
    try {
      const list = await invoke<KugouHit[]>('kugou_daily')
      setHits(list)
    } catch (e) {
      setHits([])
      setError(String(e))
    } finally {
      setLoading(false)
    }
  }, [])

  useEffect(() => {
    void loadDaily()
  }, [loadDaily])

  // 有搜索词时按关键词搜，否则显示每日推荐
  useEffect(() => {
    const q = query.trim()
    if (!q) {
      void loadDaily()
      return
    }

    let alive = true
    setSearching(true)
    setError('')

    const timer = window.setTimeout(async () => {
      try {
        const list = await invoke<KugouHit[]>('kugou_search', { query: q, page: 1 })
        if (alive) setHits(list)
      } catch (e) {
        if (alive) {
          setHits([])
          setError(String(e))
        }
      } finally {
        if (alive) setSearching(false)
      }
    }, 300)

    return () => {
      alive = false
      window.clearTimeout(timer)
    }
  }, [query, loadDaily])

  const songs = hits.map(toSong)
  const isSearch = query.trim().length > 0
  const busy = isSearch ? searching : loading

  if (!isLoggedIn) {
    return (
      <>
        <div className="page-head">
          <h1 className="page-title">{greeting()}</h1>
          <span className="page-sub">酷狗 · 为你推荐</span>
        </div>
        <div className="state">
          <div className="state-title">登录后查看每日推荐</div>
          <p>登录酷狗账号即可获取每日推荐、云端歌单与在线播放</p>
          <button className="btn-dark" onClick={onOpenLogin}>
            登录酷狗
          </button>
        </div>
      </>
    )
  }

  return (
    <>
      <div className="page-head">
        <h1 className="page-title">{greeting()}</h1>
        <span className="page-sub">{isSearch ? `搜索“${query.trim()}”` : '酷狗 · 为你推荐'}</span>
      </div>

      {error && (
        <div className="notice">
          <span>{error}</span>
          <button className="chip" onClick={() => void loadDaily()}>
            重试
          </button>
        </div>
      )}

      {busy ? (
        <div className="state">
          <div className="spinner" />
        </div>
      ) : isSearch ? (
        <SongList
          items={songs}
          activeKey={activeKey}
          onPlay={(_, index) => onPlay(songs, index)}
          onPlayNext={onPlayNext}
          onAppend={onAppend}
          emptyText="没有搜到相关曲目"
        />
      ) : (
        <div className="daily-layout">
          <div className="daily-card">
            <div>
              <div className="daily-card-title">每日推荐</div>
              <div className="daily-card-sub">每天更新</div>
            </div>
            <button
              className="btn-accent"
              onClick={() => songs.length > 0 && onPlay(songs, 0)}
              disabled={songs.length === 0}
            >
              播放全部
            </button>
          </div>

          <div className="daily-list">
            <SongList
              items={songs}
              activeKey={activeKey}
              onPlay={(_, index) => onPlay(songs, index)}
              onPlayNext={onPlayNext}
              onAppend={onAppend}
              emptyText="今天还没有推荐曲目"
            />
          </div>
        </div>
      )}

      {!isSearch && songs.length > 0 && (
        <>
          <h2 className="section-title">云端歌单</h2>
          <div className="tile-grid">
            {['我喜欢', '通勤', '深夜', '跑步'].map((name) => (
              <button key={name} className="tile" type="button">
                <span className="cv tile-cover" style={{ background: coverColor(name) }} />
                <span>
                  <span className="tile-name">{name}</span>
                  <span className="sub">酷狗账号</span>
                </span>
              </button>
            ))}
          </div>
        </>
      )}
    </>
  )
}
