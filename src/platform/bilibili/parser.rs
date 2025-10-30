use super::api::*;
use super::api::{BangumiInfoData, CheeseInfoData};
use super::VideoType;
use serde::Deserialize;
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
    wbi_manager: Option<&mut super::wbi::WbiManager>,
) -> Result<super::ParseResult> {
    match video_type {
        VideoType::Bvid(bvid) => {
            let video = fetch_video_info_by_bvid(client, &bvid, auth).await?;
            Ok(super::ParseResult::Single(Box::new(video)))
        }
        VideoType::Aid(aid) => {
            let video = fetch_video_info_by_aid(client, &aid, auth).await?;
            Ok(super::ParseResult::Single(Box::new(video)))
        }
        VideoType::Episode(ep) => {
            let video = fetch_bangumi_info_by_ep(client, &ep, auth).await?;
            Ok(super::ParseResult::Single(Box::new(video)))
        }
        VideoType::Season(ss) => {
            let video = fetch_bangumi_info_by_ss(client, &ss, auth).await?;
            Ok(super::ParseResult::Single(Box::new(video)))
        }
        VideoType::Cheese(ep) => {
            let video = fetch_cheese_info(client, &ep, auth).await?;
            Ok(super::ParseResult::Single(Box::new(video)))
        }
        VideoType::FavoriteList(fav_info) => {
            let videos = fetch_favorite_list(client, &fav_info, auth).await?;
            Ok(super::ParseResult::Batch(videos))
        }
        VideoType::SpaceVideo(mid) => {
            let wbi = wbi_manager.ok_or_else(|| {
                DownloaderError::Api("WBI manager required for space video".to_string())
            })?;
            let videos = fetch_space_videos(client, &mid, auth, wbi).await?;
            Ok(super::ParseResult::Batch(videos))
        }
        VideoType::MediaList(media_id) => {
            let videos = fetch_media_list(client, &media_id, auth).await?;
            Ok(super::ParseResult::Batch(videos))
        }
        VideoType::SeriesList(series_info) => {
            let videos = fetch_series_list(client, &series_info, auth).await?;
            Ok(super::ParseResult::Batch(videos))
        }
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
            ep_id: None,
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
        is_bangumi: false,
        ep_id: None,
    })
}

#[allow(dead_code)]
pub async fn get_play_url(
    client: &Arc<HttpClient>,
    video_id: &str,
    cid: &str,
    auth: Option<&Auth>,
) -> Result<Vec<Stream>> {
    get_play_url_with_mode(client, video_id, cid, auth, super::ApiMode::Web).await
}

pub async fn get_play_url_with_mode(
    client: &Arc<HttpClient>,
    video_id: &str,
    cid: &str,
    auth: Option<&Auth>,
    api_mode: super::ApiMode,
) -> Result<Vec<Stream>> {
    get_play_url_with_mode_and_ep(client, video_id, cid, auth, api_mode, None).await
}

pub async fn get_play_url_with_mode_and_ep(
    client: &Arc<HttpClient>,
    video_id: &str,
    cid: &str,
    auth: Option<&Auth>,
    api_mode: super::ApiMode,
    ep_id: Option<&str>,
) -> Result<Vec<Stream>> {
    let is_bangumi = ep_id.is_some();
    
    let api = match api_mode {
        super::ApiMode::Web => {
            if is_bangumi {
                // 番剧使用不同的API端点
                let ep_param = ep_id.unwrap();
                format!(
                    "https://api.bilibili.com/pgc/player/web/v2/playurl?support_multi_audio=true&avid={}&cid={}&ep_id={}&fnval=4048&fnver=0&fourk=1&qn=127",
                    video_id, cid, ep_param
                )
            } else {
                format!(
                    "https://api.bilibili.com/x/player/wbi/playurl?avid={}&cid={}&qn=127&fnval=4048&fnver=0&fourk=1",
                    video_id, cid
                )
            }
        }
        super::ApiMode::TV => {
            if is_bangumi {
                let ep_param = ep_id.unwrap();
                format!(
                    "https://api.snm0516.aisee.tv/pgc/player/api/playurltv?avid={}&cid={}&ep_id={}&qn=127&fnval=4048&fnver=0&fourk=1",
                    video_id, cid, ep_param
                )
            } else {
                format!(
                    "https://api.snm0516.aisee.tv/x/tv/playurl?avid={}&cid={}&qn=127&fnval=4048&fnver=0&fourk=1",
                    video_id, cid
                )
            }
        }
        super::ApiMode::App => {
            // APP API 需要特殊的签名，这里使用简化版本
            format!(
                "https://app.bilibili.com/x/v2/playurl?avid={}&cid={}&qn=127&fnval=4048&fnver=0&fourk=1",
                video_id, cid
            )
        }
        super::ApiMode::International => {
            format!(
                "https://app.global.bilibili.com/intl/gateway/v2/ogv/playurl?avid={}&cid={}&qn=127&fnval=4048&fnver=0&fourk=1",
                video_id, cid
            )
        }
    };

    let response = client.get_with_auth(&api, auth).await?;
    let json_text = response.text().await?;

    tracing::debug!("Play URL response: {}", json_text);

    // 番剧API返回result字段，普通视频返回data字段
    let data = if is_bangumi {
        let api_response: super::api::BangumiApiResponse<super::api::BangumiPlayUrlResult> = serde_json::from_str(&json_text)
            .map_err(|e| DownloaderError::Parse(format!("Failed to parse bangumi play URL: {}", e)))?;

        if api_response.code != 0 {
            return Err(DownloaderError::Api(format!(
                "API error: {}",
                api_response.message
            )));
        }

        api_response
            .result
            .ok_or_else(|| DownloaderError::DownloadFailed("No play URL data".to_string()))?
            .video_info
    } else {
        let api_response: ApiResponse<PlayUrlData> = serde_json::from_str(&json_text)
            .map_err(|e| DownloaderError::Parse(format!("Failed to parse play URL: {}", e)))?;

        if api_response.code != 0 {
            return Err(DownloaderError::Api(format!(
                "API error: {}",
                api_response.message
            )));
        }

        api_response
            .data
            .ok_or_else(|| DownloaderError::DownloadFailed("No play URL data".to_string()))?
    };

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

// 番剧信息获取 - 通过 ep_id
async fn fetch_bangumi_info_by_ep(
    client: &Arc<HttpClient>,
    ep_id: &str,
    auth: Option<&Auth>,
) -> Result<VideoInfo> {
    let api = format!("https://api.bilibili.com/pgc/view/web/season?ep_id={}", ep_id);
    let response = client.get_with_auth(&api, auth).await?;
    let json_text = response.text().await?;

    tracing::debug!("Bangumi info response: {}", json_text);

    let api_response: super::api::BangumiApiResponse<BangumiInfoData> = serde_json::from_str(&json_text)
        .map_err(|e| DownloaderError::Parse(format!("Failed to parse bangumi info: {}", e)))?;

    if api_response.code != 0 {
        return Err(DownloaderError::Api(format!(
            "API error: {}",
            api_response.message
        )));
    }

    let data = api_response
        .result
        .ok_or_else(|| DownloaderError::VideoNotFound(format!("ep{}", ep_id)))?;

    convert_bangumi_to_video_info(data, ep_id)
}

// 番剧信息获取 - 通过 season_id
async fn fetch_bangumi_info_by_ss(
    client: &Arc<HttpClient>,
    season_id: &str,
    auth: Option<&Auth>,
) -> Result<VideoInfo> {
    let api = format!("https://api.bilibili.com/pgc/view/web/season?season_id={}", season_id);
    let response = client.get_with_auth(&api, auth).await?;
    let json_text = response.text().await?;

    tracing::debug!("Bangumi info response: {}", json_text);

    let api_response: super::api::BangumiApiResponse<BangumiInfoData> = serde_json::from_str(&json_text)
        .map_err(|e| DownloaderError::Parse(format!("Failed to parse bangumi info: {}", e)))?;

    if api_response.code != 0 {
        return Err(DownloaderError::Api(format!(
            "API error: {}",
            api_response.message
        )));
    }

    let data = api_response
        .result
        .ok_or_else(|| DownloaderError::VideoNotFound(format!("ss{}", season_id)))?;

    convert_bangumi_to_video_info(data, "")
}

fn convert_bangumi_to_video_info(data: BangumiInfoData, target_ep_id: &str) -> Result<VideoInfo> {
    let mut episodes = data.episodes;
    let mut title = data.title.clone();

    // 如果主episodes为空或不包含目标ep，检查section
    if !target_ep_id.is_empty() && !episodes.iter().any(|ep| ep.id.to_string() == target_ep_id) {
        for section in &data.section {
            if section.episodes.iter().any(|ep| ep.id.to_string() == target_ep_id) {
                title = format!("{}[{}]", title, section.title);
                episodes = section.episodes.clone();
                break;
            }
        }
    }

    let mut pages = Vec::new();
    let mut index = 1;
    let mut ep_id_for_first_page = None;

    for episode in episodes {
        // 跳过预告
        if episode.badge == "预告" {
            continue;
        }

        let ep_title = format!("{} {}", episode.title, episode.long_title).trim().to_string();
        let current_ep_id = episode.id.to_string();

        // 保存第一个episode的ep_id
        if ep_id_for_first_page.is_none() {
            ep_id_for_first_page = Some(current_ep_id.clone());
        }

        pages.push(Page {
            number: index,
            title: ep_title,
            cid: episode.cid.to_string(),
            duration: 0, // Duration not provided in bangumi API
            ep_id: Some(current_ep_id),
        });

        index += 1;
    }

    let pub_time = if !data.publish.pub_time.is_empty() {
        data.publish.pub_time
    } else if !pages.is_empty() {
        format_timestamp(0) // Use first episode's pub_time if available
    } else {
        String::new()
    };

    Ok(VideoInfo {
        id: format!("bangumi_{}", target_ep_id),
        aid: 0, // Bangumi doesn't have aid
        title: title.trim().to_string(),
        description: data.evaluate,
        duration: 0,
        uploader: String::new(),
        uploader_mid: String::new(),
        upload_date: pub_time,
        cover_url: data.cover,
        pages,
        is_bangumi: true,
        ep_id: ep_id_for_first_page,
    })
}

// 课程信息获取
async fn fetch_cheese_info(
    client: &Arc<HttpClient>,
    ep_id: &str,
    auth: Option<&Auth>,
) -> Result<VideoInfo> {
    let api = format!("https://api.bilibili.com/pugv/view/web/season?ep_id={}", ep_id);
    let response = client.get_with_auth(&api, auth).await?;
    let json_text = response.text().await?;

    tracing::debug!("Cheese info response: {}", json_text);

    let api_response: ApiResponse<CheeseInfoData> = serde_json::from_str(&json_text)
        .map_err(|e| DownloaderError::Parse(format!("Failed to parse cheese info: {}", e)))?;

    if api_response.code != 0 {
        return Err(DownloaderError::Api(format!(
            "API error: {}",
            api_response.message
        )));
    }

    let data = api_response
        .data
        .ok_or_else(|| DownloaderError::VideoNotFound(format!("cheese/ep{}", ep_id)))?;

    convert_cheese_to_video_info(data)
}

fn convert_cheese_to_video_info(data: CheeseInfoData) -> Result<VideoInfo> {
    let pages = data
        .episodes
        .into_iter()
        .map(|ep| Page {
            number: ep.index,
            title: ep.title.trim().to_string(),
            cid: ep.cid.to_string(),
            duration: ep.duration,
            ep_id: None, // 课程不使用ep_id
        })
        .collect::<Vec<_>>();

    let pub_time = if !pages.is_empty() {
        format_timestamp(0) // Use first episode's release_date if needed
    } else {
        String::new()
    };

    Ok(VideoInfo {
        id: format!("cheese_{}", data.up_info.mid),
        aid: 0,
        title: data.title.trim().to_string(),
        description: data.subtitle,
        duration: pages.first().map(|p| p.duration).unwrap_or(0),
        uploader: data.up_info.uname,
        uploader_mid: data.up_info.mid.to_string(),
        upload_date: pub_time,
        cover_url: data.cover,
        pages,
        is_bangumi: true, // 课程也算番剧类型
        ep_id: None,
    })
}

// 收藏夹信息获取
pub async fn fetch_favorite_list(
    client: &Arc<HttpClient>,
    fav_info: &str,
    auth: Option<&Auth>,
) -> Result<Vec<VideoInfo>> {
    let parts: Vec<&str> = fav_info.split(':').collect();
    let (fav_id, mid) = if parts.len() == 2 {
        (parts[0], parts[1])
    } else {
        return Err(DownloaderError::Parse(
            "Invalid favorite list format, expected favId:mid".to_string(),
        ));
    };

    // 如果 fav_id 为空，查找默认收藏夹
    let fav_id = if fav_id.is_empty() {
        let api = format!(
            "https://api.bilibili.com/x/v3/fav/folder/created/list-all?up_mid={}",
            mid
        );
        let response = client.get_with_auth(&api, auth).await?;
        let json_text = response.text().await?;

        #[derive(Deserialize)]
        struct FavListData {
            list: Vec<FavItem>,
        }
        #[derive(Deserialize)]
        struct FavItem {
            id: u64,
        }

        let response: ApiResponse<FavListData> = serde_json::from_str(&json_text)
            .map_err(|e| DownloaderError::Parse(format!("Failed to parse fav list: {}", e)))?;

        response
            .data
            .and_then(|d| d.list.first().map(|item| item.id.to_string()))
            .ok_or_else(|| DownloaderError::Parse("No favorite list found".to_string()))?
    } else {
        fav_id.to_string()
    };

    let page_size = 20;
    let mut all_videos = Vec::new();

    // 获取第一页以确定总数
    let api = format!(
        "https://api.bilibili.com/x/v3/fav/resource/list?media_id={}&pn=1&ps={}&order=mtime&type=2&tid=0&platform=web",
        fav_id, page_size
    );
    let response = client.get_with_auth(&api, auth).await?;
    let json_text = response.text().await?;

    tracing::debug!("Favorite list response: {}", json_text);

    let api_response: ApiResponse<FavoriteListData> = serde_json::from_str(&json_text)
        .map_err(|e| DownloaderError::Parse(format!("Failed to parse favorite list: {}", e)))?;

    if api_response.code != 0 {
        return Err(DownloaderError::Api(format!(
            "API error: {}",
            api_response.message
        )));
    }

    let data = api_response
        .data
        .ok_or_else(|| DownloaderError::Parse("No favorite list data".to_string()))?;

    let total_count = data.info.media_count;
    let total_pages = (total_count as f64 / page_size as f64).ceil() as u32;

    // 处理第一页的媒体
    if let Some(medias) = data.medias {
        for media in medias {
            // 只处理未失效的视频
            if media.attr != 0 {
                continue;
            }

            if media.page > 1 {
                // 多P视频，需要获取详细信息
                let video_info = fetch_video_info_by_aid(client, &media.id.to_string(), auth).await?;
                all_videos.push(video_info);
            } else {
                // 单P视频
                let title = media.title.clone();
                let video_info = VideoInfo {
                    id: format!("av{}", media.id),
                    aid: media.id,
                    title: title.clone(),
                    description: media.intro,
                    duration: media.duration,
                    uploader: media.upper.name,
                    uploader_mid: media.upper.mid.to_string(),
                    upload_date: format_timestamp(media.pubtime as u64),
                    cover_url: media.cover,
                    pages: vec![Page {
                        number: 1,
                        title,
                        cid: media.ugc.as_ref().map(|u| u.first_cid.to_string()).unwrap_or_default(),
                        duration: media.duration,
                        ep_id: None,
                    }],
                    is_bangumi: false,
                    ep_id: None,
                };
                all_videos.push(video_info);
            }
        }
    }

    // 获取剩余页面
    for page in 2..=total_pages {
        let api = format!(
            "https://api.bilibili.com/x/v3/fav/resource/list?media_id={}&pn={}&ps={}&order=mtime&type=2&tid=0&platform=web",
            fav_id, page, page_size
        );
        let response = client.get_with_auth(&api, auth).await?;
        let json_text = response.text().await?;

        let api_response: ApiResponse<FavoriteListData> = serde_json::from_str(&json_text)
            .map_err(|e| DownloaderError::Parse(format!("Failed to parse favorite list: {}", e)))?;

        if let Some(data) = api_response.data {
            if let Some(medias) = data.medias {
                for media in medias {
                    if media.attr != 0 {
                        continue;
                    }

                    if media.page > 1 {
                        let video_info = fetch_video_info_by_aid(client, &media.id.to_string(), auth).await?;
                        all_videos.push(video_info);
                    } else {
                        let title = media.title.clone();
                        let video_info = VideoInfo {
                            id: format!("av{}", media.id),
                            aid: media.id,
                            title: title.clone(),
                            description: media.intro,
                            duration: media.duration,
                            uploader: media.upper.name,
                            uploader_mid: media.upper.mid.to_string(),
                            upload_date: format_timestamp(media.pubtime as u64),
                            cover_url: media.cover,
                            pages: vec![Page {
                                number: 1,
                                title,
                                cid: media.ugc.as_ref().map(|u| u.first_cid.to_string()).unwrap_or_default(),
                                duration: media.duration,
                                ep_id: None,
                            }],
                            is_bangumi: false,
                            ep_id: None,
                        };
                        all_videos.push(video_info);
                    }
                }
            }
        }
    }

    Ok(all_videos)
}

// UP主空间视频获取（需要WBI签名）
pub async fn fetch_space_videos(
    client: &Arc<HttpClient>,
    mid: &str,
    auth: Option<&Auth>,
    wbi_manager: &mut super::wbi::WbiManager,
) -> Result<Vec<VideoInfo>> {
    // 获取用户信息
    let user_info_api = format!("https://api.live.bilibili.com/live_user/v1/Master/info?uid={}", mid);
    let response = client.get(&user_info_api, None).await?;
    let json_text = response.text().await?;

    let user_response: ApiResponse<UserInfoData> = serde_json::from_str(&json_text)
        .map_err(|e| DownloaderError::Parse(format!("Failed to parse user info: {}", e)))?;

    let _user_name = user_response
        .data
        .map(|d| d.info.uname)
        .unwrap_or_else(|| format!("User_{}", mid));

    let page_size = 50;
    let mut all_videos = Vec::new();

    // 获取第一页 - 使用WBI签名
    let base_params = format!("mid={}&order=pubdate&pn=1&ps={}&tid=0", mid, page_size);
    let signed_params = wbi_manager.sign_url(&base_params).await?;
    let api = format!("https://api.bilibili.com/x/space/wbi/arc/search?{}", signed_params);
    
    let response = client.get_with_auth(&api, auth).await?;
    let json_text = response.text().await?;

    tracing::debug!("Space video response: {}", json_text);

    let api_response: ApiResponse<SpaceVideoData> = serde_json::from_str(&json_text)
        .map_err(|e| DownloaderError::Parse(format!("Failed to parse space videos: {}", e)))?;

    if api_response.code != 0 {
        return Err(DownloaderError::Api(format!(
            "API error: {}",
            api_response.message
        )));
    }

    let data = api_response
        .data
        .ok_or_else(|| DownloaderError::Parse("No space video data".to_string()))?;

    let total_count = data.page.count;
    let total_pages = (total_count as f64 / page_size as f64).ceil() as u32;

    // 处理第一页的视频
    for item in data.list.vlist {
        // 获取详细视频信息（包括分P信息）
        let video_info = fetch_video_info_by_aid(client, &item.aid.to_string(), auth).await?;
        all_videos.push(video_info);
    }

    // 获取剩余页面
    for page in 2..=total_pages {
        let base_params = format!("mid={}&order=pubdate&pn={}&ps={}&tid=0", mid, page, page_size);
        let signed_params = wbi_manager.sign_url(&base_params).await?;
        let api = format!("https://api.bilibili.com/x/space/wbi/arc/search?{}", signed_params);
        
        let response = client.get_with_auth(&api, auth).await?;
        let json_text = response.text().await?;

        let api_response: ApiResponse<SpaceVideoData> = serde_json::from_str(&json_text)
            .map_err(|e| DownloaderError::Parse(format!("Failed to parse space videos: {}", e)))?;

        if let Some(data) = api_response.data {
            for item in data.list.vlist {
                let video_info = fetch_video_info_by_aid(client, &item.aid.to_string(), auth).await?;
                all_videos.push(video_info);
            }
        }
    }

    Ok(all_videos)
}

// 合集视频获取
pub async fn fetch_media_list(
    client: &Arc<HttpClient>,
    media_id: &str,
    auth: Option<&Auth>,
) -> Result<Vec<VideoInfo>> {
    let api = format!(
        "https://api.bilibili.com/x/v2/medialist/resource/list?media_id={}&pn=1&ps=20&type=1",
        media_id
    );
    let response = client.get_with_auth(&api, auth).await?;
    let json_text = response.text().await?;

    tracing::debug!("Media list response: {}", json_text);

    let api_response: ApiResponse<MediaListData> = serde_json::from_str(&json_text)
        .map_err(|e| DownloaderError::Parse(format!("Failed to parse media list: {}", e)))?;

    if api_response.code != 0 {
        return Err(DownloaderError::Api(format!(
            "API error: {}",
            api_response.message
        )));
    }

    let data = api_response
        .data
        .ok_or_else(|| DownloaderError::Parse("No media list data".to_string()))?;

    let mut all_videos = Vec::new();

    for item in data.medias {
        let video_info = fetch_video_info_by_bvid(client, &item.bv_id, auth).await?;
        all_videos.push(video_info);
    }

    Ok(all_videos)
}

// 系列视频获取
pub async fn fetch_series_list(
    client: &Arc<HttpClient>,
    series_info: &str,
    auth: Option<&Auth>,
) -> Result<Vec<VideoInfo>> {
    let parts: Vec<&str> = series_info.split(':').collect();
    let (mid, sid) = if parts.len() == 2 {
        (parts[0], parts[1])
    } else {
        return Err(DownloaderError::Parse(
            "Invalid series format, expected mid:sid".to_string(),
        ));
    };

    let page_size = 30;
    let mut all_videos = Vec::new();

    // 获取第一页
    let api = format!(
        "https://api.bilibili.com/x/series/archives?mid={}&series_id={}&pn=1&ps={}",
        mid, sid, page_size
    );
    let response = client.get_with_auth(&api, auth).await?;
    let json_text = response.text().await?;

    tracing::debug!("Series list response: {}", json_text);

    let api_response: ApiResponse<SeriesListData> = serde_json::from_str(&json_text)
        .map_err(|e| DownloaderError::Parse(format!("Failed to parse series list: {}", e)))?;

    if api_response.code != 0 {
        return Err(DownloaderError::Api(format!(
            "API error: {}",
            api_response.message
        )));
    }

    let data = api_response
        .data
        .ok_or_else(|| DownloaderError::Parse("No series list data".to_string()))?;

    let total_count = data.meta.total;
    let total_pages = (total_count as f64 / page_size as f64).ceil() as u32;

    // 处理第一页的视频
    for item in data.archives {
        let video_info = fetch_video_info_by_bvid(client, &item.bvid, auth).await?;
        all_videos.push(video_info);
    }

    // 获取剩余页面
    for page in 2..=total_pages {
        let api = format!(
            "https://api.bilibili.com/x/series/archives?mid={}&series_id={}&pn={}&ps={}",
            mid, sid, page, page_size
        );
        let response = client.get_with_auth(&api, auth).await?;
        let json_text = response.text().await?;

        let api_response: ApiResponse<SeriesListData> = serde_json::from_str(&json_text)
            .map_err(|e| DownloaderError::Parse(format!("Failed to parse series list: {}", e)))?;

        if let Some(data) = api_response.data {
            for item in data.archives {
                let video_info = fetch_video_info_by_bvid(client, &item.bvid, auth).await?;
                all_videos.push(video_info);
            }
        }
    }

    Ok(all_videos)
}

// 获取章节信息
pub async fn fetch_chapters(
    client: &Arc<HttpClient>,
    aid: &str,
    cid: &str,
) -> Result<Vec<crate::types::Chapter>> {
    let api = format!(
        "https://api.bilibili.com/x/player/v2?aid={}&cid={}",
        aid, cid
    );

    let response = client.get(&api, None).await?;
    let json_text = response.text().await?;

    tracing::debug!("Chapter info response: {}", json_text);

    // 尝试解析章节信息
    #[derive(Deserialize)]
    #[allow(dead_code)]
    struct ChapterResponse {
        data: Option<ChapterData>,
    }

    #[derive(Deserialize)]
    struct ChapterData {
        #[serde(default)]
        view_points: Vec<ViewPoint>,
        #[serde(default)]
        clip_info_list: Vec<ClipInfo>,
    }

    #[derive(Deserialize)]
    struct ViewPoint {
        content: String,
        from: u64,
        to: u64,
    }

    #[derive(Deserialize)]
    struct ClipInfo {
        #[serde(rename = "clipType")]
        clip_type: String,
        start: f64,
        end: f64,
    }

    let api_response: ApiResponse<ChapterData> = serde_json::from_str(&json_text)
        .map_err(|e| DownloaderError::Parse(format!("Failed to parse chapter info: {}", e)))?;

    if api_response.code != 0 {
        // 章节信息是可选的，如果获取失败返回空列表
        return Ok(Vec::new());
    }

    let data = match api_response.data {
        Some(d) => d,
        None => return Ok(Vec::new()),
    };

    let mut chapters = Vec::new();

    // 添加普通章节
    for vp in data.view_points {
        chapters.push(crate::types::Chapter {
            title: vp.content,
            start: vp.from,
            end: vp.to,
        });
    }

    // 添加番剧的片头片尾章节
    for clip in data.clip_info_list {
        let title = match clip.clip_type.as_str() {
            "CLIP_TYPE_OP" => "片头",
            "CLIP_TYPE_ED" => "片尾",
            _ => &clip.clip_type,
        };

        chapters.push(crate::types::Chapter {
            title: title.to_string(),
            start: clip.start as u64,
            end: clip.end as u64,
        });
    }

    Ok(chapters)
}
