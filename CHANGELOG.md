# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.6] - 2025-10-31

### Fixed

#### 显示修复
- 修复Windows 10上ANSI显示异常的问题

## [0.2.5] - 2025-10-31

### Added

#### 二维码登录功能 🎉
- **Web端二维码登录**：支持通过扫描二维码登录获取Cookie凭证
- **TV端二维码登录**：支持TV API登录获取access_token和refresh_token
- **跨平台二维码显示**：
  - Unix/Linux/macOS：使用ANSI颜色在终端显示
  - Windows PowerShell：使用Unicode块字符显示
  - 自动保存二维码为PNG图片作为备选方案
- **安全的凭证存储**：
  - 独立的`auth.toml`文件存储认证信息
  - Unix系统自动设置文件权限为0600
  - 向后兼容从`config.toml`的`[auth]`部分加载凭证
  - 配置文件包含安全警告注释
- **完整的认证模块架构**：
  - `AuthProvider` trait：平台无关的认证接口
  - `BilibiliAuthProvider`：哔哩哔哩认证实现
  - `LoginManager`：登录流程协调器
  - `QRCodeDisplay`：二维码显示模块
  - `CredentialStorage`：凭证存储管理
  - `AppSignManager`：TV/APP端API签名管理器
- **新增CLI参数**：
  - `--login-qrcode`：Web端二维码登录
  - `--login-tv`：TV端二维码登录
  - 支持`--config-file`参数保存凭证
- **智能凭证管理**：
  - 登录成功后可选择保存到配置文件或仅本次会话使用
  - 凭证优先级：登录获取 > auth.toml > config.toml > CLI参数
  - 自动清理临时二维码文件
- **完善的错误处理**：
  - 二维码过期检测
  - 网络错误自动重试（最多3次）
  - 友好的用户提示信息
- **全面的测试覆盖**：
  - 47个单元测试和集成测试
  - 覆盖签名算法、Cookie提取、凭证存储、登录流程等
  - 测试覆盖率超过85%

#### 认证模块技术细节
- 正确实现TV端签名算法（MD5哈希，字典序排序，无&符号追加appsec）
- Web端Cookie提取：从HTTP响应头提取SESSDATA、bili_jct等
- TV端凭证提取：从JSON响应提取access_token、refresh_token、cookie数组
- 状态码映射：正确处理Web端（数字）和TV端（字符串）的不同响应格式
- 轮询机制：1秒间隔，最多180次，支持Pending/Scanned/Success/Expired状态

### Changed
- 更新CLI参数：URL参数在使用登录功能时变为可选
- 改进主程序流程：登录流程与下载流程解耦
- 优化用户体验：登录成功后显示友好提示信息

### Fixed
- 修复 `tests/core_danmaku_test.rs` 中的 clippy 警告（clone_on_copy）
- 修复 `tests/core_chapter_test.rs` 中的 clippy 警告（useless_vec）
- 修复 `tests/e2e_download_test.rs` 中的 clippy 警告（redundant_closure, map_or）

### Security
- Unix系统上auth.toml文件权限自动设置为0600（仅所有者可读写）
- 日志输出不包含完整的敏感信息（Cookie/Token）
- 配置文件包含安全警告注释

## [0.2.1] - 2025-10-31

### Fixed

#### 弹幕下载
- xml格式化修复

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

[0.2.5]: https://github.com/SpenserCai/rust-video-downloader/compare/v0.2.5...v0.2.6
[0.2.5]: https://github.com/SpenserCai/rust-video-downloader/compare/v0.2.1...v0.2.5
[0.2.1]: https://github.com/SpenserCai/rust-video-downloader/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/SpenserCai/rust-video-downloader/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/SpenserCai/rust-video-downloader/releases/tag/v0.1.0
