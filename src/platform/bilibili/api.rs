use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ApiResponse<T> {
    pub code: i32,
    pub message: String,
    pub data: Option<T>,
}

// 番剧API使用 result 字段而不是 data
#[derive(Debug, Deserialize)]
pub struct BangumiApiResponse<T> {
    pub code: i32,
    pub message: String,
    pub result: Option<T>,
}

// 番剧播放URL的响应结构
#[derive(Debug, Deserialize)]
pub struct BangumiPlayUrlResult {
    pub video_info: PlayUrlData,
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

// 番剧相关数据结构
#[derive(Debug, Deserialize)]
pub struct BangumiInfoData {
    pub cover: String,
    pub title: String,
    pub evaluate: String,
    pub publish: BangumiPublish,
    pub episodes: Vec<BangumiEpisode>,
    #[serde(default)]
    pub section: Vec<BangumiSection>,
}

#[derive(Debug, Deserialize)]
pub struct BangumiPublish {
    pub pub_time: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BangumiEpisode {
    #[allow(dead_code)]
    pub aid: u64,
    pub cid: u64,
    pub id: u64, // ep_id
    pub title: String,
    pub long_title: String,
    #[serde(default)]
    pub badge: String,
    #[allow(dead_code)]
    pub pub_time: i64,
    #[serde(default)]
    #[allow(dead_code)]
    pub dimension: Option<Dimension>,
}

#[derive(Debug, Deserialize)]
pub struct BangumiSection {
    pub title: String,
    pub episodes: Vec<BangumiEpisode>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Dimension {
    #[allow(dead_code)]
    pub width: u32,
    #[allow(dead_code)]
    pub height: u32,
}

// 课程相关数据结构
#[derive(Debug, Deserialize)]
pub struct CheeseInfoData {
    pub cover: String,
    pub title: String,
    pub subtitle: String,
    pub up_info: CheeseUpInfo,
    pub episodes: Vec<CheeseEpisode>,
}

#[derive(Debug, Deserialize)]
pub struct CheeseUpInfo {
    pub mid: u64,
    pub uname: String,
}

#[derive(Debug, Deserialize)]
pub struct CheeseEpisode {
    pub index: usize,
    #[allow(dead_code)]
    pub aid: u64,
    pub cid: u64,
    #[allow(dead_code)]
    pub id: u64, // ep_id
    pub title: String,
    pub duration: u64,
    #[allow(dead_code)]
    pub release_date: i64,
}

// 收藏夹相关数据结构
#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct FavoriteListData {
    pub info: FavoriteInfo,
    pub medias: Option<Vec<FavoriteMedia>>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct FavoriteInfo {
    pub id: u64,
    pub title: String,
    pub intro: String,
    pub media_count: u32,
    pub ctime: i64,
    pub upper: FavoriteUpper,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct FavoriteUpper {
    pub mid: u64,
    pub name: String,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct FavoriteMedia {
    pub id: u64,
    pub title: String,
    pub intro: String,
    pub cover: String,
    pub duration: u64,
    pub pubtime: i64,
    pub attr: i32,
    pub page: u32,
    pub upper: FavoriteMediaUpper,
    pub ugc: Option<FavoriteUgc>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct FavoriteMediaUpper {
    pub mid: u64,
    pub name: String,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct FavoriteUgc {
    pub first_cid: u64,
}

// UP主空间相关数据结构
#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct SpaceVideoData {
    #[serde(default)]
    pub list: Option<SpaceVideoList>,
    #[serde(default)]
    pub page: Option<SpaceVideoPage>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct SpaceVideoList {
    pub vlist: Vec<SpaceVideoItem>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct SpaceVideoItem {
    pub aid: u64,
    pub bvid: String,
    pub title: String,
    pub description: String,
    pub pic: String,
    pub created: i64,
    pub length: String,
    pub author: String,
    pub mid: u64,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct SpaceVideoPage {
    pub count: u32,
    pub pn: u32,
    pub ps: u32,
}

// 用户信息数据结构
#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct UserInfoData {
    pub info: UserInfo,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct UserInfo {
    pub uid: u64,
    pub uname: String,
}

// 合集相关数据结构
#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct MediaListData {
    pub info: MediaListInfo,
    pub medias: Vec<MediaListItem>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct MediaListInfo {
    pub id: u64,
    pub title: String,
    pub intro: String,
    pub media_count: u32,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct MediaListItem {
    pub id: u64,
    pub bv_id: String,
    pub title: String,
}

// 系列相关数据结构
#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct SeriesListData {
    pub meta: SeriesMeta,
    pub archives: Vec<SeriesArchive>,
    pub page: SeriesPage,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct SeriesMeta {
    pub name: String,
    pub description: String,
    pub total: u32,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct SeriesArchive {
    pub aid: u64,
    pub bvid: String,
    pub title: String,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct SeriesPage {
    pub page_num: u32,
    pub page_size: u32,
    pub total: u32,
}

// 章节相关数据结构
#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct ViewPointData {
    pub view_points: Option<Vec<ViewPoint>>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct ViewPoint {
    pub content: String,
    pub from: u64,
    pub to: u64,
    #[serde(rename = "type")]
    pub point_type: u32,
}

// 番剧章节信息（片头片尾）
#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct ClipInfo {
    pub clip_type: String,
    pub start: f64,
    pub end: f64,
}
