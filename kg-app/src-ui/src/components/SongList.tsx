/* 歌曲列表 —— 对照设计稿 .row，并补上「下一首播放 / 加到队列」入口 */
import type { SongItem } from '../playback'
import { coverColor, formatTime } from '../utils/format'

interface SongListProps {
  items: SongItem[]
  /** 当前播放项的 key（`provider:id`），用于高亮 */
  activeKey?: string | null
  onPlay: (item: SongItem, index: number) => void
  onPlayNext?: (item: SongItem) => void
  onAppend?: (item: SongItem) => void
  emptyText?: string
}

const IconNext = () => (
  <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
    <path d="M6 5v14l9-7z" />
    <path d="M18 5v14" stroke="currentColor" strokeWidth="2" strokeLinecap="round" />
  </svg>
)

const IconPlus = () => (
  <svg
    width="16"
    height="16"
    viewBox="0 0 24 24"
    fill="none"
    stroke="currentColor"
    strokeWidth="2"
    strokeLinecap="round"
  >
    <path d="M12 5v14M5 12h14" />
  </svg>
)

export default function SongList({
  items,
  activeKey = null,
  onPlay,
  onPlayNext,
  onAppend,
  emptyText = '这里还没有歌曲',
}: SongListProps) {
  if (items.length === 0) {
    return (
      <div className="state">
        <div className="state-title">{emptyText}</div>
      </div>
    )
  }

  return (
    <div>
      <div className="song-head with-actions">
        <span>#</span>
        <span>标题</span>
        <span>专辑</span>
        <span>时长</span>
        <span />
      </div>

      {items.map((item, i) => {
        const key = `${item.provider}:${item.id}`
        const playing = key === activeKey

        return (
          <div
            key={key}
            className={`song-row with-actions ${playing ? 'on' : ''}`}
            onClick={() => onPlay(item, i)}
            title="点击播放"
          >
            {/* 纯展示：悬停显示 ▶，不用独立按钮避免与整行点击重复触发 */}
            <span className="idx idx-play" aria-hidden="true">
              <span className="idx-num">{i + 1}</span>
              {playing ? (
                <span className="playing-bars">
                  <i />
                  <i />
                  <i />
                </span>
              ) : (
                <svg className="idx-icon" width="14" height="14" viewBox="0 0 24 24" fill="currentColor">
                  <path d="M7 5l12 7-12 7z" />
                </svg>
              )}
            </span>

            <div className="cell">
              <div className="cv" style={{ background: coverColor(item.album || item.title) }} />
              <div className="meta">
                <div className="name">{item.title}</div>
                <div className="by">{item.artist}</div>
              </div>
            </div>

            <span className="col-album">{item.album}</span>
            <span className="col-time">{formatTime(item.duration)}</span>

            <span className="row-actions">
              <button
                className="row-action"
                title="下一首播放"
                aria-label={`下一首播放 ${item.title}`}
                onClick={(e) => {
                  e.stopPropagation()
                  onPlayNext?.(item)
                }}
              >
                <IconNext />
              </button>
              <button
                className="row-action"
                title="添加到队列"
                aria-label={`把 ${item.title} 添加到队列`}
                onClick={(e) => {
                  e.stopPropagation()
                  onAppend?.(item)
                }}
              >
                <IconPlus />
              </button>
            </span>
          </div>
        )
      })}
    </div>
  )
}
