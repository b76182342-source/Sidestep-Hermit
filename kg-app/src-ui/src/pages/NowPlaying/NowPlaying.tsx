/* 全屏正在播放 —— 对照设计稿 12-nowplaying */
import { useMemo } from 'react'
import { actions } from '../../actions'
import { usePlayerStore } from '../../store/player'
import { useSeekDrag } from '../../hooks/useSeekDrag'
import { activeLyricIndex } from '../../components/NowPlayingPanel'
import { coverColor, formatTime } from '../../utils/format'

interface NowPlayingProps {
  onClose: () => void
}

export default function NowPlaying({ onClose }: NowPlayingProps) {
  const { current, state, position, duration, lyrics } = usePlayerStore()

  const isPlaying = state === 'Playing'
  const playedRatio = duration > 0 ? Math.min(1, position / duration) : 0

  const { dragRatio, handlers } = useSeekDrag(duration, (ratio) => {
    void actions.seekRatio(ratio)
  })

  const shownRatio = dragRatio ?? playedRatio
  const shownSeconds = dragRatio !== null ? dragRatio * duration : position

  const times = useMemo(() => lyrics.map((l) => l.timeMs), [lyrics])
  const active = activeLyricIndex(times, position)

  if (!current) return null

  return (
    <div className="now-playing">
      <button className="np-collapse" onClick={onClose}>
        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round">
          <path d="M6 9l6 6 6-6" />
        </svg>
        收起
      </button>

      <section className="np-left">
        <div className="np-art" style={{ background: coverColor(current.album || current.title) }} />

        <div className="np-title">{current.title}</div>
        <div className="np-sub">
          {current.artist}
          {current.album ? ` · ${current.album}` : ''}
        </div>

        <div className="pb-progress" style={{ marginTop: 28 }}>
          <span className="pb-time" style={{ width: 40 }}>
            {formatTime(shownSeconds)}
          </span>
          <div
            className={`track-line ${dragRatio !== null ? 'dragging' : ''}`}
            role="slider"
            aria-label="播放进度"
            aria-valuemin={0}
            aria-valuemax={Math.round(duration)}
            aria-valuenow={Math.round(shownSeconds)}
            tabIndex={0}
            {...handlers}
          >
            <div className="track-line-fill" style={{ width: `${shownRatio * 100}%` }} />
          </div>
          <span className="pb-time end" style={{ width: 40 }}>
            {formatTime(duration)}
          </span>
        </div>

        <div className="np-controls">
          <button
            className="ib"
            aria-label="上一首"
            onClick={() => void actions.previous().catch(() => {})}
          >
            <svg width="24" height="24" viewBox="0 0 24 24" fill="currentColor">
              <path d="M18 5v14l-9-7z" />
              <path d="M6 5v14" stroke="currentColor" strokeWidth="2" strokeLinecap="round" />
            </svg>
          </button>

          <button
            className="np-play"
            aria-label={isPlaying ? '暂停' : '播放'}
            onClick={() => void (isPlaying ? actions.pause() : actions.resume())}
          >
            {isPlaying ? (
              <svg width="24" height="24" viewBox="0 0 24 24" fill="currentColor">
                <path d="M7 5h3v14H7zM14 5h3v14h-3z" />
              </svg>
            ) : (
              <svg width="24" height="24" viewBox="0 0 24 24" fill="currentColor">
                <path d="M7 5l12 7-12 7z" />
              </svg>
            )}
          </button>

          <button
            className="ib"
            aria-label="下一首"
            onClick={() => void actions.next().catch(() => {})}
          >
            <svg width="24" height="24" viewBox="0 0 24 24" fill="currentColor">
              <path d="M6 5v14l9-7z" />
              <path d="M18 5v14" stroke="currentColor" strokeWidth="2" strokeLinecap="round" />
            </svg>
          </button>
        </div>
      </section>

      <section className="np-right">
        {lyrics.length === 0 ? (
          <div className="np-lyric">暂无歌词</div>
        ) : (
          lyrics.map((line, i) => (
            <div key={`${line.timeMs}-${i}`} className={`np-lyric ${i === active ? 'on' : ''}`}>
              {line.text}
            </div>
          ))
        )}
      </section>
    </div>
  )
}
