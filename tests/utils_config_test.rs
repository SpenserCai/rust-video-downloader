// 配置管理模块单元测试
use rvd::utils::config::Config;
use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;

#[test]
fn test_load_config_not_exists() {
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("nonexistent.toml");

    let config = Config::load(&config_path).unwrap();

    // 配置文件不存在时应返回默认配置
    assert!(config.default_quality.is_none());
    assert!(config.default_codec.is_none());
    assert!(config.thread_count.is_none());
}

#[test]
fn test_load_config_valid() {
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("test.toml");

    let config_content = r#"
default_quality = ["1080P 高清", "720P 高清"]
default_codec = ["hevc", "avc"]
thread_count = 8
output_template = "<videoTitle>"

[auth]
cookie = "test_cookie"
access_token = "test_token"

[paths]
ffmpeg = "/usr/bin/ffmpeg"
"#;

    fs::write(&config_path, config_content).unwrap();

    let config = Config::load(&config_path).unwrap();

    assert_eq!(
        config.default_quality,
        Some(vec!["1080P 高清".to_string(), "720P 高清".to_string()])
    );
    assert_eq!(
        config.default_codec,
        Some(vec!["hevc".to_string(), "avc".to_string()])
    );
    assert_eq!(config.thread_count, Some(8));
    assert_eq!(config.output_template, Some("<videoTitle>".to_string()));

    let auth = config.auth.unwrap();
    assert_eq!(auth.cookie, Some("test_cookie".to_string()));
    assert_eq!(auth.access_token, Some("test_token".to_string()));

    let paths = config.paths.unwrap();
    assert_eq!(paths.ffmpeg, Some(PathBuf::from("/usr/bin/ffmpeg")));
}

#[test]
fn test_load_config_invalid_toml() {
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("invalid.toml");

    let invalid_content = "this is not valid toml [[[";
    fs::write(&config_path, invalid_content).unwrap();

    let result = Config::load(&config_path);
    assert!(result.is_err());
}

#[test]
fn test_load_config_partial() {
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("partial.toml");

    let config_content = r#"
default_quality = ["1080P 高清"]
thread_count = 4
"#;

    fs::write(&config_path, config_content).unwrap();

    let config = Config::load(&config_path).unwrap();

    assert_eq!(
        config.default_quality,
        Some(vec!["1080P 高清".to_string()])
    );
    assert_eq!(config.thread_count, Some(4));
    assert!(config.default_codec.is_none());
    assert!(config.auth.is_none());
}
