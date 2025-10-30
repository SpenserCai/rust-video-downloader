use crate::error::{DownloaderError, Result};
use std::path::{Path, PathBuf};
use std::process::Command;

pub struct Muxer {
    ffmpeg_path: PathBuf,
}

impl Muxer {
    pub fn new(ffmpeg_path: Option<PathBuf>) -> Result<Self> {
        let path = if let Some(p) = ffmpeg_path {
            p
        } else {
            // Try to find ffmpeg in PATH
            PathBuf::from("ffmpeg")
        };

        let muxer = Self { ffmpeg_path: path };

        // Check if ffmpeg is available
        muxer.check_ffmpeg()?;

        Ok(muxer)
    }

    pub fn check_ffmpeg(&self) -> Result<String> {
        let output = Command::new(&self.ffmpeg_path)
            .arg("-version")
            .output()
            .map_err(|_| DownloaderError::FFmpegNotFound)?;

        if !output.status.success() {
            return Err(DownloaderError::FFmpegNotFound);
        }

        let version = String::from_utf8_lossy(&output.stdout);
        let first_line = version.lines().next().unwrap_or("Unknown version");

        tracing::info!("FFmpeg found: {}", first_line);
        Ok(first_line.to_string())
    }

    pub async fn mux(
        &self,
        video: &Path,
        audio: &Path,
        output: &Path,
        subtitles: &[PathBuf],
    ) -> Result<()> {
        tracing::info!("Muxing video and audio to {:?}", output);

        let mut cmd = Command::new(&self.ffmpeg_path);
        cmd.arg("-i").arg(video);
        cmd.arg("-i").arg(audio);

        // Add subtitles
        for subtitle in subtitles {
            cmd.arg("-i").arg(subtitle);
        }

        // Copy codecs (no re-encoding)
        cmd.arg("-c:v").arg("copy");
        cmd.arg("-c:a").arg("copy");

        if !subtitles.is_empty() {
            cmd.arg("-c:s").arg("mov_text");
        }

        // Overwrite output file
        cmd.arg("-y");

        // Output file
        cmd.arg(output);

        tracing::debug!("FFmpeg command: {:?}", cmd);

        let output = cmd
            .output()
            .map_err(|e| DownloaderError::MuxFailed(format!("Failed to execute ffmpeg: {}", e)))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(DownloaderError::MuxFailed(format!(
                "FFmpeg failed: {}",
                stderr
            )));
        }

        tracing::info!("Muxing completed successfully");
        Ok(())
    }

    #[allow(dead_code)]
    pub async fn mux_simple(&self, video: &Path, audio: &Path, output: &Path) -> Result<()> {
        self.mux(video, audio, output, &Vec::new()).await
    }
}
