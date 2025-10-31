/*
 * @Author: SpenserCai
 * @Date: 2025-10-31 15:30:00
 * @version:
 * @LastEditors: SpenserCai
 * @LastEditTime: 2025-10-31 15:30:00
 * @Description: Credential storage module
 */
//! 凭证存储模块
//!
//! 管理认证凭证的存储和加载

use crate::auth::types::{AuthError, Credentials};
use crate::error::Result;
use crate::types::Auth;
use crate::utils::config::AuthConfig;
use std::path::Path;

/// 凭证存储管理器
#[allow(dead_code)]
pub struct CredentialStorage;

#[allow(dead_code)]
impl CredentialStorage {
    /// 将凭证写入独立的auth.toml文件
    ///
    /// # Arguments
    ///
    /// * `credentials` - 要保存的凭证
    /// * `config_path` - 配置文件路径（用于确定auth.toml的位置）
    ///
    /// # Returns
    ///
    /// 成功返回Ok(())，失败返回错误
    pub fn save_to_config(credentials: &Credentials, config_path: &Path) -> Result<()> {
        // 确定auth.toml的路径（与config.toml同目录）
        let auth_path = config_path
            .parent()
            .unwrap_or_else(|| Path::new("."))
            .join("auth.toml");

        // 将Credentials转换为AuthConfig
        let auth_config = Self::credentials_to_auth_config(credentials);

        // 序列化为TOML
        let toml_string = toml::to_string_pretty(&auth_config).map_err(|e| {
            AuthError::CredentialSaveError(format!("Failed to serialize credentials: {}", e))
        })?;

        // 添加警告注释
        let content = format!(
            "# 认证凭证文件\n# 警告：此文件包含敏感信息，请勿分享或提交到版本控制系统\n\n{}",
            toml_string
        );

        // 写入文件
        std::fs::write(&auth_path, content).map_err(|e| {
            AuthError::CredentialSaveError(format!("Failed to write auth file: {}", e))
        })?;

        // 设置文件权限（仅Unix）
        Self::set_config_permissions(&auth_path)?;

        tracing::info!("Credentials saved to: {}", auth_path.display());

        Ok(())
    }

    /// 从凭证创建 Auth 对象（用于本次会话）
    ///
    /// # Arguments
    ///
    /// * `credentials` - 凭证对象
    ///
    /// # Returns
    ///
    /// 返回Auth对象
    pub fn to_auth(credentials: &Credentials) -> Auth {
        Auth {
            cookie: credentials.cookie.clone(),
            access_token: credentials.access_token.clone(),
        }
    }

    /// 从配置文件加载凭证
    ///
    /// 优先从auth.toml加载，如果不存在则尝试从config.toml的[auth]部分加载（向后兼容）
    ///
    /// # Arguments
    ///
    /// * `config_path` - 配置文件路径
    ///
    /// # Returns
    ///
    /// 返回凭证对象，如果不存在返回None
    pub fn load_from_config(config_path: &Path) -> Result<Option<Credentials>> {
        // 首先尝试从auth.toml加载
        let auth_path = config_path
            .parent()
            .unwrap_or_else(|| Path::new("."))
            .join("auth.toml");

        if auth_path.exists() {
            tracing::debug!("Loading credentials from: {}", auth_path.display());
            let content = std::fs::read_to_string(&auth_path).map_err(|e| {
                AuthError::InvalidResponse(format!("Failed to read auth file: {}", e))
            })?;

            let auth_config: AuthConfig = toml::from_str(&content).map_err(|e| {
                AuthError::InvalidResponse(format!("Failed to parse auth file: {}", e))
            })?;

            return Ok(Some(Self::auth_config_to_credentials(&auth_config)));
        }

        // 如果auth.toml不存在，尝试从config.toml的[auth]部分加载（向后兼容）
        if config_path.exists() {
            tracing::debug!(
                "auth.toml not found, trying to load from config.toml: {}",
                config_path.display()
            );
            let content = std::fs::read_to_string(config_path).map_err(|e| {
                AuthError::InvalidResponse(format!("Failed to read config file: {}", e))
            })?;

            let config: crate::utils::config::Config = toml::from_str(&content).map_err(|e| {
                AuthError::InvalidResponse(format!("Failed to parse config file: {}", e))
            })?;

            if let Some(auth_config) = config.auth {
                return Ok(Some(Self::auth_config_to_credentials(&auth_config)));
            }
        }

        Ok(None)
    }

    /// 设置配置文件权限（仅Unix）
    ///
    /// # Arguments
    ///
    /// * `path` - 文件路径
    ///
    /// # Returns
    ///
    /// 成功返回Ok(())，失败返回错误
    #[cfg(unix)]
    fn set_config_permissions(path: &Path) -> Result<()> {
        use std::os::unix::fs::PermissionsExt;

        let mut perms = std::fs::metadata(path)
            .map_err(|e| {
                AuthError::CredentialSaveError(format!("Failed to get file metadata: {}", e))
            })?
            .permissions();

        perms.set_mode(0o600); // 仅所有者可读写

        std::fs::set_permissions(path, perms).map_err(|e| {
            AuthError::CredentialSaveError(format!("Failed to set file permissions: {}", e))
        })?;

        tracing::debug!("Set file permissions to 0600 for: {}", path.display());

        Ok(())
    }

    #[cfg(not(unix))]
    fn set_config_permissions(_path: &Path) -> Result<()> {
        // Windows不需要特殊处理
        Ok(())
    }

    /// 将Credentials转换为AuthConfig
    fn credentials_to_auth_config(credentials: &Credentials) -> AuthConfig {
        AuthConfig {
            cookie: credentials.cookie.clone(),
            access_token: credentials.access_token.clone(),
            refresh_token: credentials.refresh_token.clone(),
            expires_at: credentials.expires_at,
            mid: credentials.mid,
        }
    }

    /// 将AuthConfig转换为Credentials
    fn auth_config_to_credentials(auth_config: &AuthConfig) -> Credentials {
        Credentials {
            cookie: auth_config.cookie.clone(),
            access_token: auth_config.access_token.clone(),
            refresh_token: auth_config.refresh_token.clone(),
            expires_at: auth_config.expires_at,
            mid: auth_config.mid,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_save_and_load_credentials() {
        let dir = tempdir().unwrap();
        let config_path = dir.path().join("config.toml");

        // 创建测试凭证
        let credentials = Credentials {
            cookie: Some("test_cookie".to_string()),
            access_token: Some("test_token".to_string()),
            refresh_token: Some("test_refresh".to_string()),
            expires_at: Some(1234567890),
            mid: Some(123456),
        };

        // 保存凭证
        let result = CredentialStorage::save_to_config(&credentials, &config_path);
        assert!(result.is_ok());

        // 验证auth.toml文件存在
        let auth_path = dir.path().join("auth.toml");
        assert!(auth_path.exists());

        // 加载凭证
        let loaded = CredentialStorage::load_from_config(&config_path).unwrap();
        assert!(loaded.is_some());

        let loaded_creds = loaded.unwrap();
        assert_eq!(loaded_creds.cookie, credentials.cookie);
        assert_eq!(loaded_creds.access_token, credentials.access_token);
        assert_eq!(loaded_creds.refresh_token, credentials.refresh_token);
        assert_eq!(loaded_creds.expires_at, credentials.expires_at);
        assert_eq!(loaded_creds.mid, credentials.mid);
    }

    #[test]
    fn test_to_auth() {
        let credentials = Credentials {
            cookie: Some("test_cookie".to_string()),
            access_token: Some("test_token".to_string()),
            refresh_token: None,
            expires_at: None,
            mid: None,
        };

        let auth = CredentialStorage::to_auth(&credentials);
        assert_eq!(auth.cookie, credentials.cookie);
        assert_eq!(auth.access_token, credentials.access_token);
    }

    #[test]
    fn test_load_nonexistent_config() {
        let dir = tempdir().unwrap();
        let config_path = dir.path().join("nonexistent.toml");

        let result = CredentialStorage::load_from_config(&config_path);
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }
}
