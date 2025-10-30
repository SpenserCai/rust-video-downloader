// 章节模块单元测试
use rvd::types::Chapter;

#[test]
fn test_chapter_struct() {
    let chapter = Chapter {
        title: "片头".to_string(),
        start: 0,
        end: 90,
    };
    
    assert_eq!(chapter.title, "片头");
    assert_eq!(chapter.start, 0);
    assert_eq!(chapter.end, 90);
}

#[test]
fn test_chapter_clone() {
    let chapter1 = Chapter {
        title: "正片".to_string(),
        start: 90,
        end: 1350,
    };
    
    let chapter2 = chapter1.clone();
    
    assert_eq!(chapter1.title, chapter2.title);
    assert_eq!(chapter1.start, chapter2.start);
    assert_eq!(chapter1.end, chapter2.end);
}

#[test]
fn test_chapter_debug() {
    let chapter = Chapter {
        title: "片尾".to_string(),
        start: 1350,
        end: 1440,
    };
    
    let debug_str = format!("{:?}", chapter);
    
    assert!(debug_str.contains("片尾"));
    assert!(debug_str.contains("1350"));
    assert!(debug_str.contains("1440"));
}

#[test]
fn test_multiple_chapters() {
    let chapters = vec![
        Chapter {
            title: "片头".to_string(),
            start: 0,
            end: 90,
        },
        Chapter {
            title: "正片".to_string(),
            start: 90,
            end: 1350,
        },
        Chapter {
            title: "片尾".to_string(),
            start: 1350,
            end: 1440,
        },
    ];
    
    assert_eq!(chapters.len(), 3);
    assert_eq!(chapters[0].title, "片头");
    assert_eq!(chapters[1].start, 90);
    assert_eq!(chapters[2].end, 1440);
}

#[test]
fn test_chapter_ordering() {
    let chapter1 = Chapter {
        title: "第一章".to_string(),
        start: 0,
        end: 100,
    };
    
    let chapter2 = Chapter {
        title: "第二章".to_string(),
        start: 100,
        end: 200,
    };
    
    // 验证章节顺序
    assert!(chapter1.end == chapter2.start);
    assert!(chapter1.start < chapter2.start);
    assert!(chapter1.end < chapter2.end);
}

#[test]
fn test_chapter_duration() {
    let chapter = Chapter {
        title: "测试章节".to_string(),
        start: 100,
        end: 250,
    };
    
    let duration = chapter.end - chapter.start;
    assert_eq!(duration, 150);
}
