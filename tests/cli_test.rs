// CLI参数解析单元测试
use rvd::cli::Cli;

#[test]
fn test_parse_quality_priority_default() {
    let cli = Cli {
        url: "https://www.bilibili.com/video/BV1xx411c7mD".to_string(),
        quality: None,
        codec: None,
        output: None,
        cookie: None,
        access_token: None,
        pages: None,
        threads: 4,
        skip_subtitle: false,
        skip_cover: false,
        skip_mux: false,
        interactive: false,
        config_file: None,
        verbose: false,
        info_only: false,
        ffmpeg_path: None,
    };

    let quality = cli.parse_quality_priority();
    assert_eq!(quality, vec!["1080P", "720P", "480P"]);
}

#[test]
fn test_parse_quality_priority_custom() {
    let cli = Cli {
        url: "https://www.bilibili.com/video/BV1xx411c7mD".to_string(),
        quality: Some("4K 超清,1080P 高清,720P 高清".to_string()),
        codec: None,
        output: None,
        cookie: None,
        access_token: None,
        pages: None,
        threads: 4,
        skip_subtitle: false,
        skip_cover: false,
        skip_mux: false,
        interactive: false,
        config_file: None,
        verbose: false,
        info_only: false,
        ffmpeg_path: None,
    };

    let quality = cli.parse_quality_priority();
    assert_eq!(quality, vec!["4K 超清", "1080P 高清", "720P 高清"]);
}

#[test]
fn test_parse_codec_priority_default() {
    let cli = Cli {
        url: "https://www.bilibili.com/video/BV1xx411c7mD".to_string(),
        quality: None,
        codec: None,
        output: None,
        cookie: None,
        access_token: None,
        pages: None,
        threads: 4,
        skip_subtitle: false,
        skip_cover: false,
        skip_mux: false,
        interactive: false,
        config_file: None,
        verbose: false,
        info_only: false,
        ffmpeg_path: None,
    };

    let codec = cli.parse_codec_priority();
    assert_eq!(codec, vec!["avc", "hevc", "av1"]);
}

#[test]
fn test_parse_codec_priority_custom() {
    let cli = Cli {
        url: "https://www.bilibili.com/video/BV1xx411c7mD".to_string(),
        quality: None,
        codec: Some("hevc,av1,avc".to_string()),
        output: None,
        cookie: None,
        access_token: None,
        pages: None,
        threads: 4,
        skip_subtitle: false,
        skip_cover: false,
        skip_mux: false,
        interactive: false,
        config_file: None,
        verbose: false,
        info_only: false,
        ffmpeg_path: None,
    };

    let codec = cli.parse_codec_priority();
    assert_eq!(codec, vec!["hevc", "av1", "avc"]);
}

#[test]
fn test_parse_pages_none() {
    let cli = Cli {
        url: "https://www.bilibili.com/video/BV1xx411c7mD".to_string(),
        quality: None,
        codec: None,
        output: None,
        cookie: None,
        access_token: None,
        pages: None,
        threads: 4,
        skip_subtitle: false,
        skip_cover: false,
        skip_mux: false,
        interactive: false,
        config_file: None,
        verbose: false,
        info_only: false,
        ffmpeg_path: None,
    };

    let pages = cli.parse_pages();
    assert!(pages.is_none());
}

#[test]
fn test_parse_pages_all() {
    let cli = Cli {
        url: "https://www.bilibili.com/video/BV1xx411c7mD".to_string(),
        quality: None,
        codec: None,
        output: None,
        cookie: None,
        access_token: None,
        pages: Some("ALL".to_string()),
        threads: 4,
        skip_subtitle: false,
        skip_cover: false,
        skip_mux: false,
        interactive: false,
        config_file: None,
        verbose: false,
        info_only: false,
        ffmpeg_path: None,
    };

    let pages = cli.parse_pages();
    assert!(pages.is_none());
}

#[test]
fn test_parse_pages_single() {
    let cli = Cli {
        url: "https://www.bilibili.com/video/BV1xx411c7mD".to_string(),
        quality: None,
        codec: None,
        output: None,
        cookie: None,
        access_token: None,
        pages: Some("5".to_string()),
        threads: 4,
        skip_subtitle: false,
        skip_cover: false,
        skip_mux: false,
        interactive: false,
        config_file: None,
        verbose: false,
        info_only: false,
        ffmpeg_path: None,
    };

    let pages = cli.parse_pages();
    assert_eq!(pages, Some(vec![5]));
}

#[test]
fn test_parse_pages_multiple() {
    let cli = Cli {
        url: "https://www.bilibili.com/video/BV1xx411c7mD".to_string(),
        quality: None,
        codec: None,
        output: None,
        cookie: None,
        access_token: None,
        pages: Some("1,3,5".to_string()),
        threads: 4,
        skip_subtitle: false,
        skip_cover: false,
        skip_mux: false,
        interactive: false,
        config_file: None,
        verbose: false,
        info_only: false,
        ffmpeg_path: None,
    };

    let pages = cli.parse_pages();
    assert_eq!(pages, Some(vec![1, 3, 5]));
}

#[test]
fn test_parse_pages_range() {
    let cli = Cli {
        url: "https://www.bilibili.com/video/BV1xx411c7mD".to_string(),
        quality: None,
        codec: None,
        output: None,
        cookie: None,
        access_token: None,
        pages: Some("3-7".to_string()),
        threads: 4,
        skip_subtitle: false,
        skip_cover: false,
        skip_mux: false,
        interactive: false,
        config_file: None,
        verbose: false,
        info_only: false,
        ffmpeg_path: None,
    };

    let pages = cli.parse_pages();
    assert_eq!(pages, Some(vec![3, 4, 5, 6, 7]));
}

#[test]
fn test_parse_pages_mixed() {
    let cli = Cli {
        url: "https://www.bilibili.com/video/BV1xx411c7mD".to_string(),
        quality: None,
        codec: None,
        output: None,
        cookie: None,
        access_token: None,
        pages: Some("1,3-5,8".to_string()),
        threads: 4,
        skip_subtitle: false,
        skip_cover: false,
        skip_mux: false,
        interactive: false,
        config_file: None,
        verbose: false,
        info_only: false,
        ffmpeg_path: None,
    };

    let pages = cli.parse_pages();
    assert_eq!(pages, Some(vec![1, 3, 4, 5, 8]));
}
