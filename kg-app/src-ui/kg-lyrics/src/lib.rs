use kg_core::{Lyrics, LyricsFormat, Result, TrackId, MusicProvider, Error};
use lofty::{AudioFile, Probe, TaggedFileExt};
use std::path::Path;
use tokio::fs;
use tracing::debug;

pub async fn get_lyrics_for_track(
    track_id: &TrackId,
    provider: &impl MusicProvider,
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
            let content = fs::read_to_string(&lrc_path)
                .await
                .map_err(|e| Error::Io(e))?;

            return Ok(Some(Lyrics {
                format: LyricsFormat::Lrc,
                content,
            }));
        }

        // 尝试读取内嵌歌词
        if let Ok(lyrics) = extract_embedded_lyrics(audio_path).await {
            return Ok(Some(lyrics));
        }
    }

    Ok(None)
}

async fn extract_embedded_lyrics(path: &Path) -> Result<Lyrics> {
    let tagged_file = Probe::open(path)
        .map_err(|e| Error::Parse(format!("Failed to probe file: {}", e)))?
        .read()
        .map_err(|e| Error::Parse(format!("Failed to read tags: {}", e)))?;

    let tag = tagged_file
        .primary_tag()
        .or_else(|| tagged_file.first_tag())
        .ok_or_else(|| Error::NotFound("No tags found".into()))?;

    // 尝试读取 USLT (未同步歌词) 或 SYLT (同步歌词)
    // lofty 0.22 暂不直接支持，这里返回占位
    // TODO: 升级到支持的版本或手动解析

    Err(Error::NotFound("Embedded lyrics not found".into()))
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
        if let Some(pictures) = tag.pictures().first() {
            debug!("Found album art: {} bytes", pictures.data().len());
            return Ok(Some(pictures.data().to_vec()));
        }
    }

    Ok(None)
}
