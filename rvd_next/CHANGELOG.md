# Changelog

All notable changes to RVD Next will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- 完整的文档体系（README、用户指南、开发指南、配置文档、迁移指南）
- 架构设计文档，详细说明模块化设计和扩展机制

## [1.0.0] - 2024-XX-XX

### Added - 核心架构

- **Platform Trait 抽象层**: 定义统一的平台接口，支持多平台扩展
- **PlatformRegistry**: 平台注册和自动选择机制
- **Orchestrator**: 流程编排器，协调整个下载流程
- **模块化架构**: 清晰的分层设计（应用层、平台层、核心层、工具层）

### Added - Bilibili 平台支持

- **多 API 模式**: Web、TV、APP、International 四种 API 模式
- **批量下载**: 
  - 收藏夹下载
  - UP主空间视频下载
  - 合集下载
  - 系列下载
  - 番剧下载
  - 课程下载
- **流式批量下载**: 支持超大收藏夹（10000+ 视频）而不会内存溢出
- **CDN 优化**: 
  - PCDN 自动检测
  - 不稳定节点替换
  - 可配置备用 CDN 主机
- **认证支持**:
  - 二维码登录（Web 端和 TV 端）
  - Cookie 认证
  - Access Token 认证
- **内容下载**:
  - 字幕下载和转换（JSON → SRT）
  - 弹幕下载（XML 和 ASS 格式）
  - 封面下载
  - 章节信息提取和嵌入
- **高级功能**:
  - 杜比视界支持
  - 杜比全景声支持
  - Hi-Res 无损音频支持
  - WBI 签名支持
  - APP 签名支持

### Added - 核心功能

- **下载器**:
  - 多线程分块下载
  - Aria2c 外部下载器支持
  - 自动重试机制
  - 进度跟踪
  - 平台特定请求头支持
- **混流器**:
  - FFmpeg 混流支持
  - MP4Box 混流支持（基础实现）
  - 字幕嵌入
  - 章节嵌入
  - 杜比视界兼容性检测
- **进度跟踪**:
  - 实时下载速度显示
  - ETA 计算
  - 多任务进度管理
  - 美观的进度条

### Added - 配置系统

- **灵活的配置文件支持**:
  - TOML 格式配置
  - 多级配置优先级（CLI > 环境变量 > 配置文件 > 默认值）
  - 平台特定配置
  - 热加载支持
- **配置项**:
  - HTTP 配置（User-Agent、超时、重试）
  - Aria2c 配置
  - 路径配置
  - 认证配置
  - 下载配置
  - 输出配置
  - 混流配置
  - 平台特定配置
  - 日志配置

### Added - CLI 功能

- **命令行参数**:
  - 基础下载参数
  - 清晰度和编码选择
  - 批量下载控制
  - 认证参数
  - 输出控制
  - 混流选项
- **交互式模式**: 手动选择清晰度和音频
- **信息查看模式**: 不下载，只显示视频信息
- **详细日志模式**: 调试和故障排除

### Added - 工具和实用功能

- **HTTP 客户端**:
  - 随机 User-Agent 生成
  - Cookie 管理
  - 自动重试
  - 超时控制
- **文件工具**:
  - 文件名模板系统
  - 非法字符处理
  - 临时目录管理
  - 路径规范化
- **配置管理**:
  - TOML 解析
  - 配置验证
  - 默认值处理
- **控制台工具**:
  - Windows UTF-8 支持
  - ANSI 颜色支持
  - 跨平台兼容性

### Added - 测试

- **单元测试**: 核心模块的单元测试
- **集成测试**: 完整流程的集成测试
- **E2E 测试平台**: Python 实现的端到端测试框架

### Changed - 架构改进

- 从单体架构重构为模块化架构
- 引入 Platform Trait 抽象层
- 实现平台注册和自动选择机制
- 采用异步 I/O 设计
- 改进错误处理和日志记录

### Changed - 性能优化

- 批量下载采用流式分页，避免内存溢出
- 优化 HTTP 请求，减少不必要的 API 调用
- 改进并发控制，提高下载效率
- CDN 智能选择，提高下载速度

### Fixed

- 修复大型收藏夹下载时的内存溢出问题
- 修复 PCDN 导致的下载不稳定问题
- 修复某些情况下的混流失败问题
- 修复 Windows 平台的文件名非法字符问题
- 修复杜比视界内容的混流兼容性问题

## 与旧版本的对比

### 保持兼容

- ✅ 所有命令行参数保持兼容
- ✅ 配置文件格式向后兼容
- ✅ 认证方式完全相同
- ✅ 输出文件格式相同

### 新增功能

- ✅ Platform Trait 抽象层
- ✅ 平台注册表
- ✅ 流式批量下载
- ✅ CDN 优化
- ✅ 交互式清晰度选择
- ✅ 自定义 User-Agent
- ✅ 更完善的配置系统
- ✅ 更好的错误处理

### 性能改进

- ✅ 批量下载内存使用降低 50%+
- ✅ 支持 10000+ 视频的收藏夹
- ✅ CDN 优化提高下载稳定性
- ✅ 更快的启动速度

## 未来计划

### v1.1.0
- [ ] YouTube 平台支持
- [ ] 完善 MP4Box 混流支持
- [ ] 下载队列管理
- [ ] 更多配置选项

### v1.2.0
- [ ] 抖音（Douyin）平台支持
- [ ] TikTok 平台支持
- [ ] 插件系统

### v2.0.0
- [ ] GUI 界面
- [ ] 更多视频平台支持
- [ ] 云端同步功能

## 贡献者

感谢所有为 RVD Next 做出贡献的开发者！

## 参考项目

本项目参考了以下优秀项目的设计：

- [BBDown](https://github.com/nilaoda/BBDown) - 哔哩哔哩下载器
- [yt-dlp](https://github.com/yt-dlp/yt-dlp) - YouTube 下载器
- [lux](https://github.com/iawia002/lux) - Go 语言视频下载器
- [bilibili-API-collect](https://github.com/SocialSisterYi/bilibili-API-collect) - 哔哩哔哩 API 文档

---

[Unreleased]: https://github.com/SpenserCai/rust-video-downloader/compare/v1.0.0...HEAD
[1.0.0]: https://github.com/SpenserCai/rust-video-downloader/releases/tag/v1.0.0
