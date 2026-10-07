// 从现有代码复用的酷狗 API 模块
use async_trait::async_trait;
use kg_core::{
    Error, Lyrics, MusicProvider, Page, Paged, Result, SearchItem, SearchKind, StreamSource, Track,
    TrackId,
};
use kugou_sdk::{
    AudioQuality, DeviceIdentity, KugouClient, Pagination, PlaybackRequest, SearchRequest, Session,
    SongRef,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::debug;

// ===== 认证模块 (从 src/api/auth.rs 复用) =====

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthInfo {
    pub token: String,
    pub userid: String,
    pub mid: String,
}

/// 已登录账号信息（对前端可见的部分，**不含 token**）。
#[derive(Debug, Clone, Serialize)]
pub struct AccountInfo {
    pub userid: String,
    pub mid: String,
}

/// 搜索结果条目（对前端可见）。
#[derive(Debug, Clone, Serialize)]
pub struct SearchHit {
    /// 播放时回传给后端的资源标识
    pub hash: String,
    pub title: String,
    pub artist: String,
    pub album: String,
    pub duration_secs: u64,
    pub artwork_url: Option<String>,
    /// 上游版权位；非 0 通常表示受限（会员 / 仅试听）
    pub privilege: Option<i64>,
}

/// 一行歌词（对前端可见）。
#[derive(Debug, Clone, Serialize)]
pub struct LyricLineDto {
    /// 相对歌曲开头的起始时间（毫秒）
    pub time_ms: u64,
    pub duration_ms: u64,
    pub text: String,
    pub translation: Option<String>,
}

/// 播放地址结果。
#[derive(Debug, Clone, Serialize)]
pub struct PlaybackOutcome {
    pub url: String,
    /// `true` = 账号仅有试听片段权限（不做任何绕过，如实告知前端）
    pub preview: bool,
    pub preview_end_ms: Option<u64>,
    pub bit_rate: Option<u64>,
}

/// 二维码登录挑战。
///
/// SDK 只返回二维码 **key**，扫描用的 URL 需要由 [`KugouAuth::qr_login_url`]
/// 构造，再交给前端用二维码库渲染成图片 —— 接口本身不返回图片数据。
#[derive(Debug, Clone, Serialize)]
pub struct QrChallenge {
    pub key: String,
    pub url: String,
}

pub struct KugouAuth {
    client: KugouClient,
    session: Session,
    qr_key: Option<String>,
    /// 最近一次搜索的曲目，供播放时按音质档位挑选资源
    songs: HashMap<String, SongRef>,
}

impl KugouAuth {
    pub fn new() -> Result<Self> {
        let device = DeviceIdentity::builder()
            .device_id("kgplayer_linux")
            .uuid(uuid::Uuid::new_v4().to_string())
            .brand("Linux")
            .model("KGPlayer")
            .build()
            .map_err(|e| Error::AuthFailed(e.to_string()))?;

        let session = Session::new(device);
        let client = KugouClient::builder()
            .build()
            .map_err(|e| Error::AuthFailed(e.to_string()))?;

        Ok(Self {
            client,
            session,
            qr_key: None,
            songs: HashMap::new(),
        })
    }

    /// 申请二维码登录挑战：返回轮询用的 key 与供扫码的登录 URL。
    pub async fn request_qr(&mut self) -> Result<QrChallenge> {
        let auth = self.client.auth();

        let resp = auth
            .qr_key(&mut self.session)
            .await
            .map_err(|e| Error::AuthFailed(e.to_string()))?;

        let key = resp
            .data
            .get("qrcode")
            .and_then(|k| k.as_str())
            .ok_or_else(|| Error::AuthFailed("二维码响应中缺少 qrcode 字段".into()))?
            .to_string();

        let url = auth
            .qr_login_url(&key)
            .map_err(|e| Error::AuthFailed(e.to_string()))?;

        self.qr_key = Some(key.clone());

        Ok(QrChallenge { key, url })
    }

    /// 检查扫码状态。
    ///
    /// 返回 `(状态码, 认证信息)`，状态码 `1` = 等待扫码、`2` = 已扫码待确认、
    /// `4` = 登录成功（此时才返回凭据）。
    pub async fn check_qr_status(&mut self) -> Result<(u32, Option<AuthInfo>)> {
        let auth = self.client.auth();
        let qr_key = self
            .qr_key
            .clone()
            .ok_or_else(|| Error::AuthFailed("尚未申请二维码".into()))?;

        let resp = auth
            .check_qr(&mut self.session, &qr_key)
            .await
            .map_err(|e| Error::AuthFailed(e.to_string()))?;

        let status = resp
            .data
            .get("status")
            .and_then(|s| s.as_u64())
            .or_else(|| resp.upstream_status.and_then(|s| u64::try_from(s).ok()))
            .unwrap_or(0) as u32;

        if status != 4 {
            return Ok((status, None));
        }

        // 优先取业务 data；否则回退到 SDK 已写入的 session 字段
        let token = resp
            .data
            .get("token")
            .and_then(|t| t.as_str())
            .map(str::to_string)
            .or_else(|| self.session.token.clone())
            .unwrap_or_default();

        let userid = resp
            .data
            .get("userid")
            .and_then(|u| u.as_str())
            .map(str::to_string)
            .or_else(|| self.session.user_id.map(|u| u.to_string()))
            .unwrap_or_default();

        let mid = resp
            .data
            .get("mid")
            .and_then(|m| m.as_str())
            .map(str::to_string)
            .unwrap_or_else(|| self.session.device.mid.clone());

        if token.is_empty() || userid.is_empty() {
            return Err(Error::AuthFailed("登录成功但未返回凭据".into()));
        }

        Ok((status, Some(AuthInfo { token, userid, mid })))
    }

    /// 导出会话 JSON（**含 token**）。
    ///
    /// 仅供写入系统密钥环使用 —— 不得落盘明文、不得写日志、不得返回给前端。
    pub fn export_session(&self) -> Result<String> {
        self.session
            .export()
            .map_err(|e| Error::AuthFailed(e.to_string()))
    }

    /// 由密钥环中保存的会话 JSON 恢复认证状态。
    pub fn from_session_json(json: &str) -> Result<Self> {
        let session = Session::import(json).map_err(|e| Error::AuthFailed(e.to_string()))?;
        let client = KugouClient::builder()
            .build()
            .map_err(|e| Error::AuthFailed(e.to_string()))?;

        Ok(Self {
            client,
            session,
            qr_key: None,
            songs: HashMap::new(),
        })
    }

    /// 当前登录账号（未登录返回 `None`）。**不包含 token。**
    pub fn account(&self) -> Option<AccountInfo> {
        let token = self.session.token.as_ref()?;
        if token.is_empty() {
            return None;
        }
        Some(AccountInfo {
            userid: self
                .session
                .user_id
                .map(|u| u.to_string())
                .unwrap_or_default(),
            mid: self.session.device.mid.clone(),
        })
    }

    /// 会话是否可用于已登录请求
    pub fn is_authenticated(&self) -> bool {
        self.session.token.is_some()
    }

    /// 搜索歌曲。使用当前会话（已登录则携带凭据）。
    pub async fn search(
        &mut self,
        keyword: &str,
        page: u32,
        page_size: u32,
    ) -> Result<Vec<SearchHit>> {
        let request = SearchRequest::new(keyword).pagination(Pagination::new(page, page_size));

        let resp = self
            .client
            .search()
            .songs(&mut self.session, request)
            .await
            .map_err(|e| Error::Provider(e.to_string()))?;

        let hits = self.collect(resp.data.items);

        debug!("酷狗搜索 “{}” 命中 {} 条", keyword, hits.len());
        Ok(hits)
    }

    /// 取播放地址。
    ///
    /// 只返回账号当前有权播放的资源：若上游只授权试听片段，会如实带上
    /// `preview` 标记，**不做任何绕过**。
    pub async fn playback_url(&mut self, hash: &str) -> Result<PlaybackOutcome> {
        if !self.is_authenticated() {
            return Err(Error::AuthRequired);
        }

        let cached = self.songs.get(hash).cloned();
        let request = cached
            .as_ref()
            .and_then(|s| s.playback_request(AudioQuality::Standard))
            .unwrap_or_else(|| PlaybackRequest::new(hash));

        let resp = self
            .client
            .songs()
            .playback(&mut self.session, request)
            .await
            .map_err(|e| Error::Provider(e.to_string()))?;

        let info = resp.data;

        let url = info
            .url
            .clone()
            .or_else(|| info.urls.first().cloned())
            .filter(|u| !u.is_empty())
            .ok_or_else(|| {
                // 受版权/会员限制时上游不给地址 —— 如实报错，不做绕过
                Error::PermissionDenied("该曲目当前没有可用播放地址（可能受版权或会员限制）".into())
            })?;

        Ok(PlaybackOutcome {
            url,
            preview: info.preview_end_ms.is_some(),
            preview_end_ms: info.preview_end_ms,
            bit_rate: info.bit_rate,
        })
    }

    /// 每日推荐（登录后个性化更好；未登录返回通用榜单）。
    pub async fn daily(&mut self) -> Result<Vec<SearchHit>> {
        let resp = self
            .client
            .recommend()
            .everyday(&mut self.session)
            .await
            .map_err(|e| Error::Provider(e.to_string()))?;

        Ok(self.collect(resp.data.items))
    }

    /// 取歌词（优先 KRC 逐字，取不到时回退 LRC）。
    pub async fn lyrics(&mut self, hash: &str) -> Result<Vec<LyricLineDto>> {
        let Some(song) = self.songs.get(hash).cloned() else {
            return Err(Error::NotFound("请先搜索或加载该曲目".into()));
        };

        let content = match self
            .client
            .lyrics()
            .get_for_song(&mut self.session, &song, "krc")
            .await
        {
            Ok(resp) => resp,
            Err(_) => self
                .client
                .lyrics()
                .get_for_song(&mut self.session, &song, "lrc")
                .await
                .map_err(|e| Error::Provider(e.to_string()))?,
        };

        let doc = content
            .data
            .parse()
            .map_err(|e| Error::Provider(e.to_string()))?;

        Ok(doc
            .lines
            .into_iter()
            .filter(|l| !l.text.trim().is_empty())
            .map(|l| LyricLineDto {
                time_ms: l.start_ms,
                duration_ms: l.duration_ms,
                text: l.text,
                translation: l.translation,
            })
            .collect())
    }

    /// 把一批 `SongRef` 转成可展示的搜索结果，并缓存供播放/歌词使用。
    fn collect(&mut self, songs: Vec<SongRef>) -> Vec<SearchHit> {
        let mut hits = Vec::new();

        for song in songs {
            let Some(hash) = song
                .resources
                .standard
                .clone()
                .or_else(|| song.resources.high.clone())
                .or_else(|| song.resources.flac.clone())
            else {
                continue;
            };

            let title = {
                let t = song.display_name();
                if t.is_empty() {
                    song.name.clone().unwrap_or_default()
                } else {
                    t.to_string()
                }
            };

            hits.push(SearchHit {
                hash: hash.clone(),
                title,
                artist: song.singer.clone().unwrap_or_default(),
                album: song.album.clone().unwrap_or_default(),
                duration_secs: song.duration_secs.unwrap_or(0),
                artwork_url: song.artwork_url.clone(),
                privilege: song.privilege,
            });

            self.songs.insert(hash, song);
        }

        hits
    }
}

// ===== API 模块 (从 src/api/models.rs 和 kugou.rs 复用) =====

#[derive(Debug, Clone, Deserialize)]
pub struct Song {
    #[serde(rename = "songname", default)]
    pub song_name: String,
    #[serde(rename = "hash", default)]
    pub hash: String,
    #[serde(rename = "duration", default)]
    pub duration: u64,
    #[serde(rename = "filesize", default)]
    pub file_size: u64,
    #[serde(rename = "album_id", default)]
    pub album_id: String,
    #[serde(rename = "album_name", default)]
    pub album_name: String,
    #[serde(rename = "singername", default)]
    pub singer_name: String,
    #[serde(rename = "filename", default)]
    pub filename: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SongDetail {
    #[serde(rename = "url", default)]
    pub url: String,
    #[serde(rename = "hash", default)]
    pub hash: String,
    #[serde(rename = "songName", default)]
    pub song_name: String,
    #[serde(rename = "singerName", default)]
    pub singer_name: String,
    #[serde(rename = "album_img", default)]
    pub album_img: String,
    #[serde(rename = "timeLength", default)]
    pub time_length: u64,
}

#[derive(Debug, Deserialize)]
pub struct SearchResponse {
    pub data: Option<SearchData>,
}

#[derive(Debug, Deserialize)]
pub struct SearchData {
    pub info: Option<Vec<Song>>,
    pub total: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct SongInfoResponse {
    pub url: Option<String>,
    pub hash: Option<String>,
    #[serde(rename = "songName")]
    pub song_name: Option<String>,
    #[serde(rename = "singerName")]
    pub singer_name: Option<String>,
    #[serde(rename = "album_img")]
    pub album_img: Option<String>,
    pub error: Option<String>,
    #[serde(rename = "timeLength", default)]
    pub time: Option<u64>,
}

pub struct KugouApi {
    client: reqwest::Client,
    auth: Option<AuthInfo>,
}

impl Default for KugouApi {
    fn default() -> Self {
        Self::new()
    }
}

impl KugouApi {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
            auth: None,
        }
    }

    pub fn set_auth(&mut self, auth: AuthInfo) {
        self.auth = Some(auth);
    }

    pub fn get_auth(&self) -> Option<&AuthInfo> {
        self.auth.as_ref()
    }

    pub async fn search(&self, keyword: &str, page: u32, pagesize: u32) -> Result<Vec<Song>> {
        let url = format!(
            "http://mobilecdn.kugou.com/api/v3/search/song?format=json&keyword={}&page={}&pagesize={}",
            keyword, page, pagesize
        );

        let mut request = self.client.get(&url);

        if let Some(auth) = &self.auth {
            let cookie = format!(
                "token={}; KugooID={}; kg_mid={}",
                auth.token, auth.userid, auth.mid
            );
            request = request.header("Cookie", &cookie);
        }

        let resp: SearchResponse = request
            .send()
            .await
            .map_err(|e| Error::Network(e.to_string()))?
            .json()
            .await
            .map_err(|e| Error::Parse(e.to_string()))?;

        match resp.data {
            Some(data) => Ok(data.info.unwrap_or_default()),
            None => Err(Error::NotFound("搜索结果为空".into())),
        }
    }

    pub async fn get_song_info(&self, hash: &str) -> Result<SongDetail> {
        let url = format!(
            "http://m.kugou.com/app/i/getSongInfo.php?cmd=playInfo&hash={}",
            hash
        );

        let mut request = self.client.get(&url);

        request = request
            .header(
                "User-Agent",
                "Mozilla/5.0 (Linux; Android 12) AppleWebKit/537.36",
            )
            .header("Referer", "https://www.kugou.com/");

        if let Some(auth) = &self.auth {
            let cookie = format!(
                "token={}; KugooID={}; kg_mid={}",
                auth.token, auth.userid, auth.mid
            );
            request = request.header("Cookie", &cookie);
        }

        let resp: SongInfoResponse = request
            .send()
            .await
            .map_err(|e| Error::Network(e.to_string()))?
            .json()
            .await
            .map_err(|e| Error::Parse(e.to_string()))?;

        if let Some(err) = resp.error {
            if !err.is_empty() {
                return Err(Error::Provider(err));
            }
        }

        let url = resp
            .url
            .ok_or_else(|| Error::PermissionDenied("获取播放链接失败，可能需要会员权限".into()))?;

        Ok(SongDetail {
            url,
            hash: resp.hash.unwrap_or_default(),
            song_name: resp.song_name.unwrap_or_default(),
            singer_name: resp.singer_name.unwrap_or_default(),
            album_img: resp.album_img.unwrap_or_default(),
            time_length: resp.time.unwrap_or_default(),
        })
    }
}

// ===== Provider 实现 =====

pub struct KugouProvider {
    api: Arc<Mutex<KugouApi>>,
}

impl KugouProvider {
    pub fn new() -> Self {
        Self {
            api: Arc::new(Mutex::new(KugouApi::new())),
        }
    }

    pub async fn set_auth(&self, auth: AuthInfo) {
        let mut api = self.api.lock().await;
        api.set_auth(auth);
    }

    pub async fn is_authenticated(&self) -> bool {
        let api = self.api.lock().await;
        api.get_auth().is_some()
    }
}

#[async_trait]
impl MusicProvider for KugouProvider {
    fn id(&self) -> &'static str {
        "kugou"
    }

    fn name(&self) -> &'static str {
        "酷狗音乐"
    }

    async fn search(&self, query: &str, kind: SearchKind, page: Page) -> Result<Paged<SearchItem>> {
        if !matches!(kind, SearchKind::Track) {
            return Err(Error::Provider("酷狗搜索目前仅支持歌曲搜索".into()));
        }

        let api = self.api.lock().await;
        let songs = api.search(query, page.number, page.size).await?;
        let total = songs.len() as u32;

        let items: Vec<SearchItem> = songs
            .into_iter()
            .map(|song| {
                SearchItem::Track(Track {
                    id: TrackId::kugou(&song.hash),
                    title: song.song_name,
                    artist: song.singer_name,
                    album: song.album_name,
                    duration_secs: song.duration,
                    album_art_url: None,
                    year: None,
                    track_number: None,
                    genre: None,
                })
            })
            .collect();

        Ok(Paged { items, total, page })
    }

    async fn track(&self, id: &TrackId) -> Result<Track> {
        if id.provider != "kugou" {
            return Err(Error::InvalidData(format!(
                "Invalid provider: {}",
                id.provider
            )));
        }

        let api = self.api.lock().await;
        let detail = api.get_song_info(&id.id).await?;

        Ok(Track {
            id: id.clone(),
            title: detail.song_name,
            artist: detail.singer_name,
            album: String::new(),
            duration_secs: detail.time_length / 1000,
            album_art_url: Some(detail.album_img),
            year: None,
            track_number: None,
            genre: None,
        })
    }

    async fn stream_source(&self, id: &TrackId) -> Result<StreamSource> {
        if id.provider != "kugou" {
            return Err(Error::InvalidData(format!(
                "Invalid provider: {}",
                id.provider
            )));
        }

        let api = self.api.lock().await;

        if api.get_auth().is_none() {
            return Err(Error::AuthRequired);
        }

        let detail = api.get_song_info(&id.id).await?;

        if detail.url.is_empty() {
            return Err(Error::PermissionDenied(
                "无法获取播放链接，请确认会员状态".into(),
            ));
        }

        debug!("Got stream URL for {}: {}", id.id, detail.url);
        Ok(StreamSource::Url(detail.url))
    }

    async fn lyrics(&self, _id: &TrackId) -> Result<Option<Lyrics>> {
        // TODO: 实现酷狗歌词接口
        Ok(None)
    }
}

impl Default for KugouProvider {
    fn default() -> Self {
        Self::new()
    }
}
