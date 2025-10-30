mod api;
mod parser;
pub mod selector;

use crate::error::{DownloaderError, Result};
use crate::platform::r#trait::Platform;
use crate::types::{Auth, Stream, Subtitle, VideoInfo};
use crate::utils::http::HttpClient;
use async_trait::async_trait;
use regex::Regex;
use std::sync::Arc;

pub struct BilibiliPlatform {
    client: Arc<HttpClient>,
}

#[derive(Debug, Clone)]
pub enum VideoType {
    Bvid(String),
    Aid(String),
    Episode(String),
    Season(String),
}

impl BilibiliPlatform {
    pub fn new() -> Result<Self> {
        Ok(Self {
            client: Arc::new(HttpClient::new()?),
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

        Err(DownloaderError::InvalidUrl(format!(
            "Cannot parse bilibili URL: {}",
            url
        )))
    }
}

#[async_trait]
impl Platform for BilibiliPlatform {
    fn can_handle(&self, url: &str) -> bool {
        url.contains("bilibili.com")
            || url.contains("b23.tv")
            || url.starts_with("BV")
            || url.starts_with("av")
            || url.starts_with("ep")
            || url.starts_with("ss")
    }

    async fn parse_video(&self, url: &str, auth: Option<&Auth>) -> Result<VideoInfo> {
        let video_type = self.parse_url(url)?;
        parser::parse_video_info(&self.client, video_type, auth).await
    }

    async fn get_streams(
        &self,
        video_id: &str,
        cid: &str,
        auth: Option<&Auth>,
    ) -> Result<Vec<Stream>> {
        parser::get_play_url(&self.client, video_id, cid, auth).await
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
