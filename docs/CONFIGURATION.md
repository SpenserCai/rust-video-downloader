# 配置文件详解

本文档详细说明 RVD Next 的配置文件格式和所有可用选项。

## 配置文件位置

RVD Next 支持多个配置文件位置，按以下优先级加载：

1. **命令行指定**: `--config-file <path>`
2. **当前目录**: `./rvd.toml` 或 `./config.toml`
3. **用户配置目录**:
   - Linux/macOS: `~/.config/rvd/config.toml`
   - Windows: `%APPDATA%\rvd\config.toml`

## 完整配置示例

```toml
# RVD Next 配置文件示例

# ============================================================================
# HTTP 配置
# ============================================================================
[http]
# 自定义 User-Agent
# 如果不设置，会使用随机生成的 User-Agent
user_agent = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36"

# 是否在日志中显示 User-Agent
log_user_agent = true

# 请求超时时间（秒）
timeout = 30

# 最大重试次数
max_retries = 3

# ============================================================================
# Aria2c 配置
# ============================================================================
[aria2c]
# 是否启用 Aria2c 下载
enabled = false

# Aria2c 可执行文件路径
# 如果不设置，会在 PATH 中查找
path = "/usr/bin/aria2c"

# Aria2c 自定义参数
args = [
    "--max-connection-per-server=16",
    "--split=16",
    "--min-split-size=1M",
    "--max-concurrent-downloads=5",
    "--continue=true",
    "--max-tries=5",
    "--retry-wait=3"
]

# ============================================================================
# 路径配置
# ============================================================================
[paths]
# FFmpeg 可执行文件路径
ffmpeg = "/usr/local/bin/ffmpeg"

# MP4Box 可执行文件路径（可选）
mp4box = "/usr/local/bin/MP4Box"

# 默认输出目录
output_dir = "~/Downloads/videos"

# 临时文件目录
temp_dir = "/tmp/rvd"

# ============================================================================
# 认证配置
# ============================================================================
[auth]
# Cookie 认证
# 格式: "key1=value1;key2=value2"
cookie = "SESSDATA=xxx;bili_jct=xxx;DedeUserID=xxx"

# Access Token 认证（可选）
# access_token = "your_access_token_here"

# 认证信息过期时间（天）
expire_days = 30

# ============================================================================
# 下载配置
# ============================================================================
[download]
# 默认线程数
threads = 8

# 默认清晰度优先级
quality_priority = ["8K", "4K", "1080P+", "1080P", "720P"]

# 默认编码优先级
codec_priority = ["HEVC", "AVC", "AV1"]

# 是否跳过字幕下载
skip_subtitle = false

# 是否跳过封面下载
skip_cover = false

# 是否下载弹幕
download_danmaku = false

# 弹幕格式 ("xml" 或 "ass")
danmaku_format = "ass"

# 批量下载限制（0 表示无限制）
batch_limit = 0

# 最大下载视频数（0 表示无限制）
max_videos = 0

# ============================================================================
# 输出配置
# ============================================================================
[output]
# 文件名模板
# 支持的变量:
#   {title}       - 视频标题
#   {uploader}    - UP主名称
#   {bvid}        - BV号
#   {aid}         - AV号
#   {cid}         - CID
#   {page}        - 分P编号
#   {page_title}  - 分P标题
#   {quality}     - 清晰度
#   {codec}       - 编码格式
#   {date}        - 当前日期 (YYYY-MM-DD)
#   {time}        - 当前时间 (HH-MM-SS)
template = "{uploader}/{title}.mp4"

# 多分P视频的文件名模板
multi_page_template = "{uploader}/{title}/P{page:02}_{page_title}.mp4"

# 是否创建子目录
create_subdirs = true

# 文件名最大长度
max_filename_length = 200

# 非法字符替换
# 将文件名中的非法字符替换为指定字符
illegal_char_replacement = "_"

# ============================================================================
# 混流配置
# ============================================================================
[muxer]
# 混流工具 ("ffmpeg" 或 "mp4box")
tool = "ffmpeg"

# 是否跳过混流
skip_mux = false

# FFmpeg 额外参数
ffmpeg_args = ["-c", "copy"]

# 是否嵌入字幕
embed_subtitle = true

# 是否嵌入章节
embed_chapters = true

# 是否嵌入封面
embed_cover = false

# ============================================================================
# 平台特定配置
# ============================================================================

# ----------------------------------------------------------------------------
# Bilibili 配置
# ----------------------------------------------------------------------------
[platforms.bilibili]
# 默认 API 模式 ("web", "tv", "app", "international")
api_mode = "web"

# CDN 优化配置
[platforms.bilibili.cdn]
# 是否启用 CDN 优化
enabled = true

# 备用 CDN 主机列表
# 当检测到 PCDN 或不稳定节点时，会尝试替换为这些主机
backup_hosts = [
    "upos-sz-mirrorcos.bilivideo.com",
    "upos-sz-mirrorhw.bilivideo.com",
    "upos-sz-mirrorali.bilivideo.com"
]

# PCDN 检测关键字
# 包含这些关键字的 URL 会被视为 PCDN
pcdn_keywords = [":8080", "mcdn.bilivideo.cn"]

# 是否在日志中显示 CDN 优化信息
log_optimization = true

# Bilibili 认证配置
[platforms.bilibili.auth]
# 默认登录模式 ("qrcode" 或 "tv")
default_login_mode = "qrcode"

# 二维码刷新间隔（秒）
qrcode_poll_interval = 3

# 二维码过期时间（秒）
qrcode_expire_time = 180

# Bilibili 下载配置
[platforms.bilibili.download]
# 是否优先使用 TV API（无水印）
prefer_tv_api = false

# 是否下载杜比视界内容
download_dolby_vision = true

# 是否下载杜比全景声
download_dolby_atmos = true

# 是否下载 Hi-Res 音频
download_hires_audio = true

# ----------------------------------------------------------------------------
# YouTube 配置（示例，未实现）
# ----------------------------------------------------------------------------
[platforms.youtube]
# API 密钥（可选）
# api_key = "your_youtube_api_key"

# 是否使用 yt-dlp 作为后端
# use_ytdlp = false

# 默认字幕语言
# subtitle_lang = "en"

# ============================================================================
# 日志配置
# ============================================================================
[logging]
# 日志级别 ("error", "warn", "info", "debug", "trace")
level = "info"

# 日志输出格式 ("compact", "pretty", "json")
format = "compact"

# 是否输出到文件
log_to_file = false

# 日志文件路径
log_file = "~/.local/share/rvd/rvd.log"

# 日志文件最大大小（MB）
max_log_size = 10

# 保留的日志文件数量
max_log_files = 5

# ============================================================================
# 进度显示配置
# ============================================================================
[progress]
# 进度条样式 ("bar", "spinner", "none")
style = "bar"

# 是否显示下载速度
show_speed = true

# 是否显示 ETA
show_eta = true

# 是否显示已下载大小
show_downloaded = true

# 刷新间隔（毫秒）
refresh_interval = 100

# ============================================================================
# 高级配置
# ============================================================================
[advanced]
# 是否启用实验性功能
enable_experimental = false

# 并发下载数（批量下载时）
concurrent_downloads = 3

# 内存缓冲区大小（MB）
buffer_size = 8

# 是否启用断点续传
resume_download = true

# 下载失败后的重试延迟（秒）
retry_delay = 5

# 是否在下载完成后验证文件
verify_download = false
```

## 配置项详解

### HTTP 配置

#### user_agent

自定义 HTTP 请求的 User-Agent。

- **类型**: String
- **默认值**: 随机生成
- **示例**: `"Mozilla/5.0 ..."`

#### log_user_agent

是否在日志中显示使用的 User-Agent。

- **类型**: Boolean
- **默认值**: `false`

### Aria2c 配置

#### enabled

是否启用 Aria2c 下载器。

- **类型**: Boolean
- **默认值**: `false`
- **说明**: Aria2c 通常比内置下载器更快

#### path

Aria2c 可执行文件的路径。

- **类型**: String
- **默认值**: 在 PATH 中查找
- **示例**: `"/usr/bin/aria2c"`

#### args

传递给 Aria2c 的额外参数。

- **类型**: Array of Strings
- **默认值**: `[]`
- **常用参数**:
  - `--max-connection-per-server=N`: 每个服务器的最大连接数
  - `--split=N`: 分块数量
  - `--min-split-size=SIZE`: 最小分块大小
  - `--continue=true`: 启用断点续传

### 认证配置

#### cookie

用于认证的 Cookie 字符串。

- **类型**: String
- **格式**: `"key1=value1;key2=value2"`
- **Bilibili 需要的 Cookie**:
  - `SESSDATA`: 会话标识
  - `bili_jct`: CSRF Token
  - `DedeUserID`: 用户 ID

#### access_token

用于认证的 Access Token（某些平台）。

- **类型**: String
- **默认值**: None

### 下载配置

#### threads

下载时使用的线程数。

- **类型**: Integer
- **默认值**: `8`
- **范围**: 1-32
- **说明**: 更多线程可能提高速度，但也会增加服务器负载

#### quality_priority

清晰度优先级列表。

- **类型**: Array of Strings
- **默认值**: `["8K", "4K", "1080P+", "1080P", "720P"]`
- **可用值**: `"8K"`, `"4K"`, `"1080P60"`, `"1080P+"`, `"1080P"`, `"720P60"`, `"720P"`, `"480P"`, `"360P"`

#### codec_priority

编码格式优先级列表。

- **类型**: Array of Strings
- **默认值**: `["HEVC", "AVC", "AV1"]`
- **可用值**: `"AV1"`, `"HEVC"`, `"AVC"`

### 输出配置

#### template

文件名模板。

- **类型**: String
- **默认值**: `"{uploader}/{title}.mp4"`
- **可用变量**:
  - `{title}`: 视频标题
  - `{uploader}`: UP主名称
  - `{bvid}`: BV号
  - `{aid}`: AV号
  - `{cid}`: CID
  - `{page}`: 分P编号
  - `{page_title}`: 分P标题
  - `{quality}`: 清晰度
  - `{codec}`: 编码格式
  - `{date}`: 当前日期
  - `{time}`: 当前时间

#### multi_page_template

多分P视频的文件名模板。

- **类型**: String
- **默认值**: `"{uploader}/{title}/P{page:02}_{page_title}.mp4"`
- **说明**: `{page:02}` 表示分P编号补零到2位

### Bilibili CDN 配置

#### backup_hosts

备用 CDN 主机列表。

- **类型**: Array of Strings
- **默认值**: `[]`
- **说明**: 当检测到不稳定的 CDN 节点时，会尝试替换为这些主机
- **推荐值**:
  ```toml
  backup_hosts = [
      "upos-sz-mirrorcos.bilivideo.com",
      "upos-sz-mirrorhw.bilivideo.com",
      "upos-sz-mirrorali.bilivideo.com"
  ]
  ```

#### pcdn_keywords

PCDN 检测关键字。

- **类型**: Array of Strings
- **默认值**: `[":8080", "mcdn.bilivideo.cn"]`
- **说明**: 包含这些关键字的 URL 会被视为 PCDN（不稳定）

## 配置优先级

配置项的优先级从高到低：

1. **命令行参数**: 最高优先级
2. **环境变量**: 某些配置支持环境变量
3. **配置文件**: 用户配置文件
4. **默认值**: 内置默认值

例如：

```bash
# 配置文件中设置 threads = 8
# 但命令行参数会覆盖它
rvd <URL> --threads 16  # 实际使用 16 个线程
```

## 环境变量

某些配置可以通过环境变量设置：

- `RUST_LOG`: 日志级别（`error`, `warn`, `info`, `debug`, `trace`）
- `RVD_CONFIG`: 配置文件路径
- `RVD_COOKIE`: Cookie 字符串
- `RVD_ACCESS_TOKEN`: Access Token

示例：

```bash
export RUST_LOG=debug
export RVD_COOKIE="SESSDATA=xxx;bili_jct=xxx"
rvd <URL>
```

## 配置文件示例

### 最小配置

```toml
[auth]
cookie = "SESSDATA=xxx;bili_jct=xxx"
```

### 高性能配置

```toml
[aria2c]
enabled = true
args = [
    "--max-connection-per-server=16",
    "--split=16",
    "--min-split-size=1M"
]

[download]
threads = 16
quality_priority = ["8K", "4K"]
codec_priority = ["HEVC", "AVC"]

[platforms.bilibili.cdn]
enabled = true
backup_hosts = [
    "upos-sz-mirrorcos.bilivideo.com",
    "upos-sz-mirrorhw.bilivideo.com"
]
```

### 批量下载配置

```toml
[download]
batch_limit = 100
max_videos = 50
skip_subtitle = true
skip_cover = true

[output]
template = "downloads/{uploader}/{title}.mp4"
create_subdirs = true

[advanced]
concurrent_downloads = 5
```

### 开发调试配置

```toml
[http]
log_user_agent = true

[logging]
level = "debug"
log_to_file = true
log_file = "rvd.log"

[platforms.bilibili.cdn]
log_optimization = true
```

## 配置验证

使用 `--config-file` 参数可以验证配置文件：

```bash
# 验证配置文件
rvd --config-file config.toml --info-only <URL>
```

如果配置文件有错误，会显示详细的错误信息。

## 常见配置场景

### 场景 1: 下载会员内容

```toml
[auth]
cookie = "SESSDATA=xxx;bili_jct=xxx;DedeUserID=xxx"

[download]
quality_priority = ["1080P+", "1080P"]

[platforms.bilibili]
api_mode = "tv"
```

### 场景 2: 批量下载收藏夹

```toml
[download]
max_videos = 100
skip_subtitle = true
skip_cover = true

[output]
template = "favorites/{uploader}/{title}.mp4"

[advanced]
concurrent_downloads = 3
```

### 场景 3: 下载杜比视界内容

```toml
[auth]
cookie = "SESSDATA=xxx;bili_jct=xxx"

[download]
quality_priority = ["8K", "4K"]
codec_priority = ["HEVC"]

[platforms.bilibili]
api_mode = "tv"

[platforms.bilibili.download]
download_dolby_vision = true
download_dolby_atmos = true

[paths]
ffmpeg = "/usr/local/bin/ffmpeg"  # 需要 FFmpeg 5.0+
```

### 场景 4: 网络受限环境

```toml
[http]
timeout = 60
max_retries = 5

[download]
threads = 4

[advanced]
retry_delay = 10
buffer_size = 4
```

## 故障排除

### 配置文件不生效

1. 检查配置文件路径是否正确
2. 检查 TOML 语法是否正确
3. 使用 `--verbose` 查看加载的配置

### Cookie 认证失败

1. 确保 Cookie 格式正确
2. 检查 Cookie 是否过期
3. 尝试重新登录获取新 Cookie

### CDN 优化不生效

1. 确保 `platforms.bilibili.cdn.enabled = true`
2. 检查 `backup_hosts` 是否配置
3. 使用 `log_optimization = true` 查看优化日志

## 更多信息

- [用户指南](USER_GUIDE.md) - 详细的使用说明
- [架构设计](ARCHITECTURE.md) - 了解内部实现
- [开发指南](DEVELOPMENT.md) - 贡献代码
