//! Orchestrator - coordinates the download process
//!
//! The orchestrator is responsible for coordinating all components to complete
//! the download workflow. It manages the platform registry, selects the appropriate
//! platform for each URL, and orchestrates the download process.

use crate::app::registry::PlatformRegistry;
use crate::cli::Cli;
use crate::core::danmaku;
use crate::core::downloader::Downloader;
use crate::core::muxer::Muxer;
use crate::core::progress::ProgressTracker;
use crate::core::subtitle;
use crate::error::{DownloaderError, Result};
use crate::platform::bilibili::parser;
use crate::platform::bilibili::selector::select_best_streams;
use crate::platform::{Platform, PlatformFeature};
use crate::types::{Auth, BatchType, Page, Stream, StreamPreferences, StreamType, VideoInfo};
use crate::utils::config::Config;
use crate::utils::file;
use crate::utils::http::HttpClient;
use dialoguer::Select;
use std::path::PathBuf;
use std::sync::Arc;

/// Orchestrator - coordinates the download process
///
/// The orchestrator manages the entire download workflow:
/// - Platform selection via registry
/// - Video information extraction
/// - Stream selection and downloading
/// - Muxing and post-processing
/// - Batch download with streaming pagination
pub struct Orchestrator {
    /// Platform registry for managing all registered platforms
    registry: PlatformRegistry,
    /// Downloader for fetching video/audio streams
    downloader: Arc<Downloader>,
    /// Muxer for combining video and audio
    muxer: Arc<Muxer>,
    /// Progress tracker for download progress
    progress: Arc<ProgressTracker>,
    /// Configuration
    config: Config,
    /// HTTP client
    http_client: Arc<HttpClient>,
    /// Override authentication (set via login command)
    override_auth: Option<Auth>,
}

impl Orchestrator {
    /// Create a new orchestrator
    ///
    /// # Arguments
    ///
    /// * `config` - Configuration
    /// * `cli` - CLI arguments
    ///
    /// # Returns
    ///
    /// A new orchestrator instance
    pub fn new(config: Config, cli: &Cli) -> Result<Self> {
        let http_client = Arc::new(HttpClient::new()?);

        // Create platform registry
        let mut registry = PlatformRegistry::new();

        // Register Bilibili platform
        let api_mode = cli.get_api_mode();
        let bilibili = Arc::new(crate::platform::bilibili::BilibiliPlatform::new(api_mode)?);
        registry.register(bilibili);

        // Future: Register more platforms here
        // let youtube = Arc::new(crate::platform::youtube::YouTubePlatform::new()?);
        // registry.register(youtube);

        tracing::info!("Registered {} platform(s)", registry.count());

        // Configure downloader with aria2c settings
        let mut downloader = Downloader::new(http_client.clone(), cli.threads);

        // Determine download method from CLI or config
        let use_aria2c =
            cli.use_aria2c || config.aria2c.as_ref().map(|a| a.enabled).unwrap_or(false);

        if use_aria2c {
            downloader = downloader.with_method(crate::core::downloader::DownloadMethod::Aria2c);

            // Set aria2c path from CLI or config
            if let Some(ref path) = cli.aria2c_path {
                downloader = downloader.with_aria2c_path(path.clone());
            } else if let Some(ref aria2c_config) = config.aria2c {
                if let Some(ref path) = aria2c_config.path {
                    downloader = downloader.with_aria2c_path(path.clone());
                }
            }

            // Set custom aria2c args from CLI or config
            if let Some(ref args) = cli.aria2c_args {
                downloader = downloader.with_aria2c_args(args.clone());
            } else if let Some(ref aria2c_config) = config.aria2c {
                if let Some(ref args) = aria2c_config.args {
                    downloader = downloader.with_aria2c_args(args.clone());
                }
            }

            tracing::info!("aria2c download mode enabled");
        }

        let downloader = Arc::new(downloader);

        let muxer = Arc::new(Muxer::new_with_options(
            cli.ffmpeg_path
                .clone()
                .or_else(|| config.paths.as_ref().and_then(|p| p.ffmpeg.clone())),
            cli.use_mp4box,
        )?);
        let progress = Arc::new(ProgressTracker::new());

        Ok(Self {
            registry,
            downloader,
            muxer,
            progress,
            config,
            http_client,
            override_auth: None,
        })
    }

    /// Set authentication override (used when login is performed before download)
    pub fn set_auth(&mut self, auth: Option<Auth>) {
        self.override_auth = auth;
    }

    /// Run the download process
    ///
    /// This is the main entry point for the orchestrator. It:
    /// 1. Selects the appropriate platform for the URL
    /// 2. Determines if it's a batch or single download
    /// 3. Implements streaming batch download for large collections
    /// 4. Downloads each video with progress tracking
    ///
    /// # Arguments
    ///
    /// * `cli` - CLI arguments
    ///
    /// # Returns
    ///
    /// Ok(()) on success, error otherwise
    pub async fn run(&self, cli: Cli) -> Result<()> {
        let url = cli
            .url
            .as_ref()
            .ok_or_else(|| DownloaderError::Parse("No URL provided for download".to_string()))?;

        tracing::info!("Starting download for URL: {}", url);

        // Use registry to select platform
        let platform = self.registry.select_platform(url)?;
        tracing::info!("Using platform: {}", platform.name());

        // Display platform capabilities in verbose mode
        if cli.verbose {
            let meta = platform.metadata();
            tracing::debug!("Platform: {} v{}", meta.display_name, meta.version);
            tracing::debug!("Capabilities: {:?}", meta.capabilities);
        }

        let auth = self.build_auth(&cli);

        // Check if this is a batch URL
        let is_batch = platform.is_batch_url(url);

        if is_batch {
            tracing::info!("Detected batch URL, using streaming batch download");
            self.run_batch_download(url, &cli, platform.as_ref(), auth.as_ref())
                .await?;
        } else {
            tracing::info!("Single video download");
            self.run_single_download(url, &cli, platform.as_ref(), auth.as_ref())
                .await?;
        }

        Ok(())
    }

    /// Run batch download with streaming pagination
    ///
    /// This method implements streaming batch download to avoid loading all videos
    /// into memory at once. It fetches videos page by page and processes them immediately.
    async fn run_batch_download(
        &self,
        url: &str,
        cli: &Cli,
        platform: &dyn Platform,
        auth: Option<&Auth>,
    ) -> Result<()> {
        let mut all_videos = Vec::new();
        let mut continuation: Option<String> = None;
        let mut page_num = 1;
        let mut _batch_type: Option<BatchType> = None;
        let mut _total_count: Option<usize> = None;

        // Streaming pagination loop
        loop {
            let batch_result = if let Some(ref cont) = continuation {
                tracing::debug!("Fetching page {} with continuation: {}", page_num, cont);
                platform
                    .parse_batch_page(url, Some(cont), auth)
                    .await?
            } else {
                tracing::debug!("Fetching first page");
                platform.parse_batch(url, auth).await?
            };

            let video_count = batch_result.videos.len();

            // Display batch type on first page
            if page_num == 1 {
                _batch_type = batch_result.batch_type;
                _total_count = batch_result.total_count;

                if let Some(bt) = _batch_type {
                    let type_name = match bt {
                        BatchType::Favorites => "收藏夹",
                        BatchType::UserVideos => "UP主空间",
                        BatchType::Series => "系列",
                        BatchType::Season => "番剧",
                        BatchType::Collection => "合集",
                        BatchType::Playlist => "播放列表",
                        BatchType::Custom => "批量",
                    };
                    println!("📦 {}下载", type_name);
                }

                if let Some(total) = _total_count {
                    println!("📊 总计: {} 个视频", total);
                }
            }

            // Display pagination info
            if let Some(ref page_info) = batch_result.page_info {
                if let Some(total_pages) = page_info.total_pages {
                    println!(
                        "📄 正在获取第{}/{}页 ({} 个视频)",
                        page_info.current_page, total_pages, video_count
                    );
                } else {
                    println!("📄 正在获取第{}页 ({} 个视频)", page_info.current_page, video_count);
                }
            } else if page_num > 1 {
                println!("📄 正在获取第{}页 ({} 个视频)", page_num, video_count);
            }

            all_videos.extend(batch_result.videos);

            if !batch_result.has_more {
                break;
            }

            continuation = batch_result.continuation;
            page_num += 1;

            // Safety check: prevent infinite loops
            if page_num > 1000 {
                tracing::warn!("Reached maximum page limit (1000), stopping");
                break;
            }
        }

        if all_videos.is_empty() {
            return Err(DownloaderError::Parse(
                "No videos found in batch".to_string(),
            ));
        }

        println!("\n✓ 共找到 {} 个视频", all_videos.len());

        // Check batch limit
        if let Some(limit) = cli.batch_limit {
            if all_videos.len() > limit {
                return Err(DownloaderError::BatchLimitExceeded {
                    requested: all_videos.len(),
                    max: limit,
                });
            }
        }

        if cli.info_only {
            for (idx, video) in all_videos.iter().enumerate() {
                if all_videos.len() > 1 {
                    println!("\n[{}/{}]", idx + 1, all_videos.len());
                }
                self.display_video_info(video);
            }
            return Ok(());
        }

        // Build stream preferences
        let preferences = StreamPreferences {
            quality_priority: cli.parse_quality_priority(),
            codec_priority: cli.parse_codec_priority(),
        };

        // Download each video
        for (idx, video_info) in all_videos.iter().enumerate() {
            if all_videos.len() > 1 {
                println!(
                    "\n[{}/{}] Processing: {}",
                    idx + 1,
                    all_videos.len(),
                    video_info.title
                );
            }

            let pages_to_download = self.select_pages(video_info, cli)?;

            for page in pages_to_download {
                self.process_page(video_info, &page, &preferences, cli, platform, auth)
                    .await?;
            }
        }

        self.progress.finish_all();
        println!("\n✓ All {} video(s) downloaded successfully!", all_videos.len());

        Ok(())
    }

    /// Run single video download
    async fn run_single_download(
        &self,
        url: &str,
        cli: &Cli,
        platform: &dyn Platform,
        auth: Option<&Auth>,
    ) -> Result<()> {
        let video_info = platform.parse_video(url, auth).await?;

        // Display video info
        self.display_video_info(&video_info);

        if cli.info_only {
            return Ok(());
        }

        // Determine which pages to download
        let pages_to_download = self.select_pages(&video_info, cli)?;

        tracing::info!("Will download {} page(s)", pages_to_download.len());

        // Build stream preferences
        let preferences = StreamPreferences {
            quality_priority: cli.parse_quality_priority(),
            codec_priority: cli.parse_codec_priority(),
        };

        // Download each page
        for page in pages_to_download {
            self.process_page(&video_info, &page, &preferences, cli, platform, auth)
                .await?;
        }

        self.progress.finish_all();
        println!("\n✓ All downloads completed successfully!");

        Ok(())
    }

    /// Build authentication from various sources
    ///
    /// Priority: override_auth (from login) > CLI parameters > auth.toml > config.toml
    fn build_auth(&self, cli: &Cli) -> Option<Auth> {
        // If we have override auth from login, use it directly
        if self.override_auth.is_some() {
            return self.override_auth.clone();
        }

        // Try to load from auth.toml if config file is specified
        let auth_from_file = if let Some(ref config_path) = cli.config_file {
            use crate::auth::storage::CredentialStorage;
            CredentialStorage::load_from_config(config_path)
                .ok()
                .flatten()
                .map(|creds| CredentialStorage::to_auth(&creds))
        } else {
            None
        };

        // Build final auth with priority
        let cookie = cli
            .cookie
            .clone()
            .or_else(|| {
                auth_from_file
                    .as_ref()
                    .and_then(|a| a.cookie.clone())
            })
            .or_else(|| self.config.auth.as_ref().and_then(|a| a.cookie.clone()));

        let access_token = cli
            .access_token
            .clone()
            .or_else(|| {
                auth_from_file
                    .as_ref()
                    .and_then(|a| a.access_token.clone())
            })
            .or_else(|| {
                self.config
                    .auth
                    .as_ref()
                    .and_then(|a| a.access_token.clone())
            });

        if cookie.is_some() || access_token.is_some() {
            Some(Auth {
                cookie,
                access_token,
                extra: std::collections::HashMap::new(),
            })
        } else {
            None
        }
    }

    /// Display video information
    fn display_video_info(&self, video_info: &VideoInfo) {
        println!("\n📹 Video Information:");
        println!("  Title: {}", video_info.title);
        println!("  Uploader: {}", video_info.uploader);
        println!("  Pages: {}", video_info.pages.len());
        if !video_info.description.is_empty() {
            // Safely truncate string considering UTF-8 character boundaries
            let desc = if video_info.description.chars().count() > 100 {
                let truncated: String = video_info.description.chars().take(100).collect();
                format!("{}...", truncated)
            } else {
                video_info.description.clone()
            };
            println!("  Description: {}", desc);
        }
        println!();
    }

    /// Select pages to download based on CLI arguments
    fn select_pages(&self, video_info: &VideoInfo, cli: &Cli) -> Result<Vec<Page>> {
        if let Some(page_numbers) = cli.parse_pages() {
            // Filter pages by user selection
            let mut selected = Vec::new();
            for num in page_numbers {
                if let Some(page) = video_info.pages.iter().find(|p| p.number == num) {
                    selected.push(page.clone());
                } else {
                    tracing::warn!("Page {} not found, skipping", num);
                }
            }

            if selected.is_empty() {
                return Err(DownloaderError::Parse(
                    "No valid pages selected".to_string(),
                ));
            }

            Ok(selected)
        } else {
            // Download all pages
            Ok(video_info.pages.clone())
        }
    }

    /// Interactive stream selection
    fn interactive_select_streams(&self, streams: &[Stream]) -> Result<(Stream, Stream)> {
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

        // Select video stream
        println!("\n🎬 Select video quality:");
        let video_options: Vec<String> = video_streams
            .iter()
            .map(|s| format!("{} {} - {}kbps", s.quality, s.codec, s.bandwidth / 1000))
            .collect();

        let video_selection = Select::new()
            .with_prompt("Video quality")
            .items(&video_options)
            .default(0)
            .interact()
            .map_err(|e| DownloaderError::Parse(format!("Selection failed: {}", e)))?;

        let selected_video = video_streams[video_selection].clone();

        // Select audio stream
        println!("\n🔊 Select audio quality:");
        let audio_options: Vec<String> = audio_streams
            .iter()
            .map(|s| format!("{} - {}kbps", s.codec, s.bandwidth / 1000))
            .collect();

        let audio_selection = Select::new()
            .with_prompt("Audio quality")
            .items(&audio_options)
            .default(0)
            .interact()
            .map_err(|e| DownloaderError::Parse(format!("Selection failed: {}", e)))?;

        let selected_audio = audio_streams[audio_selection].clone();

        Ok((selected_video, selected_audio))
    }

    /// Process a single page (download and mux)
    ///
    /// This method:
    /// 1. Creates a StreamContext with platform-specific parameters
    /// 2. Gets available streams using the Platform trait
    /// 3. Checks platform feature support before downloading subtitles/danmaku
    /// 4. Downloads video, audio, subtitles, cover, and danmaku
    /// 5. Muxes everything together
    #[allow(clippy::too_many_arguments)]
    async fn process_page(
        &self,
        video_info: &VideoInfo,
        page: &Page,
        preferences: &StreamPreferences,
        cli: &Cli,
        platform: &dyn Platform,
        auth: Option<&Auth>,
    ) -> Result<()> {
        println!("\n📥 Downloading: P{} - {}", page.number, page.title);

        // Create StreamContext using the convenient method
        let context = page.to_stream_context(&video_info.aid.to_string());

        // Get chapters early (before downloading) - Bilibili specific
        let chapters = if platform.supports_feature(PlatformFeature::Chapters) {
            // For Bilibili, we can use the parser directly
            if let Some(cid) = context.get_str("cid") {
                match parser::fetch_chapters(&self.http_client, &video_info.aid.to_string(), cid)
                    .await
                {
                    Ok(chapters) => {
                        if !chapters.is_empty() {
                            tracing::debug!("Found {} chapter(s)", chapters.len());
                        }
                        chapters
                    }
                    Err(e) => {
                        tracing::debug!("Failed to fetch chapters: {}", e);
                        Vec::new()
                    }
                }
            } else {
                Vec::new()
            }
        } else {
            Vec::new()
        };

        // Get streams using Platform trait
        let streams = platform.get_streams(&context, auth).await?;

        if streams.is_empty() {
            return Err(DownloaderError::DownloadFailed(
                "No streams available".to_string(),
            ));
        }

        // Select best streams (interactive or automatic)
        let (video_stream, audio_stream) = if cli.interactive {
            self.interactive_select_streams(&streams)?
        } else {
            select_best_streams(&streams, preferences)?
        };

        // Create temp directory
        let temp_dir = file::create_temp_dir(&format!("{}_{}", video_info.id, page.cid)).await?;

        // Create a downloader with auth for this download session
        let downloader_with_auth = if auth.is_some() {
            let mut new_downloader =
                Downloader::new(self.http_client.clone(), self.downloader.thread_count);
            new_downloader = new_downloader
                .with_method(self.downloader.method)
                .with_aria2c_path(self.downloader.aria2c_path.clone())
                .with_auth(auth.cloned());
            if let Some(ref args) = self.downloader.aria2c_args {
                new_downloader = new_downloader.with_aria2c_args(args.clone());
            }
            Arc::new(new_downloader)
        } else {
            self.downloader.clone()
        };

        // Download video
        let video_path = temp_dir.join("video.m4s");
        let video_pb = self.progress.create_bar("Video", 0);
        downloader_with_auth
            .download(&video_stream.url, &video_path, Some(video_pb.clone()))
            .await?;
        self.progress.finish("Video", "✓ Video downloaded");

        // Download audio
        let audio_path = temp_dir.join("audio.m4s");
        let audio_pb = self.progress.create_bar("Audio", 0);
        downloader_with_auth
            .download(&audio_stream.url, &audio_path, Some(audio_pb.clone()))
            .await?;
        self.progress.finish("Audio", "✓ Audio downloaded");

        // Download subtitles (check platform support)
        let mut subtitle_paths = Vec::new();
        if !cli.skip_subtitle && platform.supports_feature(PlatformFeature::Subtitles) {
            if let Ok(subtitles) = platform.get_subtitles(&context).await {
                for (i, subtitle) in subtitles.iter().enumerate() {
                    let subtitle_path = temp_dir.join(format!("subtitle_{}.srt", i));
                    if subtitle::download_and_convert_subtitle(
                        &self.http_client,
                        subtitle,
                        &subtitle_path,
                    )
                    .await
                    .is_ok()
                    {
                        subtitle_paths.push(subtitle_path);
                        println!("  ✓ Subtitle downloaded: {}", subtitle.language);
                    }
                }
            }
        }

        // Download danmaku (check platform support)
        let danmaku_temp_path =
            if cli.download_danmaku && platform.supports_feature(PlatformFeature::Danmaku) {
                if let Some(cid) = context.get_str("cid") {
                    let danmaku_format = cli.get_danmaku_format();
                    let danmaku_ext = match danmaku_format {
                        danmaku::DanmakuFormat::Xml => "xml",
                        danmaku::DanmakuFormat::Ass => "ass",
                    };
                    let danmaku_path = temp_dir.join(format!("danmaku.{}", danmaku_ext));

                    match danmaku::download_danmaku(
                        &self.http_client,
                        cid,
                        &danmaku_path,
                        danmaku_format,
                    )
                    .await
                    {
                        Ok(()) => {
                            println!("  ✓ Danmaku downloaded");
                            Some(danmaku_path)
                        }
                        Err(e) => {
                            tracing::warn!("Failed to download danmaku: {}", e);
                            None
                        }
                    }
                } else {
                    None
                }
            } else {
                None
            };

        // Download cover
        let _cover_path = if !cli.skip_cover {
            let cover_url = platform.get_cover(video_info);
            let cover_path = temp_dir.join("cover.jpg");
            if self
                .downloader
                .download(&cover_url, &cover_path, None)
                .await
                .is_ok()
            {
                println!("  ✓ Cover downloaded");
                Some(cover_path)
            } else {
                None
            }
        } else {
            None
        };

        // Determine output path
        let output_path = if let Some(ref output) = cli.output {
            let parsed = file::parse_template(
                output,
                video_info,
                Some(page),
                &video_stream.quality,
                &video_stream.codec,
            );
            let path = PathBuf::from(&parsed);

            // If the path is a directory or doesn't have an extension, add a filename
            if path.is_dir() || path.extension().is_none() {
                let filename = if video_info.pages.len() > 1 {
                    format!(
                        "P{:02}_{}.mp4",
                        page.number,
                        file::sanitize_filename(&page.title)
                    )
                } else {
                    format!("{}.mp4", file::sanitize_filename(&video_info.title))
                };
                path.join(filename)
            } else {
                path
            }
        } else {
            file::get_default_output_path(video_info, Some(page))
        };

        // Create output directory
        if let Some(parent) = output_path.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }

        // Mux or copy files
        if cli.skip_mux {
            // Just copy the files
            let video_out = output_path.with_extension("video.m4s");
            let audio_out = output_path.with_extension("audio.m4s");
            tokio::fs::copy(&video_path, &video_out).await?;
            tokio::fs::copy(&audio_path, &audio_out).await?;
            println!("  ✓ Files saved (muxing skipped)");
        } else {
            // Detect Dolby Vision (quality_id 126)
            let is_dolby_vision = video_stream.quality_id == 126;

            if is_dolby_vision {
                tracing::info!("检测到杜比视界清晰度");
            }

            // Mux video and audio with chapters
            println!("  🔄 Muxing...");
            self.muxer
                .mux_with_options(
                    &video_path,
                    &audio_path,
                    &output_path,
                    &subtitle_paths,
                    &chapters,
                    is_dolby_vision,
                )
                .await?;
            println!("  ✓ Muxed to: {}", output_path.display());
        }

        // Copy danmaku file to output directory (same name as video, different extension)
        if let Some(danmaku_temp_path) = danmaku_temp_path {
            if danmaku_temp_path.exists() {
                let danmaku_ext = danmaku_temp_path
                    .extension()
                    .and_then(|e| e.to_str())
                    .unwrap_or("xml");
                let danmaku_output_path = output_path.with_extension(danmaku_ext);

                tokio::fs::copy(&danmaku_temp_path, &danmaku_output_path).await?;
                println!("  ✓ Danmaku saved to: {}", danmaku_output_path.display());
            }
        }

        // Cleanup temp directory
        file::cleanup_temp_dir(&temp_dir).await?;

        Ok(())
    }
}
