/* 播放队列抽屉 —— 对照设计稿 14-queue */
import type { QueueEntry } from '../../playback'
import { actions } from '../../actions'
import { usePlayerStore } from '../../store/player'
import { coverColor, formatTime } from '../../utils/format'

interface QueueDrawerProps {
  isOpen: boolean
  onClose: () => void
}

function QueueRow({
  entry,
  index,
  playing,
  onJump,
  onRemove,
}: {
  entry: QueueEntry
  index: number
  playing?: boolean
  onJump?: () => void
  onRemove?: () => void
}) {
  return (
    <div
      className={`song-row queue-row ${playing ? 'on' : ''}`}
      onDoubleClick={onJump}
      title={onJump ? '双击播放这首' : undefined}
    >
      <span className="idx">
        {playing ? (
          <span className="playing-bars" aria-label="播放中">
            <i />
            <i />
            <i />
          </span>
        ) : (
          '⋮⋮'
        )}
      </span>

      <div className="cell">
        <div className="cv" style={{ background: coverColor(entry.album || entry.title) }} />
        <div className="meta">
          <div className="name">
            {entry.title}
            {entry.inserted && !playing && <span className="badge-insert">插队</span>}
          </div>
          <div className="by">{entry.artist}</div>
        </div>
      </div>

      <span className="col-time">{formatTime(entry.duration)}</span>

      {onRemove ? (
        <button
          className="row-action"
          aria-label={`把 ${entry.title} 移出队列`}
          onClick={(e) => {
            e.stopPropagation()
            onRemove()
          }}
          onDoubleClick={(e) => e.stopPropagation()}
        >
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round">
            <path d="M6 6l12 12M18 6L6 18" />
          </svg>
        </button>
      ) : (
        <span aria-hidden="true" />
      )}
    </div>
  )
}

export default function QueueDrawer({ isOpen, onClose }: QueueDrawerProps) {
  const { items, index, current } = usePlayerStore()

  if (!isOpen) return null

  const upcoming = index === null ? items : items.slice(index + 1)

  const jump = (absoluteIndex: number) => {
    void actions.jump(absoluteIndex).catch(() => {})
  }

  const remove = (absoluteIndex: number) => {
    void actions.remove(absoluteIndex).catch(() => {})
  }

  return (
    <>
      <div className="drawer-scrim" onClick={onClose} />
      <aside className="drawer">
        <div className="drawer-head">
          <div>
            <div className="drawer-title">播放队列</div>
            <div className="sub">共 {items.length} 首</div>
          </div>
          <div style={{ display: 'flex', alignItems: 'center', gap: 8 }}>
            <button
              className="btn-ghost"
              disabled={items.length === 0}
              onClick={() => void actions.clear().catch(() => {})}
            >
              清空
            </button>
            <button className="ib" aria-label="关闭" onClick={onClose}>
              <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round">
                <path d="M6 6l12 12M18 6L6 18" />
              </svg>
            </button>
          </div>
        </div>

        <div className="drawer-list">
          {items.length === 0 ? (
            <div className="sub" style={{ padding: '12px 14px' }}>
              队列为空。在歌曲行右侧点「+」可以把歌加进来。
            </div>
          ) : (
            <>
              {current && index !== null && (
                <>
                  <div className="nav-section">正在播放</div>
                  <QueueRow entry={current} index={index} playing />
                </>
              )}

              <div className="nav-section">
                接下来
                {upcoming.length > 0 && ` (${upcoming.length})`}
              </div>

              {upcoming.length === 0 ? (
                <div className="sub" style={{ padding: '0 14px' }}>
                  已经是队列里的最后一首
                </div>
              ) : (
                upcoming.map((entry, i) => {
                  const absolute = (index ?? -1) + 1 + i
                  return (
                    <QueueRow
                      key={`${entry.provider}:${entry.id}-${absolute}`}
                      entry={entry}
                      index={absolute}
                      onJump={() => jump(absolute)}
                      onRemove={() => remove(absolute)}
                    />
                  )
                })
              )}
            </>
          )}
        </div>
      </aside>
    </>
  )
}
