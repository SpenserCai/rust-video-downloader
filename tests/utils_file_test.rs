// 文件工具模块单元测试
use rvd::types::{Page, VideoInfo};
use rvd::utils::file::{get_default_output_path, parse_template, sanitize_filename};
use std::path::PathBuf;

#[test]
fn test_sanitize_filename() {
    // 测试非法字符替换
    assert_eq!(sanitize_filename("test<>:file"), "test___file");
    assert_eq!(sanitize_filename("test/\\file"), "test__file");
    assert_eq!(sanitize_filename("test|?*file"), "test___file");

    // 测试前后空格和点号
    assert_eq!(sanitize_filename("  test  "), "test");
    assert_eq!(sanitize_filename("...test..."), "test");

    // 测试长度限制
    let long_name = "a".repeat(250);
    let sanitized = sanitize_filename(&long_name);
    assert!(sanitized.len() <= 200);

    // 测试空字符串
    assert_eq!(sanitize_filename(""), "video");
}

#[test]
fn test_parse_template_single_page() {
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

    // 测试基本模板
    let result = parse_template("<videoTitle>", &video_info, None, "1080P", "avc");
    assert_eq!(result, "测试视频");

    // 测试多变量模板
    let result = parse_template(
        "<videoTitle>_<quality>_<codec>",
        &video_info,
        None,
        "1080P",
        "hevc",
    );
    assert_eq!(result, "测试视频_1080P_hevc");

    // 测试BVID
    let result = parse_template("<bvid>", &video_info, None, "1080P", "avc");
    assert_eq!(result, "BV1xx411c7mD");

    // 测试UP主
    let result = parse_template("<uploader>", &video_info, None, "1080P", "avc");
    assert_eq!(result, "测试UP主");
}

#[test]
fn test_parse_template_multi_page() {
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
        pages: vec![
            Page {
                number: 1,
                title: "第一集".to_string(),
                cid: "123456".to_string(),
                duration: 300,
                ep_id: None,
            },
            Page {
                number: 2,
                title: "第二集".to_string(),
                cid: "123457".to_string(),
                duration: 300,
                ep_id: None,
            },
        ],
        is_bangumi: false,
        ep_id: None,
    };

    let page = &video_info.pages[0];

    // 测试分P模板
    let result = parse_template(
        "<videoTitle>/P<pageNumber>_<pageTitle>",
        &video_info,
        Some(page),
        "1080P",
        "avc",
    );
    assert_eq!(result, "测试视频/P1_第一集");

    // 测试带前缀零的分P
    let result = parse_template(
        "P<pageNumberWithZero>_<pageTitle>",
        &video_info,
        Some(page),
        "1080P",
        "avc",
    );
    assert_eq!(result, "P01_第一集");

    // 测试CID
    let result = parse_template("<cid>", &video_info, Some(page), "1080P", "avc");
    assert_eq!(result, "123456");
}

#[test]
fn test_get_default_output_path_single_page() {
    let video_info = VideoInfo {
        id: "BV1xx411c7mD".to_string(),
        aid: 170001,
        title: "单P视频".to_string(),
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

    let path = get_default_output_path(&video_info, None);
    assert_eq!(path, PathBuf::from("单P视频.mp4"));
}

#[test]
fn test_get_default_output_path_multi_page() {
    let video_info = VideoInfo {
        id: "BV1xx411c7mD".to_string(),
        aid: 170001,
        title: "多P视频".to_string(),
        description: "描述".to_string(),
        duration: 300,
        uploader: "测试UP主".to_string(),
        uploader_mid: "12345".to_string(),
        upload_date: "2024-01-01".to_string(),
        cover_url: "https://example.com/cover.jpg".to_string(),
        pages: vec![
            Page {
                number: 1,
                title: "第一集".to_string(),
                cid: "123456".to_string(),
                duration: 300,
                ep_id: None,
            },
            Page {
                number: 2,
                title: "第二集".to_string(),
                cid: "123457".to_string(),
                duration: 300,
                ep_id: None,
            },
        ],
        is_bangumi: false,
        ep_id: None,
    };

    let page = &video_info.pages[0];
    let path = get_default_output_path(&video_info, Some(page));
    assert_eq!(path, PathBuf::from("多P视频/P01_第一集.mp4"));
}
