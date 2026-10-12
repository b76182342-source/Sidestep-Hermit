use serde::Serialize;
use std::sync::{Arc, Mutex};
use tauri::{AppHandle, Emitter, Manager};
use tokio::runtime::Runtime;
use tokio::sync::mpsc;
use tracing::{debug, error, info};

use kg_audio::{Player, PlayerCommand, PlayerEvent};
use kg_core::{PlayState, ProviderRegistry};
use kg_providers::LocalProvider;
use kg_store::Database;

/// 建立与进程同生命周期的 Tokio runtime。
///
/// Tauri 的 `setup()` 在主线程执行，且不在任何 Tokio runtime 上下文里，
/// 所以裸 `tokio::spawn` 会 panic（"there is no reactor running"）。
/// 所有后台任务都必须从这个 runtime 派生。
fn build_runtime() -> std::io::Result<Runtime> {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .thread_name("kgplayer-worker")
        .build()
}

#[cfg(target_os = "linux")]
use kg_mpris::{MprisCommand, MprisController};

mod commands;
mod commands_kugou;
mod commands_provider;
mod playback;
mod queue;
#[cfg(feature = "kugou")]
mod secrets;
mod tray;

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct PlayerEventPayload {
    #[serde(rename = "type")]
    event_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<String>,
    /// 秒。GStreamer 给的是纳秒，在转发给前端前统一换算。
    #[serde(skip_serializing_if = "Option::is_none")]
    position: Option<f64>,
    /// 秒
    #[serde(skip_serializing_if = "Option::is_none")]
    duration: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    volume: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<String>,
}

/// 纳秒 → 秒
fn ns_to_secs(ns: u64) -> f64 {
    ns as f64 / 1_000_000_000.0
}

pub struct AppState {
    /// Provider 注册中心（替代单一 provider）
    pub registry: Arc<ProviderRegistry>,
    db: Arc<Mutex<Database>>,
    /// 统一播放队列（本地 + 在线）
    pub playback: Arc<playback::PlaybackCtx>,
    pub player_cmd_tx: mpsc::UnboundedSender<PlayerCommand>,
    /// 最近一次播放状态，供托盘决定「播放 / 暂停」该发哪条指令
    pub playing: Arc<std::sync::atomic::AtomicBool>,
    #[cfg(target_os = "linux")]
    mpris: Option<Arc<MprisController>>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    if let Err(err) = run_inner() {
        // 日志可能还没初始化成功，所以同时写 stderr
        eprintln!("Sidestep Hermit 启动失败: {err}");
        std::process::exit(1);
    }
}

fn run_inner() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化日志
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    info!("Starting Sidestep Hermit with Tauri");

    // 初始化 GStreamer
    gstreamer::init()?;

    // 故意泄漏：这个 runtime 的生命周期就是进程本身，
    // drop 掉反而会让所有后台任务失去调度器。这样拿到的是 &'static，
    // setup 闭包可以直接捕获，无需全局 static + expect。
    let runtime: &'static Runtime = Box::leak(Box::new(build_runtime()?));

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(move |app| {
            let (player, event_rx) = Player::new().map_err(std::io::Error::other)?;
            let player = Arc::new(Mutex::new(player));

            let db = Arc::new(Mutex::new(Database::new().map_err(std::io::Error::other)?));

            // 默认音乐库路径；任一环节失败都退回当前目录，不 panic
            let music_dir = directories::UserDirs::new()
                .and_then(|dirs| dirs.audio_dir().map(|p| p.to_path_buf()))
                .or_else(|| std::env::current_dir().ok())
                .unwrap_or_else(|| std::path::PathBuf::from("."));

            // 创建 ProviderRegistry 并注册 LocalProvider
            let local_provider = Arc::new(LocalProvider::new(vec![music_dir]));
            let mut registry = ProviderRegistry::new();
            registry.register(local_provider.clone());

            // TODO: 注册其他 Provider (KugouProvider, NeteaseProvider)
            // #[cfg(feature = "kugou")]
            // registry.register(Arc::new(KugouProvider::new()));

            let registry = Arc::new(registry);

            let (cmd_tx, mut cmd_rx) = mpsc::unbounded_channel::<PlayerCommand>();

            // 统一播放队列：命令层与 MPRIS 任务共享同一份状态
            let playback = Arc::new(playback::PlaybackCtx::new(
                registry.clone(),
                db.clone(),
                cmd_tx.clone(),
                app.handle().clone(),
            ));

            // MPRIS2 初始化（仅 Linux）
            #[cfg(target_os = "linux")]
            let mpris = {
                let (mpris_cmd_tx, mut mpris_cmd_rx) = mpsc::unbounded_channel::<MprisCommand>();

                let mpris_ctrl = runtime
                    .block_on(async { MprisController::new(mpris_cmd_tx).await })
                    .ok()
                    .map(Arc::new);

                if mpris_ctrl.is_some() {
                    info!("MPRIS2 initialized");
                } else {
                    error!("Failed to initialize MPRIS2");
                }

                // 处理 MPRIS 命令：把媒体键转换成播放器命令
                let cmd_tx_clone = cmd_tx.clone();
                let playback_for_mpris = playback.clone();
                runtime.spawn(async move {
                    while let Some(mpris_cmd) = mpris_cmd_rx.recv().await {
                        match mpris_cmd {
                            MprisCommand::Play | MprisCommand::PlayPause => {
                                let _ = cmd_tx_clone.send(PlayerCommand::Resume);
                            }
                            MprisCommand::Pause => {
                                let _ = cmd_tx_clone.send(PlayerCommand::Pause);
                            }
                            MprisCommand::Stop => {
                                let _ = cmd_tx_clone.send(PlayerCommand::Stop);
                            }
                            MprisCommand::Next => {
                                if let Err(e) = playback_for_mpris.next().await {
                                    debug!("MPRIS 下一首未生效：{e}");
                                }
                            }
                            MprisCommand::Previous => {
                                if let Err(e) = playback_for_mpris.previous().await {
                                    debug!("MPRIS 上一首未生效：{e}");
                                }
                            }
                            MprisCommand::Seek(_offset_us) => {
                                // TODO: 相对 seek 需要当前播放位置（微秒），
                                // 目前 MPRIS 侧还没有暴露 Position，先忽略
                            }
                        }
                    }
                });

                mpris_ctrl
            };

            #[cfg(not(target_os = "linux"))]
            let mpris = None;

            let state = AppState {
                registry,
                db,
                playback,
                player_cmd_tx: cmd_tx,
                playing: Arc::new(std::sync::atomic::AtomicBool::new(false)),
                #[cfg(target_os = "linux")]
                mpris,
            };

            app.manage(state);

            // 开发构建走 `devUrl`（需要 Vite 在跑），发布构建才内嵌前端。
            // 两者界面完全不同（一个连不上就是白屏报错），标题里标出来免得混淆。
            #[cfg(dev)]
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.set_title("Sidestep Hermit (dev · 需要 Vite)");
            }

            // 系统托盘（缺失时只告警）
            if let Err(e) = tray::setup(app.handle()) {
                error!("初始化系统托盘失败: {e}");
            }

            // 播放器命令处理
            let player_clone = player.clone();
            runtime.spawn(async move {
                while let Some(cmd) = cmd_rx.recv().await {
                    if let Ok(player) = player_clone.lock() {
                        player.handle_command(cmd);
                    }
                }
            });

            // 播放器事件转发到前端
            let app_handle = app.handle().clone();
            runtime.spawn(async move {
                handle_player_events(runtime, app_handle, event_rx).await;
            });

            Ok(())
        })
        .on_window_event(|window, event| {
            // 关窗只收起，退出走托盘菜单，避免误关丢播放
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                api.prevent_close();
                tray::handle_close_requested(window);
            }
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_all_tracks,
            commands::scan_library,
            commands::search_tracks,
            commands::pause,
            commands::resume,
            commands::next_track,
            commands::previous_track,
            commands::seek,
            commands::set_volume,
            commands::get_lyrics,
            commands::get_album_art,
            commands::create_playlist,
            commands::get_playlists,
            commands::add_to_playlist,
            commands::remove_from_playlist,
            commands::delete_playlist,
            commands::rename_playlist,
            commands::get_current_queue,
            commands::clear_queue,
            // 播放队列
            commands::queue_play,
            commands::queue_append,
            commands::queue_play_next,
            commands::queue_jump,
            commands::queue_remove,
            // Provider 管理命令
            commands_provider::list_providers,
            commands_provider::account_statuses,
            // Kugou 命令
            commands_kugou::kugou_get_qr_code,
            commands_kugou::kugou_check_login,
            commands_kugou::kugou_session,
            commands_kugou::kugou_search,
            commands_kugou::kugou_daily,
            commands_kugou::kugou_liked_songs,
            commands_kugou::kugou_lyrics,
            commands_kugou::kugou_play,
            commands_kugou::kugou_get_play_url,
            commands_kugou::kugou_logout,
        ])
        .run(tauri::generate_context!())?;

    Ok(())
}

fn main() {
    run();
}

async fn handle_player_events(
    runtime: &'static Runtime,
    app: AppHandle,
    mut event_rx: mpsc::UnboundedReceiver<PlayerEvent>,
) {
    while let Some(event) = event_rx.recv().await {
        // 记录播放状态，供托盘「播放 / 暂停」判断
        if let (Some(state), PlayerEvent::StateChanged(play_state)) =
            (app.try_state::<AppState>(), &event)
        {
            state.playing.store(
                matches!(play_state, PlayState::Playing),
                std::sync::atomic::Ordering::Relaxed,
            );
        }

        // 更新 MPRIS（Linux）
        #[cfg(target_os = "linux")]
        if let Some(state) = app.try_state::<AppState>() {
            if let Some(mpris) = &state.mpris {
                match &event {
                    PlayerEvent::StateChanged(play_state) => {
                        mpris.update_state(*play_state).await;
                    }
                    PlayerEvent::PositionChanged { position, .. } => {
                        // MPRIS 要微秒；播放器给的是纳秒
                        mpris.update_position(position / 1_000).await;
                    }
                    _ => {}
                }
            }
        }

        let payload = match event {
            PlayerEvent::StateChanged(state) => PlayerEventPayload {
                event_type: "StateChanged".to_string(),
                state: Some(format!("{:?}", state)),
                position: None,
                duration: None,
                volume: None,
                message: None,
            },
            PlayerEvent::PositionChanged { position, duration } => PlayerEventPayload {
                event_type: "PositionChanged".to_string(),
                state: None,
                position: Some(ns_to_secs(position)),
                duration: Some(ns_to_secs(duration)),
                volume: None,
                message: None,
            },
            PlayerEvent::VolumeChanged(volume) => PlayerEventPayload {
                event_type: "VolumeChanged".to_string(),
                state: None,
                position: None,
                duration: None,
                volume: Some(volume),
                message: None,
            },
            PlayerEvent::EndOfStream => {
                // 一曲放完自动顺延到队列下一首
                if let Some(state) = app.try_state::<AppState>() {
                    let playback = state.playback.clone();
                    runtime.spawn(async move {
                        if let Err(e) = playback.next().await {
                            debug!("播放结束，队列没有下一首：{e}");
                        }
                    });
                }
                continue;
            }
            PlayerEvent::Error(msg) => PlayerEventPayload {
                event_type: "Error".to_string(),
                state: None,
                position: None,
                duration: None,
                volume: None,
                message: Some(msg),
            },
            PlayerEvent::Buffering(percent) => {
                debug!("Buffering: {}%", percent);
                continue;
            }
        };

        if let Err(e) = app.emit("player-event", payload) {
            error!("Failed to emit player event: {}", e);
        }
    }
}
