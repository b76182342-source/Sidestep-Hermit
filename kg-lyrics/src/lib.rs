pub mod lrc;
pub mod sync;

pub use lrc::{parse_lrc, LrcLine, LrcParser};
pub use sync::LyricsSynchronizer;

use kg_core::{Error, Lyrics, LyricsFormat, MusicProvider, Result, TrackId};
use lofty::prelude::*;
use lofty::probe::Probe;
use std::path::Path;
use tokio::fs;
use tracing::debug;

pub async fn get_lyrics_for_track(
    track_id: &TrackId,
    provider: &dyn MusicProvider,
) -> Result<Option<Lyrics>> {
    // 先尝试从 provider 获取
    if let Ok(Some(lyrics)) = provider.lyrics(track_id).await {
        return Ok(Some(lyrics));
    }

    // 对于本地文件，尝试读取同名 .lrc 文件
    if track_id.provider == "local" {
        let audio_path = Path::new(&track_id.id);
        let lrc_path = audio_path.with_extension("lrc");

        if lrc_path.exists() {
            let content = fs::read_to_string(&lrc_path).await.map_err(Error::Io)?;

            return Ok(Some(Lyrics {
                format: LyricsFormat::Lrc,
                content,
            }));
        }
    }

    Ok(None)
}

/// 提取专辑封面
pub async fn extract_album_art(track_id: &TrackId) -> Result<Option<Vec<u8>>> {
    if track_id.provider != "local" {
        return Ok(None);
    }

    let path = Path::new(&track_id.id);

    let tagged_file = Probe::open(path)
        .map_err(|e| Error::Parse(format!("Failed to probe file: {}", e)))?
        .read()
        .map_err(|e| Error::Parse(format!("Failed to read tags: {}", e)))?;

    let tag = tagged_file
        .primary_tag()
        .or_else(|| tagged_file.first_tag());

    if let Some(tag) = tag {
        if let Some(picture) = tag.pictures().first() {
            debug!("Found album art: {} bytes", picture.data().len());
            return Ok(Some(picture.data().to_vec()));
        }
    }

    Ok(None)
}
