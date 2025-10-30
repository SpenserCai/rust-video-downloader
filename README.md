# RVD - Rust Video Downloader

一个使用 Rust 编写的模块化视频下载工具，支持从哔哩哔哩等视频平台下载视频。

## 特性

- ✅ 支持哔哩哔哩视频下载（BV/av 号）
- ✅ 多线程分块下载，提高下载速度
- ✅ 自动选择最佳视频和音频流
- ✅ 自动混流（使用 FFmpeg）
- ✅ 字幕下载和转换（JSON → SRT）
- ✅ 封面下载
- ✅ 多分P视频支持
- ✅ 灵活的配置选项
- ✅ 模块化、可扩展的架构

## 安装

### 前置要求

- Rust 1.70 或更高版本
- FFmpeg（用于混流）

### 从源码编译

```bash
git clone <repository-url>
cd rust-video-downloader
cargo build --release
```

编译完成后，可执行文件位于 `target/release/rvd`

### 安装到系统

```bash
cargo install --path .
```

## 快速开始

### 基本用法

下载单个视频（使用默认设置）：

```bash
rvd "https://www.bilibili.com/video/BV1xx411c7mD"
```

或使用 BV 号：

```bash
rvd BV1xx411c7mD
```

### 指定清晰度和编码

```bash
rvd BV1xx411c7mD -q "1080P,720P" -c "hevc,avc"
```

### 下载特定分P

下载第 1 个分P：
```bash
rvd BV1xx411c7mD -p 1
```

下载多个分P：
```bash
rvd BV1xx411c7mD -p "1,2,5"
```

下载分P范围：
```bash
rvd BV1xx411c7mD -p "1-5"
```

下载所有分P：
```bash
rvd BV1xx411c7mD -p ALL
```

### 使用认证

如果需要下载会员视频，可以提供 Cookie：

```bash
rvd BV1xx411c7mD --cookie "SESSDATA=your_sessdata_here"
```

### 自定义输出路径

```bash
rvd BV1xx411c7mD -o "downloads/<videoTitle>.mp4"
```

支持的模板变量：
- `<videoTitle>`: 视频标题
- `<pageNumber>`: 分P编号
- `<pageNumberWithZero>`: 分P编号（补零）
- `<pageTitle>`: 分P标题
- `<quality>`: 清晰度
- `<codec>`: 编码格式
- `<uploader>`: UP主名称
- `<uploaderMid>`: UP主 mid
- `<bvid>`: BV号
- `<date>`: 上传日期

### 仅查看视频信息

```bash
rvd BV1xx411c7mD --info-only
```

### 跳过字幕或封面

```bash
rvd BV1xx411c7mD --skip-subtitle --skip-cover
```

### 启用详细日志

```bash
rvd BV1xx411c7mD -v
```

## 配置文件

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

# 输出文件名模板
output_template = "<videoTitle>/<pageTitle>"

# 认证信息
[auth]
cookie = "SESSDATA=your_sessdata_here"
# access_token = ""

# 外部工具路径
[paths]
ffmpeg = "/usr/local/bin/ffmpeg"
```

## 命令行参数

```
Usage: rvd [OPTIONS] <URL>

Arguments:
  <URL>  视频 URL（支持 bilibili BV/av/ep/ss）

Options:
  -q, --quality <QUALITY>
          清晰度优先级（逗号分隔，如 "1080P,720P,480P"）

  -c, --codec <CODEC>
          编码优先级（逗号分隔，如 "hevc,avc,av1"）

  -o, --output <OUTPUT>
          输出文件路径或模板

      --cookie <COOKIE>
          用于认证的 Cookie 字符串

      --access-token <ACCESS_TOKEN>
          用于认证的 Access Token

  -p, --pages <PAGES>
          选择特定分P（如 "1", "1,2,3", "1-5", "ALL"）

  -t, --threads <THREADS>
          下载线程数 [default: 4]

      --skip-subtitle
          跳过字幕下载

      --skip-cover
          跳过封面下载

      --skip-mux
          跳过混流（保留分离的视频和音频文件）

  -i, --interactive
          交互式清晰度选择

      --config-file <CONFIG_FILE>
          配置文件路径

  -v, --verbose
          启用详细日志

      --info-only
          仅显示视频信息（不下载）

      --ffmpeg-path <FFMPEG_PATH>
          FFmpeg 可执行文件路径

  -h, --help
          显示帮助信息

  -V, --version
          显示版本信息
```

## 架构设计

RVD 采用模块化、可扩展的架构设计：

```
┌─────────────────────────────────────────┐
│           CLI Layer (clap)              │
└─────────────────┬───────────────────────┘
                  │
┌─────────────────▼───────────────────────┐
│        Application Layer                │
│  (Orchestrator, Task Manager)           │
└─────────────────┬───────────────────────┘
                  │
┌─────────────────▼───────────────────────┐
│         Platform Layer                  │
│  ┌──────────┐  ┌──────────┐            │
│  │ Bilibili │  │  Future  │            │
│  │ Platform │  │ Platforms│            │
│  └──────────┘  └──────────┘            │
└─────────────────┬───────────────────────┘
                  │
┌─────────────────▼───────────────────────┐
│          Core Layer                     │
│  (Downloader, Muxer, Progress)          │
└─────────────────┬───────────────────────┘
                  │
┌─────────────────▼───────────────────────┐
│         Utility Layer                   │
│  (HTTP Client, Config, File Utils)      │
└─────────────────────────────────────────┘
```

### 扩展新平台

要添加对新视频平台的支持，只需：

1. 在 `src/platform/` 下创建新模块
2. 实现 `Platform` trait
3. 在 `Orchestrator` 中注册新平台

详见设计文档。

## 开发

### 运行测试

```bash
cargo test
```

### 代码检查

```bash
cargo clippy -- -D warnings
```

### 代码格式化

```bash
cargo fmt
```

## 许可证

MIT License

## 免责声明

本项目仅供个人学习、研究和非商业性用途。用户在使用本工具时，需自行确保遵守相关法律法规，特别是与版权相关的法律条款。开发者不对因使用本工具而产生的任何版权纠纷或法律责任承担责任。

## 致谢

本项目参考了 [BBDown](https://github.com/nilaoda/BBDown) 的设计思路。
