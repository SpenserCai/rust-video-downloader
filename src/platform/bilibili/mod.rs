mod api;
pub mod parser;
pub mod selector;
pub mod wbi;

use crate::error::{DownloaderError, Result};
use crate::platform::r#trait::Platform;
use crate::types::{Auth, Stream, Subtitle, VideoInfo};
use crate::utils::http::HttpClient;
use async_trait::async_trait;
use regex::Regex;
use std::sync::Arc;

/// Result of parsing a video URL - can be either a single video or a batch of videos
#[derive(Debug)]
pub enum ParseResult {
    Single(Box<VideoInfo>),
    Batch(Vec<VideoInfo>),
}

pub struct BilibiliPlatform {
    client: Arc<HttpClient>,
    api_mode: ApiMode,
    wbi_manager: tokio::sync::Mutex<wbi::WbiManager>,
}

#[derive(Debug, Clone, Copy)]
pub enum ApiMode {
    Web,
    TV,
    App,
    International,
}

#[derive(Debug, Clone)]
pub enum VideoType {
    Bvid(String),
    Aid(String),
    Episode(String),
    Season(String),
    Cheese(String),
    FavoriteList(String),
    SpaceVideo(String),
    MediaList(String),
    SeriesList(String),
}

impl BilibiliPlatform {
    /// 创建使用Web API模式的BilibiliPlatform实例（主要用于测试）
    #[allow(dead_code)]
    pub fn new() -> Result<Self> {
        Self::with_api_mode(ApiMode::Web)
    }

    pub fn with_api_mode(api_mode: ApiMode) -> Result<Self> {
        let client = Arc::new(HttpClient::new()?);
        let wbi_manager = wbi::WbiManager::new(client.clone());
        Ok(Self {
            client,
            api_mode,
            wbi_manager: tokio::sync::Mutex::new(wbi_manager),
        })
    }

    fn parse_url(&self, url: &str) -> Result<VideoType> {
        // BV号: BV1xx411c7mD 或 https://www.bilibili.com/video/BV1xx411c7mD
        let bv_regex = Regex::new(r"(BV[a-zA-Z0-9]+)").unwrap();
        if let Some(caps) = bv_regex.captures(url) {
            return Ok(VideoType::Bvid(caps[1].to_string()));
        }

        // av号: av170001 或 https://www.bilibili.com/video/av170001
        let av_regex = Regex::new(r"av(\d+)").unwrap();
        if let Some(caps) = av_regex.captures(url) {
            return Ok(VideoType::Aid(caps[1].to_string()));
        }

        // 番剧 ep: ep123456 或 https://www.bilibili.com/bangumi/play/ep123456
        let ep_regex = Regex::new(r"ep(\d+)").unwrap();
        if let Some(caps) = ep_regex.captures(url) {
            return Ok(VideoType::Episode(caps[1].to_string()));
        }

        // 番剧 ss: ss12345 或 https://www.bilibili.com/bangumi/play/ss12345
        let ss_regex = Regex::new(r"ss(\d+)").unwrap();
        if let Some(caps) = ss_regex.captures(url) {
            return Ok(VideoType::Season(caps[1].to_string()));
        }

        // 课程: cheese123456 或 https://www.bilibili.com/cheese/play/ep123456
        let cheese_regex = Regex::new(r"cheese/play/ep(\d+)").unwrap();
        if let Some(caps) = cheese_regex.captures(url) {
            return Ok(VideoType::Cheese(caps[1].to_string()));
        }

        // 收藏夹: favId:mid 或 https://space.bilibili.com/{mid}/favlist?fid={favId}
        let fav_regex = Regex::new(r"space\.bilibili\.com/(\d+)/favlist\?fid=(\d+)").unwrap();
        if let Some(caps) = fav_regex.captures(url) {
            let mid = caps[1].to_string();
            let fav_id = caps[2].to_string();
            return Ok(VideoType::FavoriteList(format!("{}:{}", fav_id, mid)));
        }

        // UP主空间: mid123456 或 https://space.bilibili.com/123456
        let space_regex = Regex::new(r"space\.bilibili\.com/(\d+)").unwrap();
        if let Some(caps) = space_regex.captures(url) {
            return Ok(VideoType::SpaceVideo(caps[1].to_string()));
        }

        // 合集: https://www.bilibili.com/medialist/play/ml123456
        let media_regex = Regex::new(r"medialist/play/ml(\d+)").unwrap();
        if let Some(caps) = media_regex.captures(url) {
            return Ok(VideoType::MediaList(caps[1].to_string()));
        }

        // 系列: https://space.bilibili.com/{mid}/channel/seriesdetail?sid={sid}
        let series_regex =
            Regex::new(r"space\.bilibili\.com/(\d+)/channel/seriesdetail\?sid=(\d+)").unwrap();
        if let Some(caps) = series_regex.captures(url) {
            let mid = caps[1].to_string();
            let sid = caps[2].to_string();
            return Ok(VideoType::SeriesList(format!("{}:{}", mid, sid)));
        }

        Err(DownloaderError::InvalidUrl(format!(
            "Cannot parse bilibili URL: {}",
            url
        )))
    }
}

#[async_trait]
impl Platform for BilibiliPlatform {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    
    fn can_handle(&self, url: &str) -> bool {
        url.contains("bilibili.com")
            || url.contains("b23.tv")
            || url.starts_with("BV")
            || url.starts_with("av")
            || url.starts_with("ep")
            || url.starts_with("ss")
            || url.contains("cheese")
            || url.contains("favlist")
            || url.contains("space.bilibili.com")
            || url.contains("medialist")
            || url.contains("seriesdetail")
    }

    async fn parse_video(&self, url: &str, auth: Option<&Auth>) -> Result<VideoInfo> {
        let video_type = self.parse_url(url)?;
        let mut wbi = self.wbi_manager.lock().await;
        let result = parser::parse_video_info(&self.client, video_type, auth, Some(&mut *wbi)).await?;
        
        match result {
            ParseResult::Single(video) => Ok(*video),
            ParseResult::Batch(mut videos) => {
                // For batch results, return the first video
                // The caller should use parse_video_batch for full batch support
                if videos.is_empty() {
                    Err(DownloaderError::Parse("Batch result is empty".to_string()))
                } else {
                    tracing::warn!("Batch URL detected but only returning first video. Use parse_video_batch for full batch support.");
                    Ok(videos.remove(0))
                }
            }
        }
    }

    async fn get_streams(
        &self,
        video_id: &str,
        cid: &str,
        auth: Option<&Auth>,
    ) -> Result<Vec<Stream>> {
        parser::get_play_url_with_mode(&self.client, video_id, cid, auth, self.api_mode).await
    }

    async fn get_subtitles(&self, video_id: &str, cid: &str) -> Result<Vec<Subtitle>> {
        parser::get_subtitles(&self.client, video_id, cid).await
    }

    fn get_cover(&self, video_info: &VideoInfo) -> String {
        video_info.cover_url.clone()
    }

    fn name(&self) -> &str {
        "bilibili"
    }
}

// BilibiliPlatform specific methods
impl BilibiliPlatform {
    /// Parse a video URL and return all videos (for batch downloads)
    /// 
    /// This method handles batch URLs (favorites, space videos, playlists, etc.)
    /// and returns all videos in the batch. For single video URLs, returns a Vec with one element.
    pub async fn parse_video_batch(&self, url: &str, auth: Option<&Auth>) -> Result<Vec<VideoInfo>> {
        let video_type = self.parse_url(url)?;
        let mut wbi = self.wbi_manager.lock().await;
        let result = parser::parse_video_info(&self.client, video_type, auth, Some(&mut *wbi)).await?;
        
        match result {
            ParseResult::Single(video) => Ok(vec![*video]),
            ParseResult::Batch(videos) => Ok(videos),
        }
    }

    /// Get streams for bangumi/pgc content with ep_id
    pub async fn get_bangumi_streams(
        &self,
        video_id: &str,
        cid: &str,
        ep_id: &str,
        auth: Option<&Auth>,
    ) -> Result<Vec<Stream>> {
        parser::get_play_url_with_mode_and_ep(&self.client, video_id, cid, auth, self.api_mode, Some(ep_id)).await
    }
}
