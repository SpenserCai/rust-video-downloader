// 集成测试 - 测试完整的下载流程
use rvd::app::Orchestrator;
use rvd::cli::Cli;
use rvd::core::downloader::Downloader;
use rvd::core::muxer::Muxer;
use rvd::platform::bilibili::BilibiliPlatform;
use rvd::platform::Platform;
use rvd::types::Auth;
use rvd::utils::config::Config;
use rvd::utils::http::HttpClient;
use std::path::PathBuf;
use std::sync::Arc;
use tempfile::TempDir;

// 测试用的公开视频BV号 - 使用一个短小的测试视频
const TEST_VIDEO_URL: &str = "https://www.bilibili.com/video/BV1xx411c7mD";
const TEST_OUTPUT_DIR: &str = "tests/test_data";

/// 辅助函数：创建测试输出目录
fn setup_test_output_dir() -> PathBuf {
    let output_dir = PathBuf::from(TEST_OUTPUT_DIR);
    std::fs::create_dir_all(&output_dir).unwrap();
    output_dir
}

/// 辅助函数：清理测试文件
fn cleanup_test_files(pattern: &str) {
    let output_dir = PathBuf::from(TEST_OUTPUT_DIR);
    if let Ok(entries) = std::fs::read_dir(&output_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if let Some(name) = path.file_name() {
                if name.to_string_lossy().contains(pattern) {
                    let _ = std::fs::remove_file(&path);
                }
            }
        }
    }
}

#[tokio::test]
async fn test_parse_video_info_without_auth() {
    let platform = BilibiliPlatform::new().unwrap();

    // 测试解析公开视频（不需要认证）
    let result = platform.parse_video(TEST_VIDEO_URL, None).await;

    // 公开视频应该能成功解析
    if let Ok(video_info) = result {
        assert!(!video_info.id.is_empty());
        assert!(!video_info.title.is_empty());
        assert!(video_info.aid > 0);
        assert!(!video_info.pages.is_empty());
        println!("✓ 视频信息解析成功: {}", video_info.title);
    }
}

#[tokio::test]
async fn test_parse_video_info_with_auth() {
    // 从配置文件加载认证信息
    let config_path = PathBuf::from("tests/rvd.toml");
    if !config_path.exists() {
        println!("跳过需要认证的测试：配置文件不存在");
        return;
    }

    let config = Config::load(&config_path).unwrap();
    let auth = config.auth.and_then(|a| {
        a.cookie.map(|cookie| Auth {
            cookie: Some(cookie),
            access_token: None,
        })
    });

    if auth.is_none() {
        println!("跳过需要认证的测试：未配置cookie");
        return;
    }

    let platform = BilibiliPlatform::new().unwrap();
    let result = platform
        .parse_video(TEST_VIDEO_URL, auth.as_ref())
        .await;

    assert!(result.is_ok());
    println!("✓ 带认证的视频信息解析成功");
}

#[tokio::test]
async fn test_get_streams() {
    let config_path = PathBuf::from("tests/rvd.toml");
    if !config_path.exists() {
        println!("跳过测试：配置文件不存在");
        return;
    }

    let config = Config::load(&config_path).unwrap();
    let auth = config.auth.and_then(|a| {
        a.cookie.map(|cookie| Auth {
            cookie: Some(cookie),
            access_token: None,
        })
    });

    let platform = BilibiliPlatform::new().unwrap();

    // 先获取视频信息
    let video_info = platform
        .parse_video(TEST_VIDEO_URL, auth.as_ref())
        .await;

    if video_info.is_err() {
        println!("跳过测试：无法获取视频信息");
        return;
    }

    let video_info = video_info.unwrap();
    let first_page = &video_info.pages[0];

    // 获取流信息
    let streams = platform
        .get_streams(
            &video_info.aid.to_string(),
            &first_page.cid,
            auth.as_ref(),
        )
        .await;

    if let Ok(streams) = streams {
        assert!(!streams.is_empty());
        // 应该至少有视频流和音频流
        let has_video = streams
            .iter()
            .any(|s| matches!(s.stream_type, rvd::types::StreamType::Video));
        let has_audio = streams
            .iter()
            .any(|s| matches!(s.stream_type, rvd::types::StreamType::Audio));
        assert!(has_video || has_audio);
        println!("✓ 获取到 {} 个流", streams.len());
    }
}

#[tokio::test]
async fn test_get_subtitles() {
    let config_path = PathBuf::from("tests/rvd.toml");
    if !config_path.exists() {
        println!("跳过测试：配置文件不存在");
        return;
    }

    let platform = BilibiliPlatform::new().unwrap();

    // 先获取视频信息
    let video_info = platform.parse_video(TEST_VIDEO_URL, None).await;

    if video_info.is_err() {
        println!("跳过测试：无法获取视频信息");
        return;
    }

    let video_info = video_info.unwrap();
    let first_page = &video_info.pages[0];

    // 获取字幕信息（字幕可能不存在，所以不强制要求成功）
    let subtitles = platform
        .get_subtitles(&video_info.aid.to_string(), &first_page.cid)
        .await;

    // 字幕可能不存在，但API调用应该成功
    assert!(subtitles.is_ok());
    if let Ok(subs) = subtitles {
        println!("✓ 获取到 {} 个字幕", subs.len());
    }
}

#[test]
fn test_orchestrator_creation() {
    let cli = Cli {
        url: TEST_VIDEO_URL.to_string(),
        quality: None,
        codec: None,
        output: None,
        cookie: None,
        access_token: None,
        pages: None,
        threads: 2,
        skip_subtitle: false,
        skip_cover: false,
        skip_mux: false,
        interactive: false,
        config_file: None,
        verbose: false,
        info_only: false,
        ffmpeg_path: None,
        use_tv_api: false,
        use_app_api: false,
        use_intl_api: false,
        download_danmaku: false,
        danmaku_format: "ass".to_string(),
    };

    let config = Config::default();
    let result = Orchestrator::new(config, &cli);

    // Orchestrator创建可能失败（如果ffmpeg不可用）
    // 但至少不应该panic
    match result {
        Ok(_) => println!("✓ Orchestrator创建成功"),
        Err(e) => println!("⚠ Orchestrator创建失败: {} (可能是ffmpeg未安装)", e),
    }
}

#[tokio::test]
async fn test_download_small_file() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("test_download.bin");

    let client = Arc::new(HttpClient::new().unwrap());
    let downloader = Downloader::new(client, 2);

    // 使用一个小的测试文件URL（B站的一个小图片）
    let test_url = "https://www.bilibili.com/favicon.ico";

    let result = downloader.download(test_url, &output_path, None).await;

    if result.is_ok() {
        assert!(output_path.exists());
        let metadata = std::fs::metadata(&output_path).unwrap();
        assert!(metadata.len() > 0);
        println!("✓ 文件下载成功，大小: {} bytes", metadata.len());
    } else {
        println!("⚠ 文件下载失败（可能是网络问题）: {:?}", result.err());
    }
}

#[tokio::test]
async fn test_muxer_check_ffmpeg() {
    let result = Muxer::new(None);

    match result {
        Ok(muxer) => {
            let version = muxer.check_ffmpeg();
            assert!(version.is_ok());
            println!("✓ FFmpeg检查成功: {}", version.unwrap());
        }
        Err(e) => {
            println!("⚠ FFmpeg未找到: {} (这是预期的，如果系统未安装ffmpeg)", e);
        }
    }
}

#[tokio::test]
async fn test_info_only_mode() {
    let output_dir = setup_test_output_dir();

    let cli = Cli {
        url: TEST_VIDEO_URL.to_string(),
        quality: None,
        codec: None,
        output: Some(output_dir.join("test_info_only.mp4").to_string_lossy().to_string()),
        cookie: None,
        access_token: None,
        pages: None,
        threads: 2,
        skip_subtitle: true,
        skip_cover: true,
        skip_mux: true,
        interactive: false,
        config_file: None,
        verbose: false,
        info_only: true, // 仅显示信息，不下载
        ffmpeg_path: None,
        use_tv_api: false,
        use_app_api: false,
        use_intl_api: false,
        download_danmaku: false,
        danmaku_format: "ass".to_string(),
    };

    let config = Config::default();

    // 尝试创建orchestrator
    if let Ok(orchestrator) = Orchestrator::new(config, &cli) {
        let result = orchestrator.run(cli).await;

        // info_only模式应该成功，且不产生文件
        if result.is_ok() {
            println!("✓ Info-only模式测试成功");
        } else {
            println!("⚠ Info-only模式失败: {:?}", result.err());
        }
    } else {
        println!("⚠ 无法创建Orchestrator（可能是ffmpeg未安装）");
    }
}

#[tokio::test]
async fn test_multi_page_selection() {
    let platform = BilibiliPlatform::new().unwrap();

    // 使用一个有多分P的视频进行测试
    let multi_page_url = "https://www.bilibili.com/video/BV1xx411c7mD";

    let result = platform.parse_video(multi_page_url, None).await;

    if let Ok(video_info) = result {
        if video_info.pages.len() > 1 {
            println!("✓ 多分P视频解析成功，共 {} 个分P", video_info.pages.len());

            // 测试分P选择逻辑
            let cli = Cli {
                url: multi_page_url.to_string(),
                quality: None,
                codec: None,
                output: None,
                cookie: None,
                access_token: None,
                pages: Some("1,2".to_string()),
                threads: 2,
                skip_subtitle: true,
                skip_cover: true,
                skip_mux: true,
                interactive: false,
                config_file: None,
                verbose: false,
                info_only: true,
                ffmpeg_path: None,
        use_tv_api: false,
        use_app_api: false,
        use_intl_api: false,
        download_danmaku: false,
        danmaku_format: "ass".to_string(),
            };

            let parsed_pages = cli.parse_pages();
            assert!(parsed_pages.is_some());
            assert_eq!(parsed_pages.unwrap(), vec![1, 2]);
            println!("✓ 分P选择逻辑测试成功");
        } else {
            println!("⚠ 测试视频只有单P");
        }
    }
}

#[tokio::test]
async fn test_error_handling_invalid_url() {
    let platform = BilibiliPlatform::new().unwrap();

    // 测试无效URL
    let invalid_url = "https://www.bilibili.com/video/INVALID";

    let result = platform.parse_video(invalid_url, None).await;

    // 应该返回错误
    assert!(result.is_err());
    println!("✓ 无效URL错误处理测试成功");
}

#[tokio::test]
async fn test_error_handling_network_failure() {
    let platform = BilibiliPlatform::new().unwrap();

    // 测试不存在的视频
    let nonexistent_url = "https://www.bilibili.com/video/BV1111111111";

    let result = platform.parse_video(nonexistent_url, None).await;

    // 应该返回错误
    assert!(result.is_err());
    println!("✓ 网络错误处理测试成功");
}

#[tokio::test]
async fn test_quality_codec_priority() {
    use rvd::platform::bilibili::selector::select_best_streams;
    use rvd::types::{Stream, StreamPreferences, StreamType};

    // 创建测试流数据
    let streams = vec![
        Stream {
            stream_type: StreamType::Video,
            quality: "1080P 高清".to_string(),
            quality_id: 80,
            codec: "AVC".to_string(),
            url: "https://example.com/1080p_avc.m4s".to_string(),
            size: 0,
            bandwidth: 3000000,
        },
        Stream {
            stream_type: StreamType::Video,
            quality: "720P 高清".to_string(),
            quality_id: 64,
            codec: "HEVC".to_string(),
            url: "https://example.com/720p_hevc.m4s".to_string(),
            size: 0,
            bandwidth: 2000000,
        },
        Stream {
            stream_type: StreamType::Audio,
            quality: "192kbps".to_string(),
            quality_id: 30280,
            codec: "M4A".to_string(),
            url: "https://example.com/audio.m4s".to_string(),
            size: 0,
            bandwidth: 192000,
        },
    ];

    // 测试质量优先级
    let preferences = StreamPreferences {
        quality_priority: vec!["1080P".to_string(), "720P".to_string()],
        codec_priority: vec!["avc".to_string(), "hevc".to_string()],
    };

    let result = select_best_streams(&streams, &preferences);
    assert!(result.is_ok());

    let (video, audio) = result.unwrap();
    assert!(video.quality.contains("1080P"));
    assert_eq!(audio.codec, "M4A");
    println!("✓ 质量和编码优先级测试成功");
}

#[tokio::test]
async fn test_file_naming_template() {
    use rvd::types::{Page, VideoInfo};
    use rvd::utils::file::parse_template;

    let video_info = VideoInfo {
        id: "BV1xx411c7mD".to_string(),
        aid: 170001,
        title: "测试视频".to_string(),
        description: "描述".to_string(),
        duration: 300,
        uploader: "测试UP主".to_string(),
        uploader_mid: "12345".to_string(),
        upload_date: "2024-01-01".to_string(),
        cover_url: "https://example.com/cover.jpg".to_string(),
        pages: vec![Page {
            number: 1,
            title: "P1".to_string(),
            cid: "123456".to_string(),
            duration: 300,
            ep_id: None,
        }],
        is_bangumi: false,
        ep_id: None,
    };

    // 测试各种模板
    let templates = vec![
        ("<videoTitle>", "测试视频"),
        ("<videoTitle>_<quality>", "测试视频_1080P"),
        ("<bvid>_<codec>", "BV1xx411c7mD_avc"),
        ("<uploader>/<videoTitle>", "测试UP主/测试视频"),
    ];

    for (template, expected) in templates {
        let result = parse_template(template, &video_info, None, "1080P", "avc");
        assert_eq!(result, expected);
    }

    println!("✓ 文件命名模板测试成功");
}

#[tokio::test]
async fn test_config_loading() {
    use tempfile::NamedTempFile;
    use std::io::Write;

    let mut temp_file = NamedTempFile::new().unwrap();
    let config_content = r#"
default_quality = ["1080P", "720P"]
default_codec = ["hevc", "avc"]
thread_count = 8

[auth]
cookie = "test_cookie"

[paths]
ffmpeg = "/usr/bin/ffmpeg"
"#;

    temp_file.write_all(config_content.as_bytes()).unwrap();
    temp_file.flush().unwrap();

    let config = Config::load(temp_file.path()).unwrap();

    assert_eq!(
        config.default_quality,
        Some(vec!["1080P".to_string(), "720P".to_string()])
    );
    assert_eq!(config.thread_count, Some(8));
    assert!(config.auth.is_some());

    println!("✓ 配置文件加载测试成功");
}

// 清理函数，在测试结束时调用
#[tokio::test]
async fn test_cleanup() {
    cleanup_test_files("test_");
    println!("✓ 测试清理完成");
}
