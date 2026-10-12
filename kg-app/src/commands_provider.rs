//! Provider 管理相关的 Tauri 命令

use tauri::State;
use kg_core::{ProviderInfo, AccountStatus};
use crate::AppState;

/// 列出所有可用的 Provider
#[tauri::command]
pub async fn list_providers(state: State<'_, AppState>) -> Result<Vec<ProviderInfo>, String> {
    Ok(state.registry.list())
}

/// 获取所有平台的账号状态
#[tauri::command]
pub async fn account_statuses(state: State<'_, AppState>) -> Result<Vec<AccountStatus>, String> {
    Ok(state.registry.account_statuses().await)
}
