use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ApiResponse<T> {
    pub code: i32,
    pub message: String,
    pub data: Option<T>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct VideoInfoData {
    pub aid: u64,
    pub bvid: String,
    pub cid: u64,
    pub title: String,
    pub desc: String,
    pub pic: String,
    pub pubdate: u64,
    pub owner: Owner,
    pub pages: Vec<PageData>,
}

#[derive(Debug, Deserialize)]
pub struct Owner {
    pub mid: u64,
    pub name: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PageData {
    pub page: usize,
    pub cid: u64,
    pub part: String,
    pub duration: u64,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct PlayUrlData {
    pub dash: Option<DashData>,
    pub durl: Option<Vec<DurlData>>,
}

#[derive(Debug, Deserialize)]
pub struct DashData {
    pub video: Vec<DashStream>,
    pub audio: Vec<DashStream>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct DashStream {
    pub id: u32,
    pub base_url: String,
    pub backup_url: Option<Vec<String>>,
    pub bandwidth: u64,
    pub codecid: Option<u32>,
    pub codecs: Option<String>,
    #[serde(default)]
    pub width: u32,
    #[serde(default)]
    pub height: u32,
    #[serde(default)]
    pub frame_rate: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct DurlData {
    pub url: String,
    pub size: u64,
    pub length: u64,
}

#[derive(Debug, Deserialize)]
pub struct SubtitleData {
    pub subtitle: SubtitleInfo,
}

#[derive(Debug, Deserialize)]
pub struct SubtitleInfo {
    pub subtitles: Vec<SubtitleItem>,
}

#[derive(Debug, Deserialize)]
pub struct SubtitleItem {
    pub lan: String,
    pub lan_doc: String,
    pub subtitle_url: String,
}
