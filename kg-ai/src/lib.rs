pub mod regex_backend;

use async_trait::async_trait;
use kg_core::Result;
use serde::{Deserialize, Serialize};

/// AI intent recognition result
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Intent {
    /// Play specific song
    Play {
        #[serde(skip_serializing_if = "Option::is_none")]
        artist: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        title: Option<String>,
    },
    /// Filter by mood or genre
    Filter {
        #[serde(skip_serializing_if = "Option::is_none")]
        mood: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        genre: Option<String>,
    },
    /// Playback control
    Control { action: ControlAction },
    /// Unknown intent
    Unknown { query: String },
}

/// Playback control actions
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ControlAction {
    Play,
    Pause,
    Next,
    Previous,
    Stop,
}

/// AI backend trait for intent recognition
#[async_trait]
pub trait AIBackend: Send + Sync {
    /// Recognize intent from natural language query
    async fn intent(&self, query: &str) -> Result<Intent>;
}

pub use regex_backend::RegexBackend;
