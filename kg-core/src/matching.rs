//! 歌曲匹配引擎
//!
//! 用于跨平台播放列表同步和智能音源切换

use crate::Track;
use std::cmp::Ordering;

/// 歌曲元数据指纹（用于匹配）
#[derive(Debug, Clone)]
pub struct SongFingerprint {
    pub title: String,
    pub artist: String,
    pub album: String,
    pub duration_secs: u64,
}

impl SongFingerprint {
    /// 从 Track 创建指纹
    pub fn from_track(track: &Track) -> Self {
        Self {
            title: normalize_text(&track.title),
            artist: normalize_text(&track.artist),
            album: normalize_text(&track.album),
            duration_secs: track.duration_secs,
        }
    }

    /// 计算与另一首歌的匹配度（0-100）
    pub fn similarity(&self, other: &Self) -> u8 {
        let mut score = 0u32;

        // 标题匹配（权重 40%）
        if self.title == other.title {
            score += 40;
        } else if fuzzy_match(&self.title, &other.title) {
            score += 25;
        }

        // 艺术家匹配（权重 35%）
        if self.artist == other.artist {
            score += 35;
        } else if fuzzy_match(&self.artist, &other.artist) {
            score += 20;
        }

        // 专辑匹配（权重 15%）
        if !self.album.is_empty() && self.album == other.album {
            score += 15;
        } else if fuzzy_match(&self.album, &other.album) {
            score += 8;
        }

        // 时长匹配（权重 10%，容差 3 秒）
        if self.duration_secs.abs_diff(other.duration_secs) <= 3 {
            score += 10;
        } else if self.duration_secs.abs_diff(other.duration_secs) <= 10 {
            score += 5;
        }

        score.min(100) as u8
    }
}

/// 音质评分（用于智能音源切换）
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QualityScore {
    pub bitrate_kbps: u32,
    pub format_score: u8, // FLAC(3) > AAC(2) > MP3(1)
    pub availability: bool,
}

impl QualityScore {
    /// 计算总分
    pub fn total(&self) -> u32 {
        if !self.availability {
            return 0;
        }
        (self.bitrate_kbps / 10) + (self.format_score as u32 * 100)
    }
}

impl PartialOrd for QualityScore {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.total().cmp(&other.total()))
    }
}

impl Ord for QualityScore {
    fn cmp(&self, other: &Self) -> Ordering {
        self.total().cmp(&other.total())
    }
}

/// 文本标准化（去除特殊字符、转小写）
fn normalize_text(text: &str) -> String {
    text.trim()
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric() || c.is_whitespace())
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

/// 模糊匹配（处理版本后缀、feat、括号内容）
fn fuzzy_match(a: &str, b: &str) -> bool {
    if a.is_empty() || b.is_empty() {
        return false;
    }

    // 移除括号内容
    let clean_a = remove_parentheses(a);
    let clean_b = remove_parentheses(b);

    // Levenshtein 距离 <= 2 或包含关系
    levenshtein(&clean_a, &clean_b) <= 2 || clean_a.contains(&clean_b) || clean_b.contains(&clean_a)
}

/// 移除括号内容
fn remove_parentheses(s: &str) -> String {
    s.split(|c| c == '(' || c == '（')
        .next()
        .unwrap_or(s)
        .trim()
        .to_string()
}

/// 简单 Levenshtein 距离
fn levenshtein(a: &str, b: &str) -> usize {
    let a_chars: Vec<char> = a.chars().collect();
    let b_chars: Vec<char> = b.chars().collect();
    let mut dp = vec![vec![0; b_chars.len() + 1]; a_chars.len() + 1];

    for i in 0..=a_chars.len() {
        dp[i][0] = i;
    }
    for j in 0..=b_chars.len() {
        dp[0][j] = j;
    }

    for i in 1..=a_chars.len() {
        for j in 1..=b_chars.len() {
            let cost = if a_chars[i - 1] == b_chars[j - 1] {
                0
            } else {
                1
            };
            dp[i][j] = (dp[i - 1][j] + 1)
                .min(dp[i][j - 1] + 1)
                .min(dp[i - 1][j - 1] + cost);
        }
    }

    dp[a_chars.len()][b_chars.len()]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_text() {
        assert_eq!(normalize_text("  Hello World  "), "hello world");
        assert_eq!(normalize_text("Hello-World!"), "helloworld");
    }

    #[test]
    fn test_remove_parentheses() {
        assert_eq!(remove_parentheses("歌曲名 (Live版)"), "歌曲名");
        assert_eq!(remove_parentheses("歌曲名（现场版）"), "歌曲名");
    }

    #[test]
    fn test_levenshtein() {
        assert_eq!(levenshtein("cat", "hat"), 1);
        assert_eq!(levenshtein("kitten", "sitting"), 3);
        assert_eq!(levenshtein("", "abc"), 3);
    }

    #[test]
    fn test_similarity() {
        let track1 = Track {
            id: crate::TrackId::new("local", "1"),
            title: "告白气球".to_string(),
            artist: "周杰伦".to_string(),
            album: "周杰伦的床边故事".to_string(),
            duration_secs: 213,
            album_art_url: None,
            year: Some(2016),
            track_number: None,
            genre: None,
        };

        let track2 = Track {
            id: crate::TrackId::new("kugou", "2"),
            title: "告白气球".to_string(),
            artist: "周杰伦".to_string(),
            album: "周杰伦的床边故事".to_string(),
            duration_secs: 213,
            album_art_url: None,
            year: Some(2016),
            track_number: None,
            genre: None,
        };

        let fp1 = SongFingerprint::from_track(&track1);
        let fp2 = SongFingerprint::from_track(&track2);

        // 完全相同应该是 100 分
        assert_eq!(fp1.similarity(&fp2), 100);
    }

    #[test]
    fn test_quality_score() {
        let high = QualityScore {
            bitrate_kbps: 320,
            format_score: 3,
            availability: true,
        };

        let low = QualityScore {
            bitrate_kbps: 128,
            format_score: 1,
            availability: true,
        };

        assert!(high > low);
        assert_eq!(high.total(), 332); // 320/10 + 3*100
    }
}
