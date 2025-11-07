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
    cdn_optimizer: super::cdn::BilibiliCdnOptimizer,
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
            cdn_optimizer: super::cdn::BilibiliCdnOptimizer::new(),
            metadata,
        })
    }

    /// Create a new BilibiliPlatform with a specific API mode
    pub fn with_api_mode(mut self, api_mode: super::ApiMode) -> Self {
        self.api_mode = api_mode;
        self
    }

    /// Configure CDN optimizer with custom backup hosts
    pub fn with_cdn_config(mut self, backup_hosts: Vec<String>) -> Self {
        self.cdn_optimizer = super::cdn::BilibiliCdnOptimizer::with_backup_hosts(backup_hosts);
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
            super::parser::parse_batch_videos(&self.client, video_type, auth, Some(&mut *wbi), self.api_mode).await
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
            self.api_mode,
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

        let mut streams =
            super::client::get_play_url(&self.client, video_id, cid, ep_id, auth, self.api_mode)
                .await?;

        // Check for CMCC CDN and mark streams that need single-threaded download
        for stream in &mut streams {
            if self.cdn_optimizer.is_cmcc_cdn(&stream.url) {
                tracing::debug!("Bilibili: Detected CMCC CDN, marking for single-threaded download");
                stream.extra_data = Some(serde_json::json!({
                    "disable_multithread": true
                }));
            }
        }

        Ok(streams)
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

    async fn get_chapters(
        &self,
        context: &StreamContext,
    ) -> Result<Vec<crate::types::Chapter>> {
        let video_id = &context.video_id;
        let cid = context
            .get_str("cid")
            .ok_or_else(|| DownloaderError::Parse("Missing cid in StreamContext".to_string()))?;

        // Call parser module's fetch_chapters
        super::parser::fetch_chapters(&self.client, video_id, cid).await
    }

    fn select_best_streams(
        &self,
        streams: &[crate::types::Stream],
        preferences: &crate::types::StreamPreferences,
    ) -> Result<(crate::types::Stream, crate::types::Stream)> {
        // Use Bilibili-specific stream selector
        // It includes Bilibili-specific quality mapping and Dolby audio support
        super::selector::select_best_streams(streams, preferences)
    }

    async fn get_danmaku(
        &self,
        context: &StreamContext,
        format: crate::core::danmaku::DanmakuFormat,
    ) -> Result<Option<String>> {
        let cid = context
            .get_str("cid")
            .ok_or_else(|| DownloaderError::Parse("Missing cid in StreamContext".to_string()))?;

        match super::client::get_danmaku(&self.client, cid, format).await {
            Ok(content) if !content.is_empty() => Ok(Some(content)),
            Ok(_) => Ok(None),
            Err(e) => {
                tracing::warn!("Failed to get danmaku: {}", e);
                Ok(None)
            }
        }
    }

    fn optimize_download_url(&self, url: &str) -> String {
        // Use CDN optimizer to replace PCDN and foreign sources
        self.cdn_optimizer.optimize_url(url)
    }

    fn customize_download_headers(&self, url: &str) -> Option<reqwest::header::HeaderMap> {
        // Add custom headers for bilivideo.com and bilivideo.cn URLs (Bilibili CDN)
        if url.contains("bilivideo.com") || url.contains("bilivideo.cn") {
            let mut headers = reqwest::header::HeaderMap::new();

            // Bilibili CDN requires Referer header
            if let Ok(value) = "https://www.bilibili.com".parse() {
                headers.insert("Referer", value);
            }

            // Add User-Agent
            if let Ok(value) =
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36".parse()
            {
                headers.insert("User-Agent", value);
            }
            
            // Note: Cookie will be added by Downloader from auth info
            // We don't add it here to avoid duplication

            Some(headers)
        } else {
            None
        }
    }

    fn validate_cli_args(&self, cli: &crate::cli::Cli) -> Result<()> {
        // Validate Bilibili-specific login flags
        if cli.login_bilibili_web || cli.login_bilibili_tv {
            // If URL is provided, verify it's a Bilibili URL
            if let Some(ref url) = cli.url {
                if !self.can_handle(url) {
                    return Err(DownloaderError::InvalidArgument(format!(
                        "Bilibili login flags (--login-bilibili-web/--login-bilibili-tv) can only be used with Bilibili URLs.\n\
                        Got URL: {}\n\
                        Hint: Remove the URL or use a Bilibili URL, or use generic --login-qrcode flag.",
                        url
                    )));
                }
            }
            // If no URL provided, that's fine - login-only mode
        }

        // Validate Bilibili-specific API flags
        if cli.use_tv_api || cli.use_app_api || cli.use_intl_api {
            if let Some(ref url) = cli.url {
                if !self.can_handle(url) {
                    tracing::warn!(
                        "Bilibili API mode flags (--use-tv-api/--use-app-api/--use-intl-api) \
                        are being used with a non-Bilibili URL: {}",
                        url
                    );
                }
            }
        }

        Ok(())
    }

    fn create_auth_provider(&self, cli: &crate::cli::Cli) -> Result<Box<dyn crate::auth::AuthProvider>> {
        use crate::auth::providers::BilibiliAuthProvider;
        
        // Determine API mode from CLI
        let api_mode = if cli.login_bilibili_tv {
            super::ApiMode::TV
        } else if cli.login_bilibili_web || cli.login_qrcode {
            super::ApiMode::Web
        } else {
            return Err(DownloaderError::InvalidArgument(
                "No valid Bilibili login mode specified".to_string()
            ));
        };

        Ok(Box::new(BilibiliAuthProvider::new(self.client.clone(), api_mode)))
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
