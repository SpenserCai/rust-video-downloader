//! Bilibili platform implementation
//!
//! This module implements the Platform trait for Bilibili.

use crate::error::{DownloaderError, Result};
use crate::platform::metadata::{AuthMethod, PlatformCapabilities, PlatformMetadata};
use crate::platform::Platform;
use crate::types::{Auth, BatchResult, Stream, StreamContext, Subtitle, VideoInfo};
use crate::utils::http::HttpClient;
use async_trait::async_trait;
use std::sync::Arc;

/// Bilibili platform implementation
pub struct BilibiliPlatform {
    client: Arc<HttpClient>,
    api_mode: super::ApiMode,
    wbi_manager: tokio::sync::Mutex<super::wbi::WbiManager>,
    metadata: PlatformMetadata,
}

impl BilibiliPlatform {
    /// Create a new BilibiliPlatform with the specified API mode
    pub fn new(api_mode: super::ApiMode) -> Result<Self> {
        let client = Arc::new(HttpClient::new()?);
        let wbi_manager = super::wbi::WbiManager::new(client.clone());

        let metadata = PlatformMetadata {
            name: "bilibili",
            display_name: "哔哩哔哩",
            version: "1.0.0",
            capabilities: PlatformCapabilities {
                subtitles: true,
                danmaku: true,
                batch_download: true,
                chapters: true,
                requires_auth: false,
                auth_methods: vec![AuthMethod::Cookie, AuthMethod::Token, AuthMethod::QRCode],
                max_quality: Some("8K"),
                live_stream: false,
            },
            url_patterns: vec!["bilibili.com", "b23.tv", "^BV", "^av", "^ep", "^ss"],
        };

        Ok(Self {
            client,
            api_mode,
            wbi_manager: tokio::sync::Mutex::new(wbi_manager),
            metadata,
        })
    }

    /// Create a new BilibiliPlatform with a specific API mode
    pub fn with_api_mode(mut self, api_mode: super::ApiMode) -> Self {
        self.api_mode = api_mode;
        self
    }
}

#[async_trait]
impl Platform for BilibiliPlatform {
    fn metadata(&self) -> &PlatformMetadata {
        &self.metadata
    }

    fn can_handle(&self, url: &str) -> bool {
        for pattern in &self.metadata.url_patterns {
            if let Some(stripped) = pattern.strip_prefix('^') {
                // Pattern matching (e.g., ^BV, ^av)
                if url.starts_with(stripped) {
                    return true;
                }
            } else {
                // Domain matching
                if url.contains(pattern) {
                    return true;
                }
            }
        }
        false
    }

    fn is_batch_url(&self, url: &str) -> bool {
        let video_type = match super::parser::parse_url(url) {
            Ok(vt) => vt,
            Err(_) => return false,
        };
        super::parser::is_batch_type(&video_type)
    }

    async fn parse_video(&self, url: &str, auth: Option<&Auth>) -> Result<VideoInfo> {
        let video_type = super::parser::parse_url(url)?;

        // Warn if this is a batch URL
        if super::parser::is_batch_type(&video_type) {
            tracing::warn!(
                "Batch URL detected but parse_video was called. Consider using parse_batch instead."
            );
        }

        let mut wbi = self.wbi_manager.lock().await;
        super::parser::parse_video_info(&self.client, video_type, auth, Some(&mut *wbi)).await
    }

    async fn parse_batch(&self, url: &str, auth: Option<&Auth>) -> Result<BatchResult> {
        let video_type = super::parser::parse_url(url)?;

        if super::parser::is_batch_type(&video_type) {
            let mut wbi = self.wbi_manager.lock().await;
            super::parser::parse_batch_videos(&self.client, video_type, auth, Some(&mut *wbi)).await
        } else {
            // Single video - wrap in BatchResult
            let video = self.parse_video(url, auth).await?;
            Ok(BatchResult::single(video))
        }
    }

    async fn parse_batch_page(
        &self,
        url: &str,
        continuation: Option<&str>,
        auth: Option<&Auth>,
    ) -> Result<BatchResult> {
        let video_type = super::parser::parse_url(url)?;
        let mut wbi = self.wbi_manager.lock().await;
        super::parser::parse_batch_page(
            &self.client,
            video_type,
            continuation,
            auth,
            Some(&mut *wbi),
        )
        .await
    }

    async fn get_streams(
        &self,
        context: &StreamContext,
        auth: Option<&Auth>,
    ) -> Result<Vec<Stream>> {
        let video_id = &context.video_id;
        let cid = context
            .get_str("cid")
            .ok_or_else(|| DownloaderError::Parse("Missing cid in StreamContext".to_string()))?;
        let ep_id = context.get_str("ep_id");

        super::client::get_play_url(&self.client, video_id, cid, ep_id, auth, self.api_mode).await
    }

    async fn get_subtitles(&self, context: &StreamContext) -> Result<Vec<Subtitle>> {
        let video_id = &context.video_id;
        let cid = context
            .get_str("cid")
            .ok_or_else(|| DownloaderError::Parse("Missing cid in StreamContext".to_string()))?;

        super::client::get_subtitles(&self.client, video_id, cid).await
    }

    fn get_cover(&self, video_info: &VideoInfo) -> String {
        video_info.cover_url.clone()
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
