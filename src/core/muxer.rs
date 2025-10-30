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
        self.mux_with_chapters(video, audio, output, subtitles, &[]).await
    }

    pub async fn mux_with_chapters(
        &self,
        video: &Path,
        audio: &Path,
        output: &Path,
        subtitles: &[PathBuf],
        chapters: &[crate::types::Chapter],
    ) -> Result<()> {
        tracing::info!("Muxing video and audio to {:?}", output);

        let mut cmd = Command::new(&self.ffmpeg_path);
        cmd.arg("-i").arg(video);
        cmd.arg("-i").arg(audio);

        // Add subtitles
        for subtitle in subtitles {
            cmd.arg("-i").arg(subtitle);
        }

        // 如果有章节信息，创建章节文件
        let chapter_file = if !chapters.is_empty() {
            let chapter_path = output.with_extension("chapters.txt");
            self.create_chapter_file(&chapter_path, chapters)?;
            Some(chapter_path)
        } else {
            None
        };

        // Copy codecs (no re-encoding)
        cmd.arg("-c:v").arg("copy");
        cmd.arg("-c:a").arg("copy");

        if !subtitles.is_empty() {
            cmd.arg("-c:s").arg("mov_text");
        }

        // 添加章节元数据
        if let Some(ref chapter_path) = chapter_file {
            cmd.arg("-i").arg(chapter_path);
            cmd.arg("-map_metadata").arg(format!("{}", 2 + subtitles.len()));
        }

        // Overwrite output file
        cmd.arg("-y");

        // Output file
        cmd.arg(output);

        tracing::debug!("FFmpeg command: {:?}", cmd);

        let output_result = cmd
            .output()
            .map_err(|e| DownloaderError::MuxFailed(format!("Failed to execute ffmpeg: {}", e)))?;

        // 清理章节文件
        if let Some(chapter_path) = chapter_file {
            let _ = std::fs::remove_file(chapter_path);
        }

        if !output_result.status.success() {
            let stderr = String::from_utf8_lossy(&output_result.stderr);
            return Err(DownloaderError::MuxFailed(format!(
                "FFmpeg failed: {}",
                stderr
            )));
        }

        tracing::info!("Muxing completed successfully");
        Ok(())
    }

    fn create_chapter_file(&self, path: &Path, chapters: &[crate::types::Chapter]) -> Result<()> {
        let mut content = String::from(";FFMETADATA1\n");

        for chapter in chapters {
            content.push_str("[CHAPTER]\n");
            content.push_str("TIMEBASE=1/1\n");
            content.push_str(&format!("START={}\n", chapter.start));
            content.push_str(&format!("END={}\n", chapter.end));
            content.push_str(&format!("title={}\n", chapter.title));
        }

        std::fs::write(path, content)
            .map_err(DownloaderError::Io)?;

        Ok(())
    }

    #[allow(dead_code)]
    pub async fn mux_simple(&self, video: &Path, audio: &Path, output: &Path) -> Result<()> {
        self.mux(video, audio, output, &Vec::new()).await
    }
}
