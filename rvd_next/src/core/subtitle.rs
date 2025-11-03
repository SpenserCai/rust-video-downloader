use crate::error::{DownloaderError, Result};
use crate::types::Subtitle;
use crate::utils::http::HttpClient;
use serde::Deserialize;
use std::path::Path;
use std::sync::Arc;

/// JSON subtitle format (used by multiple platforms)
#[derive(Debug, Deserialize)]
struct JsonSubtitle {
    body: Vec<SubtitleItem>,
}

#[derive(Debug, Deserialize)]
struct SubtitleItem {
    from: f64,
    to: f64,
    content: String,
}

pub async fn download_and_convert_subtitle(
    client: &Arc<HttpClient>,
    subtitle: &Subtitle,
    output: &Path,
) -> Result<()> {
    tracing::info!(
        "Downloading subtitle: {} -> {:?}",
        subtitle.language,
        output
    );

    // Download subtitle JSON
    let response = client.get(&subtitle.url, None).await?;
    let json_text = response.text().await?;

    // Parse JSON
    let json_subtitle: JsonSubtitle = serde_json::from_str(&json_text)
        .map_err(|e| DownloaderError::Parse(format!("Failed to parse subtitle: {}", e)))?;

    // Convert to SRT format
    let srt_content = convert_to_srt(&json_subtitle);

    // Write to file
    tokio::fs::write(output, srt_content).await?;

    tracing::info!("Subtitle saved: {:?}", output);
    Ok(())
}

fn convert_to_srt(subtitle: &JsonSubtitle) -> String {
    let mut srt = String::new();

    for (index, item) in subtitle.body.iter().enumerate() {
        // Subtitle index (1-based)
        srt.push_str(&format!("{}\n", index + 1));

        // Timestamp
        let start = format_timestamp(item.from);
        let end = format_timestamp(item.to);
        srt.push_str(&format!("{} --> {}\n", start, end));

        // Content
        srt.push_str(&item.content);
        srt.push_str("\n\n");
    }

    srt
}

fn format_timestamp(seconds: f64) -> String {
    let hours = (seconds / 3600.0).floor() as u32;
    let minutes = ((seconds % 3600.0) / 60.0).floor() as u32;
    let secs = (seconds % 60.0).floor() as u32;
    let millis = ((seconds % 1.0) * 1000.0).floor() as u32;

    format!("{:02}:{:02}:{:02},{:03}", hours, minutes, secs, millis)
}
