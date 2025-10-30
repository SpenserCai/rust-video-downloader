use crate::error::Result;
use crate::utils::http::HttpClient;
use std::path::Path;
use std::sync::Arc;

/// 弹幕格式
#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub enum DanmakuFormat {
    Xml,
    Ass,
}

/// 下载弹幕
#[allow(dead_code)]
pub async fn download_danmaku(
    client: &Arc<HttpClient>,
    cid: &str,
    output: &Path,
    format: DanmakuFormat,
) -> Result<()> {
    // 下载 XML 格式弹幕
    let api = format!("https://comment.bilibili.com/{}.xml", cid);
    let response = client.get(&api, None).await?;
    let xml_content = response.text().await?;

    if xml_content.is_empty() || !xml_content.contains("<d ") {
        tracing::info!("No danmaku available for cid: {}", cid);
        return Ok(());
    }

    match format {
        DanmakuFormat::Xml => {
            // 直接保存 XML
            tokio::fs::write(output, xml_content).await?;
            tracing::info!("Danmaku saved to: {:?}", output);
        }
        DanmakuFormat::Ass => {
            // 转换为 ASS 格式
            let ass_content = convert_xml_to_ass(&xml_content)?;
            tokio::fs::write(output, ass_content).await?;
            tracing::info!("Danmaku converted to ASS and saved to: {:?}", output);
        }
    }

    Ok(())
}

/// 解析 XML 弹幕
#[allow(dead_code)]
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

/// 转换 XML 弹幕为 ASS 格式
#[allow(dead_code)]
fn convert_xml_to_ass(xml: &str) -> Result<String> {
    let items = parse_danmaku_xml(xml)?;

    let mut ass = String::new();

    // ASS 文件头
    ass.push_str("[Script Info]\n");
    ass.push_str("Title: Bilibili Danmaku\n");
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
    ass.push_str("Format: Layer, Start, End, Style, Name, MarginL, MarginR, MarginV, Effect, Text\n");

    for item in items {
        let start_time = format_ass_time(item.time);
        let end_time = format_ass_time(item.time + 5.0); // 弹幕显示5秒

        let color = format!("&H{:06X}&", item.color);
        let text = item.text.replace('\n', "\\N");

        // 根据弹幕模式设置位置
        let position = match item.mode {
            1 => "\\move(1920,540,0,540)", // 滚动弹幕
            4 => "\\pos(960,100)",          // 底部弹幕
            5 => "\\pos(960,980)",          // 顶部弹幕
            _ => "\\move(1920,540,0,540)",
        };

        ass.push_str(&format!(
            "Dialogue: 0,{},{},Default,,0,0,0,,{{{}\\c{}}}{}\\N\n",
            start_time, end_time, position, color, text
        ));
    }

    Ok(ass)
}

/// 格式化时间为 ASS 格式 (H:MM:SS.CC)
#[allow(dead_code)]
fn format_ass_time(seconds: f64) -> String {
    let hours = (seconds / 3600.0) as u32;
    let minutes = ((seconds % 3600.0) / 60.0) as u32;
    let secs = (seconds % 60.0) as u32;
    let centisecs = ((seconds % 1.0) * 100.0) as u32;

    format!("{}:{:02}:{:02}.{:02}", hours, minutes, secs, centisecs)
}

/// 弹幕项
#[allow(dead_code)]
#[derive(Debug, Clone)]
struct DanmakuItem {
    time: f64,
    mode: u32,
    #[allow(dead_code)]
    font_size: u32,
    color: u32,
    text: String,
}
