use super::api::*;
use super::VideoType;
use crate::error::{DownloaderError, Result};
use crate::types::{Auth, Page, Stream, StreamType, Subtitle, VideoInfo};
use crate::utils::http::HttpClient;
use std::sync::Arc;

const QUALITY_MAP: &[(&str, u32)] = &[
    ("8K 超高清", 127),
    ("杜比视界", 126),
    ("HDR 真彩", 125),
    ("4K 超清", 120),
    ("1080P 60帧", 116),
    ("1080P 高码率", 112),
    ("1080P 高清", 80),
    ("720P 60帧", 74),
    ("720P 高清", 64),
    ("480P 清晰", 32),
    ("360P 流畅", 16),
];

pub async fn parse_video_info(
    client: &Arc<HttpClient>,
    video_type: VideoType,
    auth: Option<&Auth>,
) -> Result<VideoInfo> {
    match video_type {
        VideoType::Bvid(bvid) => fetch_video_info_by_bvid(client, &bvid, auth).await,
        VideoType::Aid(aid) => fetch_video_info_by_aid(client, &aid, auth).await,
        VideoType::Episode(_ep) => Err(DownloaderError::Parse(
            "Bangumi support not yet implemented".to_string(),
        )),
        VideoType::Season(_ss) => Err(DownloaderError::Parse(
            "Bangumi support not yet implemented".to_string(),
        )),
    }
}

async fn fetch_video_info_by_bvid(
    client: &Arc<HttpClient>,
    bvid: &str,
    auth: Option<&Auth>,
) -> Result<VideoInfo> {
    let api = format!(
        "https://api.bilibili.com/x/web-interface/view?bvid={}",
        bvid
    );
    let response = client.get_with_auth(&api, auth).await?;
    let json_text = response.text().await?;

    tracing::debug!("Video info response: {}", json_text);

    let api_response: ApiResponse<VideoInfoData> = serde_json::from_str(&json_text)
        .map_err(|e| DownloaderError::Parse(format!("Failed to parse video info: {}", e)))?;

    if api_response.code != 0 {
        return Err(DownloaderError::Api(format!(
            "API error: {}",
            api_response.message
        )));
    }

    let data = api_response
        .data
        .ok_or_else(|| DownloaderError::VideoNotFound(bvid.to_string()))?;

    convert_to_video_info(data)
}

async fn fetch_video_info_by_aid(
    client: &Arc<HttpClient>,
    aid: &str,
    auth: Option<&Auth>,
) -> Result<VideoInfo> {
    let api = format!("https://api.bilibili.com/x/web-interface/view?aid={}", aid);
    let response = client.get_with_auth(&api, auth).await?;
    let json_text = response.text().await?;

    tracing::debug!("Video info response: {}", json_text);

    let api_response: ApiResponse<VideoInfoData> = serde_json::from_str(&json_text)
        .map_err(|e| DownloaderError::Parse(format!("Failed to parse video info: {}", e)))?;

    if api_response.code != 0 {
        return Err(DownloaderError::Api(format!(
            "API error: {}",
            api_response.message
        )));
    }

    let data = api_response
        .data
        .ok_or_else(|| DownloaderError::VideoNotFound(aid.to_string()))?;

    convert_to_video_info(data)
}

fn convert_to_video_info(data: VideoInfoData) -> Result<VideoInfo> {
    let duration = data.pages.first().map(|p| p.duration).unwrap_or(0);

    let pages = data
        .pages
        .into_iter()
        .map(|p| Page {
            number: p.page,
            title: p.part,
            cid: p.cid.to_string(),
            duration: p.duration,
        })
        .collect();

    Ok(VideoInfo {
        id: data.bvid,
        aid: data.aid,
        title: data.title,
        description: data.desc,
        duration,
        uploader: data.owner.name,
        uploader_mid: data.owner.mid.to_string(),
        upload_date: format_timestamp(data.pubdate),
        cover_url: data.pic,
        pages,
    })
}

pub async fn get_play_url(
    client: &Arc<HttpClient>,
    video_id: &str,
    cid: &str,
    auth: Option<&Auth>,
) -> Result<Vec<Stream>> {
    // 使用 WBI 签名的播放地址 API
    let api = format!(
        "https://api.bilibili.com/x/player/wbi/playurl?avid={}&cid={}&qn=127&fnval=4048&fnver=0&fourk=1",
        video_id, cid
    );

    let response = client.get_with_auth(&api, auth).await?;
    let json_text = response.text().await?;

    tracing::debug!("Play URL response: {}", json_text);

    let api_response: ApiResponse<PlayUrlData> = serde_json::from_str(&json_text)
        .map_err(|e| DownloaderError::Parse(format!("Failed to parse play URL: {}", e)))?;

    if api_response.code != 0 {
        return Err(DownloaderError::Api(format!(
            "API error: {}",
            api_response.message
        )));
    }

    let data = api_response
        .data
        .ok_or_else(|| DownloaderError::DownloadFailed("No play URL data".to_string()))?;

    let mut streams = Vec::new();

    if let Some(dash) = data.dash {
        // DASH format (separate video and audio)
        for video in dash.video {
            let quality_name = get_quality_name(video.id);
            let codec = get_codec_name(video.codecid.unwrap_or(7));

            streams.push(Stream {
                stream_type: StreamType::Video,
                quality: quality_name.to_string(),
                quality_id: video.id,
                codec: codec.to_string(),
                url: video.base_url.clone(),
                size: 0, // Size not provided in API
                bandwidth: video.bandwidth,
            });
        }

        for audio in dash.audio {
            let codec = if let Some(ref codecs) = audio.codecs {
                match codecs.as_str() {
                    "mp4a.40.2" | "mp4a.40.5" => "M4A",
                    "ec-3" => "E-AC-3",
                    "fLaC" => "FLAC",
                    _ => codecs.as_str(),
                }
            } else {
                "M4A"
            };

            streams.push(Stream {
                stream_type: StreamType::Audio,
                quality: format!("{}kbps", audio.bandwidth / 1000),
                quality_id: audio.id,
                codec: codec.to_string(),
                url: audio.base_url.clone(),
                size: 0,
                bandwidth: audio.bandwidth,
            });
        }
    }

    if streams.is_empty() {
        return Err(DownloaderError::DownloadFailed(
            "No streams available".to_string(),
        ));
    }

    Ok(streams)
}

pub async fn get_subtitles(
    client: &Arc<HttpClient>,
    video_id: &str,
    cid: &str,
) -> Result<Vec<Subtitle>> {
    let api = format!(
        "https://api.bilibili.com/x/player/wbi/v2?aid={}&cid={}",
        video_id, cid
    );

    let response = client.get(&api, None).await?;
    let json_text = response.text().await?;

    tracing::debug!("Subtitle response: {}", json_text);

    let api_response: ApiResponse<SubtitleData> = serde_json::from_str(&json_text)
        .map_err(|e| DownloaderError::Parse(format!("Failed to parse subtitles: {}", e)))?;

    if api_response.code != 0 {
        // Subtitles are optional, so we just return empty vec on error
        return Ok(Vec::new());
    }

    let data = match api_response.data {
        Some(d) => d,
        None => return Ok(Vec::new()),
    };

    let subtitles = data
        .subtitle
        .subtitles
        .into_iter()
        .map(|s| Subtitle {
            language: s.lan_doc,
            language_code: s.lan,
            url: if s.subtitle_url.starts_with("//") {
                format!("https:{}", s.subtitle_url)
            } else {
                s.subtitle_url
            },
        })
        .collect();

    Ok(subtitles)
}

fn get_quality_name(quality_id: u32) -> &'static str {
    for (name, id) in QUALITY_MAP {
        if *id == quality_id {
            return name;
        }
    }
    "Unknown"
}

fn get_codec_name(codec_id: u32) -> &'static str {
    match codec_id {
        7 => "AVC",
        12 => "HEVC",
        13 => "AV1",
        _ => "UNKNOWN",
    }
}

fn format_timestamp(timestamp: u64) -> String {
    use std::time::{Duration, UNIX_EPOCH};
    let datetime = UNIX_EPOCH + Duration::from_secs(timestamp);
    // Simple formatting - in production you'd use chrono
    format!("{:?}", datetime)
}
