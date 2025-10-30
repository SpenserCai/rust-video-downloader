// Bilibili平台模块单元测试
use rvd::platform::bilibili::selector::select_best_streams;
use rvd::platform::bilibili::BilibiliPlatform;
use rvd::platform::Platform;
use rvd::types::{Stream, StreamPreferences, StreamType};

#[test]
fn test_can_handle_bilibili_urls() {
    let platform = BilibiliPlatform::new().unwrap();

    // 测试各种B站URL格式
    assert!(platform.can_handle("https://www.bilibili.com/video/BV1xx411c7mD"));
    assert!(platform.can_handle("https://www.bilibili.com/video/av170001"));
    assert!(platform.can_handle("https://www.bilibili.com/bangumi/play/ep123456"));
    assert!(platform.can_handle("https://www.bilibili.com/bangumi/play/ss12345"));
    assert!(platform.can_handle("https://b23.tv/abc123"));
    assert!(platform.can_handle("BV1xx411c7mD"));
    assert!(platform.can_handle("av170001"));
    assert!(platform.can_handle("ep123456"));
    assert!(platform.can_handle("ss12345"));

    // 测试非B站URL
    assert!(!platform.can_handle("https://www.youtube.com/watch?v=abc"));
    assert!(!platform.can_handle("https://www.example.com"));
}

#[test]
fn test_select_best_streams_by_quality() {
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
            codec: "AVC".to_string(),
            url: "https://example.com/720p_avc.m4s".to_string(),
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

    let preferences = StreamPreferences {
        quality_priority: vec!["1080P 高清".to_string(), "720P 高清".to_string()],
        codec_priority: vec!["avc".to_string(), "hevc".to_string()],
    };

    let result = select_best_streams(&streams, &preferences);
    assert!(result.is_ok());

    let (video, audio) = result.unwrap();
    assert_eq!(video.quality, "1080P 高清");
    assert_eq!(audio.codec, "M4A");
}

#[test]
fn test_select_best_streams_by_codec() {
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
            quality: "1080P 高清".to_string(),
            quality_id: 80,
            codec: "HEVC".to_string(),
            url: "https://example.com/1080p_hevc.m4s".to_string(),
            size: 0,
            bandwidth: 2500000,
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

    let preferences = StreamPreferences {
        quality_priority: vec!["1080P 高清".to_string()],
        codec_priority: vec!["hevc".to_string(), "avc".to_string()],
    };

    let result = select_best_streams(&streams, &preferences);
    assert!(result.is_ok());

    let (video, _audio) = result.unwrap();
    assert_eq!(video.codec, "HEVC");
}

#[test]
fn test_select_best_streams_fallback() {
    let streams = vec![
        Stream {
            stream_type: StreamType::Video,
            quality: "480P 清晰".to_string(),
            quality_id: 32,
            codec: "AVC".to_string(),
            url: "https://example.com/480p_avc.m4s".to_string(),
            size: 0,
            bandwidth: 1000000,
        },
        Stream {
            stream_type: StreamType::Audio,
            quality: "128kbps".to_string(),
            quality_id: 30216,
            codec: "M4A".to_string(),
            url: "https://example.com/audio.m4s".to_string(),
            size: 0,
            bandwidth: 128000,
        },
    ];

    // 请求不存在的清晰度，应该降级到可用的
    let preferences = StreamPreferences {
        quality_priority: vec!["4K 超清".to_string(), "1080P 高清".to_string()],
        codec_priority: vec!["hevc".to_string(), "avc".to_string()],
    };

    let result = select_best_streams(&streams, &preferences);
    assert!(result.is_ok());

    let (video, _audio) = result.unwrap();
    assert_eq!(video.quality, "480P 清晰");
}

#[test]
fn test_select_best_streams_no_video() {
    let streams = vec![Stream {
        stream_type: StreamType::Audio,
        quality: "192kbps".to_string(),
        quality_id: 30280,
        codec: "M4A".to_string(),
        url: "https://example.com/audio.m4s".to_string(),
        size: 0,
        bandwidth: 192000,
    }];

    let preferences = StreamPreferences::default();

    let result = select_best_streams(&streams, &preferences);
    assert!(result.is_err());
}

#[test]
fn test_select_best_streams_no_audio() {
    let streams = vec![Stream {
        stream_type: StreamType::Video,
        quality: "1080P 高清".to_string(),
        quality_id: 80,
        codec: "AVC".to_string(),
        url: "https://example.com/1080p_avc.m4s".to_string(),
        size: 0,
        bandwidth: 3000000,
    }];

    let preferences = StreamPreferences::default();

    let result = select_best_streams(&streams, &preferences);
    assert!(result.is_err());
}

// 新增：URL解析扩展测试
#[test]
fn test_can_handle_bangumi_urls() {
    let platform = BilibiliPlatform::new().unwrap();
    
    // 测试番剧ep链接
    assert!(platform.can_handle("https://www.bilibili.com/bangumi/play/ep123456"));
    assert!(platform.can_handle("ep123456"));
    
    // 测试番剧ss链接
    assert!(platform.can_handle("https://www.bilibili.com/bangumi/play/ss12345"));
    assert!(platform.can_handle("ss12345"));
}

#[test]
fn test_can_handle_cheese_urls() {
    let platform = BilibiliPlatform::new().unwrap();
    
    assert!(platform.can_handle("https://www.bilibili.com/cheese/play/ep123456"));
}

#[test]
fn test_can_handle_favorite_list_urls() {
    let platform = BilibiliPlatform::new().unwrap();
    
    assert!(platform.can_handle("https://space.bilibili.com/123456/favlist?fid=789"));
}

#[test]
fn test_can_handle_space_video_urls() {
    let platform = BilibiliPlatform::new().unwrap();
    
    assert!(platform.can_handle("https://space.bilibili.com/123456"));
}

#[test]
fn test_can_handle_media_list_urls() {
    let platform = BilibiliPlatform::new().unwrap();
    
    assert!(platform.can_handle("https://www.bilibili.com/medialist/play/ml123456"));
}

#[test]
fn test_can_handle_series_list_urls() {
    let platform = BilibiliPlatform::new().unwrap();
    
    assert!(platform.can_handle("https://space.bilibili.com/123456/channel/seriesdetail?sid=789"));
}

// API模式测试
#[test]
fn test_api_mode_web() {
    use rvd::platform::bilibili::ApiMode;
    let platform = BilibiliPlatform::with_api_mode(ApiMode::Web).unwrap();
    // 验证平台创建成功
    assert!(platform.can_handle("https://www.bilibili.com/video/BV1xx411c7mD"));
}

#[test]
fn test_api_mode_tv() {
    use rvd::platform::bilibili::ApiMode;
    let platform = BilibiliPlatform::with_api_mode(ApiMode::TV).unwrap();
    // 验证平台创建成功
    assert!(platform.can_handle("https://www.bilibili.com/video/BV1xx411c7mD"));
}

#[test]
fn test_api_mode_app() {
    use rvd::platform::bilibili::ApiMode;
    let platform = BilibiliPlatform::with_api_mode(ApiMode::App).unwrap();
    // 验证平台创建成功
    assert!(platform.can_handle("https://www.bilibili.com/video/BV1xx411c7mD"));
}

#[test]
fn test_api_mode_international() {
    use rvd::platform::bilibili::ApiMode;
    let platform = BilibiliPlatform::with_api_mode(ApiMode::International).unwrap();
    // 验证平台创建成功
    assert!(platform.can_handle("https://www.bilibili.com/video/BV1xx411c7mD"));
}
