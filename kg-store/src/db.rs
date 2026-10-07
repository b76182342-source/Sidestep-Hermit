use anyhow::{Context, Result};
use directories::ProjectDirs;
use kg_core::{Playlist, TrackId};
use rusqlite::{params, Connection};
use std::path::PathBuf;
use uuid::Uuid;

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new() -> Result<Self> {
        let db_path = Self::get_db_path()?;

        // 确保目录存在
        if let Some(parent) = db_path.parent() {
            std::fs::create_dir_all(parent).context("Failed to create database directory")?;
        }

        let conn = Connection::open(&db_path).context("Failed to open database")?;

        let mut db = Self { conn };
        db.migrate()?;

        Ok(db)
    }

    fn get_db_path() -> Result<PathBuf> {
        let proj_dirs = ProjectDirs::from("com", "kgplayer", "kgplayer")
            .context("Failed to determine project directories")?;
        Ok(proj_dirs.data_dir().join("library.db"))
    }

    fn migrate(&mut self) -> Result<()> {
        let version: i32 = self
            .conn
            .query_row("PRAGMA user_version", [], |row| row.get(0))
            .unwrap_or(0);

        if version < 1 {
            self.conn.execute_batch(
                r#"
                CREATE TABLE IF NOT EXISTS tracks (
                    id TEXT PRIMARY KEY,
                    provider TEXT NOT NULL,
                    title TEXT NOT NULL,
                    artist TEXT NOT NULL,
                    album TEXT NOT NULL,
                    duration_secs INTEGER NOT NULL,
                    album_art_url TEXT,
                    year INTEGER,
                    track_number INTEGER,
                    genre TEXT,
                    added_at INTEGER NOT NULL,
                    play_count INTEGER DEFAULT 0,
                    last_played_at INTEGER
                );

                CREATE INDEX idx_tracks_provider ON tracks(provider);
                CREATE INDEX idx_tracks_artist ON tracks(artist);
                CREATE INDEX idx_tracks_album ON tracks(album);

                CREATE VIRTUAL TABLE tracks_fts USING fts5(
                    id UNINDEXED,
                    title,
                    artist,
                    album,
                    content=tracks,
                    content_rowid=rowid
                );

                CREATE TRIGGER tracks_ai AFTER INSERT ON tracks BEGIN
                    INSERT INTO tracks_fts(rowid, id, title, artist, album)
                    VALUES (new.rowid, new.id, new.title, new.artist, new.album);
                END;

                CREATE TRIGGER tracks_ad AFTER DELETE ON tracks BEGIN
                    DELETE FROM tracks_fts WHERE rowid = old.rowid;
                END;

                CREATE TRIGGER tracks_au AFTER UPDATE ON tracks BEGIN
                    UPDATE tracks_fts SET title = new.title, artist = new.artist, album = new.album
                    WHERE rowid = new.rowid;
                END;

                CREATE TABLE IF NOT EXISTS playlists (
                    id TEXT PRIMARY KEY,
                    name TEXT NOT NULL,
                    created_at INTEGER NOT NULL,
                    updated_at INTEGER NOT NULL
                );

                CREATE TABLE IF NOT EXISTS playlist_tracks (
                    playlist_id TEXT NOT NULL,
                    track_provider TEXT NOT NULL,
                    track_id TEXT NOT NULL,
                    position INTEGER NOT NULL,
                    FOREIGN KEY (playlist_id) REFERENCES playlists(id) ON DELETE CASCADE
                );

                CREATE INDEX idx_playlist_tracks ON playlist_tracks(playlist_id);

                CREATE TABLE IF NOT EXISTS queue (
                    position INTEGER PRIMARY KEY,
                    track_provider TEXT NOT NULL,
                    track_id TEXT NOT NULL
                );

                CREATE TABLE IF NOT EXISTS settings (
                    key TEXT PRIMARY KEY,
                    value TEXT NOT NULL
                );

                PRAGMA user_version = 1;
                "#,
            )?;
        }

        Ok(())
    }

    /// 保存设置
    pub fn set_setting(&self, key: &str, value: &str) -> Result<()> {
        self.conn.execute(
            "INSERT OR REPLACE INTO settings (key, value) VALUES (?1, ?2)",
            params![key, value],
        )?;
        Ok(())
    }

    /// 获取设置
    pub fn get_setting(&self, key: &str) -> Result<Option<String>> {
        let result = self.conn.query_row(
            "SELECT value FROM settings WHERE key = ?1",
            params![key],
            |row| row.get(0),
        );

        match result {
            Ok(value) => Ok(Some(value)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    /// 全文搜索
    pub fn search_tracks(&self, query: &str, limit: usize) -> Result<Vec<String>> {
        let mut stmt = self
            .conn
            .prepare("SELECT id FROM tracks_fts WHERE tracks_fts MATCH ?1 LIMIT ?2")?;

        let ids: Vec<String> = stmt
            .query_map(params![query, limit], |row| row.get(0))?
            .filter_map(|r| r.ok())
            .collect();

        Ok(ids)
    }

    /// 创建播放列表
    pub fn create_playlist(&self, name: &str) -> Result<Playlist> {
        let id = Uuid::new_v4();
        let now = chrono::Utc::now().timestamp();

        self.conn.execute(
            "INSERT INTO playlists (id, name, created_at, updated_at) VALUES (?1, ?2, ?3, ?4)",
            params![id.to_string(), name, now, now],
        )?;

        Ok(Playlist {
            id,
            name: name.to_string(),
            tracks: Vec::new(),
            created_at: now,
            updated_at: now,
        })
    }

    /// 获取所有播放列表
    pub fn get_playlists(&self) -> Result<Vec<Playlist>> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, name, created_at, updated_at FROM playlists ORDER BY name")?;

        let playlists = stmt
            .query_map([], |row| {
                let id: String = row.get(0)?;
                let name: String = row.get(1)?;
                let created_at: i64 = row.get(2)?;
                let updated_at: i64 = row.get(3)?;

                let uuid = Uuid::parse_str(&id).map_err(|e| {
                    rusqlite::Error::FromSqlConversionFailure(
                        0,
                        rusqlite::types::Type::Text,
                        Box::new(e),
                    )
                })?;

                Ok((uuid, name, created_at, updated_at))
            })?
            .filter_map(|r| r.ok())
            .collect::<Vec<_>>();

        let mut result = Vec::new();
        for (id, name, created_at, updated_at) in playlists {
            let tracks = self.get_playlist_tracks(&id)?;
            result.push(Playlist {
                id,
                name,
                tracks,
                created_at,
                updated_at,
            });
        }

        Ok(result)
    }

    /// 获取播放列表中的曲目
    fn get_playlist_tracks(&self, playlist_id: &Uuid) -> Result<Vec<TrackId>> {
        let mut stmt = self.conn.prepare(
            "SELECT track_provider, track_id FROM playlist_tracks
             WHERE playlist_id = ?1 ORDER BY position",
        )?;

        let tracks = stmt
            .query_map(params![playlist_id.to_string()], |row| {
                let provider: String = row.get(0)?;
                let id: String = row.get(1)?;
                Ok(TrackId { provider, id })
            })?
            .filter_map(|r| r.ok())
            .collect();

        Ok(tracks)
    }

    /// 添加曲目到播放列表
    pub fn add_to_playlist(&self, playlist_id: &Uuid, track_id: &TrackId) -> Result<()> {
        let now = chrono::Utc::now().timestamp();

        // 获取当前最大位置
        let max_position: i32 = self
            .conn
            .query_row(
                "SELECT COALESCE(MAX(position), -1) FROM playlist_tracks WHERE playlist_id = ?1",
                params![playlist_id.to_string()],
                |row| row.get(0),
            )
            .unwrap_or(-1);

        self.conn.execute(
            "INSERT INTO playlist_tracks (playlist_id, track_provider, track_id, position)
             VALUES (?1, ?2, ?3, ?4)",
            params![
                playlist_id.to_string(),
                &track_id.provider,
                &track_id.id,
                max_position + 1
            ],
        )?;

        // 更新播放列表修改时间
        self.conn.execute(
            "UPDATE playlists SET updated_at = ?1 WHERE id = ?2",
            params![now, playlist_id.to_string()],
        )?;

        Ok(())
    }

    /// 从播放列表删除曲目
    pub fn remove_from_playlist(
        &self,
        playlist_id: &Uuid,
        track_id: &TrackId,
        position: i32,
    ) -> Result<()> {
        let now = chrono::Utc::now().timestamp();

        self.conn.execute(
            "DELETE FROM playlist_tracks
             WHERE playlist_id = ?1 AND track_provider = ?2 AND track_id = ?3 AND position = ?4",
            params![
                playlist_id.to_string(),
                &track_id.provider,
                &track_id.id,
                position
            ],
        )?;

        // 更新后续曲目的位置
        self.conn.execute(
            "UPDATE playlist_tracks SET position = position - 1
             WHERE playlist_id = ?1 AND position > ?2",
            params![playlist_id.to_string(), position],
        )?;

        // 更新播放列表修改时间
        self.conn.execute(
            "UPDATE playlists SET updated_at = ?1 WHERE id = ?2",
            params![now, playlist_id.to_string()],
        )?;

        Ok(())
    }

    /// 删除播放列表
    pub fn delete_playlist(&self, playlist_id: &Uuid) -> Result<()> {
        self.conn.execute(
            "DELETE FROM playlists WHERE id = ?1",
            params![playlist_id.to_string()],
        )?;
        // playlist_tracks 会通过外键级联删除
        Ok(())
    }

    /// 重命名播放列表
    pub fn rename_playlist(&self, playlist_id: &Uuid, new_name: &str) -> Result<()> {
        let now = chrono::Utc::now().timestamp();

        self.conn.execute(
            "UPDATE playlists SET name = ?1, updated_at = ?2 WHERE id = ?3",
            params![new_name, now, playlist_id.to_string()],
        )?;

        Ok(())
    }
}
