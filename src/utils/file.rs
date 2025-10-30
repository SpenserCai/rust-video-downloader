use crate::error::Result;
use crate::types::{Page, VideoInfo};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

pub fn sanitize_filename(name: &str) -> String {
    let invalid_chars = ['<', '>', ':', '"', '/', '\\', '|', '?', '*'];
    let mut sanitized = name.to_string();

    for ch in invalid_chars {
        sanitized = sanitized.replace(ch, "_");
    }

    // Remove leading/trailing spaces and dots
    sanitized = sanitized.trim().trim_matches('.').to_string();

    // Limit length
    if sanitized.len() > 200 {
        sanitized.truncate(200);
    }

    if sanitized.is_empty() {
        sanitized = "video".to_string();
    }

    sanitized
}

pub fn parse_template(
    template: &str,
    video_info: &VideoInfo,
    page: Option<&Page>,
    quality: &str,
    codec: &str,
) -> String {
    let mut result = template.to_string();
    let mut vars = HashMap::new();

    vars.insert("<videoTitle>", sanitize_filename(&video_info.title));
    vars.insert("<bvid>", video_info.id.clone());
    vars.insert("<quality>", quality.to_string());
    vars.insert("<codec>", codec.to_string());
    vars.insert("<uploader>", sanitize_filename(&video_info.uploader));
    vars.insert("<uploaderMid>", video_info.uploader_mid.clone());
    vars.insert("<date>", video_info.upload_date.clone());

    if let Some(p) = page {
        vars.insert("<pageNumber>", p.number.to_string());
        vars.insert("<pageNumberWithZero>", format!("{:02}", p.number));
        vars.insert("<pageTitle>", sanitize_filename(&p.title));
        vars.insert("<cid>", p.cid.clone());
    }

    for (key, value) in vars {
        result = result.replace(key, &value);
    }

    result
}

pub async fn create_temp_dir(video_id: &str) -> Result<PathBuf> {
    let temp_dir = std::env::temp_dir().join("rvd").join(video_id);
    tokio::fs::create_dir_all(&temp_dir).await?;
    Ok(temp_dir)
}

pub async fn cleanup_temp_dir(dir: &Path) -> Result<()> {
    if dir.exists() {
        tokio::fs::remove_dir_all(dir).await?;
    }
    Ok(())
}

#[allow(dead_code)]
pub async fn merge_files(chunks: &[PathBuf], output: &Path) -> Result<()> {
    let mut output_file = tokio::fs::File::create(output).await?;

    for chunk in chunks {
        let mut chunk_file = tokio::fs::File::open(chunk).await?;
        tokio::io::copy(&mut chunk_file, &mut output_file).await?;
    }

    Ok(())
}

pub fn get_default_output_path(video_info: &VideoInfo, page: Option<&Page>) -> PathBuf {
    if video_info.pages.len() > 1 {
        // Multi-page video
        if let Some(p) = page {
            PathBuf::from(format!(
                "{}/P{:02}_{}.mp4",
                sanitize_filename(&video_info.title),
                p.number,
                sanitize_filename(&p.title)
            ))
        } else {
            PathBuf::from(format!(
                "{}/video.mp4",
                sanitize_filename(&video_info.title)
            ))
        }
    } else {
        // Single video
        PathBuf::from(format!("{}.mp4", sanitize_filename(&video_info.title)))
    }
}
