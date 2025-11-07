use std::time::Duration;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DownloaderError {
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),

    #[error("Platform not supported: {0}")]
    UnsupportedPlatform(String),

    #[error("Platform {platform} API error: {message}")]
    PlatformApiError {
        platform: String,
        code: Option<i32>,
        message: String,
        is_retryable: bool,
    },

    #[error("Video not found: {0}")]
    VideoNotFound(String),

    #[error("Video unavailable: {reason}")]
    VideoUnavailable {
        reason: String,
        is_geo_restricted: bool,
        is_private: bool,
    },

    #[error("Authentication required for platform: {0}")]
    AuthRequired(String),

    #[error("Authentication failed for {platform}: {reason}")]
    AuthFailed { platform: String, reason: String },

    #[error("Authentication expired for {platform}")]
    AuthExpired { platform: String },

    #[error("Rate limited by {platform}, retry after {retry_after:?}")]
    RateLimited {
        platform: String,
        retry_after: Option<Duration>,
    },

    #[error("Invalid quality: {0}")]
    InvalidQuality(String),

    #[error("Download failed: {0}")]
    DownloadFailed(String),

    #[error("Mux failed: {0}")]
    MuxFailed(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Parse error: {0}")]
    Parse(String),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("FFmpeg not found or not executable")]
    FFmpegNotFound,

    #[error("Invalid URL: {0}")]
    InvalidUrl(String),

    #[error("API error: {0}")]
    Api(String),

    #[error("Authentication error: {0}")]
    Auth(crate::auth::AuthError),

    #[error("Platform {platform} requires feature {feature} which is not supported")]
    FeatureNotSupported { platform: String, feature: String },

    #[error("Batch download limit exceeded: requested {requested}, max {max}")]
    BatchLimitExceeded { requested: usize, max: usize },

    #[error("Content requires age verification on {platform}")]
    AgeRestricted { platform: String, video_id: String },

    #[error("Live stream not yet started on {platform}")]
    LiveNotStarted {
        platform: String,
        scheduled_time: Option<String>,
    },
}

pub type Result<T> = std::result::Result<T, DownloaderError>;

impl From<crate::auth::AuthError> for DownloaderError {
    fn from(err: crate::auth::AuthError) -> Self {
        DownloaderError::Auth(err)
    }
}

/// Error classification trait - used to determine if an error is retryable
pub trait ErrorClassification {
    /// Check if the error is retryable
    fn is_retryable(&self) -> bool;

    /// Get the suggested retry delay
    fn suggested_retry_after(&self) -> Option<Duration>;
}

impl ErrorClassification for DownloaderError {
    fn is_retryable(&self) -> bool {
        match self {
            Self::Network(_) => true,
            Self::PlatformApiError { is_retryable, .. } => *is_retryable,
            Self::RateLimited { .. } => true,
            Self::AuthExpired { .. } => true,
            _ => false,
        }
    }

    fn suggested_retry_after(&self) -> Option<Duration> {
        match self {
            Self::RateLimited { retry_after, .. } => *retry_after,
            Self::Network(_) => Some(Duration::from_secs(2)),
            Self::PlatformApiError {
                is_retryable: true, ..
            } => Some(Duration::from_secs(1)),
            _ => None,
        }
    }
}

impl DownloaderError {
    /// Get a user-friendly error message
    ///
    /// Returns a message that's suitable for displaying to end users.
    pub fn user_message(&self) -> String {
        match self {
            Self::AuthRequired(platform) => format!("需要登录才能访问{}平台的内容", platform),
            Self::AuthFailed { platform, reason } => {
                format!("{}平台认证失败: {}", platform, reason)
            }
            Self::AuthExpired { platform } => format!("{}平台认证已过期，请重新登录", platform),
            Self::VideoUnavailable {
                reason,
                is_geo_restricted,
                is_private,
            } => {
                if *is_geo_restricted {
                    format!("视频在您的地区不可用: {}", reason)
                } else if *is_private {
                    format!("视频为私密内容: {}", reason)
                } else {
                    format!("视频不可用: {}", reason)
                }
            }
            Self::RateLimited {
                platform,
                retry_after,
            } => {
                if let Some(duration) = retry_after {
                    format!(
                        "{}平台请求过于频繁，请等待{}秒后重试",
                        platform,
                        duration.as_secs()
                    )
                } else {
                    format!("{}平台请求过于频繁，请稍后重试", platform)
                }
            }
            Self::PlatformApiError {
                platform, message, ..
            } => format!("{}平台API错误: {}", platform, message),
            Self::FeatureNotSupported { platform, feature } => {
                format!("{}平台不支持{}功能", platform, feature)
            }
            Self::AgeRestricted { platform, .. } => format!("{}平台内容需要年龄验证", platform),
            Self::LiveNotStarted {
                platform,
                scheduled_time,
            } => {
                if let Some(time) = scheduled_time {
                    format!("{}平台直播尚未开始，预计开始时间: {}", platform, time)
                } else {
                    format!("{}平台直播尚未开始", platform)
                }
            }
            Self::BatchLimitExceeded { requested, max } => {
                format!("批量下载数量超限: 请求{}个，最大{}个", requested, max)
            }
            _ => format!("{}", self),
        }
    }

    /// Get suggested action for the error
    ///
    /// Returns an actionable suggestion for how to resolve the error.
    pub fn suggested_action(&self) -> Option<String> {
        match self {
            Self::AuthRequired(_) | Self::AuthFailed { .. } | Self::AuthExpired { .. } => {
                Some("使用 --login-qrcode 或 --login-tv 登录".to_string())
            }
            Self::VideoUnavailable {
                is_geo_restricted: true,
                ..
            } => Some("尝试使用代理或VPN".to_string()),
            Self::RateLimited { .. } => Some("减少并发请求数或等待一段时间后重试".to_string()),
            Self::FFmpegNotFound => {
                Some("请安装FFmpeg: https://ffmpeg.org/download.html".to_string())
            }
            Self::AgeRestricted { .. } => Some("使用已登录的账号或提供年龄验证信息".to_string()),
            Self::LiveNotStarted { .. } => Some("等待直播开始后再尝试".to_string()),
            Self::BatchLimitExceeded { .. } => {
                Some("使用 --max-videos 参数限制下载数量，或增加 --batch-limit 的值".to_string())
            }
            _ => None,
        }
    }
}
