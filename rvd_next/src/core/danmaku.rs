//! Danmaku format conversion utilities
//!
//! This module provides utility functions for converting danmaku (bullet comments)
//! between different formats. It does not contain platform-specific download logic.
//! Platforms should implement their own danmaku download logic and use these utilities
//! for format conversion.

use crate::error::Result;

/// Danmaku format
#[derive(Debug, Clone, Copy)]
pub enum DanmakuFormat {
    Xml,
    Ass,
}

/// Format XML danmaku
///
/// Formats raw XML danmaku into a more readable format.
///
/// # Arguments
///
/// * `xml` - Raw XML danmaku content
///
/// # Returns
///
/// Formatted XML string
pub fn format_xml(xml: &str) -> Result<String> {
    use quick_xml::events::Event;
    use quick_xml::Reader;
    use quick_xml::Writer;
    use std::io::Cursor;

    let mut reader = Reader::from_str(xml);
    reader.trim_text(true);

    let mut writer = Writer::new_with_indent(Cursor::new(Vec::new()), b' ', 2);

    loop {
        match reader.read_event() {
            Ok(Event::Eof) => break,
            Ok(event) => {
                writer.write_event(event).map_err(|e| {
                    crate::error::DownloaderError::Parse(format!(
                        "Failed to write XML event: {}",
                        e
                    ))
                })?;
            }
            Err(e) => {
                return Err(crate::error::DownloaderError::Parse(format!(
                    "Failed to parse XML: {}",
                    e
                )));
            }
        }
    }

    let result = writer.into_inner().into_inner();
    String::from_utf8(result).map_err(|e| {
        crate::error::DownloaderError::Parse(format!("Failed to convert XML to UTF-8: {}", e))
    })
}

/// Parse XML danmaku
///
/// Parses XML danmaku content into structured items.
///
/// # Arguments
///
/// * `xml` - XML danmaku content
///
/// # Returns
///
/// Vector of danmaku items
fn parse_danmaku_xml(xml: &str) -> Result<Vec<DanmakuItem>> {
    let mut items = Vec::new();

    // 简单的 XML 解析（使用正则表达式）
    let re = regex::Regex::new(r#"<d p="([^"]+)">([^<]+)</d>"#).unwrap();

    for cap in re.captures_iter(xml) {
        let params = &cap[1];
        let text = &cap[2];

        let parts: Vec<&str> = params.split(',').collect();
        if parts.len() < 8 {
            continue;
        }

        let time = parts[0].parse::<f64>().unwrap_or(0.0);
        let mode = parts[1].parse::<u32>().unwrap_or(1);
        let font_size = parts[2].parse::<u32>().unwrap_or(25);
        let color = parts[3].parse::<u32>().unwrap_or(0xFFFFFF);

        items.push(DanmakuItem {
            time,
            mode,
            font_size,
            color,
            text: text.to_string(),
        });
    }

    Ok(items)
}

/// Convert XML danmaku to ASS format
///
/// Converts XML format danmaku to ASS subtitle format, which can be displayed
/// in video players.
///
/// # Arguments
///
/// * `xml` - XML danmaku content
///
/// # Returns
///
/// ASS format subtitle string
pub fn convert_xml_to_ass(xml: &str) -> Result<String> {
    let items = parse_danmaku_xml(xml)?;

    let mut ass = String::new();

    // ASS file header
    ass.push_str("[Script Info]\n");
    ass.push_str("Title: Danmaku\n");
    ass.push_str("ScriptType: v4.00+\n");
    ass.push_str("Collisions: Normal\n");
    ass.push_str("PlayResX: 1920\n");
    ass.push_str("PlayResY: 1080\n");
    ass.push('\n');

    // 样式定义
    ass.push_str("[V4+ Styles]\n");
    ass.push_str("Format: Name, Fontname, Fontsize, PrimaryColour, SecondaryColour, OutlineColour, BackColour, Bold, Italic, Underline, StrikeOut, ScaleX, ScaleY, Spacing, Angle, BorderStyle, Outline, Shadow, Alignment, MarginL, MarginR, MarginV, Encoding\n");
    ass.push_str("Style: Default,Arial,36,&H00FFFFFF,&H00FFFFFF,&H00000000,&H00000000,0,0,0,0,100,100,0,0,1,2,0,2,20,20,20,0\n");
    ass.push('\n');

    // 事件
    ass.push_str("[Events]\n");
    ass.push_str(
        "Format: Layer, Start, End, Style, Name, MarginL, MarginR, MarginV, Effect, Text\n",
    );

    for item in items {
        let start_time = format_ass_time(item.time);
        let end_time = format_ass_time(item.time + 5.0); // 弹幕显示5秒

        let color = format!("&H{:06X}&", item.color);
        let text = item.text.replace('\n', "\\N");

        // 根据弹幕模式设置位置
        let position = match item.mode {
            1 => "\\move(1920,540,0,540)", // 滚动弹幕
            4 => "\\pos(960,100)",         // 底部弹幕
            5 => "\\pos(960,980)",         // 顶部弹幕
            _ => "\\move(1920,540,0,540)",
        };

        ass.push_str(&format!(
            "Dialogue: 0,{},{},Default,,0,0,0,,{{{}\\c{}}}{}\\N\n",
            start_time, end_time, position, color, text
        ));
    }

    Ok(ass)
}

/// Format time to ASS format (H:MM:SS.CC)
///
/// Converts seconds to ASS subtitle time format.
///
/// # Arguments
///
/// * `seconds` - Time in seconds
///
/// # Returns
///
/// Formatted time string
fn format_ass_time(seconds: f64) -> String {
    let hours = (seconds / 3600.0) as u32;
    let minutes = ((seconds % 3600.0) / 60.0) as u32;
    let secs = (seconds % 60.0) as u32;
    let centisecs = ((seconds % 1.0) * 100.0) as u32;

    format!("{}:{:02}:{:02}.{:02}", hours, minutes, secs, centisecs)
}

/// Danmaku item
///
/// Represents a single danmaku comment with its properties.
#[derive(Debug, Clone)]
struct DanmakuItem {
    time: f64,
    mode: u32,
    #[allow(dead_code)] // 保留用于未来扩展
    font_size: u32,
    color: u32,
    text: String,
}
