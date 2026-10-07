/* 侧边栏 —— 导航项随音源切换（本地 / 酷狗），对照设计稿 01 与 03 */

import type { Source } from './TopBar'

export type SidebarKey =
  // 本地
  | 'songs'
  | 'albums'
  | 'artists'
  | 'scan'
  // 酷狗
  | 'daily'
  | 'history'
  // 通用
  | 'liked'
  | 'commute'
  | 'night'
  | 'settings'

interface SidebarProps {
  source: Source
  current: SidebarKey
  onSelect: (key: SidebarKey) => void
  onScan?: () => void
  isLoggedIn?: boolean
  userid?: string | null
  onOpenLogin?: () => void
}

const LOCAL_PRIMARY: Array<{ key: SidebarKey; label: string }> = [
  { key: 'songs', label: '全部歌曲' },
  { key: 'albums', label: '专辑' },
  { key: 'artists', label: '艺术家' },
]

const ONLINE_PRIMARY: Array<{ key: SidebarKey; label: string }> = [
  { key: 'daily', label: '每日推荐' },
  { key: 'history', label: '播放历史' },
]

const PLAYLISTS: Array<{ key: SidebarKey; label: string }> = [
  { key: 'liked', label: '我喜欢' },
  { key: 'commute', label: '通勤' },
  { key: 'night', label: '深夜' },
]

export default function Sidebar({
  source,
  current,
  onSelect,
  onScan,
  isLoggedIn = false,
  userid,
  onOpenLogin,
}: SidebarProps) {
  const primary = source === 'online' ? ONLINE_PRIMARY : LOCAL_PRIMARY

  const renderNav = (key: SidebarKey, label: string) => (
    <button
      key={key}
      type="button"
      className={`nav ${current === key ? 'on' : ''}`}
      onClick={() => (key === 'scan' ? onScan?.() : onSelect(key))}
    >
      {label}
    </button>
  )

  return (
    <aside className="app-sidebar">
      <div className="sidebar-logo">KGPlayer</div>

      {primary.map((item) => renderNav(item.key, item.label))}

      <div className="nav-section">{source === 'online' ? '云端歌单' : '歌单'}</div>
      {PLAYLISTS.map((item) => renderNav(item.key, item.label))}

      <div className="sidebar-spacer" />

      {isLoggedIn ? (
        <button type="button" className="account-chip" onClick={() => onSelect('settings')}>
          <span className="account-avatar">{userid ? userid.slice(0, 1) : '用'}</span>
          <span className="account-name">{userid ? `酷狗 ${userid}` : '酷狗账号'}</span>
        </button>
      ) : (
        <button type="button" className="account-chip" onClick={onOpenLogin}>
          <span className="account-avatar muted">+</span>
          <span className="account-name">登录酷狗</span>
        </button>
      )}

      {source === 'local' && renderNav('scan', '扫描音乐库')}
      {renderNav('settings', '设置')}
    </aside>
  )
}
