# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0] - 2025-10-31

### Added

#### 番剧和课程支持
- 支持通过 ep 链接下载番剧单集
- 支持通过 ss 链接下载番剧整季
- 支持课程（cheese）链接下载
- 番剧和课程的多集选择支持

#### 批量下载功能
- 支持收藏夹批量下载（自动分页获取所有视频）
- 支持UP主空间视频批量下载（使用WBI签名）
- 支持合集（medialist）批量下载
- 支持系列（series）批量下载
- 新增 `parse_video_batch()` 方法处理批量URL

#### 弹幕下载
- 支持下载 XML 格式弹幕
- 支持将弹幕转换为 ASS 格式
- 自动处理弹幕的 deflate/gzip 压缩
- 弹幕时间轴、位置、颜色完整支持
- 新增 `--download-danmaku` 和 `--danmaku-format` 参数

#### API 模式支持
- 支持 TV API 模式（无水印片源）
- 支持 APP API 模式（杜比音频）
- 支持国际版 API 模式
- 新增 `--use-tv-api`, `--use-app-api`, `--use-intl-api` 参数
- 番剧和普通视频的不同API端点处理

#### 章节信息
- 支持从 API 提取章节信息
- 支持番剧片头片尾章节标记
- 支持将章节信息嵌入到视频文件（通过FFmpeg metadata）
- 新增 `Chapter` 数据结构和 `fetch_chapters()` 方法

#### 其他改进
- 扩展 `VideoType` 枚举支持所有B站内容类型
- 新增 `ParseResult` 枚举区分单视频和批量结果
- 改进 URL 解析，支持更多链接格式
- 新增 WBI 签名管理器用于需要签名的API
- 完善的错误处理和日志输出

### Changed
- 更新 `Platform` trait 以支持批量下载
- 改进 `BilibiliPlatform` 的 API 调用逻辑
- 优化番剧视频的流获取（支持 ep_id 参数）
- 更新命令行帮助文档

### Fixed
- 修复多分P视频的分页处理
- 修复番剧 section 的解析问题
- 改进弹幕解压缩的兼容性

### Tests
- 新增 20+ 个 E2E 测试覆盖所有新功能
- 新增单元测试覆盖弹幕、章节等模块
- 所有测试通过率 100%（除3个需要特殊配置的测试）

## [0.1.0] - 2025-10-30

### Added
- 基础视频下载功能（BV/av 号）
- 多线程分块下载
- 智能流选择（清晰度和编码格式优先级）
- 自动混流（FFmpeg）
- 字幕下载和转换（JSON → SRT）
- 封面图片下载
- 多分P视频支持
- 交互式清晰度选择
- 配置文件支持（TOML格式）
- 自定义输出文件名模板
- Cookie 认证支持
- 详细的下载进度显示
- 模块化、可扩展的架构
- 支持 AVC/HEVC/AV1 编码

[0.2.0]: https://github.com/SpenserCai/rust-video-downloader/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/SpenserCai/rust-video-downloader/releases/tag/v0.1.0
