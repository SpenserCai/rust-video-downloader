// 弹幕模块单元测试
use rvd::core::danmaku::DanmakuFormat;

#[test]
fn test_danmaku_format_enum() {
    let xml_format = DanmakuFormat::Xml;
    let ass_format = DanmakuFormat::Ass;
    
    assert!(matches!(xml_format, DanmakuFormat::Xml));
    assert!(matches!(ass_format, DanmakuFormat::Ass));
}

#[test]
fn test_danmaku_format_copy() {
    let format1 = DanmakuFormat::Xml;
    let format2 = format1; // 测试Copy trait
    
    assert!(matches!(format1, DanmakuFormat::Xml));
    assert!(matches!(format2, DanmakuFormat::Xml));
}

#[test]
fn test_danmaku_format_clone() {
    let format1 = DanmakuFormat::Ass;
    let format2 = format1.clone(); // 测试Clone trait
    
    assert!(matches!(format1, DanmakuFormat::Ass));
    assert!(matches!(format2, DanmakuFormat::Ass));
}

#[test]
fn test_danmaku_format_debug() {
    let xml_format = DanmakuFormat::Xml;
    let debug_str = format!("{:?}", xml_format);
    
    assert!(debug_str.contains("Xml"));
}
