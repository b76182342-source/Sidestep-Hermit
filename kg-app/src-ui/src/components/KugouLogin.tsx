/* 登录酷狗 —— 对照设计稿 05-login */
import { useEffect, useRef, useState } from 'react'
import { invoke } from '@tauri-apps/api/core'
import QRCode from 'qrcode'

interface KugouLoginProps {
  /** 后端不返回 token；只回传账号标识 */
  onLoginSuccess: (userid: string) => void
  onClose: () => void
}

type Status = 'loading' | 'ready' | 'checking' | 'success' | 'error'

const POLL_MS = 2000
const EXPIRE_MS = 120000

export default function KugouLogin({ onLoginSuccess, onClose }: KugouLoginProps) {
  const [qrImage, setQrImage] = useState('')
  const [status, setStatus] = useState<Status>('loading')
  const [errorMsg, setErrorMsg] = useState('')

  const timerRef = useRef<number | null>(null)
  const expireRef = useRef<number | null>(null)
  const doneRef = useRef(false)

  const stopTimers = () => {
    if (timerRef.current !== null) window.clearInterval(timerRef.current)
    if (expireRef.current !== null) window.clearTimeout(expireRef.current)
    timerRef.current = null
    expireRef.current = null
  }

  useEffect(() => {
    void getQrCode()
    return stopTimers
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [])

  const getQrCode = async () => {
    stopTimers()
    doneRef.current = false
    setStatus('loading')
    setErrorMsg('')

    try {
      // 后端返回扫码登录 URL；二维码图片由前端渲染
      const url = await invoke<string>('kugou_get_qr_code')
      const dataUrl = await QRCode.toDataURL(url, { width: 200, margin: 1 })
      setQrImage(dataUrl)
      setStatus('ready')
      startPolling()
    } catch (err) {
      setErrorMsg(String(err))
      setStatus('error')
    }
  }

  const startPolling = () => {
    timerRef.current = window.setInterval(async () => {
      if (doneRef.current) return
      try {
        const result = await invoke<{ success: boolean; userid?: string }>('kugou_check_login')
        if (result.success) {
          doneRef.current = true
          stopTimers()
          setStatus('success')
          window.setTimeout(() => onLoginSuccess(result.userid ?? ''), 800)
        }
      } catch {
        /* 单次轮询失败不致命，继续 */
      }
    }, POLL_MS)

    expireRef.current = window.setTimeout(() => {
      if (doneRef.current) return
      stopTimers()
      setErrorMsg('二维码已过期，请刷新重试')
      setStatus('error')
    }, EXPIRE_MS)
  }

  return (
    <div className="modal-scrim" onClick={onClose}>
      <div className="login-card" onClick={(e) => e.stopPropagation()}>
        <h1 className="login-title">登录酷狗</h1>
        <p className="login-sub">
          登录后可以使用每日推荐和云端歌单，不登录也能播放本地音乐。
        </p>

        <div className="login-grid">
          {/* 手机号 + 验证码 */}
          <div>
            <label className="lb">
              手机号
              <input className="in" style={{ marginTop: 8 }} placeholder="请输入手机号" inputMode="tel" />
            </label>

            <div style={{ marginTop: 20 }}>
              <label className="lb" htmlFor="kg-code">
                验证码
              </label>
              <div style={{ display: 'flex', gap: 10 }}>
                <input id="kg-code" className="in" placeholder="6 位验证码" inputMode="numeric" />
                <button className="chip" style={{ height: 44, borderRadius: 12 }}>
                  获取验证码
                </button>
              </div>
            </div>

            <button className="login-submit" type="button">
              登录
            </button>

            <button className="login-skip" type="button" onClick={onClose}>
              暂不登录，只用本地音乐
            </button>
          </div>

          <div className="login-divider" />

          {/* 扫码登录 */}
          <div style={{ display: 'flex', flexDirection: 'column' }}>
            <div className="sub" style={{ marginBottom: 8 }}>
              扫码登录
            </div>

            <div className="qr-box">
              {status === 'loading' && <span>二维码加载中…</span>}

              {(status === 'ready' || status === 'checking') && qrImage && (
                <img src={qrImage} alt="酷狗登录二维码" />
              )}

              {status === 'success' && <span style={{ color: 'var(--accent-btn)' }}>登录成功</span>}

              {status === 'error' && <span>{errorMsg || '二维码获取失败'}</span>}
            </div>

            <div className="sub" style={{ marginTop: 12 }}>
              使用酷狗手机端扫码
            </div>

            {status === 'checking' && (
              <div className="sub" style={{ marginTop: 8 }}>
                检测登录状态中…
              </div>
            )}

            {status === 'error' && (
              <button
                className="chip"
                style={{ marginTop: 12, height: 36 }}
                onClick={() => void getQrCode()}
              >
                重新获取二维码
              </button>
            )}
          </div>
        </div>

        <p className="login-note">
          凭据只保存在系统密钥环中，不会写入日志。
          <br />
          本应用为非官方客户端，与酷狗无隶属关系。
        </p>
      </div>
    </div>
  )
}
