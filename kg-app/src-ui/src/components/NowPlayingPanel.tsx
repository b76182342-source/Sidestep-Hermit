/* 右栏：正在播放 + 歌词 —— 对照设计稿 01-main 右侧栏 */
import { useMemo } from 'react'
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
      <div className="np-panel-art" style={{ background: coverColor(current.album || current.title) }} />

      <div className="np-panel-title">{current.title}</div>
      <div className="np-panel-sub">
        {current.artist}
        {current.album ? ` · ${current.album}` : ''}
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
