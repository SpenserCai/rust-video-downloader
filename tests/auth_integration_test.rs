/*
 * @Author: SpenserCai
 * @Date: 2025-10-31 17:30:00
 * @version:
 * @LastEditors: SpenserCai
 * @LastEditTime: 2025-10-31 17:30:00
 * @Description: Integration tests for auth modules
 */

use async_trait::async_trait;
use rvd::auth::login::LoginManager;
use rvd::auth::qrcode::QRCodeDisplay;
use rvd::auth::storage::CredentialStorage;
use rvd::auth::types::{Credentials, LoginStatus, QRCodeData};
use rvd::auth::AuthProvider;
use rvd::error::Result;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tempfile::tempdir;

// Mock provider for integration testing
struct IntegrationMockProvider {
    qr_requested: Arc<AtomicBool>,
}

impl IntegrationMockProvider {
    fn new() -> Self {
        Self {
            qr_requested: Arc::new(AtomicBool::new(false)),
        }
    }
}

#[async_trait]
impl AuthProvider for IntegrationMockProvider {
    async fn request_qrcode(&self) -> Result<QRCodeData> {
        self.qr_requested.store(true, Ordering::SeqCst);
        Ok(QRCodeData {
            url: "https://passport.bilibili.com/x/passport-login/web/qrcode/poll?qrcode_key=test123".to_string(),
            key: "test_key_integration".to_string(),
        })
    }

    async fn poll_login_status(&self, _key: &str) -> Result<LoginStatus> {
        // 立即返回成功
        Ok(LoginStatus::Success(Credentials {
            cookie: Some("SESSDATA=integration_test; bili_jct=test123".to_string()),
            access_token: Some("integration_access_token".to_string()),
            refresh_token: Some("integration_refresh_token".to_string()),
            expires_at: Some(9999999999),
            mid: Some(999888777),
        }))
    }
}

#[tokio::test]
async fn test_full_login_flow_with_storage() {
    // 创建临时目录
    let dir = tempdir().unwrap();
    let config_path = dir.path().join("config.toml");

    // 1. 创建登录管理器
    let provider = Box::new(IntegrationMockProvider::new());
    let manager = LoginManager::new(provider);

    // 2. 执行登录
    let credentials = manager.perform_qr_login().await.unwrap();

    // 3. 验证凭证
    assert!(credentials.cookie.is_some());
    assert!(credentials.access_token.is_some());
    assert_eq!(credentials.mid, Some(999888777));

    // 4. 保存凭证
    CredentialStorage::save_to_config(&credentials, &config_path).unwrap();

    // 5. 验证文件存在
    let auth_path = dir.path().join("auth.toml");
    assert!(auth_path.exists());

    // 6. 重新加载凭证
    let loaded_credentials = CredentialStorage::load_from_config(&config_path)
        .unwrap()
        .unwrap();

    // 7. 验证加载的凭证与原始凭证一致
    assert_eq!(loaded_credentials.cookie, credentials.cookie);
    assert_eq!(loaded_credentials.access_token, credentials.access_token);
    assert_eq!(loaded_credentials.refresh_token, credentials.refresh_token);
    assert_eq!(loaded_credentials.expires_at, credentials.expires_at);
    assert_eq!(loaded_credentials.mid, credentials.mid);

    // 8. 转换为Auth对象
    let auth = CredentialStorage::to_auth(&loaded_credentials);
    assert_eq!(auth.cookie, credentials.cookie);
    assert_eq!(auth.access_token, credentials.access_token);
}

#[test]
fn test_qrcode_generation_and_storage_integration() {
    let dir = tempdir().unwrap();
    let qrcode_path = dir.path().join("integration_qrcode.png");

    // 生成并保存二维码
    let url = "https://passport.bilibili.com/x/passport-login/web/qrcode/poll?qrcode_key=integration_test";
    let result = QRCodeDisplay::save_to_file(url, &qrcode_path);

    assert!(result.is_ok());
    assert!(qrcode_path.exists());

    // 验证文件大小合理
    let metadata = std::fs::metadata(&qrcode_path).unwrap();
    assert!(metadata.len() > 1000); // 至少1KB
}

#[tokio::test]
async fn test_login_and_auth_conversion() {
    let provider = Box::new(IntegrationMockProvider::new());
    let manager = LoginManager::new(provider);

    // 执行登录
    let credentials = manager.perform_qr_login().await.unwrap();

    // 转换为Auth对象
    let auth = CredentialStorage::to_auth(&credentials);

    // 验证转换正确
    assert_eq!(auth.cookie, credentials.cookie);
    assert_eq!(auth.access_token, credentials.access_token);

    // 验证Auth对象可以用于API调用（这里只是结构验证）
    assert!(auth.cookie.is_some() || auth.access_token.is_some());
}

#[test]
fn test_credentials_serialization_deserialization() {
    let dir = tempdir().unwrap();
    let config_path = dir.path().join("config.toml");

    // 创建完整的凭证
    let original = Credentials {
        cookie: Some("SESSDATA=test; bili_jct=test".to_string()),
        access_token: Some("access".to_string()),
        refresh_token: Some("refresh".to_string()),
        expires_at: Some(1234567890),
        mid: Some(123456),
    };

    // 保存
    CredentialStorage::save_to_config(&original, &config_path).unwrap();

    // 加载
    let loaded = CredentialStorage::load_from_config(&config_path)
        .unwrap()
        .unwrap();

    // 验证所有字段
    assert_eq!(loaded.cookie, original.cookie);
    assert_eq!(loaded.access_token, original.access_token);
    assert_eq!(loaded.refresh_token, original.refresh_token);
    assert_eq!(loaded.expires_at, original.expires_at);
    assert_eq!(loaded.mid, original.mid);
}

#[test]
fn test_partial_credentials_handling() {
    let dir = tempdir().unwrap();
    let config_path = dir.path().join("config.toml");

    // 创建部分凭证（只有cookie）
    let partial = Credentials {
        cookie: Some("SESSDATA=test".to_string()),
        access_token: None,
        refresh_token: None,
        expires_at: None,
        mid: None,
    };

    // 保存和加载
    CredentialStorage::save_to_config(&partial, &config_path).unwrap();
    let loaded = CredentialStorage::load_from_config(&config_path)
        .unwrap()
        .unwrap();

    // 验证
    assert_eq!(loaded.cookie, partial.cookie);
    assert_eq!(loaded.access_token, None);
    assert_eq!(loaded.refresh_token, None);
    assert_eq!(loaded.expires_at, None);
    assert_eq!(loaded.mid, None);
}

#[tokio::test]
async fn test_multiple_login_sessions() {
    let dir = tempdir().unwrap();
    let config_path = dir.path().join("config.toml");

    // 第一次登录
    let provider1 = Box::new(IntegrationMockProvider::new());
    let manager1 = LoginManager::new(provider1);
    let credentials1 = manager1.perform_qr_login().await.unwrap();
    CredentialStorage::save_to_config(&credentials1, &config_path).unwrap();

    // 第二次登录（覆盖）
    let provider2 = Box::new(IntegrationMockProvider::new());
    let manager2 = LoginManager::new(provider2);
    let credentials2 = manager2.perform_qr_login().await.unwrap();
    CredentialStorage::save_to_config(&credentials2, &config_path).unwrap();

    // 验证加载的是最新的凭证
    let loaded = CredentialStorage::load_from_config(&config_path)
        .unwrap()
        .unwrap();
    assert_eq!(loaded.cookie, credentials2.cookie);
}

#[test]
fn test_auth_file_format() {
    let dir = tempdir().unwrap();
    let config_path = dir.path().join("config.toml");

    let credentials = Credentials {
        cookie: Some("test_cookie".to_string()),
        access_token: Some("test_token".to_string()),
        refresh_token: None,
        expires_at: None,
        mid: None,
    };

    CredentialStorage::save_to_config(&credentials, &config_path).unwrap();

    // 读取文件内容
    let auth_path = dir.path().join("auth.toml");
    let content = std::fs::read_to_string(&auth_path).unwrap();

    // 验证文件格式
    assert!(content.contains("# 认证凭证文件"));
    assert!(content.contains("# 警告"));
    assert!(content.contains("cookie"));
    assert!(content.contains("access_token"));
    assert!(content.contains("test_cookie"));
    assert!(content.contains("test_token"));

    // 验证不包含None值的字段
    assert!(!content.contains("refresh_token"));
    assert!(!content.contains("expires_at"));
    assert!(!content.contains("mid"));
}
