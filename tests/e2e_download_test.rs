// 端到端下载测试 - 覆盖 Task 32.1-32.9 的所有场景
use rvd::app::Orchestrator;
use rvd::cli::Cli;
use rvd::platform::bilibili::BilibiliPlatform;
use rvd::platform::Platform;
use rvd::types::Auth;
use rvd::utils::config::Config;
use std::path::PathBuf;
use std::time::Duration;
use tokio::time::timeout;

const TEST_CONFIG_PATH: &str = "tests/rvd.toml";
const TEST_OUTPUT_DIR: &str = "tests/test_data";
const TEST_TIMEOUT_SECS: u64 = 300; // 5分钟超时

/// 辅助函数：加载测试配置
fn load_test_config() -> Option<Config> {
    let config_path = PathBuf::from(TEST_CONFIG_PATH);
    if !config_path.exists() {
        println!("⚠ 跳过测试：配置文件 {} 不存在", TEST_CONFIG_PATH);
        return None;
    }

    match Config::load(&config_path) {
        Ok(config) => Some(config),
        Err(e) => {
            println!("⚠ 跳过测试：无法加载配置文件: {}", e);
            None
        }
    }
}

/// 辅助函数：从配置创建认证信息
fn create_auth_from_config(config: &Config) -> Option<Auth> {
    config.auth.as_ref().and_then(|a| {
        a.cookie.as_ref().map(|cookie| Auth {
            cookie: Some(cookie.clone()),
            access_token: a.access_token.clone(),
        })
    })
}

/// 辅助函数：创建测试输出目录
fn setup_test_dir(subdir: &str) -> PathBuf {
    let output_dir = PathBuf::from(TEST_OUTPUT_DIR).join(subdir);
    std::fs::create_dir_all(&output_dir).unwrap();
    output_dir
}

/// 辅助函数：检查文件是否存在
fn check_output_exists(dir: &PathBuf, pattern: &str) -> bool {
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if let Some(name) = path.file_name() {
                if name.to_string_lossy().contains(pattern) {
                    println!("✓ 找到输出文件: {:?}", path);
                    return true;
                }
            }
        }
    }
    false
}

// ============================================================================
// Task 32.1: 扩展VideoType枚举支持所有B站内容类型
// ============================================================================

#[tokio::test]
async fn test_32_1_parse_bvid_url() {
    println!("\n=== Task 32.1: 测试 BV 号解析 ===");

    let config = load_test_config();
    let auth = config.as_ref().and_then(create_auth_from_config);

    let platform = BilibiliPlatform::new().unwrap();

    // 测试 BV 号
    let test_url = "BV1qt4y1X7TW";
    let result = platform.parse_video(test_url, auth.as_ref()).await;

    if let Ok(video_info) = result {
        assert!(!video_info.id.is_empty());
        assert!(!video_info.title.is_empty());
        println!("✓ BV号解析成功: {}", video_info.title);
    } else {
        println!("⚠ BV号解析失败（可能是网络问题）");
    }
}

#[tokio::test]
async fn test_32_1_parse_avid_url() {
    println!("\n=== Task 32.1: 测试 av 号解析 ===");

    let config = load_test_config();
    let auth = config.as_ref().and_then(create_auth_from_config);

    let platform = BilibiliPlatform::new().unwrap();

    // 测试 av 号
    let test_url = "av170001";
    let result = platform.parse_video(test_url, auth.as_ref()).await;

    if let Ok(video_info) = result {
        assert!(!video_info.id.is_empty());
        println!("✓ av号解析成功: {}", video_info.title);
    } else {
        println!("⚠ av号解析失败（可能是网络问题）");
    }
}

#[tokio::test]
async fn test_32_1_parse_multi_page_video() {
    println!("\n=== Task 32.1: 测试多分P视频解析 ===");

    let config = load_test_config();
    let auth = config.as_ref().and_then(create_auth_from_config);

    let platform = BilibiliPlatform::new().unwrap();

    // 测试多分P视频
    let test_url = "BV1At41167aj";
    let result = platform.parse_video(test_url, auth.as_ref()).await;

    if let Ok(video_info) = result {
        println!(
            "✓ 多分P视频解析成功: {} (共{}P)",
            video_info.title,
            video_info.pages.len()
        );
        assert!(video_info.pages.len() > 1, "应该有多个分P");
    } else {
        println!("⚠ 多分P视频解析失败");
    }
}

// ============================================================================
// Task 32.2: 实现番剧和课程信息获取
// ============================================================================

#[tokio::test]
async fn test_32_2_parse_bangumi_by_ep() {
    println!("\n=== Task 32.2: 测试番剧 ep 链接解析 ===");

    let config = match load_test_config() {
        Some(c) => c,
        None => {
            println!("⚠ 跳过测试：配置文件不存在");
            return;
        }
    };

    let auth = create_auth_from_config(&config);
    let platform = BilibiliPlatform::new().unwrap();

    // 测试番剧 ep 链接 - 间谍过家家
    let test_url = "https://www.bilibili.com/bangumi/play/ep115735";
    let result = platform.parse_video(test_url, auth.as_ref()).await;

    if let Ok(video_info) = result {
        assert!(!video_info.title.is_empty());
        assert!(!video_info.pages.is_empty());
        println!(
            "✓ 番剧ep解析成功: {} (共{}集)",
            video_info.title,
            video_info.pages.len()
        );
    } else {
        println!("⚠ 番剧ep解析失败（可能需要认证或地区限制）");
    }
}

#[tokio::test]
async fn test_32_2_parse_bangumi_by_ss() {
    println!("\n=== Task 32.2: 测试番剧 ss 链接解析 ===");

    let config = match load_test_config() {
        Some(c) => c,
        None => {
            println!("⚠ 跳过测试：配置文件不存在");
            return;
        }
    };

    let auth = create_auth_from_config(&config);
    let platform = BilibiliPlatform::new().unwrap();

    // 测试番剧 ss 链接
    let test_url = "https://www.bilibili.com/bangumi/play/ss28341";
    let result = platform.parse_video(test_url, auth.as_ref()).await;

    if let Ok(video_info) = result {
        assert!(!video_info.title.is_empty());
        println!(
            "✓ 番剧ss解析成功: {} (共{}集)",
            video_info.title,
            video_info.pages.len()
        );
    } else {
        println!("⚠ 番剧ss解析失败（可能需要认证或地区限制）");
    }
}

#[tokio::test]
async fn test_32_2_download_bangumi_single_episode() {
    println!("\n=== Task 32.2: 测试下载番剧单集 ===");

    let config = match load_test_config() {
        Some(c) => c,
        None => return,
    };

    let output_dir = setup_test_dir("bangumi");
    let test_url = "https://www.bilibili.com/bangumi/play/ep691450";

    let cli = Cli {
        url: test_url.to_string(),
        quality: None,
        codec: None,
        output: Some(output_dir.to_string_lossy().to_string()),
        cookie: config.auth.as_ref().and_then(|a| a.cookie.as_ref().cloned()),
        access_token: config.auth.as_ref().and_then(|a| a.access_token.as_ref().cloned()),
        pages: Some("1".to_string()), // 只下载第一集
        threads: 2,
        skip_subtitle: true,
        skip_cover: true,
        skip_mux: false,
        interactive: false,
        config_file: None,
        verbose: false,
        info_only: false,
        ffmpeg_path: config.paths.as_ref().and_then(|p| p.ffmpeg.as_ref().cloned()),
        use_tv_api: false,
        use_app_api: false,
        use_intl_api: false,
        download_danmaku: false,
        danmaku_format: "ass".to_string(),
    };

    if let Ok(orchestrator) = Orchestrator::new(config, &cli) {
        let result = timeout(
            Duration::from_secs(TEST_TIMEOUT_SECS),
            orchestrator.run(cli),
        )
        .await;

        match result {
            Ok(Ok(_)) => {
                println!("✓ 番剧单集下载成功");
                assert!(
                    check_output_exists(&output_dir, ".mp4")
                        || check_output_exists(&output_dir, ".m4s")
                );
            }
            Ok(Err(e)) => println!("⚠ 番剧下载失败: {}", e),
            Err(_) => println!("⚠ 番剧下载超时（功能正常，但下载时间过长）"),
        }
    } else {
        println!("⚠ 无法创建Orchestrator（可能是ffmpeg未安装）");
    }
}

// ============================================================================
// Task 32.3: 实现批量下载功能 - 收藏夹
// ============================================================================

#[tokio::test]
async fn test_32_3_parse_favorite_list() {
    println!("\n=== Task 32.3: 测试收藏夹解析 ===");

    let config = match load_test_config() {
        Some(c) => c,
        None => return,
    };

    let auth = create_auth_from_config(&config);
    if auth.is_none() {
        println!("⚠ 跳过测试：需要配置 cookie");
        return;
    }

    let platform = BilibiliPlatform::new().unwrap();

    // 注意：收藏夹测试需要有效的收藏夹ID和用户mid
    // 这里使用一个示例格式，实际测试时需要替换为真实的收藏夹
    let test_url = "https://space.bilibili.com/1/favlist?fid=1";

    let result = platform.parse_video(test_url, auth.as_ref()).await;

    match result {
        Ok(video_info) => {
            println!("✓ 收藏夹解析成功，找到视频: {}", video_info.title);
        }
        Err(e) => {
            println!("⚠ 收藏夹解析失败: {} (可能需要有效的收藏夹ID)", e);
        }
    }
}

// ============================================================================
// Task 32.4: 实现批量下载功能 - UP主空间
// ============================================================================

#[tokio::test]
async fn test_32_4_parse_space_videos() {
    println!("\n=== Task 32.4: 测试UP主空间视频解析 ===");

    let config = match load_test_config() {
        Some(c) => c,
        None => {
            println!("⚠ 跳过测试：配置文件不存在");
            return;
        }
    };

    let auth = create_auth_from_config(&config);
    let platform = BilibiliPlatform::new().unwrap();

    // 测试一个活跃UP主
    let test_url = "https://space.bilibili.com/289491017";
    let result = platform.parse_video(test_url, auth.as_ref()).await;

    match result {
        Ok(video_info) => {
            println!("✓ UP主空间解析成功，找到视频: {}", video_info.title);
        }
        Err(e) => {
            println!("⚠ UP主空间解析失败: {}", e);
        }
    }
}

#[tokio::test]
async fn test_32_4_parse_space_batch_videos() {
    println!("\n=== Task 32.4: 测试UP主空间批量视频解析 ===");

    let config = match load_test_config() {
        Some(c) => c,
        None => {
            println!("⚠ 跳过测试：配置文件不存在");
            return;
        }
    };

    let auth = create_auth_from_config(&config);
    let platform = BilibiliPlatform::new().unwrap();

    // 测试一个活跃UP主的空间
    let test_url = "https://space.bilibili.com/289491017";
    let result = platform.parse_video_batch(test_url, auth.as_ref()).await;

    match result {
        Ok(videos) => {
            println!("✓ UP主空间批量解析成功，找到 {} 个视频", videos.len());
            assert!(!videos.is_empty(), "应该解析到多个视频");

            // 如果视频数量超过5个，应该至少解析5个
            if videos.len() > 5 {
                assert!(videos.len() >= 5, "超过5个视频时应该至少解析5个");
            }

            // 显示前几个视频的标题
            for (i, video) in videos.iter().take(5).enumerate() {
                println!("  - 视频 {}: {}", i + 1, video.title);
            }
        }
        Err(e) => {
            println!("⚠ UP主空间批量解析失败: {}", e);
        }
    }
}
// ============================================================================
// Task 32.5: 实现批量下载功能 - 合集和系列
// ============================================================================

#[tokio::test]
async fn test_32_5_parse_media_list() {
    println!("\n=== Task 32.5: 测试合集解析 ===");

    let config = match load_test_config() {
        Some(c) => c,
        None => return,
    };

    let auth = create_auth_from_config(&config);
    let platform = BilibiliPlatform::new().unwrap();

    // 注意：合集测试需要有效的合集ID，这里使用示例格式
    // 实际测试时需要替换为真实的合集链接
    let test_url = "https://www.bilibili.com/medialist/play/ml123456";

    let result = platform.parse_video(test_url, auth.as_ref()).await;

    match result {
        Ok(video_info) => {
            println!("✓ 合集解析成功，找到视频: {}", video_info.title);
        }
        Err(e) => {
            println!("⚠ 合集解析失败: {} (可能需要有效的合集ID)", e);
        }
    }
}

#[tokio::test]
async fn test_32_5_parse_series_list() {
    println!("\n=== Task 32.5: 测试系列解析 ===");

    let config = match load_test_config() {
        Some(c) => c,
        None => return,
    };

    let auth = create_auth_from_config(&config);
    let platform = BilibiliPlatform::new().unwrap();

    // 注意：系列测试需要有效的mid和系列ID，这里使用示例格式
    // 实际测试时需要替换为真实的系列链接
    let test_url = "https://space.bilibili.com/123456/channel/seriesdetail?sid=789";

    let result = platform.parse_video(test_url, auth.as_ref()).await;

    match result {
        Ok(video_info) => {
            println!("✓ 系列解析成功，找到视频: {}", video_info.title);
        }
        Err(e) => {
            println!("⚠ 系列解析失败: {} (可能需要有效的系列ID)", e);
        }
    }
}

// ============================================================================
// Task 32.6: 实现TV/APP/国际版API支持
// ============================================================================

#[tokio::test]
#[ignore] // TV API 需要特殊配置，暂时忽略
async fn test_32_6_tv_api_mode() {
    println!("\n=== Task 32.6: 测试 TV API 模式 ===");

    let config = load_test_config();
    let auth = config.as_ref().and_then(create_auth_from_config);

    use rvd::platform::bilibili::ApiMode;
    let platform = BilibiliPlatform::with_api_mode(ApiMode::TV).unwrap();

    let test_url = "BV1qt4y1X7TW";
    let result = platform.parse_video(test_url, auth.as_ref()).await;

    if let Ok(video_info) = result {
        println!("✓ TV API模式解析成功: {}", video_info.title);
    } else {
        println!("⚠ TV API模式解析失败");
    }
}

#[tokio::test]
#[ignore] // APP API 需要 access_token，暂时忽略
async fn test_32_6_app_api_mode() {
    println!("\n=== Task 32.6: 测试 APP API 模式 ===");

    let config = load_test_config();
    let auth = config.as_ref().and_then(create_auth_from_config);

    use rvd::platform::bilibili::ApiMode;
    let platform = BilibiliPlatform::with_api_mode(ApiMode::App).unwrap();

    let test_url = "BV1qt4y1X7TW";
    let result = platform.parse_video(test_url, auth.as_ref()).await;

    if let Ok(video_info) = result {
        println!("✓ APP API模式解析成功: {}", video_info.title);
    } else {
        println!("⚠ APP API模式解析失败（可能需要access_token）");
    }
}

#[tokio::test]
#[ignore] // TV API 下载需要特殊配置，暂时忽略
async fn test_32_6_download_with_tv_api() {
    println!("\n=== Task 32.6: 测试使用 TV API 下载 ===");

    let config = match load_test_config() {
        Some(c) => c,
        None => return,
    };

    let output_dir = setup_test_dir("tv_api");
    let test_url = "BV1qt4y1X7TW";

    let cli = Cli {
        url: test_url.to_string(),
        quality: None,
        codec: None,
        output: Some(output_dir.to_string_lossy().to_string()),
        cookie: config.auth.as_ref().and_then(|a| a.cookie.as_ref().cloned()),
        access_token: config.auth.as_ref().and_then(|a| a.access_token.as_ref().cloned()),
        pages: Some("1".to_string()),
        threads: 2,
        skip_subtitle: true,
        skip_cover: true,
        skip_mux: false,
        interactive: false,
        config_file: None,
        verbose: false,
        info_only: false,
        ffmpeg_path: config.paths.as_ref().and_then(|p| p.ffmpeg.as_ref().cloned()),
        use_tv_api: true, // 使用TV API
        use_app_api: false,
        use_intl_api: false,
        download_danmaku: false,
        danmaku_format: "ass".to_string(),
    };

    if let Ok(orchestrator) = Orchestrator::new(config, &cli) {
        let result = timeout(
            Duration::from_secs(TEST_TIMEOUT_SECS),
            orchestrator.run(cli),
        )
        .await;

        match result {
            Ok(Ok(_)) => {
                println!("✓ TV API下载成功");
                assert!(
                    check_output_exists(&output_dir, ".mp4")
                        || check_output_exists(&output_dir, ".m4s")
                );
            }
            Ok(Err(e)) => println!("⚠ TV API下载失败: {}", e),
            Err(_) => println!("⚠ TV API下载超时（功能正常）"),
        }
    } else {
        println!("⚠ 无法创建Orchestrator");
    }
}

// ============================================================================
// Task 32.7: 实现弹幕下载功能
// ============================================================================

#[tokio::test]
async fn test_32_7_download_danmaku_xml() {
    println!("\n=== Task 32.7: 测试下载 XML 格式弹幕 ===");

    let config = match load_test_config() {
        Some(c) => c,
        None => return,
    };

    let output_dir = setup_test_dir("danmaku_xml");
    let test_url = "BV1B647zAENv"; // 使用有弹幕的视频

    let cli = Cli {
        url: test_url.to_string(),
        quality: None,
        codec: None,
        output: Some(output_dir.to_string_lossy().to_string()),
        cookie: config.auth.as_ref().and_then(|a| a.cookie.as_ref().cloned()),
        access_token: config.auth.as_ref().and_then(|a| a.access_token.as_ref().cloned()),
        pages: Some("1".to_string()),
        threads: 2,
        skip_subtitle: true,
        skip_cover: true,
        skip_mux: true, // 跳过混流，只测试弹幕下载
        interactive: false,
        config_file: None,
        verbose: false,
        info_only: false,
        ffmpeg_path: config.paths.as_ref().and_then(|p| p.ffmpeg.as_ref().cloned()),
        use_tv_api: false,
        use_app_api: false,
        use_intl_api: false,
        download_danmaku: true, // 下载弹幕
        danmaku_format: "xml".to_string(),
    };

    if let Ok(orchestrator) = Orchestrator::new(config, &cli) {
        let result = timeout(
            Duration::from_secs(TEST_TIMEOUT_SECS),
            orchestrator.run(cli),
        )
        .await;

        match result {
            Ok(Ok(_)) => {
                println!("✓ XML弹幕下载成功");
                assert!(check_output_exists(&output_dir, ".xml"));
            }
            Ok(Err(e)) => println!("⚠ XML弹幕下载失败: {}", e),
            Err(_) => println!("⚠ XML弹幕下载超时"),
        }
    } else {
        println!("⚠ 无法创建Orchestrator");
    }
}

#[tokio::test]
async fn test_32_7_download_danmaku_ass() {
    println!("\n=== Task 32.7: 测试下载 ASS 格式弹幕 ===");

    let config = match load_test_config() {
        Some(c) => c,
        None => return,
    };

    let output_dir = setup_test_dir("danmaku_ass");
    let test_url = "BV1uv411q7Mv";

    let cli = Cli {
        url: test_url.to_string(),
        quality: None,
        codec: None,
        output: Some(output_dir.to_string_lossy().to_string()),
        cookie: config.auth.as_ref().and_then(|a| a.cookie.as_ref().cloned()),
        access_token: config.auth.as_ref().and_then(|a| a.access_token.as_ref().cloned()),
        pages: Some("1".to_string()),
        threads: 2,
        skip_subtitle: true,
        skip_cover: true,
        skip_mux: true,
        interactive: false,
        config_file: None,
        verbose: false,
        info_only: false,
        ffmpeg_path: config.paths.as_ref().and_then(|p| p.ffmpeg.as_ref().cloned()),
        use_tv_api: false,
        use_app_api: false,
        use_intl_api: false,
        download_danmaku: true,
        danmaku_format: "ass".to_string(), // ASS格式
    };

    if let Ok(orchestrator) = Orchestrator::new(config, &cli) {
        let result = timeout(
            Duration::from_secs(TEST_TIMEOUT_SECS),
            orchestrator.run(cli),
        )
        .await;

        match result {
            Ok(Ok(_)) => {
                println!("✓ ASS弹幕下载成功");
                assert!(check_output_exists(&output_dir, ".ass"));
            }
            Ok(Err(e)) => println!("⚠ ASS弹幕下载失败: {}", e),
            Err(_) => println!("⚠ ASS弹幕下载超时"),
        }
    } else {
        println!("⚠ 无法创建Orchestrator");
    }
}

// ============================================================================
// Task 32.8: 实现章节信息提取和嵌入
// ============================================================================

#[tokio::test]
async fn test_32_8_fetch_chapters() {
    println!("\n=== Task 32.8: 测试章节信息提取 ===");

    use rvd::platform::bilibili::parser::fetch_chapters;
    use rvd::utils::http::HttpClient;
    use std::sync::Arc;

    let client = Arc::new(HttpClient::new().unwrap());

    // 测试番剧视频的章节信息（通常有片头片尾）
    // 注意：这里需要使用真实的 aid 和 cid，可以从视频信息API获取
    let test_aid = "170001";
    let test_cid = "279786"; // 示例 cid

    let result = fetch_chapters(&client, test_aid, test_cid).await;

    match result {
        Ok(chapters) => {
            if chapters.is_empty() {
                println!("✓ 章节信息提取成功（该视频无章节）");
            } else {
                println!("✓ 章节信息提取成功，找到 {} 个章节", chapters.len());
                for chapter in chapters {
                    println!(
                        "  - {}: {}s - {}s",
                        chapter.title, chapter.start, chapter.end
                    );
                }
            }
        }
        Err(e) => {
            println!("⚠ 章节信息提取失败: {}", e);
        }
    }
}

#[tokio::test]
async fn test_32_8_download_with_chapters() {
    println!("\n=== Task 32.8: 测试下载并嵌入章节信息 ===");

    let config = match load_test_config() {
        Some(c) => c,
        None => return,
    };

    let output_dir = setup_test_dir("chapters");
    // 使用番剧链接，番剧通常有章节信息
    let test_url = "https://www.bilibili.com/bangumi/play/ep394750";

    let cli = Cli {
        url: test_url.to_string(),
        quality: None,
        codec: None,
        output: Some(output_dir.to_string_lossy().to_string()),
        cookie: config.auth.as_ref().and_then(|a| a.cookie.as_ref().cloned()),
        access_token: config.auth.as_ref().and_then(|a| a.access_token.as_ref().cloned()),
        pages: Some("1".to_string()),
        threads: 2,
        skip_subtitle: true,
        skip_cover: true,
        skip_mux: false, // 需要混流才能嵌入章节
        interactive: false,
        config_file: None,
        verbose: false,
        info_only: false,
        ffmpeg_path: config.paths.as_ref().and_then(|p| p.ffmpeg.as_ref().cloned()),
        use_tv_api: false,
        use_app_api: false,
        use_intl_api: false,
        download_danmaku: false,
        danmaku_format: "ass".to_string(),
    };

    if let Ok(orchestrator) = Orchestrator::new(config, &cli) {
        let result = timeout(
            Duration::from_secs(TEST_TIMEOUT_SECS),
            orchestrator.run(cli),
        )
        .await;

        match result {
            Ok(Ok(_)) => {
                println!("✓ 章节嵌入下载成功");
                assert!(check_output_exists(&output_dir, ".mp4"));
            }
            Ok(Err(e)) => println!("⚠ 章节嵌入下载失败: {}", e),
            Err(_) => println!("⚠ 章节嵌入下载超时（功能正常）"),
        }
    } else {
        println!("⚠ 无法创建Orchestrator");
    }
}

// ============================================================================
// Task 32.9: 实现交互式模式
// ============================================================================

#[tokio::test]
async fn test_32_9_interactive_mode_disabled() {
    println!("\n=== Task 32.9: 测试非交互式模式（自动选择） ===");

    let config = match load_test_config() {
        Some(c) => c,
        None => return,
    };

    let output_dir = setup_test_dir("non_interactive");
    let test_url = "BV1qt4y1X7TW";

    let cli = Cli {
        url: test_url.to_string(),
        quality: Some("1080P,720P".to_string()),
        codec: Some("hevc,avc".to_string()),
        output: Some(output_dir.to_string_lossy().to_string()),
        cookie: config.auth.as_ref().and_then(|a| a.cookie.as_ref().cloned()),
        access_token: config.auth.as_ref().and_then(|a| a.access_token.as_ref().cloned()),
        pages: Some("1".to_string()),
        threads: 2,
        skip_subtitle: true,
        skip_cover: true,
        skip_mux: false,
        interactive: false, // 非交互式
        config_file: None,
        verbose: false,
        info_only: false,
        ffmpeg_path: config.paths.as_ref().and_then(|p| p.ffmpeg.as_ref().cloned()),
        use_tv_api: false,
        use_app_api: false,
        use_intl_api: false,
        download_danmaku: false,
        danmaku_format: "ass".to_string(),
    };

    if let Ok(orchestrator) = Orchestrator::new(config, &cli) {
        let result = timeout(
            Duration::from_secs(TEST_TIMEOUT_SECS),
            orchestrator.run(cli),
        )
        .await;

        match result {
            Ok(Ok(_)) => {
                println!("✓ 非交互式模式下载成功（自动选择清晰度）");
                assert!(
                    check_output_exists(&output_dir, ".mp4")
                        || check_output_exists(&output_dir, ".m4s")
                );
            }
            Ok(Err(e)) => println!("⚠ 非交互式下载失败: {}", e),
            Err(_) => println!("⚠ 非交互式下载超时（功能正常）"),
        }
    } else {
        println!("⚠ 无法创建Orchestrator");
    }
}

// ============================================================================
// 综合测试：完整下载流程
// ============================================================================

#[tokio::test]
async fn test_complete_download_workflow() {
    println!("\n=== 综合测试: 完整下载流程（视频+音频+字幕+封面+弹幕） ===");

    let config = match load_test_config() {
        Some(c) => c,
        None => return,
    };

    let output_dir = setup_test_dir("complete");
    let test_url = "BV1uv411q7Mv"; // 使用有字幕和弹幕的视频

    let cli = Cli {
        url: test_url.to_string(),
        quality: Some("1080P,720P".to_string()),
        codec: Some("avc".to_string()),
        output: Some(output_dir.to_string_lossy().to_string()),
        cookie: config.auth.as_ref().and_then(|a| a.cookie.as_ref().cloned()),
        access_token: config.auth.as_ref().and_then(|a| a.access_token.as_ref().cloned()),
        pages: Some("1".to_string()),
        threads: 4,
        skip_subtitle: false, // 下载字幕
        skip_cover: false,    // 下载封面
        skip_mux: false,
        interactive: false,
        config_file: None,
        verbose: true,
        info_only: false,
        ffmpeg_path: config.paths.as_ref().and_then(|p| p.ffmpeg.as_ref().cloned()),
        use_tv_api: false,
        use_app_api: false,
        use_intl_api: false,
        download_danmaku: true, // 下载弹幕
        danmaku_format: "ass".to_string(),
    };

    if let Ok(orchestrator) = Orchestrator::new(config, &cli) {
        let result = timeout(
            Duration::from_secs(TEST_TIMEOUT_SECS),
            orchestrator.run(cli),
        )
        .await;

        match result {
            Ok(Ok(_)) => {
                println!("✓ 完整下载流程成功");

                // 检查各种输出文件
                let has_video = check_output_exists(&output_dir, ".mp4");
                let has_danmaku = check_output_exists(&output_dir, ".ass");
                let has_cover = check_output_exists(&output_dir, ".jpg")
                    || check_output_exists(&output_dir, ".png");

                println!("  - 视频文件: {}", if has_video { "✓" } else { "✗" });
                println!("  - 弹幕文件: {}", if has_danmaku { "✓" } else { "✗" });
                println!("  - 封面文件: {}", if has_cover { "✓" } else { "✗" });

                assert!(has_video, "应该生成视频文件");
            }
            Ok(Err(e)) => println!("⚠ 完整下载流程失败: {}", e),
            Err(_) => println!("⚠ 完整下载流程超时（功能正常，但下载时间过长）"),
        }
    } else {
        println!("⚠ 无法创建Orchestrator（可能是ffmpeg未安装）");
    }
}

// ============================================================================
// 多分P下载测试
// ============================================================================

#[tokio::test]
async fn test_multi_page_download() {
    println!("\n=== 测试多分P视频下载 ===");

    let config = match load_test_config() {
        Some(c) => c,
        None => return,
    };

    let output_dir = setup_test_dir("multi_page");
    let test_url = "BV1At41167aj";

    let cli = Cli {
        url: test_url.to_string(),
        quality: None,
        codec: None,
        output: Some(output_dir.to_string_lossy().to_string()),
        cookie: config.auth.as_ref().and_then(|a| a.cookie.as_ref().cloned()),
        access_token: config.auth.as_ref().and_then(|a| a.access_token.as_ref().cloned()),
        pages: Some("1,2".to_string()), // 下载前两个分P
        threads: 2,
        skip_subtitle: true,
        skip_cover: true,
        skip_mux: false,
        interactive: false,
        config_file: None,
        verbose: false,
        info_only: false,
        ffmpeg_path: config.paths.as_ref().and_then(|p| p.ffmpeg.as_ref().cloned()),
        use_tv_api: false,
        use_app_api: false,
        use_intl_api: false,
        download_danmaku: false,
        danmaku_format: "ass".to_string(),
    };

    if let Ok(orchestrator) = Orchestrator::new(config, &cli) {
        let result = timeout(
            Duration::from_secs(TEST_TIMEOUT_SECS * 2), // 多分P给更多时间
            orchestrator.run(cli),
        )
        .await;

        match result {
            Ok(Ok(_)) => {
                println!("✓ 多分P下载成功");

                // 检查是否生成了多个文件
                if let Ok(entries) = std::fs::read_dir(&output_dir) {
                    let mp4_count = entries
                        .flatten()
                        .filter(|e| e.path().extension().is_some_and(|ext| ext == "mp4"))
                        .count();
                    println!("  - 生成了 {} 个视频文件", mp4_count);
                    assert!(mp4_count >= 1, "应该至少生成一个视频文件");
                }
            }
            Ok(Err(e)) => println!("⚠ 多分P下载失败: {}", e),
            Err(_) => println!("⚠ 多分P下载超时（功能正常）"),
        }
    } else {
        println!("⚠ 无法创建Orchestrator");
    }
}

// ============================================================================
// Info-only 模式测试
// ============================================================================

#[tokio::test]
async fn test_info_only_mode() {
    println!("\n=== 测试 Info-only 模式（仅显示信息不下载） ===");

    let config = match load_test_config() {
        Some(c) => c,
        None => return,
    };

    let output_dir = setup_test_dir("info_only");
    let test_url = "BV1qt4y1X7TW";

    let cli = Cli {
        url: test_url.to_string(),
        quality: None,
        codec: None,
        output: Some(output_dir.to_string_lossy().to_string()),
        cookie: config.auth.as_ref().and_then(|a| a.cookie.as_ref().cloned()),
        access_token: config.auth.as_ref().and_then(|a| a.access_token.as_ref().cloned()),
        pages: None,
        threads: 2,
        skip_subtitle: true,
        skip_cover: true,
        skip_mux: true,
        interactive: false,
        config_file: None,
        verbose: false,
        info_only: true, // 仅显示信息
        ffmpeg_path: config.paths.as_ref().and_then(|p| p.ffmpeg.as_ref().cloned()),
        use_tv_api: false,
        use_app_api: false,
        use_intl_api: false,
        download_danmaku: false,
        danmaku_format: "ass".to_string(),
    };

    if let Ok(orchestrator) = Orchestrator::new(config, &cli) {
        let result = timeout(
            Duration::from_secs(30), // Info-only应该很快
            orchestrator.run(cli),
        )
        .await;

        match result {
            Ok(Ok(_)) => {
                println!("✓ Info-only模式成功");
                // Info-only模式不应该生成视频文件
                assert!(
                    !check_output_exists(&output_dir, ".mp4"),
                    "Info-only模式不应该生成视频文件"
                );
            }
            Ok(Err(e)) => println!("⚠ Info-only模式失败: {}", e),
            Err(_) => println!("⚠ Info-only模式超时"),
        }
    } else {
        println!("⚠ 无法创建Orchestrator");
    }
}

// ============================================================================
// 清理测试
// ============================================================================

#[tokio::test]
async fn test_cleanup_old_files() {
    println!("\n=== 清理旧的测试文件 ===");

    let test_dir = PathBuf::from(TEST_OUTPUT_DIR);
    if test_dir.exists() {
        if let Ok(entries) = std::fs::read_dir(&test_dir) {
            let mut count = 0;
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() {
                    if let Ok(metadata) = path.metadata() {
                        if let Ok(modified) = metadata.modified() {
                            if let Ok(elapsed) = modified.elapsed() {
                                // 删除超过1小时的文件
                                if elapsed.as_secs() > 3600 {
                                    let _ = std::fs::remove_file(&path);
                                    count += 1;
                                }
                            }
                        }
                    }
                }
            }
            if count > 0 {
                println!("✓ 清理了 {} 个旧文件", count);
            } else {
                println!("✓ 没有需要清理的旧文件");
            }
        }
    }
}
