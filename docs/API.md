# API 文档

RVD Next 不仅是一个命令行工具，也是一个可以作为库使用的 Rust crate。本文档介绍如何在你的项目中使用 RVD Next。

## 添加依赖

在你的 `Cargo.toml` 中添加：

```toml
[dependencies]
rvd = "1.0"
tokio = { version = "1.35", features = ["full"] }
```

## 快速开始

### 示例 1: 下载单个视频

```rust
use rvd::app::Orchestrator;
use rvd::cli::Cli;
use rvd::utils::config::Config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建配置
    let config = Config::default();
    
    // 创建 CLI 参数
    let cli = Cli {
        url: Some("https://www.bilibili.com/video/BV1xx411c7mD".to_string()),
        output: Some("output.mp4".to_string()),
        ..Default::default()
    };
    
    // 创建编排器并运行
    let orchestrator = Orchestrator::new(config, &cli)?;
    orchestrator.run(cli).await?;
    
    Ok(())
}
```

### 示例 2: 获取视频信息

```rust
use rvd::platform::bilibili::BilibiliPlatform;
use rvd::platform::Platform;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建 Bilibili 平台实例
    let platform = BilibiliPlatform::new(
        rvd::platform::bilibili::ApiMode::Web
    )?;
    
    // 解析视频信息
    let url = "https://www.bilibili.com/video/BV1xx411c7mD";
    let video_info = platform.parse_video(url, None).await?;
    
    println!("标题: {}", video_info.title);
    println!("UP主: {}", video_info.uploader);
    println!("分P数: {}", video_info.pages.len());
    
    Ok(())
}
```

### 示例 3: 自定义下载流程

```rust
use rvd::core::downloader::Downloader;
use rvd::platform::bilibili::BilibiliPlatform;
use rvd::platform::Platform;
use rvd::types::StreamPreferences;
use rvd::utils::http::HttpClient;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建 HTTP 客户端
    let client = Arc::new(HttpClient::new()?);
    
    // 创建平台实例
    let platform = BilibiliPlatform::new(
        rvd::platform::bilibili::ApiMode::Web
    )?;
    
    // 解析视频
    let url = "https://www.bilibili.com/video/BV1xx411c7mD";
    let video_info = platform.parse_video(url, None).await?;
    
    // 获取第一个分P
    let page = &video_info.pages[0];
    let context = page.to_stream_context(&video_info.aid.to_string());
    
    // 获取流
    let streams = platform.get_streams(&context, None).await?;
    
    // 选择最佳流
    let preferences = StreamPreferences {
        quality_priority: vec!["1080P".to_string()],
        codec_priority: vec!["HEVC".to_string()],
    };
    let (video_stream, audio_stream) = platform.select_best_streams(&streams, &preferences)?;
    
    // 下载
    let downloader = Downloader::new(client, 8);
    downloader.download(&video_stream.url, "video.m4s", None, None).await?;
    downloader.download(&audio_stream.url, "audio.m4s", None, None).await?;
    
    println!("下载完成！");
    
    Ok(())
}
```

## 核心 API

### Platform Trait

所有平台都实现了 `Platform` trait：

```rust
pub trait Platform: Send + Sync {
    // 获取平台元数据
    fn metadata(&self) -> &PlatformMetadata;
    
    // 检查是否可以处理 URL
    fn can_handle(&self, url: &str) -> bool;
    
    // 解析视频信息
    async fn parse_video(&self, url: &str, auth: Option<&Auth>) -> Result<VideoInfo>;
    
    // 获取流
    async fn get_streams(&self, context: &StreamContext, auth: Option<&Auth>) -> Result<Vec<Stream>>;
    
    // 获取字幕
    async fn get_subtitles(&self, context: &StreamContext) -> Result<Vec<Subtitle>>;
    
    // 获取弹幕
    async fn get_danmaku(&self, context: &StreamContext, format: DanmakuFormat) -> Result<Option<String>>;
    
    // ... 更多方法
}
```

### 主要类型

#### VideoInfo

```rust
pub struct VideoInfo {
    pub id: String,           // 视频 ID
    pub aid: String,          // AV 号
    pub bvid: String,         // BV 号
    pub title: String,        // 标题
    pub uploader: String,     // UP主
    pub description: String,  // 描述
    pub cover_url: String,    // 封面 URL
    pub pages: Vec<Page>,     // 分P列表
    pub extra_data: Option<HashMap<String, serde_json::Value>>,
}
```

#### Stream

```rust
pub struct Stream {
    pub stream_type: StreamType,  // Video 或 Audio
    pub quality: String,           // 清晰度
    pub quality_id: u32,           // 清晰度 ID
    pub codec: String,             // 编码格式
    pub bandwidth: u64,            // 带宽
    pub url: String,               // 下载 URL
    pub extra_data: Option<HashMap<String, serde_json::Value>>,
}
```

#### Auth

```rust
pub struct Auth {
    pub cookie: Option<String>,
    pub access_token: Option<String>,
    pub extra: HashMap<String, String>,
}
```

## 平台 API

### Bilibili Platform

```rust
use rvd::platform::bilibili::{BilibiliPlatform, ApiMode};

// 创建平台实例
let platform = BilibiliPlatform::new(ApiMode::Web)?;

// 使用不同的 API 模式
let platform = BilibiliPlatform::new(ApiMode::TV)?;

// 配置 CDN 优化
let platform = platform.with_cdn_config(vec![
    "upos-sz-mirrorcos.bilivideo.com".to_string(),
]);
```

### Platform Registry

```rust
use rvd::app::PlatformRegistry;
use std::sync::Arc;

// 创建注册表
let mut registry = PlatformRegistry::new();

// 注册平台
let bilibili = Arc::new(BilibiliPlatform::new(ApiMode::Web)?);
registry.register(bilibili);

// 选择平台
let platform = registry.select_platform("https://www.bilibili.com/video/BV1xx411c7mD")?;
```

## 核心模块 API

### Downloader

```rust
use rvd::core::downloader::{Downloader, DownloadMethod};
use rvd::utils::http::HttpClient;
use std::sync::Arc;

// 创建下载器
let client = Arc::new(HttpClient::new()?);
let downloader = Downloader::new(client, 8);  // 8 个线程

// 使用 Aria2c
let downloader = downloader.with_method(DownloadMethod::Aria2c);

// 下载文件
downloader.download(
    "https://example.com/video.mp4",
    "output.mp4",
    None,  // 自定义请求头
    None,  // 进度条
).await?;
```

### Muxer

```rust
use rvd::core::muxer::Muxer;

// 创建混流器
let muxer = Muxer::new()?;

// 混流
muxer.mux(
    "video.m4s",
    "audio.m4s",
    "output.mp4",
).await?;

// 带字幕和章节
muxer.mux_with_options(
    "video.m4s",
    "audio.m4s",
    "output.mp4",
    &["subtitle.srt"],
    &chapters,
    false,  // 是否为杜比视界
).await?;
```

### Progress Tracker

```rust
use rvd::core::progress::ProgressTracker;

// 创建进度跟踪器
let tracker = ProgressTracker::new();

// 创建进度条
let pb = tracker.create_bar("Video", 1024 * 1024);  // 1MB

// 更新进度
pb.set_position(512 * 1024);  // 512KB

// 完成
tracker.finish("Video", "✓ Downloaded");
```

## 工具 API

### HTTP Client

```rust
use rvd::utils::http::HttpClient;

// 创建客户端
let client = HttpClient::new()?;

// 自定义 User-Agent
let client = HttpClient::with_custom_user_agent("My UA".to_string())?;

// 发送请求
let response = client.get("https://api.example.com/data").send().await?;
let data: MyData = response.json().await?;
```

### Config

```rust
use rvd::utils::config::Config;

// 加载配置
let config = Config::load_from_file("config.toml")?;

// 使用默认配置
let config = Config::default();

// 访问配置项
if let Some(ref http_config) = config.http {
    println!("User-Agent: {:?}", http_config.user_agent);
}
```

## 错误处理

```rust
use rvd::error::{DownloaderError, Result};

fn my_function() -> Result<()> {
    // 使用 ? 操作符传播错误
    let video_info = platform.parse_video(url, None).await?;
    
    // 手动创建错误
    if video_info.pages.is_empty() {
        return Err(DownloaderError::Parse("No pages found".to_string()));
    }
    
    Ok(())
}

// 错误类型
pub enum DownloaderError {
    Parse(String),
    Network(String),
    DownloadFailed(String),
    MuxFailed(String),
    AuthRequired,
    FeatureNotSupported { platform: String, feature: String },
    // ...
}
```

## 高级用法

### 实现自定义平台

```rust
use rvd::platform::{Platform, PlatformMetadata, PlatformCapabilities};
use rvd::types::{Auth, VideoInfo, Stream, StreamContext};
use rvd::error::Result;
use async_trait::async_trait;

pub struct MyPlatform {
    metadata: PlatformMetadata,
}

#[async_trait]
impl Platform for MyPlatform {
    fn metadata(&self) -> &PlatformMetadata {
        &self.metadata
    }
    
    fn can_handle(&self, url: &str) -> bool {
        url.contains("myplatform.com")
    }
    
    async fn parse_video(&self, url: &str, auth: Option<&Auth>) -> Result<VideoInfo> {
        // 实现视频解析
        todo!()
    }
    
    async fn get_streams(&self, context: &StreamContext, auth: Option<&Auth>) -> Result<Vec<Stream>> {
        // 实现流获取
        todo!()
    }
    
    fn get_cover(&self, video_info: &VideoInfo) -> String {
        video_info.cover_url.clone()
    }
    
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
```

### 自定义认证提供者

```rust
use rvd::auth::AuthProvider;
use rvd::types::Auth;
use rvd::error::Result;
use async_trait::async_trait;

pub struct MyAuthProvider;

#[async_trait]
impl AuthProvider for MyAuthProvider {
    async fn login(&self) -> Result<Auth> {
        // 实现登录逻辑
        todo!()
    }
    
    fn platform_name(&self) -> &str {
        "myplatform"
    }
}
```

## 完整示例

查看 `examples/` 目录获取更多完整示例：

- `examples/basic_download.rs` - 基础下载
- `examples/batch_download.rs` - 批量下载
- `examples/custom_platform.rs` - 自定义平台
- `examples/advanced_usage.rs` - 高级用法

## API 文档

完整的 API 文档可以通过以下方式生成：

```bash
cargo doc --open
```

## 更多资源

- [用户指南](USER_GUIDE.md) - 命令行使用说明
- [开发指南](DEVELOPMENT.md) - 如何添加新平台
- [架构设计](ARCHITECTURE.md) - 架构详解
- [配置文件](CONFIGURATION.md) - 配置选项

## 获取帮助

- GitHub Issues: https://github.com/SpenserCai/rust-video-downloader/issues
- API 文档: https://docs.rs/rvd
- 示例代码: https://github.com/SpenserCai/rust-video-downloader/tree/main/examples
