/*
 * @Author: SpenserCai
 * @Date: 2025-10-31 11:13:33
 * @version: 
 * @LastEditors: SpenserCai
 * @LastEditTime: 2025-10-31 11:38:20
 * @Description: file content
 */
//! 认证模块
//!
//! 提供平台无关的认证功能，支持多种登录方式

pub mod providers;
pub mod types;

pub use types::{AuthError, Credentials, LoginStatus, QRCodeData};

#[allow(unused_imports)]
pub use types::LoginMethod;

use async_trait::async_trait;
use crate::error::Result;

/// 认证提供者接口
///
/// 所有平台的认证实现都必须实现此trait
#[async_trait]
#[allow(dead_code)]
pub trait AuthProvider: Send + Sync {
    /// 申请二维码
    ///
    /// # Returns
    ///
    /// 返回二维码数据，包含URL和key
    async fn request_qrcode(&self) -> Result<QRCodeData>;

    /// 轮询登录状态
    ///
    /// # Arguments
    ///
    /// * `key` - 二维码的key（qrcode_key或auth_code）
    ///
    /// # Returns
    ///
    /// 返回当前的登录状态
    async fn poll_login_status(&self, key: &str) -> Result<LoginStatus>;

    /// 刷新凭证（可选）
    ///
    /// # Arguments
    ///
    /// * `refresh_token` - 刷新令牌
    ///
    /// # Returns
    ///
    /// 返回新的凭证
    async fn refresh_credentials(&self, _refresh_token: &str) -> Result<Credentials> {
        Err(crate::error::DownloaderError::Auth(
            AuthError::RefreshNotSupported,
        ))
    }
}
