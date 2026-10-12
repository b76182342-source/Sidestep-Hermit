//! Provider 注册中心
//!
//! 统一管理所有音乐平台（本地、酷狗、网易云、QQ音乐等）

use crate::{MusicProvider, Result};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Provider 元信息
#[derive(Debug, Clone, serde::Serialize)]
pub struct ProviderInfo {
    pub id: &'static str,
    pub name: &'static str,
    pub supports_auth: bool,
    pub is_online: bool,
}

/// 账号状态（对前端可见，不含凭据）
#[derive(Debug, Clone, serde::Serialize)]
pub struct AccountStatus {
    pub provider: &'static str,
    pub logged_in: bool,
    pub user_id: Option<String>,
    pub display_name: Option<String>,
}

/// 账号信息（内部使用）
#[derive(Debug, Clone)]
pub struct AccountInfo {
    pub user_id: String,
    pub display_name: Option<String>,
}

/// 认证管理器接口
#[async_trait::async_trait]
pub trait AuthManager: Send + Sync {
    /// 是否已登录
    async fn is_authenticated(&self) -> bool;

    /// 获取账号信息（不含凭据）
    async fn account_info(&self) -> Option<AccountInfo>;

    /// 登出（清除内存和密钥环）
    async fn logout(&self) -> Result<()>;
}

/// Provider 注册中心
pub struct ProviderRegistry {
    providers: HashMap<&'static str, Arc<dyn MusicProvider>>,
    auth_managers: RwLock<HashMap<&'static str, Arc<dyn AuthManager>>>,
}

impl ProviderRegistry {
    /// 创建新的注册中心
    pub fn new() -> Self {
        Self {
            providers: HashMap::new(),
            auth_managers: RwLock::new(HashMap::new()),
        }
    }

    /// 注册一个 Provider
    pub fn register(&mut self, provider: Arc<dyn MusicProvider>) {
        self.providers.insert(provider.id(), provider);
    }

    /// 注册认证管理器
    pub async fn register_auth(&self, id: &'static str, auth: Arc<dyn AuthManager>) {
        self.auth_managers.write().await.insert(id, auth);
    }

    /// 获取 Provider
    pub fn get(&self, id: &str) -> Option<Arc<dyn MusicProvider>> {
        self.providers.get(id).cloned()
    }

    /// 列出所有 Provider
    pub fn list(&self) -> Vec<ProviderInfo> {
        self.providers
            .values()
            .map(|p| ProviderInfo {
                id: p.id(),
                name: p.name(),
                supports_auth: self.supports_auth(p.id()),
                is_online: p.id() != "local",
            })
            .collect()
    }

    /// 获取所有平台的登录状态
    pub async fn account_statuses(&self) -> Vec<AccountStatus> {
        let managers = self.auth_managers.read().await;
        let mut statuses = Vec::new();

        for (provider, auth) in managers.iter() {
            let logged_in = auth.is_authenticated().await;
            let info = auth.account_info().await;

            statuses.push(AccountStatus {
                provider,
                logged_in,
                user_id: info.as_ref().map(|i| i.user_id.clone()),
                display_name: info.as_ref().and_then(|i| i.display_name.clone()),
            });
        }

        statuses
    }

    fn supports_auth(&self, id: &str) -> bool {
        id != "local"
    }
}

impl Default for ProviderRegistry {
    fn default() -> Self {
        Self::new()
    }
}
