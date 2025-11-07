# 开发指南

本指南面向希望为 RVD Next 贡献代码或添加新平台支持的开发者。

## 目录

- [开发环境设置](#开发环境设置)
- [项目结构](#项目结构)
- [添加新平台](#添加新平台)
- [代码规范](#代码规范)
- [测试](#测试)
- [调试技巧](#调试技巧)
- [贡献流程](#贡献流程)

## 开发环境设置

### 前置要求

- Rust 1.70 或更高版本
- FFmpeg（用于测试混流功能）
- Git

### 克隆仓库

```bash
git clone https://github.com/SpenserCai/rust-video-downloader.git
cd rust-video-downloader/rvd_next
```

### 构建项目

```bash
# 开发构建
cargo build

# 发布构建
cargo build --release
```

### 运行测试

```bash
# 运行所有测试
cargo test

# 运行特定测试
cargo test test_bilibili

# 显示测试输出
cargo test -- --nocapture
```

### 代码检查

```bash
# 运行 clippy
cargo clippy -- -D warnings

# 格式化代码
cargo fmt

# 检查格式
cargo fmt -- --check
```

## 项目结构

```
rvd_next/
├── src/
│   ├── app/                 # 应用层
│   │   ├── mod.rs
│   │   ├── orchestrator.rs  # 流程编排
│   │   └── registry.rs      # 平台注册表
│   ├── auth/                # 认证模块
│   │   ├── mod.rs
│   │   ├── login.rs         # 登录逻辑
│   │   ├── qrcode.rs        # 二维码生成
│   │   ├── storage.rs       # 凭证存储
│   │   └── providers/       # 平台认证提供者
│   ├── cli/                 # 命令行接口
│   │   └── mod.rs
│   ├── core/                # 核心功能
│   │   ├── mod.rs
│   │   ├── downloader.rs    # 下载器
│   │   ├── muxer.rs         # 混流器
│   │   ├── progress.rs      # 进度跟踪
│   │   ├── subtitle.rs      # 字幕处理
│   │   └── danmaku.rs       # 弹幕处理
│   ├── platform/            # 平台抽象层
│   │   ├── mod.rs
│   │   ├── trait.rs         # Platform trait 定义
│   │   ├── metadata.rs      # 平台元数据
│   │   ├── selector.rs      # 流选择器
│   │   └── bilibili/        # Bilibili 平台实现
│   │       ├── mod.rs
│   │       ├── platform.rs  # Platform trait 实现
│   │       ├── api.rs       # API 封装
│   │       ├── parser.rs    # URL 解析
│   │       ├── selector.rs  # 流选择
│   │       ├── auth.rs      # 认证
│   │       ├── wbi.rs       # WBI 签名
│   │       ├── app_sign.rs  # APP 签名
│   │       ├── cdn.rs       # CDN 优化
│   │       └── client.rs    # HTTP 客户端
│   ├── utils/               # 工具模块
│   │   ├── mod.rs
│   │   ├── http.rs          # HTTP 客户端
│   │   ├── config.rs        # 配置管理
│   │   ├── file.rs          # 文件操作
│   │   └── console.rs       # 控制台工具
│   ├── error.rs             # 错误类型
│   ├── types.rs             # 公共类型
│   ├── lib.rs               # 库入口
│   └── main.rs              # 程序入口
├── tests/                   # 集成测试
├── e2e_test_platform/       # E2E 测试平台
├── docs/                    # 文档
├── Cargo.toml               # 项目配置
└── config.example.toml      # 配置文件示例
```

## 添加新平台

添加新平台是 RVD Next 最常见的扩展场景。以下是完整的步骤指南。

### 步骤 1: 创建平台模块

在 `src/platform/` 下创建新目录：

```bash
mkdir src/platform/youtube
```

创建 `src/platform/youtube/mod.rs`：

```rust
//! YouTube platform module

pub mod platform;
pub mod api;
pub mod parser;

pub use platform::YouTubePlatform;
```

### 步骤 2: 定义平台结构

创建 `src/platform/youtube/platform.rs`：

```rust
use crate::error::Result;
use crate::platform::metadata::{AuthMethod, PlatformCapabilities, PlatformMetadata};
use crate::platform::Platform;
use crate::types::{Auth, BatchResult, Stream, StreamContext, VideoInfo};
use crate::utils::http::HttpClient;
use async_trait::async_trait;
use std::sync::Arc;

pub struct YouTubePlatform {
    client: Arc<HttpClient>,
    metadata: PlatformMetadata,
}

impl YouTubePlatform {
    pub fn new() -> Result<Self> {
        let client = Arc::new(HttpClient::new()?);
        
        let metadata = PlatformMetadata {
            name: "youtube",
            display_name: "YouTube",
            version: "1.0.0",
            capabilities: PlatformCapabilities {
                subtitles: true,
                danmaku: false,
                batch_download: true,
                chapters: true,
                requires_auth: false,
                auth_methods: vec![AuthMethod::Cookie],
                max_quality: Some("8K"),
                live_stream: true,
            },
            url_patterns: vec!["youtube.com", "youtu.be"],
        };
        
        Ok(Self { client, metadata })
    }
}
```

### 步骤 3: 实现 Platform Trait

继续在 `platform.rs` 中实现 `Platform` trait：

```rust
#[async_trait]
impl Platform for YouTubePlatform {
    fn metadata(&self) -> &PlatformMetadata {
        &self.metadata
    }
    
    fn can_handle(&self, url: &str) -> bool {
        url.contains("youtube.com") || url.contains("youtu.be")
    }
    
    fn is_batch_url(&self, url: &str) -> bool {
        url.contains("/playlist") || url.contains("/channel")
    }
    
    async fn parse_video(&self, url: &str, auth: Option<&Auth>) -> Result<VideoInfo> {
        // 1. 从 URL 提取视频 ID
        let video_id = self.extract_video_id(url)?;
        
        // 2. 调用 YouTube API 获取视频信息
        let video_data = self.fetch_video_info(&video_id, auth).await?;
        
        // 3. 解析为 VideoInfo
        let video_info = VideoInfo {
            id: video_id.clone(),
            aid: video_id.clone(),
            bvid: video_id.clone(),
            title: video_data.title,
            uploader: video_data.channel_name,
            description: video_data.description,
            cover_url: video_data.thumbnail_url,
            pages: vec![Page {
                number: 1,
                cid: video_id,
                title: video_data.title.clone(),
                duration: video_data.duration,
                extra_data: None,
            }],
            extra_data: None,
        };
        
        Ok(video_info)
    }
    
    async fn get_streams(
        &self,
        context: &StreamContext,
        auth: Option<&Auth>,
    ) -> Result<Vec<Stream>> {
        // 1. 获取视频流信息
        let streams_data = self.fetch_streams(&context.video_id, auth).await?;
        
        // 2. 解析为 Stream 对象
        let mut streams = Vec::new();
        
        for format in streams_data.formats {
            if format.has_video {
                streams.push(Stream {
                    stream_type: StreamType::Video,
                    quality: self.map_quality(&format),
                    quality_id: format.itag,
                    codec: format.video_codec,
                    bandwidth: format.bitrate,
                    url: format.url,
                    extra_data: None,
                });
            }
            
            if format.has_audio {
                streams.push(Stream {
                    stream_type: StreamType::Audio,
                    quality: "Audio".to_string(),
                    quality_id: format.itag,
                    codec: format.audio_codec,
                    bandwidth: format.bitrate,
                    url: format.url,
                    extra_data: None,
                });
            }
        }
        
        Ok(streams)
    }
    
    fn get_cover(&self, video_info: &VideoInfo) -> String {
        video_info.cover_url.clone()
    }
    
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

// 辅助方法
impl YouTubePlatform {
    fn extract_video_id(&self, url: &str) -> Result<String> {
        // 实现 URL 解析逻辑
        todo!()
    }
    
    async fn fetch_video_info(&self, video_id: &str, auth: Option<&Auth>) -> Result<VideoData> {
        // 实现 API 调用
        todo!()
    }
    
    async fn fetch_streams(&self, video_id: &str, auth: Option<&Auth>) -> Result<StreamsData> {
        // 实现流信息获取
        todo!()
    }
    
    fn map_quality(&self, format: &Format) -> String {
        // 将平台特定的清晰度映射到标准清晰度
        match format.height {
            Some(h) if h >= 2160 => "4K".to_string(),
            Some(h) if h >= 1080 => "1080P".to_string(),
            Some(h) if h >= 720 => "720P".to_string(),
            _ => "480P".to_string(),
        }
    }
}
```

### 步骤 4: 实现 API 模块

创建 `src/platform/youtube/api.rs`：

```rust
use crate::error::Result;
use crate::utils::http::HttpClient;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Deserialize)]
pub struct VideoData {
    pub title: String,
    pub channel_name: String,
    pub description: String,
    pub thumbnail_url: String,
    pub duration: u64,
}

#[derive(Debug, Deserialize)]
pub struct StreamsData {
    pub formats: Vec<Format>,
}

#[derive(Debug, Deserialize)]
pub struct Format {
    pub itag: u32,
    pub url: String,
    pub bitrate: u64,
    pub has_video: bool,
    pub has_audio: bool,
    pub video_codec: String,
    pub audio_codec: String,
    pub height: Option<u32>,
}

pub struct YouTubeApi {
    client: Arc<HttpClient>,
}

impl YouTubeApi {
    pub fn new(client: Arc<HttpClient>) -> Self {
        Self { client }
    }
    
    pub async fn get_video_info(&self, video_id: &str) -> Result<VideoData> {
        // 实现 API 调用
        todo!()
    }
    
    pub async fn get_streams(&self, video_id: &str) -> Result<StreamsData> {
        // 实现 API 调用
        todo!()
    }
}
```

### 步骤 5: 注册平台

在 `src/app/orchestrator.rs` 中注册新平台：

```rust
// 在 Orchestrator::new() 方法中添加
let youtube = Arc::new(crate::platform::youtube::YouTubePlatform::new()?);
registry.register(youtube);
```

在 `src/platform/mod.rs` 中导出：

```rust
pub mod bilibili;
pub mod youtube;  // 添加这行
```

### 步骤 6: 添加测试

创建 `tests/youtube_test.rs`：

```rust
use rvd::platform::youtube::YouTubePlatform;
use rvd::platform::Platform;

#[tokio::test]
async fn test_youtube_can_handle() {
    let platform = YouTubePlatform::new().unwrap();
    
    assert!(platform.can_handle("https://www.youtube.com/watch?v=dQw4w9WgXcQ"));
    assert!(platform.can_handle("https://youtu.be/dQw4w9WgXcQ"));
    assert!(!platform.can_handle("https://www.bilibili.com/video/BV1xx411c7mD"));
}

#[tokio::test]
async fn test_youtube_parse_video() {
    let platform = YouTubePlatform::new().unwrap();
    let url = "https://www.youtube.com/watch?v=dQw4w9WgXcQ";
    
    let result = platform.parse_video(url, None).await;
    assert!(result.is_ok());
    
    let video_info = result.unwrap();
    assert_eq!(video_info.id, "dQw4w9WgXcQ");
}
```

### 步骤 7: 添加文档

在 `src/platform/youtube/mod.rs` 顶部添加模块文档：

```rust
//! YouTube platform module
//!
//! This module provides YouTube platform support including:
//! - Video information parsing
//! - Stream URL retrieval
//! - Subtitle support
//! - Playlist downloads
//! - Live stream support
```

## 代码规范

### Rust 代码风格

遵循 Rust 官方代码风格指南：

1. 使用 `cargo fmt` 格式化代码
2. 使用 `cargo clippy` 检查代码质量
3. 为公共 API 编写文档注释
4. 使用有意义的变量名和函数名

### 命名约定

- **模块名**: 小写，使用下划线分隔（`snake_case`）
- **结构体**: 大驼峰命名（`PascalCase`）
- **函数**: 小写，使用下划线分隔（`snake_case`）
- **常量**: 大写，使用下划线分隔（`SCREAMING_SNAKE_CASE`）

### 错误处理

使用 `Result<T>` 类型处理错误：

```rust
pub async fn fetch_data(&self) -> Result<Data> {
    let response = self.client
        .get("https://api.example.com/data")
        .send()
        .await
        .map_err(|e| DownloaderError::Network(e.to_string()))?;
    
    let data = response
        .json::<Data>()
        .await
        .map_err(|e| DownloaderError::Parse(e.to_string()))?;
    
    Ok(data)
}
```

### 日志记录

使用 `tracing` 库记录日志：

```rust
use tracing::{debug, info, warn, error};

// 调试信息
debug!("Fetching video info for ID: {}", video_id);

// 一般信息
info!("Download completed: {}", filename);

// 警告
warn!("Failed to download subtitle, skipping");

// 错误
error!("Network error: {}", e);
```

### 文档注释

为公共 API 编写文档注释：

```rust
/// Fetch video information from the platform
///
/// # Arguments
///
/// * `video_id` - The unique identifier of the video
/// * `auth` - Optional authentication information
///
/// # Returns
///
/// Returns `VideoInfo` on success, or an error if the video doesn't exist
/// or network errors occur.
///
/// # Example
///
/// ```no_run
/// let platform = YouTubePlatform::new()?;
/// let video_info = platform.fetch_video_info("dQw4w9WgXcQ", None).await?;
/// ```
pub async fn fetch_video_info(&self, video_id: &str, auth: Option<&Auth>) -> Result<VideoInfo> {
    // ...
}
```

## 测试

### 单元测试

在模块内部编写单元测试：

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_extract_video_id() {
        let platform = YouTubePlatform::new().unwrap();
        let id = platform.extract_video_id("https://www.youtube.com/watch?v=dQw4w9WgXcQ").unwrap();
        assert_eq!(id, "dQw4w9WgXcQ");
    }
}
```

### 集成测试

在 `tests/` 目录下编写集成测试：

```rust
// tests/youtube_integration_test.rs
use rvd::app::Orchestrator;
use rvd::cli::Cli;
use rvd::utils::config::Config;

#[tokio::test]
async fn test_youtube_download() {
    let config = Config::default();
    let cli = Cli {
        url: Some("https://www.youtube.com/watch?v=dQw4w9WgXcQ".to_string()),
        info_only: true,
        ..Default::default()
    };
    
    let orchestrator = Orchestrator::new(config, &cli).unwrap();
    let result = orchestrator.run(cli).await;
    
    assert!(result.is_ok());
}
```

### Mock 测试

使用 `mockito` 模拟 HTTP 请求：

```rust
#[cfg(test)]
mod tests {
    use mockito::{mock, server_url};
    
    #[tokio::test]
    async fn test_api_call() {
        let _m = mock("GET", "/video/info")
            .with_status(200)
            .with_body(r#"{"title": "Test Video"}"#)
            .create();
        
        let client = HttpClient::new().unwrap();
        let api = YouTubeApi::new(Arc::new(client));
        
        let result = api.get_video_info("test_id").await;
        assert!(result.is_ok());
    }
}
```

## 调试技巧

### 启用详细日志

```bash
RUST_LOG=debug cargo run -- <URL>
```

### 使用 dbg! 宏

```rust
let video_info = platform.parse_video(url, None).await?;
dbg!(&video_info);  // 打印调试信息
```

### 使用 Rust Analyzer

推荐使用 VS Code + Rust Analyzer 插件，提供：
- 代码补全
- 类型提示
- 错误检查
- 重构工具

### 性能分析

使用 `cargo flamegraph` 生成火焰图：

```bash
cargo install flamegraph
cargo flamegraph -- <URL>
```

## 贡献流程

### 1. Fork 仓库

在 GitHub 上 Fork 项目到你的账号。

### 2. 创建分支

```bash
git checkout -b feature/youtube-support
```

### 3. 提交代码

```bash
git add .
git commit -m "feat: add YouTube platform support"
```

提交信息格式：
- `feat`: 新功能
- `fix`: Bug 修复
- `docs`: 文档更新
- `refactor`: 代码重构
- `test`: 测试相关
- `chore`: 构建/工具相关

### 4. 推送到 GitHub

```bash
git push origin feature/youtube-support
```

### 5. 创建 Pull Request

在 GitHub 上创建 Pull Request，描述你的更改。

### 6. 代码审查

维护者会审查你的代码，可能会提出修改建议。

### 7. 合并

审查通过后，代码会被合并到主分支。

## 最佳实践

### 1. 保持模块独立

每个平台应该是独立的模块，不依赖其他平台的实现。

### 2. 使用类型安全

充分利用 Rust 的类型系统，避免运行时错误。

### 3. 错误处理

不要使用 `unwrap()` 或 `expect()`，使用 `?` 操作符传播错误。

### 4. 异步编程

所有 I/O 操作都应该是异步的，使用 `async/await`。

### 5. 性能优化

- 避免不必要的克隆
- 使用 `Arc` 共享数据
- 使用流式处理大数据

### 6. 文档完善

- 为公共 API 编写文档
- 添加使用示例
- 更新 README 和用户指南

## 常见问题

### Q: 如何调试异步代码？

A: 使用 `tokio-console` 或添加日志：

```rust
tracing::debug!("Before async call");
let result = async_function().await;
tracing::debug!("After async call: {:?}", result);
```

### Q: 如何处理平台特定的数据？

A: 使用 `extra_data` 字段存储平台特定数据：

```rust
let mut extra_data = HashMap::new();
extra_data.insert("platform_specific_field".to_string(), serde_json::json!("value"));

let video_info = VideoInfo {
    // ...
    extra_data: Some(extra_data),
};
```

### Q: 如何测试需要认证的功能？

A: 使用环境变量或测试配置文件：

```rust
#[tokio::test]
async fn test_with_auth() {
    let cookie = std::env::var("TEST_COOKIE").ok();
    if cookie.is_none() {
        println!("Skipping test: TEST_COOKIE not set");
        return;
    }
    
    let auth = Auth {
        cookie,
        ..Default::default()
    };
    
    // 测试代码
}
```

## 资源链接

- [Rust 官方文档](https://doc.rust-lang.org/)
- [Tokio 文档](https://tokio.rs/)
- [Reqwest 文档](https://docs.rs/reqwest/)
- [Serde 文档](https://serde.rs/)
- [Tracing 文档](https://docs.rs/tracing/)

## 获取帮助

- GitHub Issues: 报告 Bug 或提问
- GitHub Discussions: 讨论新功能或设计
- 代码审查: 在 PR 中提问

感谢你的贡献！
