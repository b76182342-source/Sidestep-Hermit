use kg_core::{PlayState, Track};
use std::collections::HashMap;
use tokio::sync::mpsc;
use tracing::debug;
use zbus::{connection::Builder, interface, Connection};

use crate::controller::MprisCommand;

pub struct MprisServer {
    _connection: Connection,
    _cmd_tx: mpsc::UnboundedSender<MprisCommand>,
}

impl MprisServer {
    pub async fn new(
        cmd_tx: mpsc::UnboundedSender<MprisCommand>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let player = Player::new(cmd_tx.clone());

        let connection = Builder::session()?
            .name("org.mpris.MediaPlayer2.kgplayer")?
            .serve_at("/org/mpris/MediaPlayer2", player)?
            .build()
            .await?;

        debug!("MPRIS2 server listening on session bus");

        Ok(Self {
            _connection: connection,
            _cmd_tx: cmd_tx,
        })
    }

    pub async fn set_playback_status(
        &self,
        _state: PlayState,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // TODO: 发送 PropertiesChanged 信号
        Ok(())
    }

    pub async fn set_metadata(&self, _track: &Track) -> Result<(), Box<dyn std::error::Error>> {
        // TODO: 更新元数据并发送信号
        Ok(())
    }

    pub async fn set_position(&self, _position_ns: u64) -> Result<(), Box<dyn std::error::Error>> {
        // TODO: 发送 Seeked 信号
        Ok(())
    }
}

struct Player {
    cmd_tx: mpsc::UnboundedSender<MprisCommand>,
}

impl Player {
    fn new(cmd_tx: mpsc::UnboundedSender<MprisCommand>) -> Self {
        Self { cmd_tx }
    }
}

#[interface(name = "org.mpris.MediaPlayer2.Player")]
impl Player {
    async fn play(&self) {
        let _ = self.cmd_tx.send(MprisCommand::Play);
    }

    async fn pause(&self) {
        let _ = self.cmd_tx.send(MprisCommand::Pause);
    }

    async fn play_pause(&self) {
        let _ = self.cmd_tx.send(MprisCommand::PlayPause);
    }

    async fn stop(&self) {
        let _ = self.cmd_tx.send(MprisCommand::Stop);
    }

    async fn next(&self) {
        let _ = self.cmd_tx.send(MprisCommand::Next);
    }

    async fn previous(&self) {
        let _ = self.cmd_tx.send(MprisCommand::Previous);
    }

    async fn seek(&self, offset: i64) {
        let _ = self.cmd_tx.send(MprisCommand::Seek(offset));
    }

    #[zbus(property)]
    async fn playback_status(&self) -> String {
        "Stopped".to_string()
    }

    #[zbus(property)]
    async fn metadata(&self) -> HashMap<String, zbus::zvariant::Value<'static>> {
        HashMap::new()
    }

    #[zbus(property)]
    async fn can_play(&self) -> bool {
        true
    }

    #[zbus(property)]
    async fn can_pause(&self) -> bool {
        true
    }

    #[zbus(property)]
    async fn can_go_next(&self) -> bool {
        true
    }

    #[zbus(property)]
    async fn can_go_previous(&self) -> bool {
        true
    }

    #[zbus(property)]
    async fn can_seek(&self) -> bool {
        true
    }
}
