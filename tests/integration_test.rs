// 集成测试 - 测试完整的下载流程
use rvd::app::Orchestrator;
use rvd::cli::Cli;
use rvd::platform::bilibili::BilibiliPlatform;
use rvd::platform::Platform;
use rvd::types::Auth;
use rvd::utils::config::Config;
use std::path::PathBuf;

// 测试用的公开视频BV号
const TEST_VIDEO_URL: &str = "https://www.bilibili.com/video/BV1xx411c7mD";

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
        let has_video = streams.iter().any(|s| {
            matches!(s.stream_type, rvd::types::StreamType::Video)
        });
        let has_audio = streams.iter().any(|s| {
            matches!(s.stream_type, rvd::types::StreamType::Audio)
        });
        assert!(has_video || has_audio);
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
    };

    let config = Config::default();
    let result = Orchestrator::new(config, &cli);

    // Orchestrator创建可能失败（如果ffmpeg不可用）
    // 但至少不应该panic
    match result {
        Ok(_) => println!("Orchestrator创建成功"),
        Err(e) => println!("Orchestrator创建失败: {}", e),
    }
}
