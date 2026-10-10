use gstreamer::{self as gst};
use gstreamer_player::{Player as GstPlayer, PlayerGMainContextSignalDispatcher};
use kg_core::{PlayState, StreamSource};
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;
use tracing::{debug, error, warn};

/// 播放器命令
#[derive(Debug)]
pub enum PlayerCommand {
    Play(StreamSource),
    Pause,
    Resume,
    Stop,
    Seek(f64), // 0.0 - 1.0
    SetVolume(f64),
    SetEqualizer([f64; 10]),
}

/// 播放器事件
#[derive(Debug, Clone)]
pub enum PlayerEvent {
    StateChanged(PlayState),
    PositionChanged { position: u64, duration: u64 },
    VolumeChanged(f64),
    Buffering(u8),
    EndOfStream,
    Error(String),
}

struct PlayerInner {
    gst_player: GstPlayer,
    state: PlayState,
}

pub struct Player {
    inner: Arc<Mutex<PlayerInner>>,
}

impl Player {
    pub fn new() -> Result<(Self, mpsc::UnboundedReceiver<PlayerEvent>), String> {
        gst::init().map_err(|e| format!("Failed to initialize GStreamer: {}", e))?;

        let gst_player = GstPlayer::new(
            None::<gstreamer_player::PlayerVideoRenderer>,
            Some(PlayerGMainContextSignalDispatcher::new(None)),
        );

        // 配置位置更新间隔（每 100 毫秒更新一次）
        let mut config = gst_player.config();
        config.set_position_update_interval(100);
        if let Err(e) = gst_player.set_config(config) {
            eprintln!("[Player] Failed to set config: {}", e);
            return Err(format!("Failed to set player config: {}", e));
        }
        eprintln!("[Player] Position update interval set to 100ms");

        let (event_tx, event_rx) = mpsc::unbounded_channel();

        let inner = Arc::new(Mutex::new(PlayerInner {
            gst_player: gst_player.clone(),
            state: PlayState::Stopped,
        }));

        // 通道的存活由下面各信号回调持有的 `event_tx` 克隆维持，
        // 结构体本身不需要再存一份。
        let player = Self {
            inner: inner.clone(),
        };

        // Connect GStreamer signals
        let event_tx_clone = event_tx.clone();
        gst_player.connect_state_changed(move |_player, state| {
            let play_state = match state {
                gstreamer_player::PlayerState::Stopped => PlayState::Stopped,
                gstreamer_player::PlayerState::Buffering => PlayState::Buffering,
                gstreamer_player::PlayerState::Paused => PlayState::Paused,
                gstreamer_player::PlayerState::Playing => PlayState::Playing,
                _ => return,
            };

            if let Ok(mut inner) = inner.lock() {
                inner.state = play_state;
            }

            let _ = event_tx_clone.send(PlayerEvent::StateChanged(play_state));
        });

        let event_tx_clone = event_tx.clone();
        gst_player.connect_position_updated(move |player, pos| {
            eprintln!("[Player] position_updated signal received, pos={:?}", pos);
            if let Some(pos) = pos {
                // 保持纳秒，由上层转换为秒
                let position = pos.nseconds();
                let duration = player.duration().map(|d| d.nseconds()).unwrap_or(0);

                eprintln!(
                    "[Player] Sending PositionChanged: {} ns / {} ns",
                    position, duration
                );
                let _ = event_tx_clone.send(PlayerEvent::PositionChanged { position, duration });
            }
        });

        // 备用方案：由于 gstreamer-player 已被弃用，position_updated 信号可能不工作
        // 使用定时器主动查询位置
        let gst_player_clone = gst_player.clone();
        let event_tx_clone = event_tx.clone();
        std::thread::spawn(move || loop {
            std::thread::sleep(std::time::Duration::from_millis(100));

            if let Some(pos) = gst_player_clone.position() {
                let position = pos.nseconds();
                let duration = gst_player_clone
                    .duration()
                    .map(|d| d.nseconds())
                    .unwrap_or(0);

                let _ = event_tx_clone.send(PlayerEvent::PositionChanged { position, duration });
            }
        });

        let event_tx_clone = event_tx.clone();
        gst_player.connect_end_of_stream(move |_player| {
            let _ = event_tx_clone.send(PlayerEvent::EndOfStream);
        });

        let event_tx_clone = event_tx.clone();
        gst_player.connect_error(move |_player, err| {
            error!("Player error: {}", err);
            let _ = event_tx_clone.send(PlayerEvent::Error(err.to_string()));
        });

        let event_tx_clone = event_tx.clone();
        gst_player.connect_volume_changed(move |player| {
            let volume = player.volume();
            let _ = event_tx_clone.send(PlayerEvent::VolumeChanged(volume));
        });

        Ok((player, event_rx))
    }

    pub fn handle_command(&self, cmd: PlayerCommand) {
        let inner = match self.inner.lock() {
            Ok(inner) => inner,
            Err(e) => {
                error!("Failed to lock player: {}", e);
                return;
            }
        };

        match cmd {
            PlayerCommand::Play(source) => {
                let uri = match source {
                    StreamSource::LocalPath(path) => {
                        format!("file://{}", path)
                    }
                    StreamSource::Url(url) => url,
                };

                debug!("Playing: {}", uri);
                inner.gst_player.set_uri(Some(&uri));
                inner.gst_player.play();
            }
            PlayerCommand::Pause => {
                inner.gst_player.pause();
            }
            PlayerCommand::Resume => {
                inner.gst_player.play();
            }
            PlayerCommand::Stop => {
                inner.gst_player.stop();
            }
            PlayerCommand::Seek(position) => {
                if let Some(duration) = inner.gst_player.duration() {
                    let seek_pos = (duration.nseconds() as f64 * position) as u64;
                    inner
                        .gst_player
                        .seek(gst::ClockTime::from_nseconds(seek_pos));
                }
            }
            PlayerCommand::SetVolume(volume) => {
                inner.gst_player.set_volume(volume.clamp(0.0, 1.0));
            }
            PlayerCommand::SetEqualizer(_bands) => {
                warn!("Equalizer not yet implemented");
            }
        }
    }

    pub fn get_state(&self) -> PlayState {
        self.inner
            .lock()
            .map(|inner| inner.state)
            .unwrap_or(PlayState::Stopped)
    }
}

// 故意不实现 `Default`：创建播放器可能失败（GStreamer 初始化），
// `Default::default()` 没有返回错误的途径，只能 panic，因此不提供。
