use crate::error::{DownloaderError, Result};
use crate::types::{Stream, StreamPreferences, StreamType};

pub fn select_best_streams(
    streams: &[Stream],
    preferences: &StreamPreferences,
) -> Result<(Stream, Stream)> {
    let video_streams: Vec<&Stream> = streams
        .iter()
        .filter(|s| s.stream_type == StreamType::Video)
        .collect();

    let audio_streams: Vec<&Stream> = streams
        .iter()
        .filter(|s| s.stream_type == StreamType::Audio)
        .collect();

    if video_streams.is_empty() {
        return Err(DownloaderError::DownloadFailed(
            "No video streams available".to_string(),
        ));
    }

    if audio_streams.is_empty() {
        return Err(DownloaderError::DownloadFailed(
            "No audio streams available".to_string(),
        ));
    }

    let best_video = select_best_video(&video_streams, preferences)?;
    let best_audio = select_best_audio(&audio_streams)?;

    Ok((best_video.clone(), best_audio.clone()))
}

fn select_best_video<'a>(
    video_streams: &'a [&'a Stream],
    preferences: &StreamPreferences,
) -> Result<&'a Stream> {
    // First, try to match quality preference
    for quality_pref in &preferences.quality_priority {
        for codec_pref in &preferences.codec_priority {
            if let Some(stream) = video_streams.iter().find(|s| {
                s.quality.contains(quality_pref)
                    && s.codec.to_lowercase().contains(&codec_pref.to_lowercase())
            }) {
                tracing::info!(
                    "Selected video: {} {} ({}kbps)",
                    stream.quality,
                    stream.codec,
                    stream.bandwidth / 1000
                );
                return Ok(stream);
            }
        }
    }

    // If no match, try quality only
    for quality_pref in &preferences.quality_priority {
        if let Some(stream) = video_streams
            .iter()
            .find(|s| s.quality.contains(quality_pref))
        {
            tracing::info!(
                "Selected video (quality only): {} {} ({}kbps)",
                stream.quality,
                stream.codec,
                stream.bandwidth / 1000
            );
            return Ok(stream);
        }
    }

    // If still no match, select highest bandwidth
    let best = video_streams
        .iter()
        .max_by_key(|s| s.bandwidth)
        .ok_or_else(|| DownloaderError::DownloadFailed("No video stream found".to_string()))?;

    tracing::info!(
        "Selected video (highest bandwidth): {} {} ({}kbps)",
        best.quality,
        best.codec,
        best.bandwidth / 1000
    );

    Ok(best)
}

fn select_best_audio<'a>(audio_streams: &'a [&'a Stream]) -> Result<&'a Stream> {
    // Select highest bandwidth audio
    let best = audio_streams
        .iter()
        .max_by_key(|s| s.bandwidth)
        .ok_or_else(|| DownloaderError::DownloadFailed("No audio stream found".to_string()))?;

    tracing::info!(
        "Selected audio: {} ({}kbps)",
        best.codec,
        best.bandwidth / 1000
    );

    Ok(best)
}
