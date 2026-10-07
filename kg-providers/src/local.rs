use async_trait::async_trait;
use kg_core::{
    Error, Lyrics, MusicProvider, Page, Paged, Result, SearchItem, SearchKind, StreamSource, Track,
    TrackId,
};
use lofty::prelude::*;
use lofty::probe::Probe;
use std::path::{Path, PathBuf};
use tokio::fs;
use tracing::{debug, warn};
use walkdir::WalkDir;

pub struct LocalProvider {
    library_paths: Vec<PathBuf>,
}

impl LocalProvider {
    pub fn new(paths: Vec<PathBuf>) -> Self {
        Self {
            library_paths: paths,
        }
    }

    /// 扫描目录获取所有音频文件
    pub async fn scan(&self) -> Result<Vec<Track>> {
        let mut tracks = Vec::new();

        for lib_path in &self.library_paths {
            debug!("Scanning library path: {:?}", lib_path);

            if !lib_path.exists() {
                warn!("Library path does not exist: {:?}", lib_path);
                continue;
            }

            for entry in WalkDir::new(lib_path)
                .follow_links(true)
                .into_iter()
                .filter_map(|e| e.ok())
            {
                let path = entry.path();
                if !path.is_file() {
                    continue;
                }

                if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                    let ext_lower = ext.to_lowercase();
                    if matches!(
                        ext_lower.as_str(),
                        "mp3" | "flac" | "ogg" | "wav" | "m4a" | "aac" | "opus" | "wv" | "ape"
                    ) {
                        match Self::parse_track(path).await {
                            Ok(track) => tracks.push(track),
                            Err(e) => {
                                debug!("Failed to parse {}: {}", path.display(), e);
                            }
                        }
                    }
                }
            }
        }

        debug!("Scanned {} tracks", tracks.len());
        Ok(tracks)
    }

    async fn parse_track(path: &Path) -> Result<Track> {
        let path_str = path
            .to_str()
            .ok_or_else(|| Error::InvalidData("Invalid path".into()))?
            .to_string();

        let tagged_file = Probe::open(path)
            .map_err(|e| Error::Parse(format!("Failed to probe file: {}", e)))?
            .read()
            .map_err(|e| Error::Parse(format!("Failed to read tags: {}", e)))?;

        let properties = tagged_file.properties();
        let duration_secs = properties.duration().as_secs();

        let tag = tagged_file
            .primary_tag()
            .or_else(|| tagged_file.first_tag());

        let (title, artist, album, year, track_number, genre) = if let Some(tag) = tag {
            (
                tag.title()
                    .map(|s| s.to_string())
                    .unwrap_or_else(|| Self::fallback_title(path)),
                tag.artist()
                    .map(|s| s.to_string())
                    .unwrap_or_else(|| "Unknown Artist".into()),
                tag.album()
                    .map(|s| s.to_string())
                    .unwrap_or_else(|| "Unknown Album".into()),
                tag.year(),
                tag.track(),
                tag.genre().map(|s| s.to_string()),
            )
        } else {
            (
                Self::fallback_title(path),
                "Unknown Artist".into(),
                "Unknown Album".into(),
                None,
                None,
                None,
            )
        };

        Ok(Track {
            id: TrackId::local(&path_str),
            title,
            artist,
            album,
            duration_secs,
            album_art_url: None,
            year,
            track_number,
            genre,
        })
    }

    fn fallback_title(path: &Path) -> String {
        path.file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("Unknown")
            .to_string()
    }
}

#[async_trait]
impl MusicProvider for LocalProvider {
    fn id(&self) -> &'static str {
        "local"
    }

    fn name(&self) -> &'static str {
        "Local Library"
    }

    async fn search(
        &self,
        query: &str,
        kind: SearchKind,
        _page: Page,
    ) -> Result<Paged<SearchItem>> {
        // 简单实现：扫描并过滤
        let tracks = self.scan().await?;
        let query_lower = query.to_lowercase();

        let filtered: Vec<SearchItem> = tracks
            .into_iter()
            .filter(|t| match kind {
                SearchKind::Track => {
                    t.title.to_lowercase().contains(&query_lower)
                        || t.artist.to_lowercase().contains(&query_lower)
                }
                SearchKind::Album => t.album.to_lowercase().contains(&query_lower),
                SearchKind::Artist => t.artist.to_lowercase().contains(&query_lower),
            })
            .map(SearchItem::Track)
            .collect();

        let total = filtered.len() as u32;

        Ok(Paged {
            items: filtered,
            total,
            page: Page::default(),
        })
    }

    async fn track(&self, id: &TrackId) -> Result<Track> {
        if id.provider != "local" {
            return Err(Error::InvalidData(format!(
                "Invalid provider: {}",
                id.provider
            )));
        }

        let path = Path::new(&id.id);
        Self::parse_track(path).await
    }

    async fn stream_source(&self, id: &TrackId) -> Result<StreamSource> {
        if id.provider != "local" {
            return Err(Error::InvalidData(format!(
                "Invalid provider: {}",
                id.provider
            )));
        }

        let path = Path::new(&id.id);
        if !path.exists() {
            return Err(Error::NotFound(format!("File not found: {:?}", path)));
        }

        Ok(StreamSource::LocalPath(id.id.clone()))
    }

    async fn lyrics(&self, id: &TrackId) -> Result<Option<Lyrics>> {
        if id.provider != "local" {
            return Err(Error::InvalidData(format!(
                "Invalid provider: {}",
                id.provider
            )));
        }

        let audio_path = Path::new(&id.id);
        let lrc_path = audio_path.with_extension("lrc");

        if lrc_path.exists() {
            let content = fs::read_to_string(&lrc_path).await.map_err(Error::Io)?;

            return Ok(Some(Lyrics {
                format: kg_core::LyricsFormat::Lrc,
                content,
            }));
        }

        // TODO: 读取内嵌歌词
        Ok(None)
    }
}
