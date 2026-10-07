//! 系统密钥环读写。
//!
//! 合规要求（见 CLAUDE.md）：用户凭据只存系统密钥环，日志中不得出现 token。
//! 因此这里返回的密钥内容只用于在内存中重建会话，调用方 **不得** 打日志或落盘。

#[cfg(target_os = "linux")]
mod backend {
    use oo7::Keyring;
    use std::collections::HashMap;

    /// 密钥环条目标签（用户可见）
    const LABEL: &str = "KGPlayer 酷狗登录凭据";

    fn attributes() -> HashMap<&'static str, &'static str> {
        HashMap::from([("service", "com.kgplayer.app"), ("account", "kugou")])
    }

    /// 写入（覆盖）凭据
    pub async fn save(secret: &str) -> Result<(), String> {
        let keyring = Keyring::new().await.map_err(|e| e.to_string())?;
        keyring
            .create_item(LABEL, &attributes(), secret, true)
            .await
            .map_err(|e| e.to_string())
    }

    /// 读取凭据；不存在返回 `Ok(None)`
    pub async fn load() -> Result<Option<String>, String> {
        let keyring = Keyring::new().await.map_err(|e| e.to_string())?;
        let items = keyring
            .search_items(&attributes())
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
    pub async fn clear() -> Result<(), String> {
        let keyring = Keyring::new().await.map_err(|e| e.to_string())?;
        keyring
            .delete(&attributes())
            .await
            .map_err(|e| e.to_string())
    }
}

#[cfg(not(target_os = "linux"))]
mod backend {
    pub async fn save(_secret: &str) -> Result<(), String> {
        Err("当前平台尚未接入系统密钥环".into())
    }

    pub async fn load() -> Result<Option<String>, String> {
        Ok(None)
    }

    pub async fn clear() -> Result<(), String> {
        Ok(())
    }
}

pub use backend::{clear, load, save};
