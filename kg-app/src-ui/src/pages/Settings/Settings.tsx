/* 设置 —— 对照设计稿 04-settings */
import { useState } from 'react'

const BANDS = ['31', '62', '125', '250', '500', '1k', '2k', '4k', '8k', '16k']
const PRESETS = ['平衡', '人声', '低音增强', '古典', '自定义']

interface SettingsProps {
  isLoggedIn?: boolean
  userid?: string | null
  onOpenLogin?: () => void
  onLogout?: () => void
}

export default function Settings({
  isLoggedIn = false,
  userid,
  onOpenLogin,
  onLogout,
}: SettingsProps) {
  const [eqOn, setEqOn] = useState(true)
  const [preset, setPreset] = useState('平衡')
  const [gains, setGains] = useState<number[]>(BANDS.map(() => 0.5))
  const [watch, setWatch] = useState(true)
  const [followSystem, setFollowSystem] = useState(true)

  const setGain = (i: number, v: number) => {
    setGains((prev) => prev.map((g, idx) => (idx === i ? v : g)))
    setPreset('自定义')
  }

  return (
    <>
      <h1 className="page-title" style={{ margin: '0 14px 24px' }}>
        设置
      </h1>

      <div className="card" style={{ padding: 24 }}>
        <div
          style={{
            display: 'flex',
            alignItems: 'center',
            justifyContent: 'space-between',
            marginBottom: 20,
          }}
        >
          <div style={{ display: 'flex', alignItems: 'center', gap: 16 }}>
            <h2 className="card-title" style={{ margin: 0, fontSize: 20 }}>
              均衡器
            </h2>
            <button
              className={`tg ${eqOn ? '' : 'off'}`}
              role="switch"
              aria-checked={eqOn}
              aria-label="启用均衡器"
              onClick={() => setEqOn((v) => !v)}
            >
              <i />
            </button>
          </div>

          <div className="chip-row">
            {PRESETS.map((p) => (
              <button
                key={p}
                className={`chip ${preset === p ? 'on' : ''}`}
                onClick={() => {
                  setPreset(p)
                  if (p === '平衡') setGains(BANDS.map(() => 0.5))
                }}
              >
                {p}
              </button>
            ))}
          </div>
        </div>

        <div className="eq">
          {BANDS.map((band, i) => (
            <div key={band} className="eq-band">
              <div className="eq-rail">
                <div
                  className="eq-knob"
                  style={{ top: `${(1 - gains[i]) * 150}px` }}
                  role="slider"
                  aria-label={`${band} Hz`}
                  aria-valuemin={-12}
                  aria-valuemax={12}
                  aria-valuenow={Math.round((gains[i] - 0.5) * 24)}
                  onPointerDown={(e) => {
                    const rail = e.currentTarget.parentElement
                    if (!rail) return
                    const move = (ev: PointerEvent) => {
                      const rect = rail.getBoundingClientRect()
                      const ratio = 1 - (ev.clientY - rect.top) / rect.height
                      setGain(i, Math.min(1, Math.max(0, ratio)))
                    }
                    const up = () => {
                      window.removeEventListener('pointermove', move)
                      window.removeEventListener('pointerup', up)
                    }
                    window.addEventListener('pointermove', move)
                    window.addEventListener('pointerup', up)
                  }}
                />
              </div>
              <span className="sub">{band}</span>
            </div>
          ))}
        </div>
      </div>

      <div className="card-grid-3">
        <div className="card">
          <h3 className="card-title">音乐库</h3>
          <div className="set">
            <span>目录</span>
            <span className="sub">~/音乐</span>
          </div>
          <div className="set">
            <span>自动监听变化</span>
            <button
              className={`tg ${watch ? '' : 'off'}`}
              role="switch"
              aria-checked={watch}
              aria-label="自动监听变化"
              onClick={() => setWatch((v) => !v)}
            >
              <i />
            </button>
          </div>
        </div>

        <div className="card">
          <h3 className="card-title">外观</h3>
          <div className="set">
            <span>跟随系统深浅色</span>
            <button
              className={`tg ${followSystem ? '' : 'off'}`}
              role="switch"
              aria-checked={followSystem}
              aria-label="跟随系统深浅色"
              onClick={() => setFollowSystem((v) => !v)}
            >
              <i />
            </button>
          </div>
          <div className="set">
            <span>语言</span>
            <span className="sub">简体中文</span>
          </div>
        </div>

        <div className="card">
          <h3 className="card-title">酷狗账号</h3>
          <div className="set">
            <span>状态</span>
            <span className="sub">{isLoggedIn ? '已登录' : '未登录'}</span>
          </div>
          <div className="set">
            <span>登录信息</span>
            {isLoggedIn ? (
              <button className="chip" onClick={onLogout}>
                退出登录
              </button>
            ) : (
              <button className="chip" onClick={onOpenLogin}>
                登录
              </button>
            )}
          </div>
        </div>
      </div>
    </>
  )
}
