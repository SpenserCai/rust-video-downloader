/*
 * @Author: SpenserCai
 * @Date: 2025-10-31 17:00:00
 * @version:
 * @LastEditors: SpenserCai
 * @LastEditTime: 2025-10-31 17:00:00
 * @Description: Login manager module tests
 */

use async_trait::async_trait;
use rvd::auth::login::LoginManager;
use rvd::auth::types::{Credentials, LoginStatus, QRCodeData};
use rvd::auth::AuthProvider;
use rvd::error::Result;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

// Mock provider for testing
struct MockAuthProvider {
    poll_count: Arc<AtomicUsize>,
    success_after: usize,
    should_expire: bool,
    should_fail: bool,
}

impl MockAuthProvider {
    fn new_success(success_after: usize) -> Self {
        Self {
            poll_count: Arc::new(AtomicUsize::new(0)),
            success_after,
            should_expire: false,
            should_fail: false,
        }
    }

    fn new_expired() -> Self {
        Self {
            poll_count: Arc::new(AtomicUsize::new(0)),
            success_after: 0,
            should_expire: true,
            should_fail: false,
        }
    }

    fn new_failed() -> Self {
        Self {
            poll_count: Arc::new(AtomicUsize::new(0)),
            success_after: 0,
            should_expire: false,
            should_fail: true,
        }
    }
}

#[async_trait]
impl AuthProvider for MockAuthProvider {
    async fn request_qrcode(&self) -> Result<QRCodeData> {
        Ok(QRCodeData {
            url: "https://www.bilibili.com/test".to_string(),
            key: "test_key_12345".to_string(),
        })
    }

    async fn poll_login_status(&self, _key: &str) -> Result<LoginStatus> {
        let count = self.poll_count.fetch_add(1, Ordering::SeqCst);

        if self.should_expire {
            return Ok(LoginStatus::Expired);
        }

        if self.should_fail {
            return Ok(LoginStatus::Failed("Test failure".to_string()));
        }

        if count < self.success_after {
            if count == self.success_after / 2 {
                // 在中间返回Scanned状态
                Ok(LoginStatus::Scanned)
            } else {
                Ok(LoginStatus::Pending)
            }
        } else {
            Ok(LoginStatus::Success(Credentials {
                cookie: Some("test_cookie".to_string()),
                access_token: Some("test_token".to_string()),
                refresh_token: Some("test_refresh".to_string()),
                expires_at: Some(1234567890),
                mid: Some(123456),
            }))
        }
    }
}

#[tokio::test]
async fn test_login_manager_creation() {
    let provider = Box::new(MockAuthProvider::new_success(1));
    let _manager = LoginManager::new(provider);
}

#[tokio::test]
async fn test_login_manager_immediate_success() {
    let provider = Box::new(MockAuthProvider::new_success(0));
    let manager = LoginManager::new(provider);

    let result = manager.perform_qr_login().await;
    assert!(result.is_ok(), "Login should succeed immediately");

    let credentials = result.unwrap();
    assert_eq!(credentials.cookie, Some("test_cookie".to_string()));
    assert_eq!(credentials.access_token, Some("test_token".to_string()));
}

#[tokio::test]
async fn test_login_manager_success_after_retries() {
    let provider = Box::new(MockAuthProvider::new_success(3));
    let manager = LoginManager::new(provider);

    let result = manager.perform_qr_login().await;
    assert!(result.is_ok(), "Login should succeed after retries");
}

#[tokio::test]
async fn test_login_manager_expired() {
    let provider = Box::new(MockAuthProvider::new_expired());
    let manager = LoginManager::new(provider);

    let result = manager.perform_qr_login().await;
    assert!(result.is_err(), "Login should fail with expired QR code");

    if let Err(e) = result {
        let error_msg = format!("{}", e);
        assert!(
            error_msg.contains("expired") || error_msg.contains("Expired"),
            "Error should mention expiration: {}",
            error_msg
        );
    }
}

#[tokio::test]
async fn test_login_manager_failed() {
    let provider = Box::new(MockAuthProvider::new_failed());
    let manager = LoginManager::new(provider);

    let result = manager.perform_qr_login().await;
    assert!(result.is_err(), "Login should fail");

    if let Err(e) = result {
        let error_msg = format!("{}", e);
        assert!(
            error_msg.contains("failed") || error_msg.contains("Failed"),
            "Error should mention failure: {}",
            error_msg
        );
    }
}

#[tokio::test]
async fn test_login_manager_with_scanned_status() {
    // 测试包含Scanned状态的流程
    let provider = Box::new(MockAuthProvider::new_success(4));
    let manager = LoginManager::new(provider);

    let result = manager.perform_qr_login().await;
    assert!(result.is_ok(), "Login should succeed even with Scanned status");
}

// Mock provider that simulates network errors
struct NetworkErrorProvider {
    error_count: Arc<AtomicUsize>,
    max_errors: usize,
}

impl NetworkErrorProvider {
    fn new(max_errors: usize) -> Self {
        Self {
            error_count: Arc::new(AtomicUsize::new(0)),
            max_errors,
        }
    }
}

#[async_trait]
impl AuthProvider for NetworkErrorProvider {
    async fn request_qrcode(&self) -> Result<QRCodeData> {
        Ok(QRCodeData {
            url: "https://www.bilibili.com/test".to_string(),
            key: "test_key".to_string(),
        })
    }

    async fn poll_login_status(&self, _key: &str) -> Result<LoginStatus> {
        let count = self.error_count.fetch_add(1, Ordering::SeqCst);

        if count < self.max_errors {
            // 模拟网络错误 - 使用Parse错误代替
            Err(rvd::error::DownloaderError::Parse(
                "Simulated network error".to_string(),
            ))
        } else {
            // 最终成功
            Ok(LoginStatus::Success(Credentials {
                cookie: Some("test_cookie".to_string()),
                access_token: None,
                refresh_token: None,
                expires_at: None,
                mid: None,
            }))
        }
    }
}

#[tokio::test]
async fn test_login_manager_network_error_retry() {
    // 测试网络错误重试（最多3次）
    let provider = Box::new(NetworkErrorProvider::new(2));
    let manager = LoginManager::new(provider);

    let result = manager.perform_qr_login().await;
    // 应该在重试后成功
    assert!(result.is_ok(), "Login should succeed after network error retries");
}

#[tokio::test]
async fn test_login_manager_network_error_fail() {
    // 测试网络错误超过重试次数
    let provider = Box::new(NetworkErrorProvider::new(5));
    let manager = LoginManager::new(provider);

    let result = manager.perform_qr_login().await;
    // 应该失败
    assert!(result.is_err(), "Login should fail after too many network errors");
}

#[test]
fn test_credentials_structure() {
    // 测试Credentials结构的基本功能
    let credentials = Credentials {
        cookie: Some("test".to_string()),
        access_token: Some("token".to_string()),
        refresh_token: Some("refresh".to_string()),
        expires_at: Some(1234567890),
        mid: Some(123),
    };

    assert!(credentials.cookie.is_some());
    assert!(credentials.access_token.is_some());
    assert!(credentials.refresh_token.is_some());
    assert!(credentials.expires_at.is_some());
    assert!(credentials.mid.is_some());
}
