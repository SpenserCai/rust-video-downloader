use crate::cli::Cli;
use crate::core::danmaku;
use crate::core::downloader::Downloader;
use crate::core::muxer::Muxer;
use crate::core::progress::ProgressTracker;
use crate::core::subtitle;
use crate::error::{DownloaderError, Result};
use crate::platform::bilibili::parser;
use crate::platform::bilibili::selector::select_best_streams;
use crate::platform::bilibili::BilibiliPlatform;
use crate::platform::Platform;
use crate::types::{Auth, Page, Stream, StreamPreferences, StreamType, VideoInfo};
use crate::utils::config::Config;
use crate::utils::file;
use crate::utils::http::HttpClient;
use dialoguer::Select;
use std::path::PathBuf;
use std::sync::Arc;

pub struct Orchestrator {
    platforms: Vec<Box<dyn Platform>>,
    downloader: Arc<Downloader>,
    muxer: Arc<Muxer>,
    progress: Arc<ProgressTracker>,
    config: Config,
    http_client: Arc<HttpClient>,
}

impl Orchestrator {
    pub fn new(config: Config, cli: &Cli) -> Result<Self> {
        let http_client = Arc::new(HttpClient::new()?);
        let downloader = Arc::new(Downloader::new(http_client.clone(), cli.threads));
        let muxer =
            Arc::new(Muxer::new(cli.ffmpeg_path.clone().or_else(|| {
                config.paths.as_ref().and_then(|p| p.ffmpeg.clone())
            }))?);
        let progress = Arc::new(ProgressTracker::new());

        // 根据CLI参数选择API模式
        let api_mode = cli.get_api_mode();
        let platforms: Vec<Box<dyn Platform>> = vec![
            Box::new(BilibiliPlatform::with_api_mode(api_mode)?)
        ];

        Ok(Self {
            platforms,
            downloader,
            muxer,
            progress,
            config,
            http_client,
        })
    }

    fn select_platform(&self, url: &str) -> Result<&dyn Platform> {
        for platform in &self.platforms {
            if platform.can_handle(url) {
                return Ok(platform.as_ref());
            }
        }

        Err(DownloaderError::UnsupportedPlatform(url.to_string()))
    }

    fn is_batch_url(&self, url: &str) -> bool {
        // Check if URL is a batch download type (favorites, space, medialist, series)
        url.contains("favlist") 
            || (url.contains("space.bilibili.com") && !url.contains("/video/"))
            || url.contains("medialist")
            || url.contains("seriesdetail")
    }

    pub async fn run(&self, cli: Cli) -> Result<()> {
        tracing::info!("Starting download for URL: {}", cli.url);

        // Select platform
        let platform = self.select_platform(&cli.url)?;
        tracing::info!("Using platform: {}", platform.name());

        // Build auth
        let auth = self.build_auth(&cli);

        // Check if this is a batch download URL (for bilibili)
        let is_batch = self.is_batch_url(&cli.url);
        
        if is_batch {
            // Handle batch download
            if let Some(bilibili) = platform.as_any().downcast_ref::<BilibiliPlatform>() {
                let videos = bilibili.parse_video_batch(&cli.url, auth.as_ref()).await?;
                
                if videos.is_empty() {
                    return Err(DownloaderError::Parse("No videos found in batch".to_string()));
                }
                
                println!("\n📦 Batch download: {} video(s) found", videos.len());
                
                if cli.info_only {
                    for (idx, video) in videos.iter().enumerate() {
                        println!("\n[{}/{}]", idx + 1, videos.len());
                        self.display_video_info(video);
                    }
                    return Ok(());
                }
                
                // Build stream preferences
                let preferences = StreamPreferences {
                    quality_priority: cli.parse_quality_priority(),
                    codec_priority: cli.parse_codec_priority(),
                };
                
                // Download each video in the batch
                for (idx, video_info) in videos.iter().enumerate() {
                    println!("\n[{}/{}] Processing: {}", idx + 1, videos.len(), video_info.title);
                    
                    // Determine which pages to download
                    let pages_to_download = self.select_pages(video_info, &cli)?;
                    
                    // Download each page
                    for page in pages_to_download {
                        self.process_page(
                            video_info,
                            &page,
                            &preferences,
                            &cli,
                            platform,
                            auth.as_ref(),
                        )
                        .await?;
                    }
                }
                
                self.progress.finish_all();
                println!("\n✓ All {} video(s) downloaded successfully!", videos.len());
                
                return Ok(());
            }
        }

        // Single video download (original logic)
        let video_info = platform.parse_video(&cli.url, auth.as_ref()).await?;

        // Display video info
        self.display_video_info(&video_info);

        if cli.info_only {
            return Ok(());
        }

        // Determine which pages to download
        let pages_to_download = self.select_pages(&video_info, &cli)?;

        tracing::info!("Will download {} page(s)", pages_to_download.len());

        // Build stream preferences
        let preferences = StreamPreferences {
            quality_priority: cli.parse_quality_priority(),
            codec_priority: cli.parse_codec_priority(),
        };

        // Download each page
        for page in pages_to_download {
            self.process_page(
                &video_info,
                &page,
                &preferences,
                &cli,
                platform,
                auth.as_ref(),
            )
            .await?;
        }

        self.progress.finish_all();
        println!("\n✓ All downloads completed successfully!");

        Ok(())
    }

    fn build_auth(&self, cli: &Cli) -> Option<Auth> {
        let cookie = cli
            .cookie
            .clone()
            .or_else(|| self.config.auth.as_ref()?.cookie.clone());

        let access_token = cli
            .access_token
            .clone()
            .or_else(|| self.config.auth.as_ref()?.access_token.clone());

        if cookie.is_some() || access_token.is_some() {
            Some(Auth {
                cookie,
                access_token,
            })
        } else {
            None
        }
    }

    fn display_video_info(&self, video_info: &VideoInfo) {
        println!("\n📹 Video Information:");
        println!("  Title: {}", video_info.title);
        println!("  Uploader: {}", video_info.uploader);
        println!("  Pages: {}", video_info.pages.len());
        if !video_info.description.is_empty() {
            // 安全地截断字符串，考虑 UTF-8 字符边界
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

        // Get chapters early (before downloading)
        let chapters = match parser::fetch_chapters(
            &self.http_client,
            &video_info.aid.to_string(),
            &page.cid,
        )
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
        };

        // Get streams (use aid for bilibili API)
        let streams = if video_info.is_bangumi {
            // 番剧需要使用特殊的API
            if let Some(platform_bilibili) = platform.as_any().downcast_ref::<crate::platform::bilibili::BilibiliPlatform>() {
                if let Some(ref ep_id) = page.ep_id {
                    // 使用page的ep_id（每个episode有自己的ep_id）
                    platform_bilibili
                        .get_bangumi_streams(&video_info.aid.to_string(), &page.cid, ep_id, auth)
                        .await?
                } else if let Some(ref ep_id) = video_info.ep_id {
                    // 如果page没有ep_id，使用video_info的ep_id
                    platform_bilibili
                        .get_bangumi_streams(&video_info.aid.to_string(), &page.cid, ep_id, auth)
                        .await?
                } else {
                    // 如果都没有ep_id，尝试使用普通API
                    platform
                        .get_streams(&video_info.aid.to_string(), &page.cid, auth)
                        .await?
                }
            } else {
                platform
                    .get_streams(&video_info.aid.to_string(), &page.cid, auth)
                    .await?
            }
        } else {
            platform
                .get_streams(&video_info.aid.to_string(), &page.cid, auth)
                .await?
        };

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

        // Download video
        let video_path = temp_dir.join("video.m4s");
        let video_pb = self.progress.create_bar("Video", 0);
        self.downloader
            .download(&video_stream.url, &video_path, Some(video_pb.clone()))
            .await?;
        self.progress.finish("Video", "✓ Video downloaded");

        // Download audio
        let audio_path = temp_dir.join("audio.m4s");
        let audio_pb = self.progress.create_bar("Audio", 0);
        self.downloader
            .download(&audio_stream.url, &audio_path, Some(audio_pb.clone()))
            .await?;
        self.progress.finish("Audio", "✓ Audio downloaded");

        // Download subtitles
        let mut subtitle_paths = Vec::new();
        if !cli.skip_subtitle {
            if let Ok(subtitles) = platform
                .get_subtitles(&video_info.aid.to_string(), &page.cid)
                .await
            {
                for (i, subtitle) in subtitles.iter().enumerate() {
                    let subtitle_path = temp_dir.join(format!("subtitle_{}.srt", i));
                    if let Ok(()) = subtitle::download_and_convert_subtitle(
                        &self.http_client,
                        subtitle,
                        &subtitle_path,
                    )
                    .await
                    {
                        subtitle_paths.push(subtitle_path);
                        println!("  ✓ Subtitle downloaded: {}", subtitle.language);
                    }
                }
            }
        }

        // Download danmaku
        let danmaku_temp_path = if cli.download_danmaku {
            let danmaku_format = cli.get_danmaku_format();
            let danmaku_ext = match danmaku_format {
                danmaku::DanmakuFormat::Xml => "xml",
                danmaku::DanmakuFormat::Ass => "ass",
            };
            let danmaku_path = temp_dir.join(format!("danmaku.{}", danmaku_ext));
            
            match danmaku::download_danmaku(
                &self.http_client,
                &page.cid,
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
                    format!("P{:02}_{}.mp4", page.number, file::sanitize_filename(&page.title))
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
            // Mux video and audio with chapters
            println!("  🔄 Muxing...");
            self.muxer
                .mux_with_chapters(&video_path, &audio_path, &output_path, &subtitle_paths, &chapters)
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
