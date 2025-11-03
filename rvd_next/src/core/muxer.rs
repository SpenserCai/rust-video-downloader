use crate::error::{DownloaderError, Result};
use std::path::{Path, PathBuf};
use std::process::Command;

pub struct Muxer {
    ffmpeg_path: PathBuf,
    ffmpeg_version: Option<(u32, u32)>, // (major, minor)
    use_mp4box: bool,
}

impl Muxer {
    #[allow(dead_code)]
    pub fn new(ffmpeg_path: Option<PathBuf>) -> Result<Self> {
        Self::new_with_options(ffmpeg_path, false)
    }

    pub fn new_with_options(ffmpeg_path: Option<PathBuf>, use_mp4box: bool) -> Result<Self> {
        let path = if let Some(p) = ffmpeg_path {
            p
        } else {
            // Try to find ffmpeg in PATH
            PathBuf::from("ffmpeg")
        };

        let mut muxer = Self {
            ffmpeg_path: path,
            ffmpeg_version: None,
            use_mp4box,
        };

        // Check if ffmpeg is available and get version
        muxer.check_ffmpeg()?;

        Ok(muxer)
    }

    pub fn check_ffmpeg(&mut self) -> Result<String> {
        let output = Command::new(&self.ffmpeg_path)
            .arg("-version")
            .output()
            .map_err(|_| DownloaderError::FFmpegNotFound)?;

        if !output.status.success() {
            return Err(DownloaderError::FFmpegNotFound);
        }

        let version_output = String::from_utf8_lossy(&output.stdout);
        let first_line = version_output.lines().next().unwrap_or("Unknown version");

        // 解析FFmpeg版本号
        self.ffmpeg_version = self.parse_ffmpeg_version(&version_output);

        if let Some((major, minor)) = self.ffmpeg_version {
            tracing::info!("FFmpeg found: {} (version {}.{})", first_line, major, minor);
        } else {
            tracing::info!("FFmpeg found: {}", first_line);
        }

        Ok(first_line.to_string())
    }

    fn parse_ffmpeg_version(&self, version_output: &str) -> Option<(u32, u32)> {
        // 查找 "ffmpeg version X.Y" 或 "libavutil X.Y"
        for line in version_output.lines() {
            if line.contains("ffmpeg version") {
                // 尝试解析 "ffmpeg version 5.1.2" 格式
                if let Some(version_str) = line.split_whitespace().nth(2) {
                    if let Some((major, minor)) = Self::parse_version_string(version_str) {
                        return Some((major, minor));
                    }
                }
            } else if line.contains("libavutil") {
                // 尝试解析 "libavutil 57.17.100" 格式
                if let Some(version_str) = line.split_whitespace().nth(1) {
                    if let Some((major, minor)) = Self::parse_version_string(version_str) {
                        return Some((major, minor));
                    }
                }
            }
        }
        None
    }

    fn parse_version_string(version_str: &str) -> Option<(u32, u32)> {
        let parts: Vec<&str> = version_str.split('.').collect();
        if parts.len() >= 2 {
            if let (Ok(major), Ok(minor)) = (parts[0].parse::<u32>(), parts[1].parse::<u32>()) {
                return Some((major, minor));
            }
        }
        None
    }

    /// 检查FFmpeg是否支持杜比视界 (Dolby Vision)
    /// FFmpeg 5.0+ 或 libavutil 57.17+ 支持杜比视界
    pub fn supports_dolby_vision(&self) -> bool {
        if let Some((major, minor)) = self.ffmpeg_version {
            // 检查是否是 libavutil 版本
            if major == 57 && minor >= 17 {
                return true;
            }
            // 检查是否是 FFmpeg 主版本
            if major >= 5 {
                return true;
            }
        }
        false
    }

    pub async fn mux(
        &self,
        video: &Path,
        audio: &Path,
        output: &Path,
        subtitles: &[PathBuf],
    ) -> Result<()> {
        self.mux_with_chapters(video, audio, output, subtitles, &[])
            .await
    }

    pub async fn mux_with_chapters(
        &self,
        video: &Path,
        audio: &Path,
        output: &Path,
        subtitles: &[PathBuf],
        chapters: &[crate::types::Chapter],
    ) -> Result<()> {
        self.mux_with_options(video, audio, output, subtitles, chapters, false)
            .await
    }

    pub async fn mux_with_options(
        &self,
        video: &Path,
        audio: &Path,
        output: &Path,
        subtitles: &[PathBuf],
        chapters: &[crate::types::Chapter],
        is_dolby_vision: bool,
    ) -> Result<()> {
        tracing::info!("Muxing video and audio to {:?}", output);

        // 检查是否需要使用mp4box处理杜比视界
        let should_use_mp4box =
            self.use_mp4box || (is_dolby_vision && !self.supports_dolby_vision());

        if should_use_mp4box && is_dolby_vision && !self.supports_dolby_vision() {
            tracing::warn!(
                "检测到杜比视界清晰度且您的FFmpeg版本小于5.0，建议使用mp4box混流或升级FFmpeg"
            );
            tracing::warn!("当前将使用FFmpeg继续混流，但可能无法正确处理杜比视界元数据");
            // 注意：实际的mp4box实现需要额外的工作，这里先使用FFmpeg
            // TODO: 实现mp4box混流逻辑
        }

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
            cmd.arg("-map_metadata")
                .arg(format!("{}", 2 + subtitles.len()));
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

        std::fs::write(path, content).map_err(DownloaderError::Io)?;

        Ok(())
    }

    #[allow(dead_code)]
    pub async fn mux_simple(&self, video: &Path, audio: &Path, output: &Path) -> Result<()> {
        self.mux(video, audio, output, &Vec::new()).await
    }
}
