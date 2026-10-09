/* 右栏：正在播放 + 歌词 —— 对照设计稿 01-main 右侧栏 */
import { useMemo, useEffect } from 'react'
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

  // 调试日志
  useEffect(() => {
    console.log('[NowPlayingPanel] position:', position, 'active index:', active, 'total lyrics:', lyrics.length)
    if (lyrics.length > 0) {
      console.log('[NowPlayingPanel] First 3 lyric times:', times.slice(0, 3))
      console.log('[NowPlayingPanel] position in ms:', position * 1000)
    }
  }, [position, active, lyrics.length, times])

  if (!current) {
    return (
      <>
        <div className="np-panel-art np-panel-art-empty" />
        <div className="np-panel-title">未在播放</div>
        <div className="np-panel-sub">双击列表里的任意一首开始播放，歌词会显示在这里</div>
      </>
    )
  }

  return (
    <>
      <div
        className="np-panel-art"
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
        )}
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

      {lyrics.length === 0 ? (
        <div className="sub" style={{ marginTop: 20 }}>
          暂无歌词
        </div>
      ) : (
        <div className="np-lyrics">
          {lyrics.map((line, i) => (
            <div key={`${line.timeMs}-${i}`} className={`np-line ${i === active ? 'on' : ''}`}>
              {line.text}
            </div>
          ))}
        </div>
      )}
    </>
  )
}
