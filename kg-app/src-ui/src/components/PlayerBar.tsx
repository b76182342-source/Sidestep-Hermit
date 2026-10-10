/* 播放控制栏 —— 对照设计稿页脚 */
import { actions } from '../actions'
import { usePlayerStore } from '../store/player'
import { useSeekDrag } from '../hooks/useSeekDrag'
import { coverColor, formatTime } from '../utils/format'
import { useVolumeDrag } from '../hooks/useVolumeDrag'

interface PlayerBarProps {
  onOpenQueue: () => void
  onOpenNowPlaying: () => void
}

const IconShuffle = () => (
  <svg
    width="20"
    height="20"
    viewBox="0 0 24 24"
    fill="none"
    stroke="currentColor"
    strokeWidth="2"
    strokeLinecap="round"
    strokeLinejoin="round"
  >
    <path d="M16 4h4v4M4 20L20 4M20 16v4h-4M4 4l5 5M15 15l5 5" />
  </svg>
)

const IconPrev = () => (
  <svg width="22" height="22" viewBox="0 0 24 24" fill="currentColor">
    <path d="M18 5v14l-9-7z" />
    <path d="M6 5v14" stroke="currentColor" strokeWidth="2" strokeLinecap="round" />
  </svg>
)

const IconPause = () => (
  <svg width="22" height="22" viewBox="0 0 24 24" fill="currentColor">
    <rect x="6" y="5" width="4" height="14" rx="1" />
    <rect x="14" y="5" width="4" height="14" rx="1" />
  </svg>
)

const IconPlay = () => (
  <svg width="22" height="22" viewBox="0 0 24 24" fill="currentColor">
    <path d="M7 5l12 7-12 7z" />
  </svg>
)

const IconNext = () => (
  <svg width="22" height="22" viewBox="0 0 24 24" fill="currentColor">
    <path d="M6 5v14l9-7z" />
    <path d="M18 5v14" stroke="currentColor" strokeWidth="2" strokeLinecap="round" />
  </svg>
)

const IconRepeat = () => (
  <svg
    width="20"
    height="20"
    viewBox="0 0 24 24"
    fill="none"
    stroke="currentColor"
    strokeWidth="2"
    strokeLinecap="round"
    strokeLinejoin="round"
  >
    <path d="M17 2l4 4-4 4M3 11V9a3 3 0 013-3h15M7 22l-4-4 4-4M21 13v2a3 3 0 01-3 3H3" />
  </svg>
)

const IconQueue = () => (
  <svg
    width="20"
    height="20"
    viewBox="0 0 24 24"
    fill="none"
    stroke="currentColor"
    strokeWidth="2"
    strokeLinecap="round"
  >
    <path d="M4 6h16M4 12h16M4 18h10" />
  </svg>
)

const IconVolume = () => (
  <svg
    width="20"
    height="20"
    viewBox="0 0 24 24"
    fill="none"
    stroke="currentColor"
    strokeWidth="2"
    strokeLinecap="round"
    strokeLinejoin="round"
  >
    <path d="M4 9v6h4l5 4V5L8 9z" />
    <path d="M16 9a4 4 0 010 6" />
  </svg>
)

export default function PlayerBar({ onOpenQueue, onOpenNowPlaying }: PlayerBarProps) {
  const { current, state, position, duration, volume: storeVolume } = usePlayerStore()

  const isPlaying = state === 'Playing'
  const playedRatio = duration > 0 ? Math.min(1, position / duration) : 0

  const { dragRatio, handlers } = useSeekDrag(duration, (ratio) => {
    void actions.seekRatio(ratio)
  })

  const shownRatio = dragRatio ?? playedRatio
  const shownSeconds = dragRatio !== null ? dragRatio * duration : position

  const volume = useVolumeDrag(storeVolume)

  const togglePlay = () => {
    void (isPlaying ? actions.pause() : actions.resume())
  }

  return (
    <div className="player-bar">
      {/* 左：当前曲目 */}
      <div className="pb-left">
        {current ? (
          <>
            <div className="pb-cover" onClick={onOpenNowPlaying} title="打开正在播放">
              <div
                style={{
                  width: '100%',
                  height: '100%',
                  background: `url(/mascots/mascot-cool-v2.png) center/contain no-repeat`,
                  imageRendering: 'pixelated'
                }}
              >
              </div>
            </div>
            <div className="pb-meta">
              <div className="pb-title">{current.title}</div>
              <div className="pb-artist">{current.artist}</div>
            </div>
          </>
        ) : (
          <div className="pb-empty">未在播放</div>
        )}
      </div>

      {/* 中：控制 + 进度 */}
      <div className="pb-center">
        <div className="pb-controls">
          <button className="ib muted" aria-label="随机播放" disabled>
            <IconShuffle />
          </button>
          <button
            className="ib"
            aria-label="上一首"
            onClick={() => void actions.previous().catch(() => {})}
          >
            <IconPrev />
          </button>
          <button
            className="pb-play"
            aria-label={isPlaying ? '暂停' : '播放'}
            onClick={togglePlay}
          >
            {isPlaying ? <IconPause /> : <IconPlay />}
          </button>
          <button
            className="ib"
            aria-label="下一首"
            onClick={() => void actions.next().catch(() => {})}
          >
            <IconNext />
          </button>
          <button className="ib muted" aria-label="循环模式" disabled>
            <IconRepeat />
          </button>
        </div>

        <div className="pb-progress">
          <span className="pb-time">{formatTime(shownSeconds)}</span>
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
          <span className="pb-time end">{formatTime(duration)}</span>
        </div>
      </div>

      {/* 右：队列 + 音量 */}
      <div className="pb-right">
        <button className="ib" aria-label="播放队列" onClick={onOpenQueue}>
          <IconQueue />
        </button>
        <span className="ib muted" aria-hidden="true" style={{ pointerEvents: 'none' }}>
          <IconVolume />
        </span>
        <div
          className="pb-volume"
          role="slider"
          aria-label="音量"
          aria-valuemin={0}
          aria-valuemax={100}
          aria-valuenow={Math.round(volume.value * 100)}
          tabIndex={0}
          {...volume.handlers}
        >
          <div className="pb-volume-fill" style={{ width: `${volume.value * 100}%` }} />
        </div>
      </div>
    </div>
  )
}
