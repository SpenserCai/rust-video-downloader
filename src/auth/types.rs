use serde::{Deserialize, Serialize};
use thiserror::Error;

/// 登录方式
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)] // Reserved for future login methods
pub enum LoginMethod {
    /// 二维码登录
    QRCode,
    // 未来扩展：Password, SMS
}

/// 登录凭证（平台无关）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Credentials {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cookie: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mid: Option<u64>, // 用户ID（可选）
}

/// 二维码数据
#[derive(Debug, Clone)]
pub struct QRCodeData {
    pub url: String,
    pub key: String, // qrcode_key 或 auth_code
}

/// 登录状态
#[derive(Debug, Clone)]
pub enum LoginStatus {
    /// 等待扫码
    Pending,
    /// 已扫码未确认
    Scanned,
    /// 登录成功
    Success(Credentials),
    /// 二维码过期
    Expired,
    /// 登录失败
    Failed(String),
}

/// 认证错误类型
#[derive(Debug, Error)]
#[allow(dead_code)] // RefreshNotSupported is reserved for future use
pub enum AuthError {
    #[error("QR code expired")]
    QRCodeExpired,

    #[error("Login failed: {0}")]
    LoginFailed(String),

    #[error("Failed to save QR code image: {0}")]
    QRCodeSaveError(String),

    #[error("Failed to display QR code in terminal: {0}")]
    QRCodeDisplayError(String),

    #[error("Network error: {0}")]
    Network(String),

    #[error("Credential refresh not supported")]
    RefreshNotSupported,

    #[error("Failed to save credentials to config: {0}")]
    CredentialSaveError(String),

    #[error("Invalid response from server: {0}")]
    InvalidResponse(String),
}
