<div align="center">

# 🎬 RVD Next - Rust Video Downloader

**新一代高性能、模块化的跨平台视频下载工具**

[![Crates.io](https://img.shields.io/crates/v/rvd.svg?style=flat&color=blue)](https://crates.io/crates/rvd)
[![Downloads](https://img.shields.io/crates/d/rvd.svg?style=flat&color=green)](https://crates.io/crates/rvd)
[![License](https://img.shields.io/crates/l/rvd.svg?style=flat&color=yellow)](https://github.com/SpenserCai/rust-video-downloader/blob/main/LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg?style=flat)](https://www.rust-lang.org/)

[安装](#-安装) • [快速开始](#-快速开始) • [文档](#-文档) • [贡献](#-贡献)

</div>

---

## 🎯 项目简介

RVD Next 是 RVD 的重构版本，采用全新的模块化架构设计，旨在提供更好的可扩展性和多平台支持。

### 为什么重构？

原版 RVD 主要针对哔哩哔哩平台优化，虽然功能完善，但在架构上难以扩展到其他视频平台。RVD Next 通过引入 **Platform Trait** 抽象层，实现了真正的多平台支持架构，为未来支持 YouTube、抖音等平台奠定了基础。

### 核心改进

- 🏗️ **模块化架构**: 基于 Trait 的平台抽象层，新增平台只需实现标准接口
- 🌍 **多平台就绪**: 架构设计参考 yt-dlp 和 lux，支持快速集成新平台
- ⚡ **性能优化**: 流式批量下载、智能 CDN 选择、异步 I/O
- 🔧 **更好的可维护性**: 清晰的模块边界、完善的错误处理、丰富的日志

## ✨ 特性概览

### 🎯 核心功能

| 功能             | 说明                                       |
| ---------------- | ------------------------------------------ |
| 🌍 **跨平台支持** | Windows、macOS、Linux 全平台支持           |
| 🎥 **多平台下载** | 哔哩哔哩（普通视频、番剧、课程、直播回放） |
| ⚡ **高速下载**   | 多线程分块下载 + Aria2c 支持               |
| 🎨 **智能流选择** | 自动选择最佳视频和音频流                   |
| 🔐 **认证支持**   | 二维码登录（Web/TV）、Cookie、Access Token |
| 📦 **批量下载**   | 收藏夹、UP主空间、合集、系列               |

### 🚀 高级特性

<details>
<summary><b>点击展开查看完整功能列表</b></summary>

#### 视频处理
- ✅ 多分P视频支持（支持选择特定分P或范围）
- ✅ 自动混流（FFmpeg / MP4Box）
- ✅ 支持 AVC/HEVC/AV1 编码
- ✅ 杜比视界和杜比全景声支持
- ✅ Hi-Res 无损音频（FLAC）支持
- ✅ 清晰度和编码格式优先级设置
- ✅ 交互式清晰度选择模式
- ✅ 章节信息提取和嵌入

#### 内容下载
- ✅ 字幕下载和转换（JSON → SRT）
- ✅ 弹幕下载（XML/ASS 格式）
- ✅ 封面图片下载
- ✅ 番剧下载（ep/ss 链接）
- ✅ 课程下载（cheese 链接）

#### 认证与 API
- ✅ 二维码登录（Web端和TV端）
- ✅ Cookie 认证支持（下载会员内容）
- ✅ TV/APP API 支持（无水印片源）
- ✅ 国际版 API 支持

#### 配置与自定义
- ✅ 灵活的配置文件支持（TOML格式）
- ✅ 自定义输出文件名模板
- ✅ 详细的下载进度显示
- ✅ FFmpeg 版本检测（杜比视界兼容性）
- ✅ CDN 智能优化（PCDN 检测和替换）

#### 下载引擎
- ✅ 内置多线程下载器
- ✅ Aria2c 下载支持（更快的下载速度）

#### 架构特性
- ✅ 模块化、可扩展的架构
- ✅ 异步 I/O 高性能设计
- ✅ 流式批量下载（避免内存溢出）
- ✅ 平台注册和自动选择机制

</details>

### 📋 计划支持

- ⬜ YouTube 平台支持
- ⬜ 抖音（Douyin）平台支持
- ⬜ TikTok 平台支持
- ⬜ 更多视频平台（爱奇艺、腾讯视频等）
- ⬜ GUI 界面
- ⬜ 下载队列管理

## 📦 安装

### 方式一：从源码编译

```bash
# 克隆仓库
git clone https://github.com/SpenserCai/rust-video-downloader.git
cd rust-video-downloader/rvd_next

# 编译
cargo build --release

# 可执行文件位于 target/release/rvd
```

### 方式二：从 Crates.io 安装（即将支持）

```bash
cargo install rvd
```

### 依赖要求

- **FFmpeg**: 用于视频混流（必需）
  - 下载杜比视界内容需要 FFmpeg 5.0+
- **Aria2c**: 可选，用于更快的下载速度

## 🚀 快速开始

### 基础用法

```bash
# 下载单个视频
rvd https://www.bilibili.com/video/BV1xx411c7mD

# 下载指定分P
rvd https://www.bilibili.com/video/BV1xx411c7mD --pages 1,3-5

# 下载收藏夹
rvd "https://space.bilibili.com/123456/favlist?fid=789"

# 下载UP主所有视频
rvd "https://space.bilibili.com/123456/video"
```

### 认证登录

```bash
# 二维码登录（Web端）
rvd login --mode qrcode

# 二维码登录（TV端，获取更高清晰度）
rvd login --mode tv

# 使用 Cookie 登录
rvd --cookie "SESSDATA=xxx;bili_jct=xxx" <URL>
```

### 高级选项

```bash
# 指定清晰度和编码优先级
rvd <URL> --quality-priority "8K,4K,1080P" --codec-priority "AV1,HEVC,AVC"

# 使用 TV API（无水印）
rvd <URL> --api-mode tv

# 使用 Aria2c 下载
rvd <URL> --use-aria2c --threads 16

# 下载弹幕和字幕
rvd <URL> --download-danmaku --danmaku-format ass

# 自定义输出路径
rvd <URL> --output "downloads/{uploader}/{title}.mp4"
```

## 📚 文档

- [用户指南](docs/USER_GUIDE.md) - 详细的使用说明
- [配置文件](docs/CONFIGURATION.md) - 配置文件详解
- [开发指南](docs/DEVELOPMENT.md) - 如何添加新平台
- [API 文档](docs/API.md) - 库 API 文档
- [架构设计](docs/ARCHITECTURE.md) - 架构设计说明

## 🏗️ 架构设计

RVD Next 采用分层架构设计：

```
┌─────────────────────────────────────┐
│         CLI / Application           │  命令行界面和应用层
├─────────────────────────────────────┤
│         Orchestrator                │  下载流程编排
├─────────────────────────────────────┤
│      Platform Registry              │  平台注册和选择
├─────────────────────────────────────┤
│  Platform Trait (抽象层)            │  统一的平台接口
├──────────┬──────────┬───────────────┤
│ Bilibili │ YouTube  │  Douyin ...   │  具体平台实现
├──────────┴──────────┴───────────────┤
│  Core (下载、混流、进度跟踪)         │  核心功能模块
├─────────────────────────────────────┤
│  Utils (HTTP、配置、文件)            │  工具模块
└─────────────────────────────────────┘
```

### 核心概念

- **Platform Trait**: 定义所有平台必须实现的接口
- **PlatformRegistry**: 管理已注册的平台，自动选择合适的平台
- **Orchestrator**: 协调整个下载流程
- **StreamContext**: 平台无关的流上下文

详见 [架构设计文档](docs/ARCHITECTURE.md)

## 🤝 贡献

我们欢迎各种形式的贡献！

### 如何添加新平台

1. 在 `src/platform/` 下创建新平台模块
2. 实现 `Platform` trait
3. 在 `Orchestrator` 中注册平台
4. 添加测试和文档

详见 [开发指南](docs/DEVELOPMENT.md)

### 贡献指南

- 提交 Issue 报告 Bug 或建议新功能
- 提交 Pull Request 贡献代码
- 完善文档和示例
- 分享使用经验

## 📄 许可证

本项目采用 [MIT License](../LICENSE) 开源。

## 🙏 致谢

本项目参考了以下优秀项目的设计：

- [BBDown](https://github.com/nilaoda/BBDown) - 哔哩哔哩下载器
- [yt-dlp](https://github.com/yt-dlp/yt-dlp) - YouTube 下载器
- [lux](https://github.com/iawia002/lux) - Go 语言视频下载器
- [bilibili-API-collect](https://github.com/SocialSisterYi/bilibili-API-collect) - 哔哩哔哩 API 文档

## 📞 联系方式

- GitHub Issues: [提交问题](https://github.com/SpenserCai/rust-video-downloader/issues)
- 讨论区: [GitHub Discussions](https://github.com/SpenserCai/rust-video-downloader/discussions)

---

<div align="center">

**如果这个项目对你有帮助，请给我们一个 ⭐️**

Made with ❤️ by RVD Contributors

</div>
