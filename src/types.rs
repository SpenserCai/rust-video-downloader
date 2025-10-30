use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoInfo {
    pub id: String, // BVID
    pub aid: u64,   // Numeric AID
    pub title: String,
    pub description: String,
    pub duration: u64,
    pub uploader: String,
    pub uploader_mid: String,
    pub upload_date: String,
    pub cover_url: String,
    pub pages: Vec<Page>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Page {
    pub number: usize,
    pub title: String,
    pub cid: String,
    pub duration: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stream {
    pub stream_type: StreamType,
    pub quality: String,
    pub quality_id: u32,
    pub codec: String,
    pub url: String,
    pub size: u64,
    pub bandwidth: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StreamType {
    Video,
    Audio,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subtitle {
    pub language: String,
    pub language_code: String,
    pub url: String,
}

#[derive(Debug, Clone)]
pub struct Auth {
    pub cookie: Option<String>,
    pub access_token: Option<String>,
}

#[derive(Debug, Clone)]
pub struct StreamPreferences {
    pub quality_priority: Vec<String>,
    pub codec_priority: Vec<String>,
}

impl Default for StreamPreferences {
    fn default() -> Self {
        Self {
            quality_priority: vec!["1080P".to_string(), "720P".to_string(), "480P".to_string()],
            codec_priority: vec!["avc".to_string(), "hevc".to_string(), "av1".to_string()],
        }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct DownloadedComponents {
    pub video_path: std::path::PathBuf,
    pub audio_path: std::path::PathBuf,
    pub subtitle_paths: Vec<std::path::PathBuf>,
    pub cover_path: Option<std::path::PathBuf>,
}
