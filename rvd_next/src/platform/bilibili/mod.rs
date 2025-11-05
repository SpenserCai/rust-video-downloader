//! Bilibili platform module
//!
//! This module provides Bilibili platform support including:
//! - Video information parsing
//! - Stream URL retrieval
//! - Subtitle and danmaku support
//! - Batch downloads (favorites, playlists, user videos, etc.)
//! - Multiple API modes (Web, TV, APP, International)

pub mod api;
pub mod auth;
pub mod cdn;
pub mod client;
pub mod parser;
mod platform;
pub mod selector;
pub mod wbi;

pub use platform::BilibiliPlatform;

/// API mode for Bilibili requests
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApiMode {
    /// Web API mode (default)
    Web,
    /// TV API mode (higher quality, requires TV login)
    TV,
    /// APP API mode
    App,
    /// International API mode
    International,
}

impl Default for ApiMode {
    fn default() -> Self {
        Self::Web
    }
}

impl ApiMode {
    /// Parse API mode from string
    pub fn parse(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "web" => Some(Self::Web),
            "tv" => Some(Self::TV),
            "app" => Some(Self::App),
            "international" | "intl" => Some(Self::International),
            _ => None,
        }
    }

    /// Get the API mode as a string
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Web => "web",
            Self::TV => "tv",
            Self::App => "app",
            Self::International => "international",
        }
    }
}

/// Video type parsed from URL
#[derive(Debug, Clone)]
pub enum VideoType {
    /// BV ID (e.g., BV1xx411c7mD)
    Bvid(String),
    /// AV ID (e.g., av170001)
    Aid(String),
    /// Episode ID (e.g., ep123456)
    Episode(String),
    /// Season ID (e.g., ss12345)
    Season(String),
    /// Cheese course episode (e.g., cheese/play/ep123456)
    Cheese(String),
    /// Favorite list (format: "fav_id:mid")
    FavoriteList(String),
    /// User space videos (mid)
    SpaceVideo(String),
    /// Media list (ml_id)
    MediaList(String),
    /// Series list (format: "mid:sid")
    SeriesList(String),
}
