// CLI参数解析单元测试
use rvd::cli::Cli;

#[test]
fn test_parse_quality_priority_default() {
    let cli = Cli {
        url: Some("https://www.bilibili.com/video/BV1xx411c7mD".to_string()),
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
        use_tv_api: false,
        use_app_api: false,
        use_intl_api: false,
        download_danmaku: false,
        danmaku_format: "ass".to_string(),
        login_qrcode: false,
        login_tv: false,
        use_aria2c: false,
        aria2c_path: None,
        aria2c_args: None,
    };

    let quality = cli.parse_quality_priority();
    assert_eq!(quality, vec!["1080P", "720P", "480P"]);
}

#[test]
fn test_parse_quality_priority_custom() {
    let cli = Cli {
        url: Some("https://www.bilibili.com/video/BV1xx411c7mD".to_string()),
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
        use_tv_api: false,
        use_app_api: false,
        use_intl_api: false,
        download_danmaku: false,
        danmaku_format: "ass".to_string(),
        login_qrcode: false,
        login_tv: false,
        use_aria2c: false,
        aria2c_path: None,
        aria2c_args: None,
    };

    let quality = cli.parse_quality_priority();
    assert_eq!(quality, vec!["4K 超清", "1080P 高清", "720P 高清"]);
}

#[test]
fn test_parse_codec_priority_default() {
    let cli = Cli {
        url: Some("https://www.bilibili.com/video/BV1xx411c7mD".to_string()),
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
        use_tv_api: false,
        use_app_api: false,
        use_intl_api: false,
        download_danmaku: false,
        danmaku_format: "ass".to_string(),
        login_qrcode: false,
        login_tv: false,
        use_aria2c: false,
        aria2c_path: None,
        aria2c_args: None,
    };

    let codec = cli.parse_codec_priority();
    assert_eq!(codec, vec!["avc", "hevc", "av1"]);
}

#[test]
fn test_parse_codec_priority_custom() {
    let cli = Cli {
        url: Some("https://www.bilibili.com/video/BV1xx411c7mD".to_string()),
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
        use_tv_api: false,
        use_app_api: false,
        use_intl_api: false,
        download_danmaku: false,
        danmaku_format: "ass".to_string(),
        login_qrcode: false,
        login_tv: false,
        use_aria2c: false,
        aria2c_path: None,
        aria2c_args: None,
    };

    let codec = cli.parse_codec_priority();
    assert_eq!(codec, vec!["hevc", "av1", "avc"]);
}

#[test]
fn test_parse_pages_none() {
    let cli = Cli {
        url: Some("https://www.bilibili.com/video/BV1xx411c7mD".to_string()),
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
        use_tv_api: false,
        use_app_api: false,
        use_intl_api: false,
        download_danmaku: false,
        danmaku_format: "ass".to_string(),
        login_qrcode: false,
        login_tv: false,
        use_aria2c: false,
        aria2c_path: None,
        aria2c_args: None,
    };

    let pages = cli.parse_pages();
    assert!(pages.is_none());
}

#[test]
fn test_parse_pages_all() {
    let cli = Cli {
        url: Some("https://www.bilibili.com/video/BV1xx411c7mD".to_string()),
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
        use_tv_api: false,
        use_app_api: false,
        use_intl_api: false,
        download_danmaku: false,
        danmaku_format: "ass".to_string(),
        login_qrcode: false,
        login_tv: false,
        use_aria2c: false,
        aria2c_path: None,
        aria2c_args: None,
    };

    let pages = cli.parse_pages();
    assert!(pages.is_none());
}

#[test]
fn test_parse_pages_single() {
    let cli = Cli {
        url: Some("https://www.bilibili.com/video/BV1xx411c7mD".to_string()),
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
        use_tv_api: false,
        use_app_api: false,
        use_intl_api: false,
        download_danmaku: false,
        danmaku_format: "ass".to_string(),
        login_qrcode: false,
        login_tv: false,
        use_aria2c: false,
        aria2c_path: None,
        aria2c_args: None,
    };

    let pages = cli.parse_pages();
    assert_eq!(pages, Some(vec![5]));
}

#[test]
fn test_parse_pages_multiple() {
    let cli = Cli {
        url: Some("https://www.bilibili.com/video/BV1xx411c7mD".to_string()),
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
        use_tv_api: false,
        use_app_api: false,
        use_intl_api: false,
        download_danmaku: false,
        danmaku_format: "ass".to_string(),
        login_qrcode: false,
        login_tv: false,
        use_aria2c: false,
        aria2c_path: None,
        aria2c_args: None,
    };

    let pages = cli.parse_pages();
    assert_eq!(pages, Some(vec![1, 3, 5]));
}

#[test]
fn test_parse_pages_range() {
    let cli = Cli {
        url: Some("https://www.bilibili.com/video/BV1xx411c7mD".to_string()),
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
        use_tv_api: false,
        use_app_api: false,
        use_intl_api: false,
        download_danmaku: false,
        danmaku_format: "ass".to_string(),
        login_qrcode: false,
        login_tv: false,
        use_aria2c: false,
        aria2c_path: None,
        aria2c_args: None,
    };

    let pages = cli.parse_pages();
    assert_eq!(pages, Some(vec![3, 4, 5, 6, 7]));
}

#[test]
fn test_parse_pages_mixed() {
    let cli = Cli {
        url: Some("https://www.bilibili.com/video/BV1xx411c7mD".to_string()),
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
        use_tv_api: false,
        use_app_api: false,
        use_intl_api: false,
        download_danmaku: false,
        danmaku_format: "ass".to_string(),
        login_qrcode: false,
        login_tv: false,
        use_aria2c: false,
        aria2c_path: None,
        aria2c_args: None,
    };

    let pages = cli.parse_pages();
    assert_eq!(pages, Some(vec![1, 3, 4, 5, 8]));
}

// 新增：API模式标志测试
#[test]
fn test_cli_api_mode_tv() {
    let cli = Cli {
        url: Some("https://www.bilibili.com/video/BV1xx".to_string()),
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
        use_tv_api: true,
        use_app_api: false,
        use_intl_api: false,
        download_danmaku: false,
        danmaku_format: "ass".to_string(),
        login_qrcode: false,
        login_tv: false,
        use_aria2c: false,
        aria2c_path: None,
        aria2c_args: None,
    };
    
    assert!(cli.use_tv_api);
    let api_mode = cli.get_api_mode();
    assert!(matches!(api_mode, rvd::platform::bilibili::ApiMode::TV));
}

#[test]
fn test_cli_api_mode_app() {
    let cli = Cli {
        url: Some("https://www.bilibili.com/video/BV1xx".to_string()),
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
        use_tv_api: false,
        use_app_api: true,
        use_intl_api: false,
        download_danmaku: false,
        danmaku_format: "ass".to_string(),
        login_qrcode: false,
        login_tv: false,
        use_aria2c: false,
        aria2c_path: None,
        aria2c_args: None,
    };
    
    assert!(cli.use_app_api);
    let api_mode = cli.get_api_mode();
    assert!(matches!(api_mode, rvd::platform::bilibili::ApiMode::App));
}

#[test]
fn test_cli_api_mode_international() {
    let cli = Cli {
        url: Some("https://www.bilibili.com/video/BV1xx".to_string()),
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
        use_tv_api: false,
        use_app_api: false,
        use_intl_api: true,
        download_danmaku: false,
        danmaku_format: "ass".to_string(),
        login_qrcode: false,
        login_tv: false,
        use_aria2c: false,
        aria2c_path: None,
        aria2c_args: None,
    };
    
    assert!(cli.use_intl_api);
    let api_mode = cli.get_api_mode();
    assert!(matches!(api_mode, rvd::platform::bilibili::ApiMode::International));
}

// 弹幕标志测试
#[test]
fn test_cli_danmaku_download() {
    let cli = Cli {
        url: Some("https://www.bilibili.com/video/BV1xx".to_string()),
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
        use_tv_api: false,
        use_app_api: false,
        use_intl_api: false,
        download_danmaku: true,
        danmaku_format: "ass".to_string(),
        login_qrcode: false,
        login_tv: false,
        use_aria2c: false,
        aria2c_path: None,
        aria2c_args: None,
    };
    
    assert!(cli.download_danmaku);
}

#[test]
fn test_cli_danmaku_format_xml() {
    let cli = Cli {
        url: Some("https://www.bilibili.com/video/BV1xx".to_string()),
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
        use_tv_api: false,
        use_app_api: false,
        use_intl_api: false,
        download_danmaku: true,
        danmaku_format: "xml".to_string(),
        login_qrcode: false,
        login_tv: false,
        use_aria2c: false,
        aria2c_path: None,
        aria2c_args: None,
    };
    
    assert_eq!(cli.danmaku_format, "xml");
    let format = cli.get_danmaku_format();
    assert!(matches!(format, rvd::core::danmaku::DanmakuFormat::Xml));
}

#[test]
fn test_cli_danmaku_format_ass() {
    let cli = Cli {
        url: Some("https://www.bilibili.com/video/BV1xx".to_string()),
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
        use_tv_api: false,
        use_app_api: false,
        use_intl_api: false,
        download_danmaku: true,
        danmaku_format: "ass".to_string(),
        login_qrcode: false,
        login_tv: false,
        use_aria2c: false,
        aria2c_path: None,
        aria2c_args: None,
    };
    
    assert_eq!(cli.danmaku_format, "ass");
    let format = cli.get_danmaku_format();
    assert!(matches!(format, rvd::core::danmaku::DanmakuFormat::Ass));
}

// 交互式模式测试
#[test]
fn test_cli_interactive_flag() {
    let cli = Cli {
        url: Some("https://www.bilibili.com/video/BV1xx".to_string()),
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
        interactive: true,
        config_file: None,
        verbose: false,
        info_only: false,
        ffmpeg_path: None,
        use_tv_api: false,
        use_app_api: false,
        use_intl_api: false,
        download_danmaku: false,
        danmaku_format: "ass".to_string(),
        login_qrcode: false,
        login_tv: false,
        use_aria2c: false,
        aria2c_path: None,
        aria2c_args: None,
    };
    
    assert!(cli.interactive);
}
