/* 账号管理 —— 显示所有音乐平台的登录状态 */
import { useEffect, useState } from 'react'
import { invoke } from '@tauri-apps/api/core'

interface ProviderInfo {
  id: string
  name: string
  supports_auth: boolean
  is_online: boolean
}

interface AccountStatus {
  provider: string
  logged_in: boolean
  user_id?: string
  display_name?: string
}

interface AccountManagementProps {
  onOpenKugouLogin?: () => void
}

export default function AccountManagement({ onOpenKugouLogin }: AccountManagementProps) {
  const [providers, setProviders] = useState<ProviderInfo[]>([])
  const [statuses, setStatuses] = useState<AccountStatus[]>([])
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)

  // 加载 Provider 列表和账号状态
  useEffect(() => {
    let alive = true
    setLoading(true)
    setError(null)

    Promise.all([
      invoke<ProviderInfo[]>('list_providers'),
      invoke<AccountStatus[]>('account_statuses'),
    ])
      .then(([providerList, statusList]) => {
        if (alive) {
          setProviders(providerList)
          setStatuses(statusList)
        }
      })
      .catch((err) => {
        if (alive) {
          setError(String(err))
        }
      })
      .finally(() => {
        if (alive) {
          setLoading(false)
        }
      })

    return () => {
      alive = false
    }
  }, [])

  // 根据 provider ID 查找账号状态
  const getStatus = (providerId: string): AccountStatus | undefined => {
    return statuses.find((s) => s.provider === providerId)
  }

  // 处理登录
  const handleLogin = (providerId: string) => {
    if (providerId === 'kugou') {
      onOpenKugouLogin?.()
    } else {
      // TODO: 处理其他平台的登录
      console.log(`登录 ${providerId}`)
    }
  }

  // 处理登出
  const handleLogout = async (providerId: string) => {
    try {
      if (providerId === 'kugou') {
        await invoke('kugou_logout')
        // 刷新状态
        const statusList = await invoke<AccountStatus[]>('account_statuses')
        setStatuses(statusList)
      } else {
        // TODO: 处理其他平台的登出
        console.log(`登出 ${providerId}`)
      }
    } catch (err) {
      setError(String(err))
    }
  }

  if (loading) {
    return (
      <div className="state">
        <div className="spinner" />
      </div>
    )
  }

  if (error) {
    return (
      <div className="state">
        <div className="state-title">加载失败</div>
        <p>{error}</p>
      </div>
    )
  }

  return (
    <>
      <h1 className="page-title" style={{ margin: '0 14px 24px' }}>
        账号管理
      </h1>

      <div className="card-grid-3">
        {providers.map((provider) => {
          const status = getStatus(provider.id)
          const isLoggedIn = status?.logged_in ?? false
          const displayName = status?.display_name || status?.user_id

          return (
            <div key={provider.id} className="card">
              <h3 className="card-title">{provider.name}</h3>

              <div className="set">
                <span>平台类型</span>
                <span className="sub">{provider.is_online ? '在线平台' : '本地平台'}</span>
              </div>

              {provider.supports_auth && (
                <>
                  <div className="set">
                    <span>登录状态</span>
                    <span className="sub">{isLoggedIn ? '已登录' : '未登录'}</span>
                  </div>

                  {isLoggedIn && displayName && (
                    <div className="set">
                      <span>账号信息</span>
                      <span className="sub">{displayName}</span>
                    </div>
                  )}

                  <div className="set">
                    <span>操作</span>
                    {isLoggedIn ? (
                      <button
                        className="chip"
                        onClick={() => void handleLogout(provider.id)}
                      >
                        退出登录
                      </button>
                    ) : (
                      <button
                        className="chip"
                        onClick={() => handleLogin(provider.id)}
                      >
                        登录
                      </button>
                    )}
                  </div>
                </>
              )}

              {!provider.supports_auth && (
                <div className="set">
                  <span className="sub">本地平台无需登录</span>
                </div>
              )}
            </div>
          )
        })}
      </div>

      {providers.length === 0 && (
        <div className="state">
          <div className="state-title">没有可用的平台</div>
          <p>系统尚未配置任何音乐平台</p>
        </div>
      )}
    </>
  )
}
