/* 右栏：正在播放 + 歌词 —— 对照设计稿 01-main 右侧栏 */
import { useMemo, useEffect, useRef } from 'react'
import { usePlayerStore } from '../store/player'
import { coverColor } from '../utils/format'

/** 依据播放位置找出当前歌词行 */
export function activeLyricIndex(times: number[], positionSec: number): number {
  const ms = positionSec * 1000
  let idx = -1
  for (let i = 0; i < times.length; i += 1) {
    if (times[i] <= ms) idx = i
    else break
  }
  return idx
}

export default function NowPlayingPanel() {
  const { current, lyrics, position } = usePlayerStore()

  const times = useMemo(() => lyrics.map((l) => l.timeMs), [lyrics])
  const active = activeLyricIndex(times, position)
  const activeLineRef = useRef<HTMLDivElement>(null)

  // 核心：使用 scrollIntoView API 实现自动滚动
  // 当 active 变化时，活跃行自动滚动到视图中心
  useEffect(() => {
    if (activeLineRef.current) {
      activeLineRef.current.scrollIntoView({
        behavior: 'smooth',
        block: 'center',     // 垂直方向：居中对齐
        inline: 'nearest'    // 水平方向：最近的可滚动容器
      })
    }
  }, [active])

  // 调试日志 - 详细显示时间对齐情况
  useEffect(() => {
    if (lyrics.length > 0 && active >= 0) {
      const currentLyric = lyrics[active]
      const currentTime = times[active]
      const positionMs = position * 1000
      const timeDiff = positionMs - currentTime

      console.log('=== Lyric Sync Debug ===')
      console.log('Position:', position.toFixed(3), 's =', positionMs.toFixed(0), 'ms')
      console.log('Active index:', active, '/', lyrics.length)
      console.log('Current lyric:', currentLyric?.text)
      console.log('Lyric time:', currentTime, 'ms')
      console.log('Time difference:', timeDiff.toFixed(0), 'ms')

      if (active < lyrics.length - 1) {
        const nextLyric = lyrics[active + 1]
        const nextTime = times[active + 1]
        const timeToNext = nextTime - positionMs
        console.log('Next lyric:', nextLyric?.text)
        console.log('Next lyric time:', nextTime, 'ms')
        console.log('Time to next:', timeToNext.toFixed(0), 'ms')
      }
      console.log('========================')
    }
  }, [position, active, lyrics, times])

  if (!current) {
    return (
      <>
        <div className="np-panel-header">
          <div className="np-panel-art np-panel-art-empty" />
          <div className="np-panel-title">未在播放</div>
          <div className="np-panel-sub">双击列表里的任意一首开始播放，歌词会显示在这里</div>
        </div>
      </>
    )
  }

  return (
    <>
      {/* 上方区块：吉祥物 + 歌曲信息（固定） */}
      <div className="np-panel-header">
        <div
          className="np-panel-art"
          style={{
            background: coverColor(current.album || current.title),
            display: 'flex',
            alignItems: 'center',
            justifyContent: 'center'
          }}
        >
          <img
            src="/mascots/mascot-mustache.png"
            alt="Music"
            style={{
              width: '100%',
              height: '100%',
              objectFit: 'cover',
              imageRendering: 'pixelated',
              opacity: 0.8
            }}
          />
        </div>

        <div className="np-panel-title">{current.title}</div>
        <div className="np-panel-sub">
          {current.artist}
          {current.album ? ` · ${current.album}` : ''}
        </div>

        {/* 调试信息 */}
        <div style={{ fontSize: '12px', color: 'red', marginTop: '10px', fontWeight: 'bold' }}>
          DEBUG: pos={position.toFixed(2)}s active={active}/{lyrics.length}
        </div>
      </div>

      {/* 下方区块：歌词（可滚动） */}
      <div className="np-panel-lyrics-container">
        {lyrics.length === 0 ? (
          <div className="sub" style={{ marginTop: 20 }}>
            暂无歌词
          </div>
        ) : (
          <div className="np-lyrics">
            {lyrics.map((line, i) => (
              <div
                key={`${line.timeMs}-${i}`}
                className={`np-line ${i === active ? 'on' : ''}`}
                ref={i === active ? activeLineRef : null}
              >
                {line.text}
              </div>
            ))}
          </div>
        )}
      </div>
    </>
  )
}
