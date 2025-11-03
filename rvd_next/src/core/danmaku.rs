use crate::error::Result;
use crate::utils::http::HttpClient;
use std::path::Path;
use std::sync::Arc;

/// 弹幕格式
#[derive(Debug, Clone, Copy)]
pub enum DanmakuFormat {
    Xml,
    Ass,
}

/// 下载弹幕
pub async fn download_danmaku(
    _client: &Arc<HttpClient>,
    cid: &str,
    output: &Path,
    format: DanmakuFormat,
) -> Result<()> {
    // 下载 XML 格式弹幕
    let api = format!("https://comment.bilibili.com/{}.xml", cid);
    tracing::debug!("Fetching danmaku from: {}", api);

    // Create a client with automatic decompression disabled
    // We need to manually decompress because Bilibili's deflate encoding causes issues with reqwest
    let raw_client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .timeout(std::time::Duration::from_secs(60))
        .no_gzip()
        .no_deflate()
        .no_brotli()
        .build()
        .map_err(crate::error::DownloaderError::Network)?;

    let response = raw_client.get(&api).send().await?;

    if !response.status().is_success() {
        return Err(crate::error::DownloaderError::DownloadFailed(format!(
            "Failed to fetch danmaku: HTTP {}",
            response.status()
        )));
    }

    tracing::debug!("Response status: {}", response.status());

    let bytes = response.bytes().await?;

    // Try to decode as UTF-8 directly first, or decompress if needed
    let xml_content = match String::from_utf8(bytes.to_vec()) {
        Ok(text) => text,
        Err(_) => {
            // Try deflate decompression (most common for Bilibili danmaku API)
            use flate2::read::DeflateDecoder;
            use std::io::Read;

            let mut decoder = DeflateDecoder::new(&bytes[..]);
            let mut decompressed = String::new();
            match decoder.read_to_string(&mut decompressed) {
                Ok(_) => {
                    tracing::debug!("Decompressed danmaku with deflate");
                    decompressed
                }
                Err(_) => {
                    // Try gzip as fallback
                    use flate2::read::GzDecoder;
                    let mut decoder = GzDecoder::new(&bytes[..]);
                    let mut decompressed = String::new();
                    decoder.read_to_string(&mut decompressed).map_err(|e| {
                        crate::error::DownloaderError::DownloadFailed(format!(
                            "Failed to decompress danmaku: {}",
                            e
                        ))
                    })?;
                    tracing::debug!("Decompressed danmaku with gzip");
                    decompressed
                }
            }
        }
    };

    if xml_content.is_empty() || !xml_content.contains("<d ") {
        tracing::info!("No danmaku available for cid: {}", cid);
        return Ok(());
    }

    match format {
        DanmakuFormat::Xml => {
            // 格式化并保存 XML
            let formatted_xml = format_xml(&xml_content)?;
            tokio::fs::write(output, formatted_xml).await?;
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

/// 格式化 XML 弹幕
fn format_xml(xml: &str) -> Result<String> {
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

/// 解析 XML 弹幕
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

/// 格式化时间为 ASS 格式 (H:MM:SS.CC)
fn format_ass_time(seconds: f64) -> String {
    let hours = (seconds / 3600.0) as u32;
    let minutes = ((seconds % 3600.0) / 60.0) as u32;
    let secs = (seconds % 60.0) as u32;
    let centisecs = ((seconds % 1.0) * 100.0) as u32;

    format!("{}:{:02}:{:02}.{:02}", hours, minutes, secs, centisecs)
}

/// 弹幕项
#[derive(Debug, Clone)]
struct DanmakuItem {
    time: f64,
    mode: u32,
    #[allow(dead_code)] // 保留用于未来扩展
    font_size: u32,
    color: u32,
    text: String,
}
