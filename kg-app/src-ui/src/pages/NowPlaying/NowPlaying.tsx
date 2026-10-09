/* 全屏正在播放 —— 对照设计稿 12-nowplaying */
import { useMemo, useEffect, useRef } from 'react'
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
  const lyricContainerRef = useRef<HTMLElement>(null)
  const activeLyricRef = useRef<HTMLDivElement>(null)

  const isPlaying = state === 'Playing'
  const playedRatio = duration > 0 ? Math.min(1, position / duration) : 0

  // 调试：打印当前歌曲数据和歌词
  useEffect(() => {
    if (current) {
      console.log('Current song:', current)
      console.log('Artwork URL:', current.artworkUrl)
      console.log('Lyrics count:', lyrics.length)
      if (lyrics.length > 0) {
        console.log('First lyric:', lyrics[0])
      }
    }
  }, [current, lyrics])

  const { dragRatio, handlers } = useSeekDrag(duration, (ratio) => {
    void actions.seekRatio(ratio)
  })

  const shownRatio = dragRatio ?? playedRatio
  const shownSeconds = dragRatio !== null ? dragRatio * duration : position

  const times = useMemo(() => lyrics.map((l) => l.timeMs), [lyrics])
  const active = activeLyricIndex(times, position)

  // 自动滚动到当前歌词 - 使用标准的 scrollIntoView API
  useEffect(() => {
    if (activeLyricRef.current) {
      activeLyricRef.current.scrollIntoView({
        behavior: 'smooth',
        block: 'center',      // 垂直方向：居中对齐
        inline: 'nearest'     // 水平方向：最近的可滚动容器
      })
    }
  }, [active])

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
        <div
          className="np-art"
          style={{
            background: current.artworkUrl
              ? `url(${current.artworkUrl}) center/cover no-repeat`
              : coverColor(current.album || current.title),
            display: 'flex',
            alignItems: 'center',
            justifyContent: 'center'
          }}
        >
          {!current.artworkUrl && (
            <img
              src="/mascots/mascot-typing.png"
              alt="Music"
              style={{
                width: '100%',
                height: '100%',
                objectFit: 'cover',
                imageRendering: 'pixelated',
                opacity: 0.9
              }}
            />
          )}
        </div>

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

      <section className="np-right" ref={lyricContainerRef}>
        {lyrics.length === 0 ? (
          <div className="np-lyric">暂无歌词</div>
        ) : (
          lyrics.map((line, i) => {
            const isActive = i === active
            // 计算逐字进度（仅当前行且有逐字数据时）
            let wordProgress = 0
            if (isActive && line.words && line.words.length > 0) {
              const lineStartMs = line.timeMs
              const currentMs = position * 1000
              const elapsedMs = currentMs - lineStartMs

              // 计算当前应该高亮到哪个字
              let accumulated = 0
              for (let j = 0; j < line.words.length; j++) {
                accumulated += line.words[j].durationMs
                if (elapsedMs < accumulated) {
                  wordProgress = j
                  break
                }
              }
              if (elapsedMs >= accumulated) {
                wordProgress = line.words.length
              }
            }

            return (
              <div
                key={`${line.timeMs}-${i}`}
                ref={isActive ? activeLyricRef : null}
                className={`np-lyric ${isActive ? 'on' : ''}`}
              >
                {line.words && line.words.length > 0 ? (
                  // 有逐字数据，渲染每个字
                  line.words.map((word, j) => (
                    <span
                      key={j}
                      className={isActive && j < wordProgress ? 'word-active' : ''}
                    >
                      {word.text}
                    </span>
                  ))
                ) : (
                  // 无逐字数据，显示整行
                  line.text
                )}
              </div>
            )
          })
        )}
      </section>
    </div>
  )
}
