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
    Error, Lyrics, MusicProvider, Page, Paged, Result, SearchItem, SearchKind, StreamSource, Track,
    TrackId,
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

    async fn search(
        &self,
        query: &str,
        _kind: SearchKind,
        page: Page,
    ) -> Result<Paged<SearchItem>> {
        let api = self.api.lock().await;
        let songs = api.search_songs(query, page.number, page.size).await?;

        let total = songs.len() as u32;
        let items: Vec<SearchItem> = songs
            .into_iter()
            .map(|song| {
                SearchItem::Track(Track {
                    id: TrackId::new("netease", song.id.to_string()),
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
            return Err(Error::PermissionDenied(
                "该曲目需要 VIP 权限或不可用".into(),
            ));
        }

        Ok(StreamSource::Url(url_info.url))
    }

    async fn lyrics(&self, id: &TrackId) -> Result<Option<Lyrics>> {
        let api = self.api.lock().await;
        api.get_lyrics(&id.id).await
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
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
                .expect("Failed to build HTTP client"),
            cookies: None,
        }
    }

    fn set_cookies(&mut self, cookies: String) {
        self.cookies = Some(cookies);
    }

    /// 搜索歌曲
    async fn search_songs(&self, keyword: &str, page: u32, limit: u32) -> Result<Vec<NeteaseSong>> {
        // 网易云音乐搜索 API
        let url = "https://music.163.com/weapi/search/get";
        let offset_str = ((page - 1) * limit).to_string();
        let limit_str = limit.to_string();

        let mut params = std::collections::HashMap::new();
        params.insert("s", keyword);
        params.insert("type", "1"); // 1 = 单曲
        params.insert("offset", offset_str.as_str());
        params.insert("limit", limit_str.as_str());

        let mut request = self
            .client
            .post(url)
            .header("Referer", "https://music.163.com");

        if let Some(cookies) = &self.cookies {
            request = request.header("Cookie", cookies);
        }

        // 发送表单数据
        let response = request
            .form(&params)
            .send()
            .await
            .map_err(|e| Error::Network(format!("搜索请求失败: {}", e)))?;

        if !response.status().is_success() {
            return Err(Error::Network(format!(
                "搜索请求返回错误状态: {}",
                response.status()
            )));
        }

        let text = response
            .text()
            .await
            .map_err(|e| Error::Network(format!("读取搜索响应失败: {}", e)))?;

        // 解析 JSON 响应
        let json: serde_json::Value = serde_json::from_str(&text)
            .map_err(|e| Error::Parse(format!("解析搜索响应失败: {}", e)))?;

        // 检查响应码
        let code = json
            .get("code")
            .and_then(|v| v.as_i64())
            .ok_or_else(|| Error::Parse("响应中缺少 code 字段".into()))?;

        if code != 200 {
            return Err(Error::Network(format!("搜索失败，错误码: {}", code)));
        }

        // 解析歌曲列表
        let songs_array = json
            .get("result")
            .and_then(|r| r.get("songs"))
            .and_then(|s| s.as_array())
            .ok_or_else(|| Error::Parse("响应中缺少歌曲列表".into()))?;

        let mut songs = Vec::new();
        for song_value in songs_array {
            if let Ok(song) = self.parse_song(song_value) {
                songs.push(song);
            }
        }

        Ok(songs)
    }

    /// 解析单个歌曲 JSON
    fn parse_song(&self, value: &serde_json::Value) -> Result<NeteaseSong> {
        let id = value
            .get("id")
            .and_then(|v| v.as_u64())
            .ok_or_else(|| Error::Parse("歌曲缺少 id".into()))?;

        let name = value
            .get("name")
            .and_then(|v| v.as_str())
            .ok_or_else(|| Error::Parse("歌曲缺少 name".into()))?
            .to_string();

        // 解析艺术家列表
        let artists_array = value
            .get("artists")
            .or_else(|| value.get("ar"))
            .and_then(|a| a.as_array())
            .ok_or_else(|| Error::Parse("歌曲缺少 artists".into()))?;

        let artists: Vec<NeteaseArtist> = artists_array
            .iter()
            .filter_map(|a| {
                a.get("name")
                    .and_then(|n| n.as_str())
                    .map(|n| NeteaseArtist {
                        name: n.to_string(),
                    })
            })
            .collect();

        // 解析专辑信息
        let album_obj = value
            .get("album")
            .or_else(|| value.get("al"))
            .ok_or_else(|| Error::Parse("歌曲缺少 album".into()))?;

        let album_name = album_obj
            .get("name")
            .and_then(|v| v.as_str())
            .unwrap_or("Unknown Album")
            .to_string();

        let pic_url = album_obj
            .get("picUrl")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        let duration = value
            .get("duration")
            .or_else(|| value.get("dt"))
            .and_then(|v| v.as_u64())
            .unwrap_or(0);

        Ok(NeteaseSong {
            id,
            name,
            artists,
            album: NeteaseAlbum {
                name: album_name,
                pic_url,
            },
            duration,
        })
    }

    /// 获取播放地址
    async fn get_song_url(&self, id: &str) -> Result<NeteaseSongUrl> {
        // 使用 /api/song/enhance/player/url 接口
        let url = "https://music.163.com/api/song/enhance/player/url";

        let song_id: u64 = id
            .parse()
            .map_err(|_| Error::InvalidData(format!("无效的歌曲 ID: {}", id)))?;

        let ids = serde_json::json!([song_id]);

        let mut params = std::collections::HashMap::new();
        params.insert("ids", ids.to_string());
        params.insert("br", "320000".to_string()); // 请求 320kbps 比特率

        let mut request = self
            .client
            .post(url)
            .header("Referer", "https://music.163.com");

        if let Some(cookies) = &self.cookies {
            request = request.header("Cookie", cookies);
        }

        let response = request
            .form(&params)
            .send()
            .await
            .map_err(|e| Error::Network(format!("获取播放地址失败: {}", e)))?;

        if !response.status().is_success() {
            return Err(Error::Network(format!(
                "播放地址请求返回错误状态: {}",
                response.status()
            )));
        }

        let text = response
            .text()
            .await
            .map_err(|e| Error::Network(format!("读取播放地址响应失败: {}", e)))?;

        // 解析 JSON 响应
        let json: serde_json::Value = serde_json::from_str(&text)
            .map_err(|e| Error::Parse(format!("解析播放地址响应失败: {}", e)))?;

        // 检查响应码
        let code = json
            .get("code")
            .and_then(|v| v.as_i64())
            .ok_or_else(|| Error::Parse("响应中缺少 code 字段".into()))?;

        if code != 200 {
            return Err(Error::Network(format!(
                "获取播放地址失败，错误码: {}",
                code
            )));
        }

        // 解析播放 URL
        let data_array = json
            .get("data")
            .and_then(|d| d.as_array())
            .ok_or_else(|| Error::Parse("响应中缺少 data 字段".into()))?;

        if let Some(first_song) = data_array.first() {
            let song_url = first_song
                .get("url")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();

            // 合规检查：空 URL 表示需要 VIP 或不可播放
            // 在 stream_source() 中会检查并返回 PermissionDenied

            Ok(NeteaseSongUrl { url: song_url })
        } else {
            Err(Error::NotFound("未找到歌曲播放地址".into()))
        }
    }

    /// 获取歌词
    async fn get_lyrics(&self, id: &str) -> Result<Option<Lyrics>> {
        // 使用 /api/song/lyric 接口
        let url = format!("https://music.163.com/api/song/lyric?id={}&lv=-1&tv=-1", id);

        let mut request = self
            .client
            .get(&url)
            .header("Referer", "https://music.163.com");

        if let Some(cookies) = &self.cookies {
            request = request.header("Cookie", cookies);
        }

        let response = request
            .send()
            .await
            .map_err(|e| Error::Network(format!("获取歌词失败: {}", e)))?;

        if !response.status().is_success() {
            return Err(Error::Network(format!(
                "歌词请求返回错误状态: {}",
                response.status()
            )));
        }

        let text = response
            .text()
            .await
            .map_err(|e| Error::Network(format!("读取歌词响应失败: {}", e)))?;

        // 解析 JSON 响应
        let json: serde_json::Value = serde_json::from_str(&text)
            .map_err(|e| Error::Parse(format!("解析歌词响应失败: {}", e)))?;

        // 检查响应码
        let code = json
            .get("code")
            .and_then(|v| v.as_i64())
            .ok_or_else(|| Error::Parse("响应中缺少 code 字段".into()))?;

        if code != 200 {
            return Err(Error::Network(format!("获取歌词失败，错误码: {}", code)));
        }

        // 尝试获取原始歌词
        let lrc_text = json
            .get("lrc")
            .and_then(|l| l.get("lyric"))
            .and_then(|v| v.as_str());

        if let Some(lrc) = lrc_text {
            if lrc.trim().is_empty() {
                // 无歌词
                return Ok(None);
            }

            // 返回 LRC 格式歌词
            Ok(Some(Lyrics {
                format: kg_core::LyricsFormat::Lrc,
                content: lrc.to_string(),
            }))
        } else {
            // 无歌词
            Ok(None)
        }
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
