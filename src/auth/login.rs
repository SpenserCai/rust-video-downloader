/*
 * @Author: SpenserCai
 * @Date: 2025-10-31 16:00:00
 * @version:
 * @LastEditors: SpenserCai
 * @LastEditTime: 2025-10-31 16:00:00
 * @Description: Login manager module
 */
//! 登录管理器模块
//!
//! 协调完整的登录流程

use crate::auth::qrcode::QRCodeDisplay;
use crate::auth::types::{AuthError, Credentials, LoginStatus};
use crate::auth::AuthProvider;
use crate::error::Result;
use std::path::Path;
use std::time::Duration;

/// 登录管理器
#[allow(dead_code)]
pub struct LoginManager {
    provider: Box<dyn AuthProvider>,
}

#[allow(dead_code)]
impl LoginManager {
    /// 创建登录管理器
    ///
    /// # Arguments
    ///
    /// * `provider` - 认证提供者
    pub fn new(provider: Box<dyn AuthProvider>) -> Self {
        Self { provider }
    }

    /// 执行二维码登录
    ///
    /// 完整的登录流程：
    /// 1. 申请二维码
    /// 2. 显示二维码（终端 + 图片文件）
    /// 3. 轮询登录状态
    /// 4. 清理资源
    /// 5. 返回凭证
    ///
    /// # Returns
    ///
    /// 返回登录凭证或错误
    pub async fn perform_qr_login(&self) -> Result<Credentials> {
        // 步骤1: 申请二维码
        tracing::info!("获取登录地址...");
        let qr_data = self.provider.request_qrcode().await?;

        // 步骤2: 显示二维码
        tracing::info!("生成二维码...");

        // 尝试在终端显示
        if let Err(e) = QRCodeDisplay::display_in_terminal(&qr_data.url) {
            tracing::warn!("无法在终端显示二维码: {}", e);
            tracing::warn!("请打开图片文件扫描二维码");
        }

        // 步骤3: 保存二维码图片
        let qrcode_path = Path::new("qrcode.png");
        if let Err(e) = QRCodeDisplay::save_to_file(&qr_data.url, qrcode_path) {
            tracing::warn!("无法保存二维码图片: {}", e);
        } else {
            tracing::info!("二维码已保存到 qrcode.png");
        }

        // 步骤4: 轮询登录状态
        tracing::info!("等待扫码...");

        let credentials = self
            .poll_with_retry(&qr_data.key, 180)
            .await?;

        // 步骤5: 清理资源
        if qrcode_path.exists() {
            if let Err(e) = std::fs::remove_file(qrcode_path) {
                tracing::warn!("无法删除二维码图片: {}", e);
            }
        }

        Ok(credentials)
    }

    /// 轮询登录状态的内部实现
    ///
    /// 包含重试逻辑和超时处理
    ///
    /// # Arguments
    ///
    /// * `key` - 二维码的key
    /// * `max_attempts` - 最大轮询次数
    ///
    /// # Returns
    ///
    /// 返回登录凭证或错误
    async fn poll_with_retry(&self, key: &str, max_attempts: usize) -> Result<Credentials> {
        let mut scanned_shown = false; // 标记是否已显示"已扫码"提示

        for attempt in 1..=max_attempts {
            // 等待1秒
            tokio::time::sleep(Duration::from_secs(1)).await;

            // 轮询状态
            match self.provider.poll_login_status(key).await {
                Ok(LoginStatus::Pending) => {
                    // 未扫码，继续等待（不输出日志，避免刷屏）
                    continue;
                }
                Ok(LoginStatus::Scanned) => {
                    // 已扫码未确认，显示提示（只显示一次）
                    if !scanned_shown {
                        tracing::info!("已扫码，等待确认...");
                        scanned_shown = true;
                    }
                    continue;
                }
                Ok(LoginStatus::Success(credentials)) => {
                    // 登录成功
                    tracing::info!("登录成功！");
                    return Ok(credentials);
                }
                Ok(LoginStatus::Expired) => {
                    // 二维码过期
                    return Err(AuthError::QRCodeExpired.into());
                }
                Ok(LoginStatus::Failed(msg)) => {
                    // 登录失败
                    return Err(AuthError::LoginFailed(msg).into());
                }
                Err(e) => {
                    // 网络错误或其他错误
                    tracing::warn!("轮询登录状态失败 (尝试 {}/{}): {}", attempt, max_attempts, e);

                    // 如果是网络错误，重试几次
                    if attempt < 3 {
                        continue;
                    } else {
                        return Err(e);
                    }
                }
            }
        }

        // 超时
        Err(AuthError::QRCodeExpired.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::auth::types::QRCodeData;
    use async_trait::async_trait;

    // Mock provider for testing
    struct MockAuthProvider {
        should_succeed: bool,
    }

    #[async_trait]
    impl AuthProvider for MockAuthProvider {
        async fn request_qrcode(&self) -> Result<QRCodeData> {
            Ok(QRCodeData {
                url: "https://www.bilibili.com/".to_string(),
                key: "test_key".to_string(),
            })
        }

        async fn poll_login_status(&self, _key: &str) -> Result<LoginStatus> {
            if self.should_succeed {
                Ok(LoginStatus::Success(Credentials {
                    cookie: Some("test_cookie".to_string()),
                    access_token: None,
                    refresh_token: None,
                    expires_at: None,
                    mid: None,
                }))
            } else {
                Ok(LoginStatus::Pending)
            }
        }
    }

    #[tokio::test]
    async fn test_login_manager_creation() {
        let provider = Box::new(MockAuthProvider {
            should_succeed: true,
        });
        let _manager = LoginManager::new(provider);
    }

    #[tokio::test]
    async fn test_poll_with_retry_success() {
        let provider = Box::new(MockAuthProvider {
            should_succeed: true,
        });
        let manager = LoginManager::new(provider);

        let result = manager.poll_with_retry("test_key", 5).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_poll_with_retry_timeout() {
        let provider = Box::new(MockAuthProvider {
            should_succeed: false,
        });
        let manager = LoginManager::new(provider);

        // 使用较小的max_attempts以加快测试
        let result = manager.poll_with_retry("test_key", 2).await;
        assert!(result.is_err());
    }
}
