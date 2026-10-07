use tauri::State;
use uuid::Uuid;

use kg_audio::PlayerCommand;
use kg_core::{Playlist, Track, TrackId};
use kg_lyrics::{extract_album_art, get_lyrics_for_track};

use crate::queue::{QueueEntry, QueueSnapshot};
use crate::AppState;

#[tauri::command]
pub async fn get_all_tracks(state: State<'_, AppState>) -> Result<Vec<Track>, String> {
    state
        .provider
        .scan()
        .await
        .map_err(|e| format!("Failed to scan: {}", e))
}

#[tauri::command]
pub async fn scan_library(state: State<'_, AppState>) -> Result<(), String> {
    let tracks = state
        .provider
        .scan()
        .await
        .map_err(|e| format!("Failed to scan: {}", e))?;

    tracing::info!("Scanned {} tracks", tracks.len());
    Ok(())
}

#[tauri::command]
pub async fn search_tracks(
    state: State<'_, AppState>,
    query: String,
) -> Result<Vec<String>, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    db.search_tracks(&query, 100)
        .map_err(|e| format!("Failed to search: {}", e))
}

/// 用给定列表替换播放队列，并从 `start` 开始播放。
///
/// 本地与在线曲目共用这条入口：条目自带 provider，取流时再分派。
#[tauri::command]
pub async fn queue_play(
    state: State<'_, AppState>,
    entries: Vec<QueueEntry>,
    start: usize,
) -> Result<(), String> {
    state.playback.play_from(entries, start).await
}

#[tauri::command]
pub async fn queue_remove(state: State<'_, AppState>, index: usize) -> Result<(), String> {
    state.playback.remove(index).await
}

/// 追加到播放队列末尾（队列为空时直接开始播放）。
#[tauri::command]
pub async fn queue_append(
    state: State<'_, AppState>,
    entries: Vec<QueueEntry>,
) -> Result<(), String> {
    state.playback.append(entries).await
}

/// 插到当前曲目之后（「下一首播放」）。
#[tauri::command]
pub async fn queue_play_next(state: State<'_, AppState>, entry: QueueEntry) -> Result<(), String> {
    state.playback.play_next(entry).await
}

/// 跳转到队列中的指定位置并播放。
#[tauri::command]
pub async fn queue_jump(state: State<'_, AppState>, index: usize) -> Result<(), String> {
    state.playback.jump_to(index).await
}

#[tauri::command]
pub async fn pause(state: State<'_, AppState>) -> Result<(), String> {
    state
        .player_cmd_tx
        .send(PlayerCommand::Pause)
        .map_err(|e| format!("Failed to send pause command: {}", e))
}

#[tauri::command]
pub async fn resume(state: State<'_, AppState>) -> Result<(), String> {
    state
        .player_cmd_tx
        .send(PlayerCommand::Resume)
        .map_err(|e| format!("Failed to send resume command: {}", e))
}

#[tauri::command]
pub async fn next_track(state: State<'_, AppState>) -> Result<(), String> {
    state.playback.next().await
}

#[tauri::command]
pub async fn previous_track(state: State<'_, AppState>) -> Result<(), String> {
    state.playback.previous().await
}

#[tauri::command]
pub async fn seek(state: State<'_, AppState>, position: f64) -> Result<(), String> {
    state
        .player_cmd_tx
        .send(PlayerCommand::Seek(position.clamp(0.0, 1.0)))
        .map_err(|e| format!("Failed to send seek command: {}", e))
}

#[tauri::command]
pub async fn set_volume(state: State<'_, AppState>, volume: f64) -> Result<(), String> {
    state
        .player_cmd_tx
        .send(PlayerCommand::SetVolume(volume.clamp(0.0, 1.0)))
        .map_err(|e| format!("Failed to send volume command: {}", e))
}

#[derive(serde::Serialize)]
pub struct LyricsResponse {
    lines: Vec<LrcLineResponse>,
}

#[derive(serde::Serialize)]
pub struct LrcLineResponse {
    timestamp_ms: u64,
    text: String,
}

#[tauri::command]
pub async fn get_lyrics(
    state: State<'_, AppState>,
    track_id: TrackId,
) -> Result<Option<LyricsResponse>, String> {
    let lyrics = get_lyrics_for_track(&track_id, state.provider.as_ref())
        .await
        .map_err(|e| format!("Failed to get lyrics: {}", e))?;

    if let Some(lyrics) = lyrics {
        use kg_lyrics::LrcParser;
        let (_, lines) = LrcParser::parse(&lyrics.content)
            .map_err(|e| format!("Failed to parse lyrics: {}", e))?;

        let response_lines = lines
            .into_iter()
            .map(|line| LrcLineResponse {
                timestamp_ms: line.timestamp.as_millis() as u64,
                text: line.text,
            })
            .collect();

        Ok(Some(LyricsResponse {
            lines: response_lines,
        }))
    } else {
        Ok(None)
    }
}

#[tauri::command]
pub async fn get_album_art(
    _state: State<'_, AppState>,
    track_id: TrackId,
) -> Result<Option<Vec<u8>>, String> {
    extract_album_art(&track_id)
        .await
        .map_err(|e| format!("Failed to extract album art: {}", e))
}

// ===== 播放列表相关命令 =====

#[tauri::command]
pub async fn create_playlist(state: State<'_, AppState>, name: String) -> Result<Playlist, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    db.create_playlist(&name)
        .map_err(|e| format!("Failed to create playlist: {}", e))
}

#[tauri::command]
pub async fn get_playlists(state: State<'_, AppState>) -> Result<Vec<Playlist>, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    db.get_playlists()
        .map_err(|e| format!("Failed to get playlists: {}", e))
}

#[tauri::command]
pub async fn add_to_playlist(
    state: State<'_, AppState>,
    playlist_id: String,
    track_id: TrackId,
) -> Result<(), String> {
    let uuid = Uuid::parse_str(&playlist_id).map_err(|e| format!("Invalid playlist ID: {}", e))?;

    let db = state.db.lock().map_err(|e| e.to_string())?;
    db.add_to_playlist(&uuid, &track_id)
        .map_err(|e| format!("Failed to add to playlist: {}", e))
}

#[tauri::command]
pub async fn remove_from_playlist(
    state: State<'_, AppState>,
    playlist_id: String,
    track_id: TrackId,
    position: i32,
) -> Result<(), String> {
    let uuid = Uuid::parse_str(&playlist_id).map_err(|e| format!("Invalid playlist ID: {}", e))?;

    let db = state.db.lock().map_err(|e| e.to_string())?;
    db.remove_from_playlist(&uuid, &track_id, position)
        .map_err(|e| format!("Failed to remove from playlist: {}", e))
}

#[tauri::command]
pub async fn delete_playlist(
    state: State<'_, AppState>,
    playlist_id: String,
) -> Result<(), String> {
    let uuid = Uuid::parse_str(&playlist_id).map_err(|e| format!("Invalid playlist ID: {}", e))?;

    let db = state.db.lock().map_err(|e| e.to_string())?;
    db.delete_playlist(&uuid)
        .map_err(|e| format!("Failed to delete playlist: {}", e))
}

#[tauri::command]
pub async fn rename_playlist(
    state: State<'_, AppState>,
    playlist_id: String,
    new_name: String,
) -> Result<(), String> {
    let uuid = Uuid::parse_str(&playlist_id).map_err(|e| format!("Invalid playlist ID: {}", e))?;

    let db = state.db.lock().map_err(|e| e.to_string())?;
    db.rename_playlist(&uuid, &new_name)
        .map_err(|e| format!("Failed to rename playlist: {}", e))
}

#[tauri::command]
pub async fn get_current_queue(state: State<'_, AppState>) -> Result<QueueSnapshot, String> {
    state.playback.snapshot()
}

#[tauri::command]
pub async fn clear_queue(state: State<'_, AppState>) -> Result<(), String> {
    state.playback.clear()
}
