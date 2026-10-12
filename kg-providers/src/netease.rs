//! 网易云音乐 Provider
//!
//! **重要提示**：
//! - 本实现基于非官方 API（参考 Binaryify/NeteaseCloudMusicApi）
//! - 仅供个人学习研究使用
//! - 不得用于商业用途
//! - 请尊重版权方和服务提供商的权益
//! - 严格遵守 CLAUDE.md 合规红线：
//!   1. 只播放用户有权播放的内容
//!   2. 不绕过 VIP/付费限制
//!   3. 不破解 DRM
//!   4. 凭据只存系统密钥环

use async_trait::async_trait;
use kg_core::{
    Error, Lyrics, MusicProvider, Page, Paged, Result, SearchItem, SearchKind, StreamSource,
    Track, TrackId,
};
use std::sync::Arc;
use tokio::sync::Mutex;

/// 网易云音乐 Provider
pub struct NeteaseProvider {
    api: Arc<Mutex<NeteaseApi>>,
}

impl NeteaseProvider {
    /// 创建新实例
    pub fn new() -> Self {
        Self {
            api: Arc::new(Mutex::new(NeteaseApi::new())),
        }
    }

    /// 设置认证信息（登录后调用）
    pub async fn set_auth(&self, cookies: String) {
        let mut api = self.api.lock().await;
        api.set_cookies(cookies);
    }
}

impl Default for NeteaseProvider {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl MusicProvider for NeteaseProvider {
    fn id(&self) -> &'static str {
        "netease"
    }

    fn name(&self) -> &'static str {
        "网易云音乐"
    }

    async fn search(&self, query: &str, _kind: SearchKind, page: Page) -> Result<Paged<SearchItem>> {
        let api = self.api.lock().await;
        let songs = api.search_songs(query, page.number, page.size).await?;

        let total = songs.len() as u32;
        let items: Vec<SearchItem> = songs
            .into_iter()
            .map(|song| {
                SearchItem::Track(Track {
                    id: TrackId::new("netease", &song.id.to_string()),
                    title: song.name,
                    artist: song
                        .artists
                        .iter()
                        .map(|a| a.name.as_str())
                        .collect::<Vec<_>>()
                        .join(" / "),
                    album: song.album.name.clone(),
                    duration_secs: song.duration / 1000,
                    album_art_url: song.album.pic_url.clone(),
                    year: None,
                    track_number: None,
                    genre: None,
                })
            })
            .collect();

        Ok(Paged { items, total, page })
    }

    async fn track(&self, id: &TrackId) -> Result<Track> {
        // TODO: 实现歌曲详情获取
        Err(Error::NotFound(format!(
            "网易云 track() 尚未实现: {}",
            id.id
        )))
    }

    async fn stream_source(&self, id: &TrackId) -> Result<StreamSource> {
        let api = self.api.lock().await;
        let url_info = api.get_song_url(&id.id).await?;

        // 合规检查：只返回用户有权播放的 URL
        if url_info.url.is_empty() {
            return Err(Error::PermissionDenied("该曲目需要 VIP 权限或不可用".into()));
        }

        Ok(StreamSource::Url(url_info.url))
    }

    async fn lyrics(&self, id: &TrackId) -> Result<Option<Lyrics>> {
        let api = self.api.lock().await;
        api.get_lyrics(&id.id).await
    }
}

/// 网易云 API 客户端（内部实现）
struct NeteaseApi {
    client: reqwest::Client,
    cookies: Option<String>,
}

impl NeteaseApi {
    fn new() -> Self {
        Self {
            client: reqwest::Client::builder()
                .user_agent("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36")
                .build()
                .unwrap(),
            cookies: None,
        }
    }

    fn set_cookies(&mut self, cookies: String) {
        self.cookies = Some(cookies);
    }

    /// 搜索歌曲
    async fn search_songs(
        &self,
        _keyword: &str,
        _page: u32,
        _limit: u32,
    ) -> Result<Vec<NeteaseSong>> {
        // TODO: 实现搜索 API 调用
        // POST /api/cloudsearch/get/web
        Ok(Vec::new())
    }

    /// 获取播放地址
    async fn get_song_url(&self, _id: &str) -> Result<NeteaseSongUrl> {
        // TODO: 实现播放地址获取
        // POST /api/song/enhance/player/url/v1
        Ok(NeteaseSongUrl {
            url: String::new(),
        })
    }

    /// 获取歌词
    async fn get_lyrics(&self, _id: &str) -> Result<Option<Lyrics>> {
        // TODO: 实现歌词获取
        // GET /api/song/lyric
        Ok(None)
    }
}

/// 网易云歌曲信息
#[derive(Debug, Clone)]
struct NeteaseSong {
    id: u64,
    name: String,
    artists: Vec<NeteaseArtist>,
    album: NeteaseAlbum,
    duration: u64, // 毫秒
}

#[derive(Debug, Clone)]
struct NeteaseArtist {
    name: String,
}

#[derive(Debug, Clone)]
struct NeteaseAlbum {
    name: String,
    pic_url: Option<String>,
}

/// 网易云播放地址信息
#[derive(Debug, Clone)]
struct NeteaseSongUrl {
    url: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_provider_info() {
        let provider = NeteaseProvider::new();
        assert_eq!(provider.id(), "netease");
        assert_eq!(provider.name(), "网易云音乐");
    }
}
