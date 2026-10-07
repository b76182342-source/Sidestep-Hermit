use kg_core::{PlayState, Track};
use tokio::sync::mpsc;
use tracing::{error, info};

use crate::server::MprisServer;

pub struct MprisController {
    server: MprisServer,
}

impl MprisController {
    pub async fn new(
        cmd_tx: mpsc::UnboundedSender<MprisCommand>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let server = MprisServer::new(cmd_tx).await?;
        info!("MPRIS2 server started");
        Ok(Self { server })
    }

    pub async fn update_state(&self, state: PlayState) {
        if let Err(e) = self.server.set_playback_status(state).await {
            error!("Failed to update MPRIS state: {}", e);
        }
    }

    pub async fn update_metadata(&self, track: &Track) {
        if let Err(e) = self.server.set_metadata(track).await {
            error!("Failed to update MPRIS metadata: {}", e);
        }
    }

    pub async fn update_position(&self, position_ns: u64) {
        if let Err(e) = self.server.set_position(position_ns).await {
            error!("Failed to update MPRIS position: {}", e);
        }
    }
}

#[derive(Debug, Clone)]
pub enum MprisCommand {
    Play,
    Pause,
    PlayPause,
    Stop,
    Next,
    Previous,
    Seek(i64), // 微秒
}
