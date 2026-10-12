use kg_ai::{AIBackend, RegexBackend};
use tauri::State;

use crate::AppState;

/// Process natural language command using AI intent recognition
#[tauri::command]
pub async fn ai_command(text: String, _state: State<'_, AppState>) -> Result<String, String> {
    let backend = RegexBackend::new();

    // Recognize intent
    let intent = backend
        .intent(&text)
        .await
        .map_err(|e| format!("Intent recognition failed: {}", e))?;

    // Serialize intent for frontend
    let intent_json = serde_json::to_string_pretty(&intent)
        .map_err(|e| format!("Failed to serialize intent: {}", e))?;

    // TODO: Execute the intent using AppState (player, providers, etc.)
    // For now, just return the recognized intent

    Ok(format!("Recognized intent:\n{}", intent_json))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_intent_recognition() {
        let backend = RegexBackend::new();

        let tests = vec!["播放周杰伦", "暂停", "下一首", "来点轻音乐"];

        for query in tests {
            let result = backend.intent(query).await;
            assert!(result.is_ok(), "Failed for query: {}", query);
        }
    }
}
