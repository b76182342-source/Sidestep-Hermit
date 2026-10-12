use crate::{AIBackend, ControlAction, Intent};
use async_trait::async_trait;
use kg_core::Result;
use regex::Regex;
use std::sync::OnceLock;

/// Regex-based intent recognition backend
#[derive(Debug, Clone, Default)]
pub struct RegexBackend;

impl RegexBackend {
    pub fn new() -> Self {
        Self
    }

    fn parse_intent(&self, query: &str) -> Intent {
        // Normalize query: lowercase and trim
        let query = query.trim().to_lowercase();

        // Control actions
        if let Some(action) = self.parse_control(&query) {
            return Intent::Control { action };
        }

        // Play patterns
        if let Some(intent) = self.parse_play(&query) {
            return intent;
        }

        // Filter patterns
        if let Some(intent) = self.parse_filter(&query) {
            return intent;
        }

        // Unknown
        Intent::Unknown {
            query: query.to_string(),
        }
    }

    fn parse_control(&self, query: &str) -> Option<ControlAction> {
        static PAUSE_RE: OnceLock<Regex> = OnceLock::new();
        static PLAY_RE: OnceLock<Regex> = OnceLock::new();
        static NEXT_RE: OnceLock<Regex> = OnceLock::new();
        static PREV_RE: OnceLock<Regex> = OnceLock::new();
        static STOP_RE: OnceLock<Regex> = OnceLock::new();

        let pause_re = PAUSE_RE.get_or_init(|| Regex::new(r"^(暂停|pause)$").unwrap());
        let play_re = PLAY_RE.get_or_init(|| Regex::new(r"^(播放|play|继续)$").unwrap());
        let next_re = NEXT_RE.get_or_init(|| Regex::new(r"^(下一首|next|下一曲)$").unwrap());
        let prev_re =
            PREV_RE.get_or_init(|| Regex::new(r"^(上一首|previous|prev|上一曲)$").unwrap());
        let stop_re = STOP_RE.get_or_init(|| Regex::new(r"^(停止|stop)$").unwrap());

        if pause_re.is_match(query) {
            Some(ControlAction::Pause)
        } else if play_re.is_match(query) {
            Some(ControlAction::Play)
        } else if next_re.is_match(query) {
            Some(ControlAction::Next)
        } else if prev_re.is_match(query) {
            Some(ControlAction::Previous)
        } else if stop_re.is_match(query) {
            Some(ControlAction::Stop)
        } else {
            None
        }
    }

    fn parse_play(&self, query: &str) -> Option<Intent> {
        static PLAY_ARTIST_RE: OnceLock<Regex> = OnceLock::new();
        static PLAY_TITLE_RE: OnceLock<Regex> = OnceLock::new();

        let play_artist_re = PLAY_ARTIST_RE
            .get_or_init(|| Regex::new(r"^(?:播放|play|来点)\s*(.+?)(?:的歌|的音乐)?$").unwrap());
        let play_title_re = PLAY_TITLE_RE
            .get_or_init(|| Regex::new(r"^(?:播放|play)\s*[《「]?(.+?)[》」]?$").unwrap());

        // Try to match "播放周杰伦" or "播放周杰伦的歌"
        if let Some(caps) = play_artist_re.captures(query) {
            let text = caps.get(1)?.as_str().trim();

            // If it contains brackets, treat as title
            if text.contains('《') || text.contains('「') {
                return Some(Intent::Play {
                    artist: None,
                    title: Some(text.replace(['《', '》', '「', '」'], "")),
                });
            }

            // Otherwise treat as artist
            return Some(Intent::Play {
                artist: Some(text.to_string()),
                title: None,
            });
        }

        // Try to match "播放《告白气球》"
        if let Some(caps) = play_title_re.captures(query) {
            let title = caps.get(1)?.as_str().trim();
            return Some(Intent::Play {
                artist: None,
                title: Some(title.to_string()),
            });
        }

        None
    }

    fn parse_filter(&self, query: &str) -> Option<Intent> {
        static MOOD_RE: OnceLock<Regex> = OnceLock::new();
        static GENRE_RE: OnceLock<Regex> = OnceLock::new();

        let mood_re = MOOD_RE.get_or_init(|| {
            Regex::new(r"(?:来点|播放)?\s*(轻音乐|舒缓|放松|激情|欢快|伤感|安静)").unwrap()
        });
        let genre_re = GENRE_RE.get_or_init(|| {
            Regex::new(r"(?:来点|播放)?\s*(摇滚|流行|民谣|古典|电子|爵士|说唱|hip[\s-]?hop)")
                .unwrap()
        });

        // Check mood
        if let Some(caps) = mood_re.captures(query) {
            let mood = caps.get(1)?.as_str();
            let mood_en = match mood {
                "轻音乐" | "舒缓" | "放松" | "安静" => "Calm",
                "激情" | "欢快" => "Energetic",
                "伤感" => "Melancholic",
                _ => mood,
            };
            return Some(Intent::Filter {
                mood: Some(mood_en.to_string()),
                genre: None,
            });
        }

        // Check genre
        if let Some(caps) = genre_re.captures(query) {
            let genre = caps.get(1)?.as_str();
            let genre_en = match genre {
                "摇滚" => "Rock",
                "流行" => "Pop",
                "民谣" => "Folk",
                "古典" => "Classical",
                "电子" => "Electronic",
                "爵士" => "Jazz",
                "说唱" => "Hip-Hop",
                g if g.contains("hip") => "Hip-Hop",
                _ => genre,
            };
            return Some(Intent::Filter {
                mood: None,
                genre: Some(genre_en.to_string()),
            });
        }

        None
    }
}

#[async_trait]
impl AIBackend for RegexBackend {
    async fn intent(&self, query: &str) -> Result<Intent> {
        Ok(self.parse_intent(query))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_control_actions() {
        let backend = RegexBackend::new();

        let tests = vec![
            ("暂停", ControlAction::Pause),
            ("pause", ControlAction::Pause),
            ("播放", ControlAction::Play),
            ("play", ControlAction::Play),
            ("继续", ControlAction::Play),
            ("下一首", ControlAction::Next),
            ("next", ControlAction::Next),
            ("上一首", ControlAction::Previous),
            ("prev", ControlAction::Previous),
            ("停止", ControlAction::Stop),
            ("stop", ControlAction::Stop),
        ];

        for (query, expected) in tests {
            let intent = backend.intent(query).await.unwrap();
            if let Intent::Control { action } = intent {
                assert_eq!(
                    format!("{:?}", action),
                    format!("{:?}", expected),
                    "Failed for query: {}",
                    query
                );
            } else {
                panic!("Expected Control intent for query: {}", query);
            }
        }
    }

    #[tokio::test]
    async fn test_play_artist() {
        let backend = RegexBackend::new();

        let tests = vec!["播放周杰伦", "play 周杰伦", "来点周杰伦", "播放周杰伦的歌"];

        for query in tests {
            let intent = backend.intent(query).await.unwrap();
            if let Intent::Play { artist, title } = intent {
                assert_eq!(artist, Some("周杰伦".to_string()), "Query: {}", query);
                assert_eq!(title, None, "Query: {}", query);
            } else {
                panic!("Expected Play intent for query: {}", query);
            }
        }
    }

    #[tokio::test]
    async fn test_play_title() {
        let backend = RegexBackend::new();

        let tests = vec![
            ("播放告白气球", "告白气球"),
            ("play 告白气球", "告白气球"),
            ("播放《告白气球》", "告白气球"),
        ];

        for (query, expected_title) in tests {
            let intent = backend.intent(query).await.unwrap();
            if let Intent::Play { artist, title } = intent {
                assert_eq!(title, Some(expected_title.to_string()), "Query: {}", query);
                assert!(
                    artist.is_none() || artist == Some(expected_title.to_string()),
                    "Query: {}",
                    query
                );
            } else {
                panic!("Expected Play intent for query: {}", query);
            }
        }
    }

    #[tokio::test]
    async fn test_filter_mood() {
        let backend = RegexBackend::new();

        let tests = vec![
            ("来点轻音乐", "Calm"),
            ("播放舒缓", "Calm"),
            ("来点欢快", "Energetic"),
            ("播放伤感", "Melancholic"),
        ];

        for (query, expected_mood) in tests {
            let intent = backend.intent(query).await.unwrap();
            if let Intent::Filter { mood, genre } = intent {
                assert_eq!(mood, Some(expected_mood.to_string()), "Query: {}", query);
                assert_eq!(genre, None, "Query: {}", query);
            } else {
                panic!("Expected Filter intent for query: {}", query);
            }
        }
    }

    #[tokio::test]
    async fn test_filter_genre() {
        let backend = RegexBackend::new();

        let tests = vec![
            ("来点摇滚", "Rock"),
            ("播放流行", "Pop"),
            ("来点爵士", "Jazz"),
            ("播放hip-hop", "Hip-Hop"),
        ];

        for (query, expected_genre) in tests {
            let intent = backend.intent(query).await.unwrap();
            if let Intent::Filter { mood, genre } = intent {
                assert_eq!(genre, Some(expected_genre.to_string()), "Query: {}", query);
                assert_eq!(mood, None, "Query: {}", query);
            } else {
                panic!("Expected Filter intent for query: {}", query);
            }
        }
    }

    #[tokio::test]
    async fn test_unknown() {
        let backend = RegexBackend::new();

        let intent = backend.intent("这是什么鬼").await.unwrap();
        assert!(matches!(intent, Intent::Unknown { .. }));
    }
}
