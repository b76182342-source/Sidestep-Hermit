/* 艺术家 —— 对照设计稿 11-artists */
import { useEffect, useMemo, useState } from 'react'
import { invoke } from '@tauri-apps/api/core'
import { coverColor } from '../../utils/format'
import type { SongItem } from '../../playback'
import type { Track } from '../../types'

interface ArtistsProps {
  activeKey: string | null
  onPlay: (items: SongItem[], index: number) => void
  reloadToken: number
}

type Sort = 'name' | 'count'

interface ArtistGroup {
  name: string
  albums: Set<string>
  songs: SongItem[]
}

export default function Artists({ activeKey, onPlay, reloadToken }: ArtistsProps) {
  const [tracks, setTracks] = useState<Track[]>([])
  const [loading, setLoading] = useState(true)
  const [sort, setSort] = useState<Sort>('name')

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

  const artists: ArtistGroup[] = useMemo(() => {
    const map = new Map<string, ArtistGroup>()

    tracks.forEach((t) => {
      const song: SongItem = {
        provider: 'local',
        id: t.id.id,
        title: t.title,
        artist: t.artist,
        album: t.album,
        duration: t.duration,
      }

      const existing = map.get(t.artist)
      if (existing) {
        existing.albums.add(t.album)
        existing.songs.push(song)
      } else {
        map.set(t.artist, { name: t.artist, albums: new Set([t.album]), songs: [song] })
      }
    })

    const list = Array.from(map.values())
    list.sort((a, b) =>
      sort === 'count' ? b.songs.length - a.songs.length : a.name.localeCompare(b.name, 'zh')
    )
    return list
  }, [tracks, sort])

  const playingArtistName = useMemo(() => {
    if (!activeKey) return null
    return (
      artists.find((a) => a.songs.some((s) => `${s.provider}:${s.id}` === activeKey))?.name ?? null
    )
  }, [artists, activeKey])

  return (
    <>
      <div className="list-head">
        <div className="list-head-titles">
          <h1 className="page-title">艺术家</h1>
          <div className="page-sub">本地音乐库</div>
        </div>
        <div className="chip-row">
          <button className={`chip ${sort === 'name' ? 'on' : ''}`} onClick={() => setSort('name')}>
            名称
          </button>
          <button className={`chip ${sort === 'count' ? 'on' : ''}`} onClick={() => setSort('count')}>
            歌曲数
          </button>
        </div>
      </div>

      {loading ? (
        <div className="state">
          <div className="spinner" />
        </div>
      ) : artists.length === 0 ? (
        <div className="state">
          <div className="state-title">还没有艺术家</div>
          <p>曲库为空，或者音乐文件缺少艺术家信息</p>
        </div>
      ) : (
        <div className="grid-artists">
          {artists.map((artist) => (
            <div
              key={artist.name}
              className="ar"
              onDoubleClick={() => onPlay(artist.songs, 0)}
              title="双击播放该艺术家的全部曲目"
            >
              <div
                className={`av serif ${playingArtistName === artist.name ? 'on' : ''}`}
                style={{ background: coverColor(artist.name) }}
              >
                {artist.name.slice(0, 1)}
              </div>
              <div className="nm" style={{ marginTop: 0 }}>
                {artist.name}
              </div>
              <div className="sub">
                {artist.albums.size} 张专辑 · {artist.songs.length} 首
              </div>
            </div>
          ))}
        </div>
      )}
    </>
  )
}
