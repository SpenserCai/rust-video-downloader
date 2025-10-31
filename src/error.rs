use thiserror::Error;

#[derive(Debug, Error)]
#[allow(dead_code)] // Some variants are reserved for future use
pub enum DownloaderError {
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),

    #[error("Platform not supported: {0}")]
    UnsupportedPlatform(String),

    #[error("Video not found: {0}")]
    VideoNotFound(String),

    #[error("Authentication required")]
    AuthRequired,

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
}

pub type Result<T> = std::result::Result<T, DownloaderError>;

impl From<crate::auth::AuthError> for DownloaderError {
    fn from(err: crate::auth::AuthError) -> Self {
        DownloaderError::Auth(err)
    }
}
