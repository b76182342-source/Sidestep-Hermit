import { useCallback, useEffect, useRef, useState } from 'react'
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'

import Home from './pages/Home/Home'
import Albums from './pages/Albums/Albums'
import Artists from './pages/Artists/Artists'
import Daily from './pages/Daily/Daily'
import History from './pages/History/History'
import Settings from './pages/Settings/Settings'
import NowPlaying from './pages/NowPlaying/NowPlaying'

import Sidebar, { type SidebarKey } from './components/Sidebar'
import TopBar, { type Source } from './components/TopBar'
import PlayerBar from './components/PlayerBar'
import NowPlayingPanel from './components/NowPlayingPanel'
import QueueDrawer from './components/QueueDrawer/QueueDrawer'
import KugouLogin from './components/KugouLogin'

import { AppLayout } from './components/layout'
import { actions } from './actions'
import type { LyricLine, QueueEntry, SongItem } from './playback'
import { usePlayerStore } from './store/player'
import { useHistoryStore } from './store/history'
import { useKugouStore } from './store/kugou'

/** 展示右栏（正在播放 / 歌词）的视图 */
const PANEL_VIEWS = new Set<SidebarKey>(['songs', 'daily', 'history'])

export default function App() {
  const [source, setSource] = useState<Source>('local')
  const [view, setView] = useState<SidebarKey>('songs')
  const [query, setQuery] = useState('')
  const [showQueue, setShowQueue] = useState(false)
  const [showLogin, setShowLogin] = useState(false)
  const [fullPlayer, setFullPlayer] = useState(false)
  const [reloadToken, setReloadToken] = useState(0)

  const { isLoggedIn, userid, restore, setAccount, logout } = useKugouStore()
  const {
    current,
    error,
    applyPlayback,
    setState,
    setPosition,
    setDuration,
    setVolume,
    setLyrics,
    setError,
  } = usePlayerStore()
  const recordHistory = useHistoryStore((s) => s.record)

  // 启动时恢复登录态（凭据在系统密钥环）
  useEffect(() => {
    void restore()
  }, [restore])

  // 播放器事件
  useEffect(() => {
    const unlisten = listen('player-event', (event: any) => {
      const p = event.payload ?? {}
      switch (p.type) {
        case 'StateChanged':
          setState(p.state ?? 'Stopped')
          break
        case 'PositionChanged':
          if (p.position !== undefined) setPosition(p.position)
          if (p.duration !== undefined) setDuration(p.duration)
          break
        case 'VolumeChanged':
          if (p.volume !== undefined) setVolume(p.volume)
          break
      }
    })

    return () => {
      unlisten.then((fn) => fn())
    }
  }, [setState, setPosition, setDuration, setVolume])

  // 队列变化（换歌、增删、跳转都会广播）
  useEffect(() => {
    const unlisten = listen('playback-changed', (event: any) => {
      applyPlayback({
        items: (event.payload?.items ?? []) as QueueEntry[],
        index: event.payload?.index ?? null,
        current: (event.payload?.current ?? null) as QueueEntry | null,
      })
    })

    return () => {
      unlisten.then((fn) => fn())
    }
  }, [applyPlayback])

  // 首次挂载时同步一次队列（例如热重载后后端仍在播放）
  useEffect(() => {
    invoke<{ items: QueueEntry[]; index: number | null; current: QueueEntry | null }>(
      'get_current_queue'
    )
      .then((snap) =>
        applyPlayback({
          items: snap?.items ?? [],
          index: snap?.index ?? null,
          current: snap?.current ?? null,
        })
      )
      .catch(() => {
        /* 后端未就绪时忽略 */
      })
  }, [applyPlayback])

  // 换歌后记播放历史
  const currentProvider = current?.provider ?? null
  const currentId = current?.id ?? null

  useEffect(() => {
    if (current) recordHistory(current)
    // 只在真正换歌时记录
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [currentProvider, currentId])

  // 换歌后拉歌词。
  //
  // 注意：不要用 ref 去重 —— StrictMode 会「挂载→卸载→再挂载」，
  // 第一轮请求会被 cleanup 作废，而 ref 已记下 key 导致第二轮直接跳过，
  // 歌词就永远到不了。这里以 key 作为依赖，每轮都重新发起。
  useEffect(() => {
    if (!currentProvider || !currentId) {
      setLyrics([])
      return
    }

    if (currentProvider !== 'kugou') {
      setLyrics([])
      return
    }

    let alive = true
    setLyrics([])

    invoke<LyricLine[]>('kugou_lyrics', { hash: currentId })
      .then((lines) => {
        if (alive) setLyrics(lines ?? [])
      })
      .catch(() => {
        // 部分曲目没有歌词，属于常态
      })

    return () => {
      alive = false
    }
  }, [currentProvider, currentId, setLyrics])

  // 切音源时把视图落到该音源的默认页
  const changeSource = (next: Source) => {
    setSource(next)
    setQuery('')
    setView(next === 'online' ? 'daily' : 'songs')
  }

  const handleSelect = (key: SidebarKey) => {
    setView(key)
  }

  const scanLibrary = async () => {
    try {
      await invoke('scan_library')
      setReloadToken((n) => n + 1)
    } catch (e) {
      console.error('scan_library failed', e)
    }
  }

  // ── 播放操作 ──
  const playAll = useCallback((items: SongItem[], start: number) => {
    void actions.playAll(items, start)
  }, [])

  const playNext = useCallback((item: SongItem) => {
    void actions.playNext(item)
  }, [])

  const append = useCallback((item: SongItem) => {
    void actions.append(item)
  }, [])

  const shared = {
    activeKey: current ? `${current.provider}:${current.id}` : null,
    onPlay: playAll,
    onPlayNext: playNext,
    onAppend: append,
  }

  const renderMain = () => {
    switch (view) {
      case 'albums':
        return <Albums {...shared} reloadToken={reloadToken} />
      case 'artists':
        return <Artists {...shared} reloadToken={reloadToken} />
      case 'settings':
        return (
          <Settings
            isLoggedIn={isLoggedIn}
            userid={userid}
            onOpenLogin={() => setShowLogin(true)}
            onLogout={() => void logout()}
          />
        )
      case 'history':
        return <History {...shared} query={query} />
      case 'daily':
        return (
          <Daily
            {...shared}
            query={query}
            isLoggedIn={isLoggedIn}
            onOpenLogin={() => setShowLogin(true)}
          />
        )
      case 'liked':
      case 'commute':
      case 'night':
        return (
          <>
            <div className="page-head">
              <h1 className="page-title">
                {view === 'liked' ? '我喜欢' : view === 'commute' ? '通勤' : '深夜'}
              </h1>
              <span className="page-sub">酷狗云端歌单</span>
            </div>
            <div className="state">
              <div className="state-title">云端歌单还没接通</div>
              <p>歌单同步接口尚未接入，每日推荐与搜索已经可用</p>
            </div>
          </>
        )
      case 'songs':
      case 'scan':
      default:
        return (
          <Home
            {...shared}
            query={query}
            onScan={scanLibrary}
            reloadToken={reloadToken}
          />
        )
    }
  }

  // 右栏（正在播放 + 歌词）常驻在列表类页面上，没有曲目时显示占位
  const showPanel = PANEL_VIEWS.has(view)

  return (
    <>
      <AppLayout
        sidebar={
          <Sidebar
            source={source}
            current={view}
            onSelect={handleSelect}
            onScan={scanLibrary}
            isLoggedIn={isLoggedIn}
            userid={userid}
            onOpenLogin={() => setShowLogin(true)}
          />
        }
        panel={showPanel ? <NowPlayingPanel /> : undefined}
        player={
          <PlayerBar
            onOpenQueue={() => setShowQueue(true)}
            onOpenNowPlaying={() => setFullPlayer(true)}
          />
        }
      >
        <TopBar
          source={source}
          onSourceChange={changeSource}
          query={query}
          onQueryChange={setQuery}
          isLoggedIn={isLoggedIn}
          userid={userid}
          onOpenLogin={() => setShowLogin(true)}
        />

        {error && (
          <div className="notice">
            <span>{error}</span>
            <button className="chip" onClick={() => setError(null)}>
              知道了
            </button>
          </div>
        )}

        {renderMain()}
      </AppLayout>

      <QueueDrawer isOpen={showQueue} onClose={() => setShowQueue(false)} />

      {fullPlayer && current && <NowPlaying onClose={() => setFullPlayer(false)} />}

      {showLogin && (
        <KugouLogin
          onClose={() => setShowLogin(false)}
          onLoginSuccess={(uid) => {
            setAccount(uid)
            setShowLogin(false)
            setSource('online')
            setView('daily')
          }}
        />
      )}
    </>
  )
}
