use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
    #[serde(default)]
    pub is_bangumi: bool, // 是否是番剧/课程
    #[serde(default)]
    pub ep_id: Option<String>, // 番剧的ep_id（如果是番剧）

    // 扩展字段（用于平台特定数据）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extra_data: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Page {
    pub number: usize,
    pub title: String,
    pub cid: String,
    pub duration: u64,
    #[serde(default)]
    pub ep_id: Option<String>, // 番剧的ep_id（如果是番剧的话）
}

impl Page {
    /// 便捷地创建StreamContext
    /// 自动包含cid和ep_id（如果存在）
    pub fn to_stream_context(&self, video_id: &str) -> StreamContext {
        let mut context = StreamContext::new(video_id);
        context = context.with_extra("cid", &self.cid);
        if let Some(ref ep_id) = self.ep_id {
            context = context.with_extra("ep_id", ep_id);
        }
        context
    }
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

    // 扩展字段
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extra_data: Option<serde_json::Value>,
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

    // 扩展字段（用于平台特定认证）
    pub extra: HashMap<String, String>,
}

impl Auth {
    pub fn new() -> Self {
        Self {
            cookie: None,
            access_token: None,
            extra: HashMap::new(),
        }
    }

    pub fn with_cookie(mut self, cookie: String) -> Self {
        self.cookie = Some(cookie);
        self
    }

    pub fn with_token(mut self, token: String) -> Self {
        self.access_token = Some(token);
        self
    }

    pub fn with_extra(mut self, key: String, value: String) -> Self {
        self.extra.insert(key, value);
        self
    }
}

impl Default for Auth {
    fn default() -> Self {
        Self::new()
    }
}

impl From<&crate::utils::config::AuthConfig> for Auth {
    fn from(config: &crate::utils::config::AuthConfig) -> Self {
        Self {
            cookie: config.cookie.clone(),
            access_token: config.access_token.clone(),
            extra: HashMap::new(),
        }
    }
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chapter {
    pub title: String,
    pub start: u64, // 开始时间（秒）
    pub end: u64,   // 结束时间（秒）
}

/// 流上下文 - 用于传递平台特定的参数
#[derive(Debug, Clone)]
pub struct StreamContext {
    /// 视频标识符
    pub video_id: String,
    /// 平台特定参数（使用JSON Value支持任意类型）
    pub extra: HashMap<String, serde_json::Value>,
}

impl StreamContext {
    pub fn new(video_id: impl Into<String>) -> Self {
        Self {
            video_id: video_id.into(),
            extra: HashMap::new(),
        }
    }

    /// 添加任意可序列化的值
    pub fn with_extra<T: serde::Serialize>(mut self, key: impl Into<String>, value: T) -> Self {
        if let Ok(json_value) = serde_json::to_value(value) {
            self.extra.insert(key.into(), json_value);
        }
        self
    }

    /// 获取字符串值
    pub fn get_str(&self, key: &str) -> Option<&str> {
        self.extra.get(key)?.as_str()
    }

    /// 获取u64值
    pub fn get_u64(&self, key: &str) -> Option<u64> {
        self.extra.get(key)?.as_u64()
    }

    /// 获取i64值
    pub fn get_i64(&self, key: &str) -> Option<i64> {
        self.extra.get(key)?.as_i64()
    }

    /// 获取bool值
    pub fn get_bool(&self, key: &str) -> Option<bool> {
        self.extra.get(key)?.as_bool()
    }

    /// 获取原始JSON值
    pub fn get(&self, key: &str) -> Option<&serde_json::Value> {
        self.extra.get(key)
    }

    /// 获取并反序列化为指定类型
    pub fn get_as<T: serde::de::DeserializeOwned>(&self, key: &str) -> Option<T> {
        let value = self.extra.get(key)?;
        serde_json::from_value(value.clone()).ok()
    }
}

/// 批量下载类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BatchType {
    Playlist,
    Favorites,
    UserVideos,
    Series,
    Season,
    Collection,
    Custom,
}

/// 分页信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageInfo {
    pub current_page: usize,
    pub page_size: usize,
    pub total_pages: Option<usize>,
}

/// 批量下载结果
#[derive(Debug, Clone)]
pub struct BatchResult {
    /// 当前批次的视频列表
    pub videos: Vec<VideoInfo>,
    /// 总数量（如果已知）
    pub total_count: Option<usize>,
    /// 是否还有更多数据
    pub has_more: bool,
    /// 继续标记（用于获取下一页）
    pub continuation: Option<String>,
    /// 批量类型
    pub batch_type: Option<BatchType>,
    /// 分页信息
    pub page_info: Option<PageInfo>,
}

impl BatchResult {
    /// 创建单个视频的批量结果
    pub fn single(video: VideoInfo) -> Self {
        Self {
            videos: vec![video],
            total_count: Some(1),
            has_more: false,
            continuation: None,
            batch_type: None,
            page_info: Some(PageInfo {
                current_page: 1,
                page_size: 1,
                total_pages: Some(1),
            }),
        }
    }

    /// 创建完整批量结果
    pub fn batch(videos: Vec<VideoInfo>) -> Self {
        let count = videos.len();
        Self {
            videos,
            total_count: Some(count),
            has_more: false,
            continuation: None,
            batch_type: None,
            page_info: None,
        }
    }

    /// 创建带类型和分页信息的批量结果
    pub fn with_metadata(
        videos: Vec<VideoInfo>,
        batch_type: BatchType,
        page_info: PageInfo,
        total_count: Option<usize>,
    ) -> Self {
        let has_more = page_info
            .total_pages
            .map(|total| page_info.current_page < total)
            .unwrap_or(false);

        Self {
            videos,
            total_count,
            has_more,
            continuation: None,
            batch_type: Some(batch_type),
            page_info: Some(page_info),
        }
    }
}
