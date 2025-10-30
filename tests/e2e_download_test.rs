// 端到端测试 - 测试完整的下载流程（需要网络连接）
// 这些测试会实际下载小文件，因此标记为 #[ignore]，需要手动运行
// 
// 运行方式：
// cargo test --test e2e_download_test -- --ignored --nocapture

use rvd::app::Orchestrator;
use rvd::cli::Cli;
use rvd::utils::config::Config;
use std::path::PathBuf;

const TEST_OUTPUT_DIR: &str = "tests/test_data";
const TEST_CONFIG_PATH: &str = "tests/rvd.toml";

/// 辅助函数：加载测试配置（包含认证信息）
fn load_test_config() -> Config {
    let config_path = PathBuf::from(TEST_CONFIG_PATH);
    if config_path.exists() {
        Config::load(&config_path).unwrap_or_default()
    } else {
        eprintln!("⚠ 警告: 测试配置文件不存在: {}", TEST_CONFIG_PATH);
        eprintln!("提示: 创建该文件并添加认证信息以测试会员功能");
        Config::default()
    }
}

/// 辅助函数：创建测试CLI配置
fn create_test_cli(url: &str, output_name: &str, skip_mux: bool) -> Cli {
    let output_path = PathBuf::from(TEST_OUTPUT_DIR).join(output_name);

    Cli {
        url: url.to_string(),
        quality: Some("1080P,720P,480P,360P".to_string()), // 会员可以下载高清
        codec: Some("avc,hevc".to_string()),
        output: Some(output_path.to_string_lossy().to_string()),
        cookie: None, // 从配置文件加载
        access_token: None,
        pages: Some("1".to_string()), // 只下载第一个分P
        threads: 2,
        skip_subtitle: true, // 跳过字幕以加快测试
        skip_cover: true,    // 跳过封面以加快测试
        skip_mux,
        interactive: false,
        config_file: Some(PathBuf::from(TEST_CONFIG_PATH)),
        verbose: true,
        info_only: false,
        ffmpeg_path: None,
        use_tv_api: false,
        use_app_api: false,
        use_intl_api: false,
        download_danmaku: false,
        danmaku_format: "ass".to_string(),
    }
}

/// 辅助函数：创建带自定义配置的CLI
#[allow(dead_code)]
fn create_test_cli_with_config(url: &str, output_name: &str, skip_mux: bool, config_path: Option<PathBuf>) -> Cli {
    let output_path = PathBuf::from(TEST_OUTPUT_DIR).join(output_name);

    Cli {
        url: url.to_string(),
        quality: Some("1080P,720P,480P,360P".to_string()),
        codec: Some("avc,hevc".to_string()),
        output: Some(output_path.to_string_lossy().to_string()),
        cookie: None,
        access_token: None,
        pages: Some("1".to_string()),
        threads: 2,
        skip_subtitle: true,
        skip_cover: true,
        skip_mux,
        interactive: false,
        config_file: config_path,
        verbose: true,
        info_only: false,
        ffmpeg_path: None,
        use_tv_api: false,
        use_app_api: false,
        use_intl_api: false,
        download_danmaku: false,
        danmaku_format: "ass".to_string(),
    }
}

/// 辅助函数：清理测试文件
fn cleanup_test_file(filename: &str) {
    let path = PathBuf::from(TEST_OUTPUT_DIR).join(filename);
    if path.exists() {
        let _ = std::fs::remove_file(&path);
    }
    // 也清理可能的临时文件
    let video_path = path.with_extension("video.m4s");
    let audio_path = path.with_extension("audio.m4s");
    if video_path.exists() {
        let _ = std::fs::remove_file(&video_path);
    }
    if audio_path.exists() {
        let _ = std::fs::remove_file(&audio_path);
    }
}

#[tokio::test]
#[ignore] // 需要手动运行: cargo test --test e2e_download_test -- --ignored
async fn test_e2e_download_single_video() {
    println!("\n=== 测试: 单视频完整下载 ===");
    
    // 使用一个短小的公开视频进行测试
    let test_url = "https://www.bilibili.com/video/BV1xx411c7mD";
    let output_name = "e2e_test_single.mp4";

    cleanup_test_file(output_name);

    let cli = create_test_cli(test_url, output_name, false);
    let config = load_test_config();

    println!("配置加载: {}", if config.auth.is_some() { "✓ 已加载认证信息" } else { "⚠ 未配置认证" });

    // 尝试创建orchestrator
    let orchestrator = match Orchestrator::new(config, &cli) {
        Ok(o) => o,
        Err(e) => {
            eprintln!("⚠ 无法创建Orchestrator: {}", e);
            eprintln!("提示: 请确保已安装ffmpeg");
            return;
        }
    };

    // 执行下载
    let result = orchestrator.run(cli).await;

    match result {
        Ok(()) => {
            let output_path = PathBuf::from(TEST_OUTPUT_DIR).join(output_name);
            assert!(
                output_path.exists(),
                "下载的文件应该存在: {:?}",
                output_path
            );

            let metadata = std::fs::metadata(&output_path).unwrap();
            assert!(metadata.len() > 0, "下载的文件大小应该大于0");

            println!("✓ 端到端下载测试成功");
            println!("  文件: {:?}", output_path);
            println!("  大小: {} bytes ({:.2} MB)", metadata.len(), metadata.len() as f64 / 1024.0 / 1024.0);

            // 清理测试文件
            cleanup_test_file(output_name);
        }
        Err(e) => {
            eprintln!("✗ 下载失败: {}", e);
            panic!("端到端下载测试失败");
        }
    }
}

#[tokio::test]
#[ignore]
async fn test_e2e_download_skip_mux() {
    println!("\n=== 测试: 跳过混流下载 ===");
    
    // 测试跳过混流，保留分离的视频和音频文件
    let test_url = "https://www.bilibili.com/video/BV1xx411c7mD";
    let output_name = "e2e_test_skip_mux.mp4";

    cleanup_test_file(output_name);

    let cli = create_test_cli(test_url, output_name, true); // skip_mux = true
    let config = load_test_config();

    let orchestrator = match Orchestrator::new(config, &cli) {
        Ok(o) => o,
        Err(e) => {
            eprintln!("⚠ 无法创建Orchestrator: {}", e);
            return;
        }
    };

    let result = orchestrator.run(cli).await;

    match result {
        Ok(()) => {
            let base_path = PathBuf::from(TEST_OUTPUT_DIR).join(output_name);
            let video_path = base_path.with_extension("video.m4s");
            let audio_path = base_path.with_extension("audio.m4s");

            // 应该存在分离的视频和音频文件
            assert!(
                video_path.exists(),
                "视频文件应该存在: {:?}",
                video_path
            );
            assert!(
                audio_path.exists(),
                "音频文件应该存在: {:?}",
                audio_path
            );

            let video_size = std::fs::metadata(&video_path).unwrap().len();
            let audio_size = std::fs::metadata(&audio_path).unwrap().len();

            println!("✓ 跳过混流测试成功");
            println!("  视频: {:?} ({:.2} MB)", video_path, video_size as f64 / 1024.0 / 1024.0);
            println!("  音频: {:?} ({:.2} MB)", audio_path, audio_size as f64 / 1024.0 / 1024.0);

            // 清理测试文件
            cleanup_test_file(output_name);
        }
        Err(e) => {
            eprintln!("✗ 下载失败: {}", e);
            panic!("跳过混流测试失败");
        }
    }
}

#[tokio::test]
#[ignore]
async fn test_e2e_download_with_config() {
    println!("\n=== 测试: 使用配置文件下载 ===");
    
    // 测试使用配置文件的下载
    let test_url = "https://www.bilibili.com/video/BV1xx411c7mD";
    let output_name = "e2e_test_with_config.mp4";

    cleanup_test_file(output_name);

    let output_path = PathBuf::from(TEST_OUTPUT_DIR).join(output_name);

    // 使用测试配置文件（包含认证信息）
    let cli = Cli {
        url: test_url.to_string(),
        quality: None, // 使用配置文件中的设置
        codec: None,   // 使用配置文件中的设置
        output: Some(output_path.to_string_lossy().to_string()),
        cookie: None,
        access_token: None,
        pages: Some("1".to_string()),
        threads: 2,
        skip_subtitle: true,
        skip_cover: true,
        skip_mux: false,
        interactive: false,
        config_file: Some(PathBuf::from(TEST_CONFIG_PATH)),
        verbose: true,
        info_only: false,
        ffmpeg_path: None,
        use_tv_api: false,
        use_app_api: false,
        use_intl_api: false,
        download_danmaku: false,
        danmaku_format: "ass".to_string(),
    };

    let config = load_test_config();
    println!("使用配置文件: {}", TEST_CONFIG_PATH);

    let orchestrator = match Orchestrator::new(config, &cli) {
        Ok(o) => o,
        Err(e) => {
            eprintln!("⚠ 无法创建Orchestrator: {}", e);
            return;
        }
    };

    let result = orchestrator.run(cli).await;

    match result {
        Ok(()) => {
            assert!(output_path.exists(), "下载的文件应该存在");
            let metadata = std::fs::metadata(&output_path).unwrap();
            println!("✓ 配置文件下载测试成功");
            println!("  大小: {:.2} MB", metadata.len() as f64 / 1024.0 / 1024.0);

            cleanup_test_file(output_name);
        }
        Err(e) => {
            eprintln!("✗ 下载失败: {}", e);
            panic!("配置文件下载测试失败");
        }
    }
}

#[tokio::test]
#[ignore]
async fn test_e2e_multi_page_download() {
    println!("\n=== 测试: 多分P下载 ===");
    
    // 测试多分P下载（如果测试视频有多个分P）
    let test_url = "https://www.bilibili.com/video/BV1xx411c7mD";
    let output_name = "e2e_test_multi_page";

    // 清理可能存在的文件
    cleanup_test_file(&format!("{}.mp4", output_name));

    let output_path = PathBuf::from(TEST_OUTPUT_DIR).join(output_name);

    let cli = Cli {
        url: test_url.to_string(),
        quality: Some("720P,480P,360P".to_string()),
        codec: Some("avc".to_string()),
        output: Some(output_path.to_string_lossy().to_string()),
        cookie: None,
        access_token: None,
        pages: Some("1,2".to_string()), // 尝试下载前两个分P
        threads: 2,
        skip_subtitle: true,
        skip_cover: true,
        skip_mux: false,
        interactive: false,
        config_file: Some(PathBuf::from(TEST_CONFIG_PATH)),
        verbose: true,
        info_only: false,
        ffmpeg_path: None,
        use_tv_api: false,
        use_app_api: false,
        use_intl_api: false,
        download_danmaku: false,
        danmaku_format: "ass".to_string(),
    };

    let config = load_test_config();

    let orchestrator = match Orchestrator::new(config, &cli) {
        Ok(o) => o,
        Err(e) => {
            eprintln!("⚠ 无法创建Orchestrator: {}", e);
            return;
        }
    };

    let result = orchestrator.run(cli).await;

    match result {
        Ok(()) => {
            println!("✓ 多分P下载测试完成");

            // 清理测试文件
            if let Ok(entries) = std::fs::read_dir(TEST_OUTPUT_DIR) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if let Some(name) = path.file_name() {
                        if name.to_string_lossy().contains(output_name) {
                            let metadata = std::fs::metadata(&path).ok();
                            if let Some(m) = metadata {
                                println!("  清理: {:?} ({:.2} MB)", path, m.len() as f64 / 1024.0 / 1024.0);
                            }
                            let _ = std::fs::remove_file(&path);
                        }
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("⚠ 多分P下载测试: {}", e);
            // 不panic，因为测试视频可能只有单P
        }
    }
}

#[tokio::test]
#[ignore]
async fn test_e2e_error_recovery() {
    println!("\n=== 测试: 错误恢复 ===");
    
    // 测试错误恢复：使用无效URL
    let invalid_url = "https://www.bilibili.com/video/BV1111111111";
    let output_name = "e2e_test_error.mp4";

    let cli = create_test_cli(invalid_url, output_name, false);
    let config = load_test_config();

    let orchestrator = match Orchestrator::new(config, &cli) {
        Ok(o) => o,
        Err(e) => {
            eprintln!("⚠ 无法创建Orchestrator: {}", e);
            return;
        }
    };

    let result = orchestrator.run(cli).await;

    // 应该返回错误
    assert!(result.is_err(), "无效URL应该返回错误");
    println!("✓ 错误恢复测试成功 - 正确处理了无效URL");
}




#[tokio::test]
#[ignore]
async fn test_e2e_quality_fallback() {
    println!("\n=== 测试: 质量降级 ===");
    
    // 测试质量降级：请求不存在的高质量，应该自动降级
    let test_url = "https://www.bilibili.com/video/BV1xx411c7mD";
    let output_name = "e2e_test_fallback.mp4";

    cleanup_test_file(output_name);

    let output_path = PathBuf::from(TEST_OUTPUT_DIR).join(output_name);

    let cli = Cli {
        url: test_url.to_string(),
        quality: Some("8K,4K,1080P,720P,480P".to_string()), // 请求高质量，会自动降级
        codec: Some("av1,hevc,avc".to_string()),
        output: Some(output_path.to_string_lossy().to_string()),
        cookie: None,
        access_token: None,
        pages: Some("1".to_string()),
        threads: 2,
        skip_subtitle: true,
        skip_cover: true,
        skip_mux: false,
        interactive: false,
        config_file: Some(PathBuf::from(TEST_CONFIG_PATH)),
        verbose: true,
        info_only: false,
        ffmpeg_path: None,
        use_tv_api: false,
        use_app_api: false,
        use_intl_api: false,
        download_danmaku: false,
        danmaku_format: "ass".to_string(),
    };

    let config = load_test_config();
    println!("请求质量: 8K -> 4K -> 1080P -> 720P -> 480P");
    println!("会员状态: {}", if config.auth.is_some() { "✓ 已认证" } else { "⚠ 未认证" });

    let orchestrator = match Orchestrator::new(config, &cli) {
        Ok(o) => o,
        Err(e) => {
            eprintln!("⚠ 无法创建Orchestrator: {}", e);
            return;
        }
    };

    let result = orchestrator.run(cli).await;

    match result {
        Ok(()) => {
            assert!(output_path.exists(), "下载的文件应该存在");
            let metadata = std::fs::metadata(&output_path).unwrap();
            println!("✓ 质量降级测试成功");
            println!("  最终下载大小: {:.2} MB", metadata.len() as f64 / 1024.0 / 1024.0);

            cleanup_test_file(output_name);
        }
        Err(e) => {
            eprintln!("✗ 下载失败: {}", e);
            panic!("质量降级测试失败");
        }
    }
}

#[tokio::test]
#[ignore]
async fn test_e2e_high_quality_with_auth() {
    println!("\n=== 测试: 会员高清下载 ===");
    
    // 测试会员高清下载
    let test_url = "https://www.bilibili.com/video/BV1xx411c7mD";
    let output_name = "e2e_test_high_quality.mp4";

    cleanup_test_file(output_name);

    let output_path = PathBuf::from(TEST_OUTPUT_DIR).join(output_name);

    let cli = Cli {
        url: test_url.to_string(),
        quality: Some("1080P 高清,720P 高清".to_string()), // 会员可以下载1080P
        codec: Some("hevc,avc".to_string()),
        output: Some(output_path.to_string_lossy().to_string()),
        cookie: None,
        access_token: None,
        pages: Some("1".to_string()),
        threads: 4, // 使用更多线程
        skip_subtitle: false, // 下载字幕
        skip_cover: false,    // 下载封面
        skip_mux: false,
        interactive: false,
        config_file: Some(PathBuf::from(TEST_CONFIG_PATH)),
        verbose: true,
        info_only: false,
        ffmpeg_path: None,
        use_tv_api: false,
        use_app_api: false,
        use_intl_api: false,
        download_danmaku: false,
        danmaku_format: "ass".to_string(),
    };

    let config = load_test_config();
    
    if config.auth.is_none() {
        println!("⚠ 跳过测试: 未配置认证信息");
        println!("提示: 在 {} 中配置cookie以测试会员功能", TEST_CONFIG_PATH);
        return;
    }

    println!("✓ 已加载会员认证信息");
    println!("请求质量: 1080P 高清 (HEVC优先)");

    let orchestrator = match Orchestrator::new(config, &cli) {
        Ok(o) => o,
        Err(e) => {
            eprintln!("⚠ 无法创建Orchestrator: {}", e);
            return;
        }
    };

    let result = orchestrator.run(cli).await;

    match result {
        Ok(()) => {
            assert!(output_path.exists(), "下载的文件应该存在");
            let metadata = std::fs::metadata(&output_path).unwrap();
            
            println!("✓ 会员高清下载测试成功");
            println!("  文件大小: {:.2} MB", metadata.len() as f64 / 1024.0 / 1024.0);
            
            // 检查字幕和封面
            let subtitle_exists = std::fs::read_dir(TEST_OUTPUT_DIR)
                .ok()
                .and_then(|entries| {
                    entries.flatten().find(|e| {
                        e.path().extension().and_then(|s| s.to_str()) == Some("srt")
                    })
                })
                .is_some();
            
            let cover_exists = std::fs::read_dir(TEST_OUTPUT_DIR)
                .ok()
                .and_then(|entries| {
                    entries.flatten().find(|e| {
                        let path = e.path();
                        let ext = path.extension().and_then(|s| s.to_str());
                        ext == Some("jpg") || ext == Some("png")
                    })
                })
                .is_some();
            
            if subtitle_exists {
                println!("  ✓ 字幕已下载");
            }
            if cover_exists {
                println!("  ✓ 封面已下载");
            }

            // 清理测试文件
            cleanup_test_file(output_name);
            // 清理字幕和封面
            if let Ok(entries) = std::fs::read_dir(TEST_OUTPUT_DIR) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
                        if ext == "srt" || ext == "jpg" || ext == "png" {
                            let _ = std::fs::remove_file(&path);
                        }
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("✗ 下载失败: {}", e);
            panic!("会员高清下载测试失败");
        }
    }
}
