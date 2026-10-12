// Kugou 相关的 Tauri 命令
// 注意：kugou 功能需要启用 feature = "kugou"
//
// 凭据处理约定（CLAUDE.md 合规红线）：
//   * 会话（含 token）只写入系统密钥环；
//   * token 不进日志、不回传前端 —— 前端只拿到「是否已登录 + 账号标识」。

#[cfg(feature = "kugou")]
use crate::secrets;
#[cfg(feature = "kugou")]
use kg_providers::kugou::KugouAuth;

#[cfg(feature = "kugou")]
use once_cell::sync::Lazy;
#[cfg(feature = "kugou")]
use serde_json::json;
#[cfg(feature = "kugou")]
use std::sync::Arc;
#[cfg(feature = "kugou")]
use tokio::sync::Mutex;
#[cfg(feature = "kugou")]
use tracing::{debug, warn};

#[cfg(feature = "kugou")]
static KUGOU_AUTH: Lazy<Arc<Mutex<Option<KugouAuth>>>> = Lazy::new(|| Arc::new(Mutex::new(None)));

/// 确保内存中有可用会话：优先密钥环恢复，否则建匿名会话。
///
/// 匿名会话足以支撑搜索/每日推荐；取播放地址仍要求已登录。
#[cfg(feature = "kugou")]
async fn ensure_auth(
    guard: &mut tokio::sync::MutexGuard<'_, Option<KugouAuth>>,
) -> Result<(), String> {
    if guard.is_some() {
        return Ok(());
    }

    if let Ok(Some(json)) = secrets::load(secrets::CredentialKey::Kugou).await {
        if let Ok(auth) = KugouAuth::from_session_json(&json) {
            **guard = Some(auth);
            return Ok(());
        }
        warn!("密钥环中的会话已失效，改用匿名会话");
    }

    **guard = Some(KugouAuth::new().map_err(|e| e.to_string())?);
    Ok(())
}

/// 取回某条曲目的授权播放地址（供播放队列内部调用）。
#[cfg(feature = "kugou")]
pub async fn resolve_kugou_url(hash: &str) -> Result<kg_providers::kugou::PlaybackOutcome, String> {
    let mut guard = KUGOU_AUTH.lock().await;
    let auth = guard.as_mut().ok_or("请先登录酷狗账号")?;
    auth.playback_url(hash).await.map_err(|e| e.to_string())
}

/// 把当前会话写入系统密钥环。失败只告警，不影响本次登录。
#[cfg(feature = "kugou")]
async fn persist_session(auth: &KugouAuth) {
    match auth.export_session() {
        Ok(json) => {
            if let Err(e) = secrets::save(secrets::CredentialKey::Kugou, &json).await {
                warn!("写入系统密钥环失败，本次登录无法跨重启保留: {e}");
            }
        }
        Err(e) => warn!("导出会话失败: {e}"),
    }
}

#[cfg(feature = "kugou")]
#[tauri::command]
pub async fn kugou_get_qr_code() -> Result<String, String> {
    let mut auth = KUGOU_AUTH.lock().await;

    // 创建新的认证实例
    let mut kugou_auth = KugouAuth::new().map_err(|e| format!("Failed to create auth: {}", e))?;

    // 申请二维码挑战：SDK 只返回 key，扫码 URL 需自行构造后由前端渲染
    let challenge = kugou_auth
        .request_qr()
        .await
        .map_err(|e| format!("Failed to request QR code: {}", e))?;

    *auth = Some(kugou_auth);

    Ok(challenge.url)
}

#[cfg(not(feature = "kugou"))]
#[tauri::command]
pub async fn kugou_get_qr_code() -> Result<String, String> {
    Err("Kugou feature not enabled".to_string())
}

#[cfg(feature = "kugou")]
#[tauri::command]
pub async fn kugou_check_login() -> Result<serde_json::Value, String> {
    let mut auth_guard = KUGOU_AUTH.lock().await;

    let Some(auth) = auth_guard.as_mut() else {
        return Err("Auth not initialized".to_string());
    };

    let (status, auth_info) = auth
        .check_qr_status()
        .await
        .map_err(|e| format!("Check login failed: {}", e))?;

    // status == 4 表示扫码登录成功，此时才应该拿到凭据
    if status == 4 {
        if auth_info.is_some() {
            persist_session(auth).await;
            debug!("酷狗扫码登录成功，凭据已写入系统密钥环");
        }

        Ok(json!({
            "success": true,
            "userid": auth.account().map(|a| a.userid).unwrap_or_default(),
        }))
    } else {
        Ok(json!({ "success": false, "status": status }))
    }
}

#[cfg(not(feature = "kugou"))]
#[tauri::command]
pub async fn kugou_check_login() -> Result<serde_json::Value, String> {
    Err("Kugou feature not enabled".to_string())
}

/// 查询当前登录态；启动时调用，用于恢复上次登录。
///
/// 内存里没有会话时会尝试从系统密钥环恢复。
#[cfg(feature = "kugou")]
#[tauri::command]
pub async fn kugou_session() -> Result<serde_json::Value, String> {
    let mut auth_guard = KUGOU_AUTH.lock().await;

    if auth_guard.is_none() {
        match secrets::load(secrets::CredentialKey::Kugou).await {
            Ok(Some(json)) => match KugouAuth::from_session_json(&json) {
                Ok(auth) => {
                    debug!("已从系统密钥环恢复酷狗登录态");
                    *auth_guard = Some(auth);
                }
                Err(e) => warn!("密钥环中的会话已失效: {e}"),
            },
            Ok(None) => {}
            Err(e) => warn!("读取系统密钥环失败: {e}"),
        }
    }

    match auth_guard.as_ref().filter(|a| a.is_authenticated()) {
        Some(auth) => Ok(json!({
            "loggedIn": true,
            "userid": auth.account().map(|a| a.userid).unwrap_or_default(),
            "mid": auth.account().map(|a| a.mid).unwrap_or_default(),
        })),
        None => Ok(json!({ "loggedIn": false })),
    }
}

#[cfg(not(feature = "kugou"))]
#[tauri::command]
pub async fn kugou_session() -> Result<serde_json::Value, String> {
    Ok(serde_json::json!({ "loggedIn": false }))
}

#[cfg(feature = "kugou")]
#[tauri::command]
pub async fn kugou_search(query: String, page: Option<u32>) -> Result<serde_json::Value, String> {
    let mut guard = KUGOU_AUTH.lock().await;
    ensure_auth(&mut guard).await?;

    let auth = guard.as_mut().ok_or("认证状态初始化失败")?;
    let hits = auth
        .search(&query, page.unwrap_or(1), 30)
        .await
        .map_err(|e| e.to_string())?;

    serde_json::to_value(hits).map_err(|e| e.to_string())
}

/// 每日推荐。
#[cfg(feature = "kugou")]
#[tauri::command]
pub async fn kugou_daily() -> Result<serde_json::Value, String> {
    let mut guard = KUGOU_AUTH.lock().await;
    ensure_auth(&mut guard).await?;

    let auth = guard.as_mut().ok_or("认证状态初始化失败")?;
    let hits = auth.daily().await.map_err(|e| e.to_string())?;

    // 调试：输出第一首歌的封面 URL
    if let Some(first) = hits.first() {
        eprintln!(
            "[DAILY] First song: {} - artwork_url: {:?}",
            first.title, first.artwork_url
        );
    }

    serde_json::to_value(hits).map_err(|e| e.to_string())
}

#[cfg(not(feature = "kugou"))]
#[tauri::command]
pub async fn kugou_daily() -> Result<serde_json::Value, String> {
    Err("Kugou feature not enabled".to_string())
}

/// 我喜欢的歌曲（系统默认收藏歌单）。
#[cfg(feature = "kugou")]
#[tauri::command]
pub async fn kugou_liked_songs() -> Result<serde_json::Value, String> {
    let mut guard = KUGOU_AUTH.lock().await;
    ensure_auth(&mut guard).await?;

    let auth = guard.as_mut().ok_or("认证状态初始化失败")?;
    let hits = auth.liked_songs().await.map_err(|e| e.to_string())?;

    // 调试：输出第一首歌的封面 URL
    if let Some(first) = hits.first() {
        eprintln!(
            "[LIKED] First song: {} - artwork_url: {:?}",
            first.title, first.artwork_url
        );
    }

    serde_json::to_value(hits).map_err(|e| e.to_string())
}

#[cfg(not(feature = "kugou"))]
#[tauri::command]
pub async fn kugou_liked_songs() -> Result<serde_json::Value, String> {
    Err("Kugou feature not enabled".to_string())
}

/// 取歌词（需先通过搜索/每日推荐加载过该曲目）。
#[cfg(feature = "kugou")]
#[tauri::command]
pub async fn kugou_lyrics(hash: String) -> Result<serde_json::Value, String> {
    let mut guard = KUGOU_AUTH.lock().await;
    let auth = guard.as_mut().ok_or("请先加载该曲目")?;

    let lines = match auth.lyrics(&hash).await {
        Ok(lines) => lines,
        Err(e) => {
            // 无歌词 / 取不到都是可能发生的情况，留下原因便于排查
            debug!("曲目 {hash} 未取到歌词：{e}");
            return Err(e.to_string());
        }
    };

    debug!("曲目 {hash} 取到 {} 行歌词", lines.len());
    serde_json::to_value(lines).map_err(|e| e.to_string())
}

#[cfg(not(feature = "kugou"))]
#[tauri::command]
pub async fn kugou_lyrics(_hash: String) -> Result<serde_json::Value, String> {
    Err("Kugou feature not enabled".to_string())
}

#[cfg(not(feature = "kugou"))]
#[tauri::command]
pub async fn kugou_search(_query: String, _page: Option<u32>) -> Result<serde_json::Value, String> {
    Err("Kugou feature not enabled".to_string())
}

/// 播放指定酷狗曲目：取回授权地址后交给本地播放器。
#[cfg(feature = "kugou")]
#[tauri::command]
pub async fn kugou_play(
    state: tauri::State<'_, crate::AppState>,
    hash: String,
) -> Result<serde_json::Value, String> {
    let outcome = {
        let mut guard = KUGOU_AUTH.lock().await;
        let auth = guard.as_mut().ok_or("请先登录酷狗账号")?;
        auth.playback_url(&hash).await.map_err(|e| e.to_string())?
    };

    state
        .player_cmd_tx
        .send(kg_audio::PlayerCommand::Play(kg_core::StreamSource::Url(
            outcome.url.clone(),
        )))
        .map_err(|e| format!("Failed to send play command: {}", e))?;

    if outcome.preview {
        debug!("该曲目仅获授权试听片段");
    } else {
        debug!("已取回完整播放地址");
    }

    serde_json::to_value(outcome).map_err(|e| e.to_string())
}

#[cfg(not(feature = "kugou"))]
#[tauri::command]
pub async fn kugou_play(
    _state: tauri::State<'_, crate::AppState>,
    _hash: String,
) -> Result<serde_json::Value, String> {
    Err("Kugou feature not enabled".to_string())
}

/// 仅取播放地址，不开始播放。
#[cfg(feature = "kugou")]
#[tauri::command]
pub async fn kugou_get_play_url(hash: String) -> Result<String, String> {
    let mut guard = KUGOU_AUTH.lock().await;
    let auth = guard.as_mut().ok_or("请先登录酷狗账号")?;
    let outcome = auth.playback_url(&hash).await.map_err(|e| e.to_string())?;
    Ok(outcome.url)
}

#[cfg(not(feature = "kugou"))]
#[tauri::command]
pub async fn kugou_get_play_url(_hash: String) -> Result<String, String> {
    Err("Kugou feature not enabled".to_string())
}

#[cfg(feature = "kugou")]
#[tauri::command]
pub async fn kugou_logout() -> Result<(), String> {
    {
        let mut auth_guard = KUGOU_AUTH.lock().await;
        *auth_guard = None;
    }

    if let Err(e) = secrets::clear(secrets::CredentialKey::Kugou).await {
        warn!("清除系统密钥环失败: {e}");
    }

    Ok(())
}

#[cfg(not(feature = "kugou"))]
#[tauri::command]
pub async fn kugou_logout() -> Result<(), String> {
    Err("Kugou feature not enabled".to_string())
}

/// 获取歌曲的真实封面 URL
#[cfg(feature = "kugou")]
pub async fn get_artwork_url(hash: &str) -> Result<String, String> {
    use kg_core::{MusicProvider, TrackId};
    use kg_providers::KugouProvider;

    let provider = KugouProvider::new();
    let track_id = TrackId::new("kugou", hash);
    let track = provider.track(&track_id).await.map_err(|e| e.to_string())?;

    track
        .album_art_url
        .ok_or_else(|| "No artwork URL available".to_string())
}

#[cfg(not(feature = "kugou"))]
pub async fn get_artwork_url(_hash: &str) -> Result<String, String> {
    Err("Kugou feature not enabled".to_string())
}
