//! Bilibili API client
//!
//! This module provides functions for calling Bilibili APIs.

use super::api::*;
use super::app_sign::AppSignManager;
use super::ApiMode;
use crate::error::{DownloaderError, Result};
use crate::types::{Auth, Stream, StreamType, Subtitle};
use crate::utils::http::HttpClient;
use std::collections::HashMap;
use std::sync::Arc;

/// Get play URL for a video
pub async fn get_play_url(
    client: &Arc<HttpClient>,
    video_id: &str,
    cid: &str,
    ep_id: Option<&str>,
    auth: Option<&Auth>,
    api_mode: ApiMode,
) -> Result<Vec<Stream>> {
    let is_bangumi = ep_id.is_some();

    let api = match api_mode {
        ApiMode::Web => {
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
        ApiMode::TV => {
            // TV mode requires appkey signature
            let sign_manager = AppSignManager::new_tv();
            let mut params = HashMap::new();
            
            // Add access_key if available
            if let Some(auth) = auth {
                if let Some(ref token) = auth.access_token {
                    params.insert("access_key".to_string(), token.clone());
                }
            }
            
            params.insert("appkey".to_string(), sign_manager.appkey().to_string());
            params.insert("build".to_string(), "106500".to_string());
            params.insert("cid".to_string(), cid.to_string());
            params.insert("device".to_string(), "android".to_string());
            
            if is_bangumi {
                let ep_param = ep_id.unwrap();
                params.insert("ep_id".to_string(), ep_param.to_string());
                params.insert("expire".to_string(), "0".to_string());
            }
            
            params.insert("fnval".to_string(), "4048".to_string());
            params.insert("fnver".to_string(), "0".to_string());
            params.insert("fourk".to_string(), "1".to_string());
            params.insert("mid".to_string(), "0".to_string());
            params.insert("mobi_app".to_string(), "android_tv_yst".to_string());
            params.insert("object_id".to_string(), video_id.to_string());
            params.insert("platform".to_string(), "android".to_string());
            params.insert("playurl_type".to_string(), "1".to_string());
            params.insert("qn".to_string(), "127".to_string());
            params.insert("ts".to_string(), sign_manager.get_timestamp().to_string());
            
            // Generate signature
            let sign = sign_manager.sign_params(&params);
            params.insert("sign".to_string(), sign);
            
            // Build query string
            let query_string = params
                .iter()
                .map(|(k, v)| format!("{}={}", k, v))
                .collect::<Vec<_>>()
                .join("&");
            
            let base_url = if is_bangumi {
                "https://api.snm0516.aisee.tv/pgc/player/api/playurltv"
            } else {
                "https://api.snm0516.aisee.tv/x/tv/playurl"
            };
            
            format!("{}?{}", base_url, query_string)
        }
        ApiMode::App => {
            // APP API 需要特殊的签名，这里使用简化版本
            format!(
                "https://app.bilibili.com/x/v2/playurl?avid={}&cid={}&qn=127&fnval=4048&fnver=0&fourk=1",
                video_id, cid
            )
        }
        ApiMode::International => {
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
        let api_response: BangumiApiResponse<BangumiPlayUrlResult> =
            serde_json::from_str(&json_text).map_err(|e| {
                DownloaderError::Parse(format!("Failed to parse bangumi play URL: {}", e))
            })?;

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
                extra_data: None,
            });
        }

        // 处理普通音频流
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
                extra_data: None,
            });
        }

        // 处理杜比全景声 (Dolby Atmos)
        if let Some(dolby) = dash.dolby {
            if let Some(dolby_audios) = dolby.audio {
                for audio in dolby_audios {
                    let codec = if let Some(ref codecs) = audio.codecs {
                        match codecs.as_str() {
                            "ec-3" => "E-AC-3 (Dolby)",
                            _ => codecs.as_str(),
                        }
                    } else {
                        "E-AC-3 (Dolby)"
                    };

                    streams.push(Stream {
                        stream_type: StreamType::Audio,
                        quality: format!("{}kbps (Dolby)", audio.bandwidth / 1000),
                        quality_id: audio.id,
                        codec: codec.to_string(),
                        url: audio.base_url.clone(),
                        size: 0,
                        bandwidth: audio.bandwidth,
                        extra_data: None,
                    });
                }
            }
        }

        // 处理Hi-Res无损音频 (FLAC)
        if let Some(flac) = dash.flac {
            if let Some(flac_audio) = flac.audio {
                let codec = if let Some(ref codecs) = flac_audio.codecs {
                    match codecs.as_str() {
                        "fLaC" => "FLAC (Hi-Res)",
                        _ => codecs.as_str(),
                    }
                } else {
                    "FLAC (Hi-Res)"
                };

                streams.push(Stream {
                    stream_type: StreamType::Audio,
                    quality: format!("{}kbps (Hi-Res)", flac_audio.bandwidth / 1000),
                    quality_id: flac_audio.id,
                    codec: codec.to_string(),
                    url: flac_audio.base_url.clone(),
                    size: 0,
                    bandwidth: flac_audio.bandwidth,
                    extra_data: None,
                });
            }
        }
    }

    if streams.is_empty() {
        return Err(DownloaderError::DownloadFailed(
            "No streams available".to_string(),
        ));
    }

    Ok(streams)
}

/// Get subtitles for a video
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
            url: if s.subtitle_url.starts_with("http") {
                s.subtitle_url
            } else {
                format!("https:{}", s.subtitle_url)
            },
        })
        .collect();

    Ok(subtitles)
}

/// Get chapters for a video
pub async fn fetch_chapters(
    client: &Arc<HttpClient>,
    aid: &str,
    cid: &str,
) -> Result<Vec<(String, u64, u64)>> {
    let api = format!(
        "https://api.bilibili.com/x/player/v2?aid={}&cid={}",
        aid, cid
    );

    let response = client.get(&api, None).await?;
    let json_text = response.text().await?;

    tracing::debug!("Chapter response: {}", json_text);

    let api_response: ApiResponse<ViewPointData> = serde_json::from_str(&json_text)
        .map_err(|e| DownloaderError::Parse(format!("Failed to parse chapters: {}", e)))?;

    if api_response.code != 0 {
        return Ok(Vec::new());
    }

    let data = match api_response.data {
        Some(d) => d,
        None => return Ok(Vec::new()),
    };

    let view_points = match data.view_points {
        Some(vp) => vp,
        None => return Ok(Vec::new()),
    };

    let chapters = view_points
        .into_iter()
        .map(|vp| (vp.content, vp.from, vp.to))
        .collect();

    Ok(chapters)
}

// Helper functions

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

fn get_quality_name(quality_id: u32) -> &'static str {
    for (name, id) in QUALITY_MAP {
        if *id == quality_id {
            return name;
        }
    }
    "未知画质"
}

fn get_codec_name(codec_id: u32) -> &'static str {
    match codec_id {
        7 => "AVC",
        12 => "HEVC",
        13 => "AV1",
        _ => "Unknown",
    }
}

/// Get danmaku (bullet comments) for a video
///
/// Downloads and formats Bilibili danmaku content.
///
/// # Arguments
///
/// * `client` - HTTP client
/// * `cid` - Video CID
/// * `format` - Desired danmaku format (XML or ASS)
///
/// # Returns
///
/// Formatted danmaku content as a string
pub async fn get_danmaku(
    _client: &Arc<HttpClient>,
    cid: &str,
    format: crate::core::danmaku::DanmakuFormat,
) -> Result<String> {
    use crate::core::danmaku::{convert_xml_to_ass, format_xml, DanmakuFormat};

    // Download XML format danmaku
    let api = format!("https://comment.bilibili.com/{}.xml", cid);
    tracing::debug!("Fetching danmaku from: {}", api);

    // Create a client with automatic decompression disabled
    // We need to manually decompress because Bilibili's deflate encoding causes issues with reqwest
    let raw_client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .timeout(std::time::Duration::from_secs(60))
        .no_gzip()
        .no_deflate()
        .no_brotli()
        .build()
        .map_err(DownloaderError::Network)?;

    let response = raw_client.get(&api).send().await?;

    if !response.status().is_success() {
        return Err(DownloaderError::DownloadFailed(format!(
            "Failed to fetch danmaku: HTTP {}",
            response.status()
        )));
    }

    let bytes = response.bytes().await?;

    // Decompress XML content
    let xml_content = decompress_danmaku(&bytes)?;

    if xml_content.is_empty() || !xml_content.contains("<d ") {
        tracing::info!("No danmaku available for cid: {}", cid);
        return Ok(String::new());
    }

    // Format according to requested format
    match format {
        DanmakuFormat::Xml => format_xml(&xml_content),
        DanmakuFormat::Ass => convert_xml_to_ass(&xml_content),
    }
}

/// Decompress danmaku data
///
/// Tries multiple decompression methods (UTF-8, deflate, gzip) to handle
/// Bilibili's various compression formats.
fn decompress_danmaku(bytes: &[u8]) -> Result<String> {
    // Try to decode as UTF-8 directly first
    match String::from_utf8(bytes.to_vec()) {
        Ok(text) => Ok(text),
        Err(_) => {
            // Try deflate decompression (most common for Bilibili danmaku API)
            use flate2::read::DeflateDecoder;
            use std::io::Read;

            let mut decoder = DeflateDecoder::new(bytes);
            let mut decompressed = String::new();
            match decoder.read_to_string(&mut decompressed) {
                Ok(_) => {
                    tracing::debug!("Decompressed danmaku with deflate");
                    Ok(decompressed)
                }
                Err(_) => {
                    // Try gzip as fallback
                    use flate2::read::GzDecoder;
                    let mut decoder = GzDecoder::new(bytes);
                    let mut decompressed = String::new();
                    decoder.read_to_string(&mut decompressed).map_err(|e| {
                        DownloaderError::DownloadFailed(format!(
                            "Failed to decompress danmaku: {}",
                            e
                        ))
                    })?;
                    tracing::debug!("Decompressed danmaku with gzip");
                    Ok(decompressed)
                }
            }
        }
    }
}
