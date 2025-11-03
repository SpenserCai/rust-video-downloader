//! Platform metadata structures
//!
//! This module defines structures for describing platform capabilities and features.

/// Platform metadata
///
/// Describes a platform's capabilities, supported features, and URL patterns.
///
/// # Example
///
/// ```
/// use rvd::platform::metadata::{PlatformMetadata, PlatformCapabilities, AuthMethod};
///
/// let metadata = PlatformMetadata {
///     name: "bilibili",
///     display_name: "哔哩哔哩",
///     version: "1.0.0",
///     capabilities: PlatformCapabilities {
///         subtitles: true,
///         danmaku: true,
///         batch_download: true,
///         chapters: true,
///         requires_auth: false,
///         auth_methods: vec![AuthMethod::Cookie, AuthMethod::QRCode],
///         max_quality: Some("8K"),
///         live_stream: false,
///     },
///     url_patterns: vec!["bilibili.com", "b23.tv", "^BV", "^av"],
/// };
/// ```
#[derive(Debug, Clone)]
pub struct PlatformMetadata {
    /// Platform name (e.g., "bilibili", "youtube")
    ///
    /// This should be a lowercase identifier used internally.
    pub name: &'static str,

    /// Platform display name (e.g., "哔哩哔哩", "YouTube")
    ///
    /// This is the human-readable name shown to users.
    pub display_name: &'static str,

    /// Platform implementation version
    pub version: &'static str,

    /// Platform capabilities
    ///
    /// Describes what features this platform supports.
    pub capabilities: PlatformCapabilities,

    /// URL patterns for quick matching
    ///
    /// Patterns can be:
    /// - Domain names: "bilibili.com", "youtube.com"
    /// - Regex patterns (starting with ^): "^BV", "^av"
    ///
    /// These patterns are used by the platform registry for fast URL matching.
    pub url_patterns: Vec<&'static str>,
}

/// Platform capabilities
///
/// Describes the features and limitations of a platform.
#[derive(Debug, Clone, Default)]
pub struct PlatformCapabilities {
    /// Whether the platform supports subtitle downloads
    pub subtitles: bool,

    /// Whether the platform supports danmaku (bullet comments)
    pub danmaku: bool,

    /// Whether the platform supports batch downloads (playlists, favorites, etc.)
    pub batch_download: bool,

    /// Whether the platform supports chapter information
    pub chapters: bool,

    /// Whether the platform requires authentication for basic functionality
    pub requires_auth: bool,

    /// Supported authentication methods
    pub auth_methods: Vec<AuthMethod>,

    /// Maximum supported quality (e.g., "4K", "8K")
    ///
    /// `None` if there's no specific limit or it's unknown.
    pub max_quality: Option<&'static str>,

    /// Whether the platform supports live streams
    pub live_stream: bool,
}

/// Authentication method
///
/// Represents the different ways a platform can authenticate users.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AuthMethod {
    /// Cookie-based authentication
    ///
    /// The platform uses HTTP cookies for authentication.
    Cookie,

    /// Token-based authentication
    ///
    /// The platform uses access tokens (e.g., JWT, OAuth tokens).
    Token,

    /// OAuth authentication
    ///
    /// The platform uses OAuth 2.0 flow for authentication.
    OAuth,

    /// QR code authentication
    ///
    /// The platform supports QR code scanning for authentication.
    QRCode,
}

impl PlatformMetadata {
    /// Check if the platform supports a specific authentication method
    ///
    /// # Arguments
    ///
    /// * `method` - The authentication method to check
    ///
    /// # Returns
    ///
    /// `true` if the method is supported, `false` otherwise
    pub fn supports_auth_method(&self, method: AuthMethod) -> bool {
        self.capabilities.auth_methods.contains(&method)
    }

    /// Get a human-readable description of supported features
    ///
    /// Returns a string listing all supported features.
    pub fn feature_description(&self) -> String {
        let mut features = Vec::new();

        if self.capabilities.subtitles {
            features.push("字幕");
        }
        if self.capabilities.danmaku {
            features.push("弹幕");
        }
        if self.capabilities.batch_download {
            features.push("批量下载");
        }
        if self.capabilities.chapters {
            features.push("章节");
        }
        if self.capabilities.live_stream {
            features.push("直播");
        }

        if features.is_empty() {
            "无特殊功能".to_string()
        } else {
            features.join("、")
        }
    }
}
