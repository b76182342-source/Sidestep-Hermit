/* 播放历史 */
import { useMemo } from 'react'
import SongList from '../../components/SongList'
import { useHistoryStore } from '../../store/history'
import type { SongItem } from '../../playback'

interface HistoryProps {
  query: string
  activeKey: string | null
  onPlay: (items: SongItem[], index: number) => void
  onPlayNext: (item: SongItem) => void
  onAppend: (item: SongItem) => void
}

export default function History({
  query,
  activeKey,
  onPlay,
  onPlayNext,
  onAppend,
}: HistoryProps) {
  const { items, clear } = useHistoryStore()

  const songs: SongItem[] = useMemo(
    () =>
      items.map((i) => ({
        provider: i.provider,
        id: i.id,
        title: i.title,
        artist: i.artist,
        album: i.album,
        duration: i.duration,
      })),
    [items]
  )

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
      <div className="list-head">
        <div className="list-head-titles">
          <h1 className="page-title">播放历史</h1>
          <div className="page-sub">{songs.length} 首 · 仅保存在本机</div>
        </div>
        {songs.length > 0 && (
          <div className="chip-row">
            <button className="chip" onClick={clear}>
              清空
            </button>
          </div>
        )}
      </div>

      <SongList
        items={visible}
        activeKey={activeKey}
        onPlay={(_, index) => onPlay(visible, index)}
        onPlayNext={onPlayNext}
        onAppend={onAppend}
        emptyText="还没有播放记录"
      />
    </>
  )
}
