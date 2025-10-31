use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "rvd")]
#[command(author = "RVD Contributors")]
#[command(version = "0.2.5")]
#[command(about = "A modular video downloader written in Rust", long_about = None)]
pub struct Cli {
    /// Video URL to download (supports bilibili BV/av/ep/ss)
    /// Optional when using --login-qrcode or --login-tv
    #[arg(required_unless_present_any = ["login_qrcode", "login_tv"])]
    pub url: Option<String>,

    /// Quality priority (comma-separated, e.g., "1080P,720P,480P")
    #[arg(short = 'q', long)]
    pub quality: Option<String>,

    /// Codec priority (comma-separated, e.g., "hevc,avc,av1")
    #[arg(short = 'c', long)]
    pub codec: Option<String>,

    /// Output file path or template
    #[arg(short = 'o', long)]
    pub output: Option<String>,

    /// Cookie string for authentication
    #[arg(long)]
    pub cookie: Option<String>,

    /// Access token for authentication
    #[arg(long)]
    pub access_token: Option<String>,

    /// Select specific pages (e.g., "1", "1,2,3", "1-5", "ALL")
    #[arg(short = 'p', long)]
    pub pages: Option<String>,

    /// Number of download threads
    #[arg(short = 't', long, default_value = "4")]
    pub threads: usize,

    /// Skip subtitle download
    #[arg(long)]
    pub skip_subtitle: bool,

    /// Skip cover download
    #[arg(long)]
    pub skip_cover: bool,

    /// Skip muxing (keep separate video and audio files)
    #[arg(long)]
    pub skip_mux: bool,

    /// Interactive mode for quality selection
    #[arg(short = 'i', long)]
    pub interactive: bool,

    /// Config file path
    #[arg(long)]
    pub config_file: Option<PathBuf>,

    /// Enable verbose logging
    #[arg(short = 'v', long)]
    pub verbose: bool,

    /// Show video info only (no download)
    #[arg(long)]
    pub info_only: bool,

    /// Path to ffmpeg binary
    #[arg(long)]
    pub ffmpeg_path: Option<PathBuf>,

    /// Use TV API mode (for higher quality streams)
    #[arg(long)]
    pub use_tv_api: bool,

    /// Use APP API mode (for Dolby audio support)
    #[arg(long)]
    pub use_app_api: bool,

    /// Use International API mode
    #[arg(long)]
    pub use_intl_api: bool,

    /// Download danmaku (bullet comments)
    #[arg(long)]
    pub download_danmaku: bool,

    /// Danmaku format (xml or ass)
    #[arg(long, default_value = "ass")]
    pub danmaku_format: String,

    /// Login using QR code (Web mode)
    #[arg(long, conflicts_with = "login_tv")]
    pub login_qrcode: bool,

    /// Login using QR code (TV mode, gets access_token)
    #[arg(long, conflicts_with = "login_qrcode")]
    pub login_tv: bool,
}

impl Cli {
    pub fn parse_quality_priority(&self) -> Vec<String> {
        if let Some(ref q) = self.quality {
            q.split(',').map(|s| s.trim().to_string()).collect()
        } else {
            vec!["1080P".to_string(), "720P".to_string(), "480P".to_string()]
        }
    }

    pub fn parse_codec_priority(&self) -> Vec<String> {
        if let Some(ref c) = self.codec {
            c.split(',').map(|s| s.trim().to_string()).collect()
        } else {
            vec!["avc".to_string(), "hevc".to_string(), "av1".to_string()]
        }
    }

    pub fn parse_pages(&self) -> Option<Vec<usize>> {
        if let Some(ref pages_str) = self.pages {
            if pages_str.to_uppercase() == "ALL" {
                return None; // None means all pages
            }

            let mut pages = Vec::new();

            for part in pages_str.split(',') {
                let part = part.trim();

                if part.contains('-') {
                    // Range: "1-5"
                    let range: Vec<&str> = part.split('-').collect();
                    if range.len() == 2 {
                        if let (Ok(start), Ok(end)) =
                            (range[0].parse::<usize>(), range[1].parse::<usize>())
                        {
                            for i in start..=end {
                                pages.push(i);
                            }
                        }
                    }
                } else {
                    // Single page: "1"
                    if let Ok(page) = part.parse::<usize>() {
                        pages.push(page);
                    }
                }
            }

            if pages.is_empty() {
                None
            } else {
                Some(pages)
            }
        } else {
            None
        }
    }

    pub fn get_api_mode(&self) -> crate::platform::bilibili::ApiMode {
        use crate::platform::bilibili::ApiMode;
        
        if self.use_tv_api {
            ApiMode::TV
        } else if self.use_app_api {
            ApiMode::App
        } else if self.use_intl_api {
            ApiMode::International
        } else {
            ApiMode::Web
        }
    }

    pub fn get_danmaku_format(&self) -> crate::core::danmaku::DanmakuFormat {
        use crate::core::danmaku::DanmakuFormat;
        
        match self.danmaku_format.to_lowercase().as_str() {
            "xml" => DanmakuFormat::Xml,
            "ass" => DanmakuFormat::Ass,
            _ => DanmakuFormat::Ass,
        }
    }

    /// Check if login is requested
    pub fn needs_login(&self) -> bool {
        self.login_qrcode || self.login_tv
    }

    /// Get the API mode for login (if login is requested)
    pub fn get_login_api_mode(&self) -> Option<crate::platform::bilibili::ApiMode> {
        use crate::platform::bilibili::ApiMode;
        
        if self.login_tv {
            Some(ApiMode::TV)
        } else if self.login_qrcode {
            Some(ApiMode::Web)
        } else {
            None
        }
    }
}
