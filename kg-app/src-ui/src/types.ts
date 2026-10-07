/* 类型定义扩展 */

// 扩展现有的 Track 类型
export interface Track {
  id: {
    provider: string
    id: string
  }
  title: string
  artist: string
  album: string
  duration: number
  file_path: string
  cover_art?: string
  year?: number
}

// 新增 Album 类型
export interface Album {
  id: string
  name: string
  artist: string
  cover_art?: string
  year?: number
  tracks?: Track[]
  track_count?: number
}

// 新增 Artist 类型
export interface Artist {
  id: string
  name: string
  image?: string
  album_count?: number
  track_count?: number
  bio?: string
}

// 新增 Playlist 类型
export interface Playlist {
  id: string
  name: string
  description?: string
  cover_art?: string
  tracks: Track[]
  track_count?: number
  created_at: string
  updated_at: string
}

// 播放器事件类型
export interface PlayerEvent {
  type: 'StateChanged' | 'PositionUpdated' | 'DurationChanged' | 'VolumeChanged'
  state?: 'Playing' | 'Paused' | 'Stopped'
  position?: number
  duration?: number
  volume?: number
}

// 歌词行类型
export interface LyricLine {
  time: number
  text: string
  duration?: number
}

// 搜索结果类型
export interface SearchResult {
  tracks: Track[]
  albums: Album[]
  artists: Artist[]
  playlists: Playlist[]
}
