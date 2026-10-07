/* 顶栏：搜索 + 音源切换 + 账号 —— 对照设计稿 */

export type Source = 'local' | 'online'

interface TopBarProps {
  source: Source
  onSourceChange: (source: Source) => void
  query: string
  onQueryChange: (q: string) => void
  isLoggedIn: boolean
  userid: string | null
  onOpenLogin: () => void
}

export default function TopBar({
  source,
  onSourceChange,
  query,
  onQueryChange,
  isLoggedIn,
  userid,
  onOpenLogin,
}: TopBarProps) {
  return (
    <div className="topbar">
      <label className="search-field">
        <svg
          width="18"
          height="18"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          strokeWidth="2"
          strokeLinecap="round"
        >
          <circle cx="11" cy="11" r="6" />
          <path d="M20 20l-4-4" />
        </svg>
        <input
          placeholder={source === 'online' ? '搜索酷狗曲库' : '搜索歌曲、专辑、艺术家'}
          value={query}
          onChange={(e) => onQueryChange(e.target.value)}
        />
      </label>

      <div className="segmented">
        <button
          type="button"
          className={source === 'local' ? 'on' : ''}
          onClick={() => onSourceChange('local')}
        >
          本地
        </button>
        <button
          type="button"
          className={source === 'online' ? 'on' : ''}
          onClick={() => onSourceChange('online')}
        >
          酷狗
        </button>
      </div>

      {isLoggedIn ? (
        <button type="button" className="account-pill" onClick={onOpenLogin} title="酷狗账号">
          <span className="account-avatar">{userid ? userid.slice(0, 1) : '已'}</span>
          <span>已登录{userid ? ` · ${userid}` : ''}</span>
        </button>
      ) : (
        <button type="button" className="btn-dark" onClick={onOpenLogin}>
          登录
        </button>
      )}
    </div>
  )
}
