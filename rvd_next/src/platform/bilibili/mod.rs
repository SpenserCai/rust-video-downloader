//! Bilibili platform implementation
//!
//! This module implements the `Platform` trait for Bilibili, supporting:
//! - Videos (BV/av)
//! - Bangumi (episodes and seasons)
//! - Courses
//! - Batch downloads (favorites, user videos, series, collections)
//! - Subtitles and danmaku
//! - Multiple API modes (Web, TV, APP, International)

// Module declarations will be added in Phase 2

/// API模式
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApiMode {
    Web,
    TV,
    App,
    International,
}

impl Default for ApiMode {
    fn default() -> Self {
        Self::Web
    }
}
