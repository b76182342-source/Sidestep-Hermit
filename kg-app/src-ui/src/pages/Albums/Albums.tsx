/* 专辑 —— 对照设计稿 02-albums */
import { useEffect, useMemo, useState } from 'react'
import { invoke } from '@tauri-apps/api/core'
import { coverColor } from '../../utils/format'
import type { SongItem } from '../../playback'
import type { Track } from '../../types'

interface AlbumsProps {
  activeKey: string | null
  onPlay: (items: SongItem[], index: number) => void
  reloadToken: number
}

type Sort = 'recent' | 'name' | 'artist'

interface AlbumGroup {
  name: string
  artist: string
  songs: SongItem[]
}

export default function Albums({ activeKey, onPlay, reloadToken }: AlbumsProps) {
  const [tracks, setTracks] = useState<Track[]>([])
  const [loading, setLoading] = useState(true)
  const [sort, setSort] = useState<Sort>('recent')

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

  const albums: AlbumGroup[] = useMemo(() => {
    const map = new Map<string, AlbumGroup>()

    tracks.forEach((t) => {
      const song: SongItem = {
        provider: 'local',
        id: t.id.id,
        title: t.title,
        artist: t.artist,
        album: t.album,
        duration: t.duration,
      }

      const existing = map.get(t.album)
      if (existing) existing.songs.push(song)
      else map.set(t.album, { name: t.album, artist: t.artist, songs: [song] })
    })

    const list = Array.from(map.values())
    if (sort === 'name') list.sort((a, b) => a.name.localeCompare(b.name, 'zh'))
    if (sort === 'artist') list.sort((a, b) => a.artist.localeCompare(b.artist, 'zh'))
    return list
  }, [tracks, sort])

  const playingAlbumName = useMemo(() => {
    if (!activeKey) return null
    return albums.find((a) => a.songs.some((s) => `${s.provider}:${s.id}` === activeKey))?.name ?? null
  }, [albums, activeKey])

  return (
    <>
      <div className="list-head">
        <div className="list-head-titles">
          <h1 className="page-title">专辑</h1>
          <div className="page-sub">本地音乐库</div>
        </div>
        <div className="chip-row">
          <button className={`chip ${sort === 'recent' ? 'on' : ''}`} onClick={() => setSort('recent')}>
            最近添加
          </button>
          <button className={`chip ${sort === 'name' ? 'on' : ''}`} onClick={() => setSort('name')}>
            名称
          </button>
          <button className={`chip ${sort === 'artist' ? 'on' : ''}`} onClick={() => setSort('artist')}>
            艺术家
          </button>
        </div>
      </div>

      {loading ? (
        <div className="state">
          <div className="spinner" />
        </div>
      ) : albums.length === 0 ? (
        <div className="state">
          <div className="state-title">还没有专辑</div>
          <p>曲库为空，或者音乐文件缺少专辑信息</p>
        </div>
      ) : (
        <div className="grid-albums">
          {albums.map((album) => (
            <div
              key={album.name}
              className="album-cell"
              onDoubleClick={() => onPlay(album.songs, 0)}
              title="双击播放整张专辑"
            >
              <div className="ac" style={{ background: coverColor(album.name) }}>
                {playingAlbumName === album.name && (
                  <button
                    className="pl"
                    aria-label="正在播放"
                    onClick={(e) => {
                      e.stopPropagation()
                      onPlay(album.songs, 0)
                    }}
                  >
                    <svg width="18" height="18" viewBox="0 0 24 24" fill="currentColor">
                      <path d="M7 5h3v14H7zM14 5h3v14h-3z" />
                    </svg>
                  </button>
                )}
              </div>
              <div className={`nm ${playingAlbumName === album.name ? 'on' : ''}`}>{album.name}</div>
              <div className="sub">{album.artist}</div>
            </div>
          ))}
        </div>
      )}
    </>
  )
}
