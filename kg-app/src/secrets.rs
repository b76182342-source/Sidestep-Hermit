//! 系统密钥环读写（支持多平台）。
//!
//! 合规要求（见 CLAUDE.md）：用户凭据只存系统密钥环，日志中不得出现 token。
//! 因此这里返回的密钥内容只用于在内存中重建会话，调用方 **不得** 打日志或落盘。

/// 凭据密钥（按平台区分）
#[derive(Debug, Clone, Copy)]
pub enum CredentialKey {
    Kugou,
    Netease,
    QQMusic,
}

impl CredentialKey {
    fn account(&self) -> &'static str {
        match self {
            Self::Kugou => "kugou",
            Self::Netease => "netease",
            Self::QQMusic => "qqmusic",
        }
    }

    fn label(&self) -> &'static str {
        match self {
            Self::Kugou => "Sidestep Hermit 酷狗登录凭据",
            Self::Netease => "Sidestep Hermit 网易云登录凭据",
            Self::QQMusic => "Sidestep Hermit QQ音乐登录凭据",
        }
    }
}

#[cfg(target_os = "linux")]
mod backend {
    use super::CredentialKey;
    use oo7::Keyring;
    use std::collections::HashMap;

    /// 密钥环 service 名称（统一）
    const SERVICE: &str = "com.kgplayer.app";

    fn attributes(key: CredentialKey) -> HashMap<&'static str, &'static str> {
        HashMap::from([("service", SERVICE), ("account", key.account())])
    }

    /// 写入（覆盖）凭据
    pub async fn save(key: CredentialKey, secret: &str) -> Result<(), String> {
        let keyring = Keyring::new().await.map_err(|e| e.to_string())?;
        keyring
            .create_item(key.label(), &attributes(key), secret, true)
            .await
            .map_err(|e| e.to_string())
    }

    /// 读取凭据；不存在返回 `Ok(None)`
    pub async fn load(key: CredentialKey) -> Result<Option<String>, String> {
        let keyring = Keyring::new().await.map_err(|e| e.to_string())?;
        let items = keyring
            .search_items(&attributes(key))
            .await
            .map_err(|e| e.to_string())?;

        let Some(item) = items.into_iter().next() else {
            return Ok(None);
        };

        let secret = item.secret().await.map_err(|e| e.to_string())?;
        String::from_utf8(secret.to_vec())
            .map(Some)
            .map_err(|e| e.to_string())
    }

    /// 删除凭据（不存在时也视为成功）
    pub async fn clear(key: CredentialKey) -> Result<(), String> {
        let keyring = Keyring::new().await.map_err(|e| e.to_string())?;
        keyring
            .delete(&attributes(key))
            .await
            .map_err(|e| e.to_string())
    }
}

#[cfg(not(target_os = "linux"))]
mod backend {
    use super::CredentialKey;

    pub async fn save(_key: CredentialKey, _secret: &str) -> Result<(), String> {
        Err("当前平台尚未接入系统密钥环".into())
    }

    pub async fn load(_key: CredentialKey) -> Result<Option<String>, String> {
        Ok(None)
    }

    pub async fn clear(_key: CredentialKey) -> Result<(), String> {
        Ok(())
    }
}

pub use backend::{clear, load, save};
