/* 应用骨架：侧边栏 / 主区 / 右栏 / 播放器 */
import type { ReactNode } from 'react'

export interface AppLayoutProps {
  sidebar: ReactNode
  panel?: ReactNode
  player?: ReactNode
  children: ReactNode
}

export function AppLayout({ sidebar, panel, player, children }: AppLayoutProps) {
  return (
    <div className={`app-layout ${panel ? 'has-panel' : ''}`}>
      {sidebar}

      <main className="app-main">
        <div className="app-content scroll-flush">{children}</div>
      </main>

      {panel && <section className="app-panel">{panel}</section>}

      {player && <footer className="app-player">{player}</footer>}
    </div>
  )
}

export default AppLayout
