<div align="center">

# 🎬 RVD - Rust Video Downloader

**一个高性能、模块化的跨平台视频下载工具**

[![Crates.io](https://img.shields.io/crates/v/rvd.svg?style=flat&color=blue)](https://crates.io/crates/rvd)
[![Downloads](https://img.shields.io/crates/d/rvd.svg?style=flat&color=green)](https://crates.io/crates/rvd)
[![License](https://img.shields.io/crates/l/rvd.svg?style=flat&color=yellow)](https://github.com/SpenserCai/rust-video-downloader/blob/main/LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg?style=flat)](https://www.rust-lang.org/)
[![GitHub Release](https://img.shields.io/github/v/release/SpenserCai/rust-video-downloader?style=flat&color=purple)](https://github.com/SpenserCai/rust-video-downloader/releases)

[安装](#-安装) • [快速开始](#-快速开始) • [文档](#-使用指南) • [贡献](#-贡献)

</div>

---

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

#### 下载引擎
- ✅ 内置多线程下载器
- ✅ Aria2c 下载支持（更快的下载速度）

#### 架构特性
- ✅ 模块化、可扩展的架构
- ✅ 异步 I/O 高性能设计

</details>

### 📋 计划支持

- ⬜ MP4Box 混流支持（完整实现）
- ⬜ 更多视频平台支持（YouTube、爱奇艺等）
- ⬜ GUI 界面
- ⬜ 下载队列管理

## 📦 安装

### 方式一：下载预编译二进制文件

从 [GitHub Releases](https://github.com/SpenserCai/rust-video-downloader/releases) 下载适合你系统的预编译版本：

- **Windows**: `rvd-x86_64-pc-windows-msvc.zip`
- **macOS**: `rvd-x86_64-apple-darwin.tar.gz` / `rvd-aarch64-apple-darwin.tar.gz`
- **Linux**: `rvd-x86_64-unknown-linux-gnu.tar.gz`

下载后解压并将可执行文件添加到系统 PATH。

### 方式二：从 Crates.io 安装

```bash
cargo install rvd
```

<details>
<summary><b>方式三：从源码编译</b></summary>

#### 前置要求
- Rust 1.70 或更高版本
- Git

#### 编译步骤

```bash
# 克隆仓库
git clone https://github.com/SpenserCai/rust-video-downloader.git
cd rust-video-downloader

# 编译发布版本
cargo build --release

# 可执行文件位于 target/release/rvd
```

#### 安装到系统

```bash
cargo install --path .
```

</details>

### 🔧 依赖项

RVD 需要 **FFmpeg** 来进行视频混流。请确保已安装：

| 平台              | 安装命令                                                |
| ----------------- | ------------------------------------------------------- |
| **macOS**         | `brew install ffmpeg`                                   |
| **Ubuntu/Debian** | `sudo apt install ffmpeg`                               |
| **Windows**       | 从 [FFmpeg 官网](https://ffmpeg.org/download.html) 下载 |

> 💡 **提示**: 如果需要杜比视界支持，建议使用 FFmpeg 5.0 或更高版本

## 🚀 快速开始

### 30 秒上手

```bash
# 1. 从 GitHub Releases 下载最新版本（推荐）
# 访问: https://github.com/SpenserCai/rust-video-downloader/releases
# 下载适合你系统的预编译版本并解压

# 或使用 cargo 安装
cargo install rvd

# 2. 下载你的第一个视频
rvd "https://www.bilibili.com/video/BV1xx411c7mD"

# 3. 完成！视频已下载到当前目录
```

### 基本用法

下载单个视频（使用默认设置）：

```bash
# 使用完整 URL
rvd "https://www.bilibili.com/video/BV1xx411c7mD"

# 或直接使用 BV 号
rvd BV1xx411c7mD

# 或使用 av 号
rvd av170001
```

## 📖 使用指南

### 常用场景

<details>
<summary><b>🎯 指定清晰度和编码</b></summary>

指定清晰度优先级（按顺序尝试）：

```bash
rvd BV1xx411c7mD -q "1080P,720P,480P"
```

指定编码格式优先级：

```bash
rvd BV1xx411c7mD -c "hevc,avc,av1"
```

同时指定清晰度和编码：

```bash
rvd BV1xx411c7mD -q "1080P,720P" -c "hevc,avc"
```

</details>

<details>
<summary><b>🖱️ 交互式选择</b></summary>

使用交互式模式手动选择清晰度和编码：

```bash
rvd BV1xx411c7mD -i
```

程序会列出所有可用的视频流和音频流供你选择。

</details>

<details>
<summary><b>📑 下载特定分P</b></summary>

```bash
# 下载第 1 个分P
rvd BV1xx411c7mD -p 1

# 下载多个分P
rvd BV1xx411c7mD -p "1,2,5"

# 下载分P范围
rvd BV1xx411c7mD -p "1-5"

# 下载所有分P
rvd BV1xx411c7mD -p ALL
```

</details>

<details>
<summary><b>🔐 二维码登录（推荐）</b></summary>

RVD 支持通过扫描二维码登录获取认证凭证，无需手动复制Cookie。

#### Web端登录（获取Cookie）

```bash
# 临时登录
rvd --login-qrcode

# 保存凭证以供后续使用
rvd --login-qrcode --config-file config.toml
```

登录成功后，凭证会保存到 `auth.toml` 文件中。

#### TV端登录（获取access_token）

TV端登录可以获取无水印片源：

```bash
rvd --login-tv --config-file config.toml
```

#### 二维码显示

| 平台                   | 显示方式                     |
| ---------------------- | ---------------------------- |
| **Unix/Linux/macOS**   | 终端中以彩色方块显示         |
| **Windows PowerShell** | Unicode字符显示              |
| **备选方案**           | 保存为 `qrcode.png` 图片文件 |

#### 登录流程
1. 程序生成二维码并显示在终端
2. 使用哔哩哔哩手机APP扫描二维码
3. 在手机上确认登录
4. 程序自动获取凭证并保存（如果指定了配置文件）

</details>

<details>
<summary><b>🔑 手动认证</b></summary>

除了二维码登录，也可以手动提供认证信息。

使用 Cookie：

```bash
rvd BV1xx411c7mD --cookie "SESSDATA=your_sessdata_here"
```

使用 Access Token（用于 TV/APP API）：

```bash
rvd BV1xx411c7mD --access-token "your_token_here"
```

> 💡 **提示**: 认证信息也可以保存在配置文件中，避免每次输入。

</details>

<details>
<summary><b>📁 自定义输出路径</b></summary>

```bash
rvd BV1xx411c7mD -o "downloads/<videoTitle>.mp4"
```

#### 支持的模板变量

| 变量                   | 说明            | 示例           |
| ---------------------- | --------------- | -------------- |
| `<videoTitle>`         | 视频标题        | "我的视频"     |
| `<pageNumber>`         | 分P编号         | "1"            |
| `<pageNumberWithZero>` | 分P编号（补零） | "01"           |
| `<pageTitle>`          | 分P标题         | "第一集"       |
| `<quality>`            | 清晰度          | "1080P 高清"   |
| `<codec>`              | 编码格式        | "AVC"          |
| `<uploader>`           | UP主名称        | "UP主"         |
| `<uploaderMid>`        | UP主 mid        | "123456"       |
| `<bvid>`               | BV号            | "BV1xx411c7mD" |
| `<cid>`                | 视频 cid        | "123456"       |
| `<date>`               | 上传日期        | "2024-01-01"   |

#### 示例

```bash
# 单P视频
rvd BV1xx411c7mD -o "<videoTitle>_<quality>.mp4"

# 多P视频（自动创建文件夹）
rvd BV1xx411c7mD -o "<videoTitle>/P<pageNumberWithZero>_<pageTitle>.mp4"

# 按UP主分类
rvd BV1xx411c7mD -o "<uploader>/<videoTitle>.mp4"
```

</details>

<details>
<summary><b>⚙️ 其他常用选项</b></summary>

#### 仅查看视频信息

```bash
rvd BV1xx411c7mD --info-only
```

#### 跳过字幕或封面

```bash
# 跳过字幕下载
rvd BV1xx411c7mD --skip-subtitle

# 跳过封面下载
rvd BV1xx411c7mD --skip-cover

# 跳过混流（保留分离的视频和音频文件）
rvd BV1xx411c7mD --skip-mux
```

#### 指定下载线程数

```bash
rvd BV1xx411c7mD -t 8
```

#### 启用详细日志

```bash
rvd BV1xx411c7mD -v
```

#### 指定 FFmpeg 路径

```bash
rvd BV1xx411c7mD --ffmpeg-path /path/to/ffmpeg
```

</details>

<details>
<summary><b>⚡ 使用 Aria2c 加速下载</b></summary>

Aria2c 是一个强大的下载工具，通常比内置下载器更快，特别是对于大文件。

#### 安装 Aria2c

| 平台              | 安装命令                                       |
| ----------------- | ---------------------------------------------- |
| **macOS**         | `brew install aria2`                           |
| **Ubuntu/Debian** | `sudo apt install aria2`                       |
| **Windows**       | 从 [aria2 官网](https://aria2.github.io/) 下载 |

#### 使用方法

```bash
# 启用 aria2c 下载
rvd BV1xx411c7mD --use-aria2c

# 指定 aria2c 路径（如果不在 PATH 中）
rvd BV1xx411c7mD --use-aria2c --aria2c-path /path/to/aria2c

# 自定义参数（例如减少连接数以避免被限速）
rvd BV1xx411c7mD --use-aria2c --aria2c-args "-x8 -s8 -j8"
```

#### 默认参数说明

| 参数   | 说明                   |
| ------ | ---------------------- |
| `-x16` | 每个服务器最多16个连接 |
| `-s16` | 分割成16个部分下载     |
| `-j16` | 最多同时下载16个文件   |
| `-k5M` | 最小分割大小5MB        |

#### 在配置文件中启用

```toml
[aria2c]
enabled = true
# path = "/usr/local/bin/aria2c"  # 可选
# args = "-x8 -s8 -j8"  # 可选
```

</details>

## ⚙️ 配置文件

<details>
<summary><b>配置文件说明</b></summary>

RVD 支持使用配置文件来设置默认选项。配置文件使用 TOML 格式。

### 配置文件位置

RVD 会按以下顺序查找配置文件：
1. 当前目录的 `rvd.toml`
2. `~/.config/rvd/config.toml`

你也可以使用 `--config-file` 参数指定配置文件路径。

### 示例配置文件

创建 `rvd.toml`：

```toml
# 默认清晰度优先级
default_quality = ["1080P", "720P", "480P"]

# 默认编码优先级
default_codec = ["hevc", "avc", "av1"]

# 下载线程数
thread_count = 8

# 单P视频输出文件名模板
output_template = "<videoTitle>_<quality>"

# 多P视频输出文件名模板
multi_output_template = "<videoTitle>/P<pageNumberWithZero>_<pageTitle>"

# 认证信息（也可以使用独立的 auth.toml 文件）
[auth]
cookie = "SESSDATA=your_sessdata_here"
# access_token = ""
# refresh_token = ""
# expires_at = 1234567890
# mid = 123456

# 外部工具路径
[paths]
ffmpeg = "/usr/local/bin/ffmpeg"

# Aria2c 下载配置（可选）
[aria2c]
enabled = false
# path = "/usr/local/bin/aria2c"
# args = "-x8 -s8 -j8"
```

> 💡 **提示**: 配置文件中的设置会被命令行参数覆盖。

### 认证凭证文件

使用二维码登录后，凭证会保存到独立的 `auth.toml` 文件中：

```toml
# 认证凭证文件
# 警告：此文件包含敏感信息，请勿分享或提交到版本控制系统

cookie = "SESSDATA=xxx; bili_jct=xxx; DedeUserID=xxx; DedeUserID__ckMd5=xxx; sid=xxx"
access_token = "xxx"  # TV/APP 登录时使用
refresh_token = "xxx"  # 用于刷新 access_token
expires_at = 1234567890  # 过期时间戳（可选）
mid = 123456  # 用户ID（可选）
```

#### 🔒 安全提示
- Unix/Linux/macOS系统上，`auth.toml` 文件权限会自动设置为 `0600`（仅所有者可读写）
- 建议将 `auth.toml` 添加到 `.gitignore` 中，避免意外提交
- 凭证有效期通常为几个月，过期后需要重新登录

</details>

## 🎯 高级功能

<details>
<summary><b>📺 下载番剧和课程</b></summary>

#### 下载番剧

```bash
# 通过 ep 链接下载单集
rvd "https://www.bilibili.com/bangumi/play/ep123456"

# 通过 ss 链接下载整季
rvd "https://www.bilibili.com/bangumi/play/ss12345"

# 下载特定集数
rvd ep123456 -p "1,2,3"
```

#### 下载课程

```bash
rvd "https://www.bilibili.com/cheese/play/ep123456"
```

</details>

<details>
<summary><b>📦 批量下载</b></summary>

```bash
# 下载收藏夹中的所有视频
rvd "https://space.bilibili.com/{mid}/favlist?fid={fav_id}"

# 下载UP主空间的所有视频
rvd "https://space.bilibili.com/{mid}"

# 下载合集
rvd "https://www.bilibili.com/medialist/play/ml{media_id}"

# 下载系列
rvd "https://space.bilibili.com/{mid}/channel/seriesdetail?sid={series_id}"
```

</details>

<details>
<summary><b>💬 下载弹幕</b></summary>

```bash
# 下载 ASS 格式弹幕（默认，可直接嵌入视频）
rvd BV1xx411c7mD --download-danmaku

# 下载 XML 格式弹幕（原始格式）
rvd BV1xx411c7mD --download-danmaku --danmaku-format xml
```

</details>

<details>
<summary><b>🔌 使用不同 API 模式</b></summary>

```bash
# 使用 TV API（获取无水印片源）
rvd BV1xx411c7mD --use-tv-api

# 使用 APP API（支持杜比音频）
rvd BV1xx411c7mD --use-app-api

# 使用国际版 API
rvd BV1xx411c7mD --use-intl-api
```

</details>

<details>
<summary><b>🎬 杜比视界和杜比全景声支持</b></summary>

RVD 支持下载杜比视界（Dolby Vision）视频和杜比全景声（Dolby Atmos）音频，以及 Hi-Res 无损音频（FLAC）。

### 自动识别

当视频包含杜比音视频流时，RVD 会自动识别并在流选择时显示：

```bash
rvd BV1xx411c7mD -q "杜比视界,4K 超清,1080P"
```

### 可用的高级格式

| 格式               | Quality ID | 说明                  |
| ------------------ | ---------- | --------------------- |
| **杜比视界**       | 126        | 需要支持 HDR 的显示器 |
| **HDR 真彩**       | 125        | HDR10 格式            |
| **E-AC-3 (Dolby)** | -          | 杜比全景声音频        |
| **FLAC (Hi-Res)**  | -          | 无损音频              |

### FFmpeg 版本要求

> ⚠️ **重要**: 杜比视界需要 FFmpeg 5.0 或更高版本才能正确处理元数据。

检查 FFmpeg 版本：

```bash
ffmpeg -version
```

如果版本低于 5.0，RVD 会显示警告并建议升级。

### 升级 FFmpeg

| 平台              | 升级命令                                                                                     |
| ----------------- | -------------------------------------------------------------------------------------------- |
| **macOS**         | `brew upgrade ffmpeg`                                                                        |
| **Ubuntu/Debian** | `sudo add-apt-repository ppa:savoury1/ffmpeg5 && sudo apt update && sudo apt install ffmpeg` |
| **Windows**       | 从 [FFmpeg 官网](https://ffmpeg.org/download.html) 下载最新版本                              |

### 使用 MP4Box（可选）

如果无法升级 FFmpeg，可以使用 MP4Box 进行混流：

```bash
rvd BV1xx411c7mD --use-mp4box
```

> 📝 **注意**: MP4Box 的完整集成仍在开发中。

### 示例

```bash
# 下载杜比视界视频
rvd BV1xx411c7mD -q "杜比视界,4K 超清,1080P"

# 使用 TV API 获取更高质量的流
rvd BV1xx411c7mD --use-tv-api -q "杜比视界"

# 下载杜比全景声音频
rvd BV1xx411c7mD -c "E-AC-3,FLAC,M4A"

# 交互式选择（可以看到所有可用的音视频流）
rvd BV1xx411c7mD -i
```

</details>

## 📚 命令行参数

<details>
<summary><b>点击查看完整参数列表</b></summary>

### 基本用法

```
rvd [OPTIONS] <URL>
```

### 参数说明

#### 位置参数

| 参数    | 说明           | 示例                                   |
| ------- | -------------- | -------------------------------------- |
| `<URL>` | 视频 URL 或 ID | `BV1xx411c7mD`, `av170001`, `ep123456` |

#### 视频选项

| 参数                | 说明                     | 示例                               |
| ------------------- | ------------------------ | ---------------------------------- |
| `-q, --quality`     | 清晰度优先级（逗号分隔） | `"1080P,720P,480P"`                |
| `-c, --codec`       | 编码格式优先级           | `"hevc,avc,av1"`                   |
| `-p, --pages`       | 选择特定分P或集数        | `"1"`, `"1,2,5"`, `"1-5"`, `"ALL"` |
| `-i, --interactive` | 交互式清晰度选择模式     | -                                  |

#### 输出选项

| 参数              | 说明               | 示例                           |
| ----------------- | ------------------ | ------------------------------ |
| `-o, --output`    | 输出文件路径或模板 | `"<videoTitle>_<quality>.mp4"` |
| `--skip-subtitle` | 跳过字幕下载       | -                              |
| `--skip-cover`    | 跳过封面下载       | -                              |
| `--skip-mux`      | 跳过混流           | -                              |

#### 下载选项

| 参数            | 说明                  | 默认值 |
| --------------- | --------------------- | ------ |
| `-t, --threads` | 下载线程数            | `4`    |
| `--use-aria2c`  | 使用 aria2c 下载      | -      |
| `--aria2c-path` | aria2c 可执行文件路径 | -      |
| `--aria2c-args` | 自定义 aria2c 参数    | -      |

#### 认证选项

| 参数             | 说明                  |
| ---------------- | --------------------- |
| `--cookie`       | Cookie 字符串         |
| `--access-token` | Access Token          |
| `--login-qrcode` | 二维码登录（Web模式） |
| `--login-tv`     | 二维码登录（TV模式）  |

#### API 选项

| 参数             | 说明                      |
| ---------------- | ------------------------- |
| `--use-tv-api`   | 使用 TV API（无水印片源） |
| `--use-app-api`  | 使用 APP API（杜比音频）  |
| `--use-intl-api` | 使用国际版 API            |

#### 其他选项

| 参数                 | 说明                  |
| -------------------- | --------------------- |
| `--info-only`        | 仅显示视频信息        |
| `--download-danmaku` | 下载弹幕文件          |
| `--danmaku-format`   | 弹幕格式（xml/ass）   |
| `--config-file`      | 指定配置文件路径      |
| `--ffmpeg-path`      | FFmpeg 可执行文件路径 |
| `--use-mp4box`       | 使用 MP4Box 混流      |
| `-v, --verbose`      | 启用详细日志          |
| `-h, --help`         | 显示帮助信息          |
| `-V, --version`      | 显示版本信息          |

### 可用清晰度

`8K 超高清`, `杜比视界`, `HDR 真彩`, `4K 超清`, `1080P 60帧`, `1080P 高码率`, `1080P 高清`, `720P 60帧`, `720P 高清`, `480P 清晰`, `360P 流畅`

### 可用编码格式

`avc` (H.264), `hevc` (H.265), `av1`, `E-AC-3` (Dolby), `FLAC` (Hi-Res)

</details>

## 🏗️ 技术特点

<details>
<summary><b>模块化架构</b></summary>

RVD 采用分层架构设计，各层职责清晰：

```
┌─────────────────────────────────────┐
│          CLI 层                      │  命令行参数解析和用户交互
├─────────────────────────────────────┤
│          应用层                      │  协调各模块完成下载任务
├─────────────────────────────────────┤
│          平台层                      │  特定视频平台的实现（可扩展）
├─────────────────────────────────────┤
│          核心层                      │  下载、混流、进度跟踪功能
├─────────────────────────────────────┤
│          工具层                      │  HTTP 客户端、配置管理、文件操作
└─────────────────────────────────────┘
```

</details>

<details>
<summary><b>可扩展设计</b></summary>

通过 `Platform` trait 定义统一接口，添加新平台支持无需修改核心代码：

```rust
#[async_trait]
pub trait Platform: Send + Sync {
    fn can_handle(&self, url: &str) -> bool;
    async fn parse_video(&self, url: &str, auth: Option<&Auth>) -> Result<VideoInfo>;
    async fn get_streams(&self, video_id: &str, cid: &str, auth: Option<&Auth>) -> Result<Vec<Stream>>;
    async fn get_subtitles(&self, video_id: &str, cid: &str) -> Result<Vec<Subtitle>>;
    fn get_cover(&self, video_info: &VideoInfo) -> String;
    fn name(&self) -> &str;
}
```

</details>

<details>
<summary><b>性能优化</b></summary>

- ⚡ **多线程分块下载**: 充分利用带宽
- 💾 **流式写入**: 避免内存占用过大
- 🔄 **智能重试机制**: 提高下载成功率
- 🚀 **异步 I/O**: 提升并发性能
- 📦 **Aria2c 集成**: 可选的高性能下载引擎

</details>

## 👨‍💻 开发

<details>
<summary><b>构建项目</b></summary>

```bash
# 开发构建
cargo build

# 发布构建（优化）
cargo build --release
```

</details>

<details>
<summary><b>运行测试</b></summary>

```bash
# 运行所有测试
cargo test

# 运行特定测试
cargo test test_parse_video_info

# 显示测试输出
cargo test -- --nocapture
```

</details>

<details>
<summary><b>代码质量检查</b></summary>

```bash
# Clippy 检查
cargo clippy -- -D warnings

# 代码格式化
cargo fmt

# 格式检查（不修改文件）
cargo fmt -- --check
```

</details>

<details>
<summary><b>项目结构</b></summary>

```
src/
├── cli/           # 命令行参数解析
├── app/           # 应用协调逻辑
├── auth/          # 认证模块
│   ├── login.rs       # 登录管理器
│   ├── qrcode.rs      # 二维码显示
│   ├── storage.rs     # 凭证存储
│   ├── types.rs       # 认证类型定义
│   └── providers/     # 认证提供者
│       ├── bilibili.rs    # 哔哩哔哩认证
│       └── mod.rs         # 签名管理器
├── platform/      # 平台特定实现
│   ├── bilibili/  # 哔哩哔哩平台
│   └── trait.rs   # 平台接口定义
├── core/          # 核心功能
│   ├── downloader.rs  # 下载引擎
│   ├── muxer.rs       # 混流器
│   ├── progress.rs    # 进度跟踪
│   ├── subtitle.rs    # 字幕处理
│   ├── danmaku.rs     # 弹幕处理
│   └── chapter.rs     # 章节处理
├── utils/         # 工具模块
│   ├── http.rs    # HTTP 客户端
│   ├── config.rs  # 配置管理
│   └── file.rs    # 文件操作
├── error.rs       # 错误类型定义
├── types.rs       # 数据结构定义
└── main.rs        # 程序入口
```

</details>

## ❓ 常见问题

<details>
<summary><b>FFmpeg 相关</b></summary>

#### Q: 提示找不到 FFmpeg？

请确保 FFmpeg 已安装并在系统 PATH 中，或使用 `--ffmpeg-path` 参数指定路径。

#### Q: 如何安装 FFmpeg？

| 平台              | 安装命令                                                |
| ----------------- | ------------------------------------------------------- |
| **macOS**         | `brew install ffmpeg`                                   |
| **Ubuntu/Debian** | `sudo apt install ffmpeg`                               |
| **Windows**       | 从 [FFmpeg 官网](https://ffmpeg.org/download.html) 下载 |

</details>

<details>
<summary><b>下载相关</b></summary>

#### Q: 下载速度慢？

可以尝试：
- 增加线程数：`rvd <url> -t 8`
- 使用 Aria2c：`rvd <url> --use-aria2c`

#### Q: 下载失败或中断？

程序会自动重试 3 次。如果仍然失败，请：
- 检查网络连接
- 尝试使用认证信息
- 使用 `-v` 参数查看详细日志

#### Q: 如何下载会员视频？

需要提供有效的认证凭证：
```bash
# 推荐：使用二维码登录
rvd --login-qrcode --config-file config.toml

# 或手动提供 Cookie
rvd <url> --cookie "SESSDATA=..."
```

</details>

<details>
<summary><b>认证相关</b></summary>

#### Q: 如何获取认证凭证？

**推荐方式**（最简单）：
```bash
rvd --login-qrcode --config-file config.toml
```

**手动获取Cookie**：
1. 在浏览器中登录 bilibili.com
2. 打开开发者工具（F12）
3. 在 Application/Storage > Cookies 中找到 SESSDATA
4. 复制其值使用

#### Q: Cookie 会过期吗？

是的，Cookie 有效期通常为几个月。过期后需要重新登录。

#### Q: Web端登录和TV端登录有什么区别？

| 登录方式      | 命令             | 用途                                          |
| ------------- | ---------------- | --------------------------------------------- |
| **Web端登录** | `--login-qrcode` | 获取Cookie，适用于普通下载                    |
| **TV端登录**  | `--login-tv`     | 获取access_token，访问TV端API，获取无水印片源 |

#### Q: 二维码在终端显示不正常？

程序会同时保存二维码为 `qrcode.png` 图片文件，可以打开图片扫描。

#### Q: 凭证保存在哪里？

使用 `--config-file` 参数时，凭证会保存到同目录下的 `auth.toml` 文件中。不指定配置文件时，凭证仅在本次会话有效。

</details>

## 🗺️ 路线图

| 版本       | 状态       | 主要功能                                  |
| ---------- | ---------- | ----------------------------------------- |
| **v0.2.0** | ✅ 已完成   | 番剧、课程、批量下载、弹幕、TV/APP API    |
| **v0.2.5** | ✅ 已完成   | 二维码登录、认证模块、凭证存储            |
| **v0.2.7** | ✅ 当前版本 | Aria2c 支持、杜比视界/全景声、Hi-Res 音频 |
| **v0.3.0** | 🚧 计划中   | MP4Box 混流、凭证自动刷新、性能优化       |
| **v1.0.0** | 📋 长期目标 | 多平台支持、GUI 界面、下载队列、断点续传  |

<details>
<summary><b>查看详细路线图</b></summary>

### v0.2.0（已完成）✅
- [x] 番剧和课程下载支持
- [x] 批量下载功能（收藏夹、UP主空间、合集、系列）
- [x] 弹幕下载（XML/ASS 格式）
- [x] TV/APP/国际版 API 支持
- [x] 章节信息提取和嵌入

### v0.2.5（已完成）✅
- [x] 二维码登录（Web端和TV端）
- [x] 完整的认证模块架构
- [x] 安全的凭证存储（独立auth.toml文件）
- [x] 跨平台二维码显示支持

### v0.2.7（当前版本）✅
- [x] Aria2c 下载引擎支持
- [x] 杜比视界和杜比全景声支持
- [x] Hi-Res 无损音频（FLAC）支持
- [x] FFmpeg 版本检测

### v0.3.0（计划中）🚧
- [ ] MP4Box 混流支持（完整实现）
- [ ] 凭证自动刷新
- [ ] 性能优化和内存使用改进
- [ ] 更多视频平台支持

### v1.0.0（长期目标）📋
- [ ] 完整的 BBDown 功能对等
- [ ] 多平台支持（YouTube、爱奇艺等）
- [ ] GUI 界面
- [ ] 下载队列管理
- [ ] 断点续传支持

</details>

## 🤝 贡献

欢迎提交 Issue 和 Pull Request！

### 贡献指南

在提交 PR 前，请确保：
- ✅ 代码通过 `cargo test`
- ✅ 代码通过 `cargo clippy`
- ✅ 代码已格式化 `cargo fmt`

### 报告问题

如果你发现了 bug 或有功能建议，请在 [GitHub Issues](https://github.com/SpenserCai/rust-video-downloader/issues) 中提交。

---

## 📄 许可证

本项目采用 [MIT License](LICENSE) 开源协议。

## ⚠️ 免责声明

本项目仅供个人学习、研究和非商业性用途。用户在使用本工具时，需自行确保遵守相关法律法规，特别是与版权相关的法律条款。开发者不对因使用本工具而产生的任何版权纠纷或法律责任承担责任。

## 🙏 致谢

- [BBDown](https://github.com/nilaoda/BBDown) - 设计思路参考
- [bilibili-API-collect](https://github.com/SocialSisterYi/bilibili-API-collect) - API 文档参考
- Rust 社区的优秀开源项目

---

<div align="center">

**如果这个项目对你有帮助，请给它一个 ⭐️**

Made with ❤️ by [RVD Contributors](https://github.com/SpenserCai/rust-video-downloader/graphs/contributors)

</div>
