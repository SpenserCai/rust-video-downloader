# RVD - Rust Video Downloader

一个使用 Rust 编写的模块化视频下载工具，支持从哔哩哔哩等视频平台下载视频。

## 特性

### 已实现功能

- ✅ 支持哔哩哔哩普通视频下载（BV/av 号）
- ✅ 多线程分块下载，提高下载速度
- ✅ 智能流选择（自动选择最佳视频和音频流）
- ✅ 自动混流（使用 FFmpeg）
- ✅ 字幕下载和转换（JSON → SRT）
- ✅ 封面图片下载
- ✅ 多分P视频支持（支持选择特定分P或范围）
- ✅ 清晰度和编码格式优先级设置
- ✅ 交互式清晰度选择模式
- ✅ 灵活的配置文件支持（TOML格式）
- ✅ 自定义输出文件名模板
- ✅ Cookie认证支持（下载会员内容）
- ✅ 详细的下载进度显示
- ✅ 模块化、可扩展的架构
- ✅ 支持 AVC/HEVC/AV1 编码
- ✅ 番剧下载（ep/ss 链接）
- ✅ 课程下载（cheese 链接）
- ✅ 批量下载（收藏夹、UP主空间、合集、系列）
- ✅ 弹幕下载（XML/ASS 格式）
- ✅ 章节信息提取和嵌入
- ✅ TV/APP API 支持（无水印片源）
- ✅ 国际版 API 支持

### 计划支持功能

- ⬜ 8K/HDR/杜比视界/杜比全景声支持
- ⬜ Aria2c 下载支持
- ⬜ MP4Box 混流支持
- ⬜ 二维码登录

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

或使用 av 号：

```bash
rvd av170001
```

### 指定清晰度和编码

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

### 交互式选择

使用交互式模式手动选择清晰度和编码：

```bash
rvd BV1xx411c7mD -i
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

如果需要下载会员视频或高清晰度内容，可以提供 Cookie：

```bash
rvd BV1xx411c7mD --cookie "SESSDATA=your_sessdata_here"
```

或使用 Access Token（用于 TV/APP API，未来支持）：

```bash
rvd BV1xx411c7mD --access-token "your_token_here"
```

认证信息也可以保存在配置文件中，避免每次输入。

### 自定义输出路径

```bash
rvd BV1xx411c7mD -o "downloads/<videoTitle>.mp4"
```

支持的模板变量：
- `<videoTitle>`: 视频标题
- `<pageNumber>`: 分P编号
- `<pageNumberWithZero>`: 分P编号（补零，如 01, 02）
- `<pageTitle>`: 分P标题
- `<quality>`: 清晰度（如 1080P 高清）
- `<codec>`: 编码格式（如 AVC, HEVC）
- `<uploader>`: UP主名称
- `<uploaderMid>`: UP主 mid
- `<bvid>`: BV号
- `<cid>`: 视频 cid
- `<date>`: 上传日期

示例：
```bash
# 单P视频
rvd BV1xx411c7mD -o "<videoTitle>_<quality>.mp4"

# 多P视频
rvd BV1xx411c7mD -o "<videoTitle>/P<pageNumberWithZero>_<pageTitle>.mp4"
```

### 仅查看视频信息

```bash
rvd BV1xx411c7mD --info-only
```

### 跳过字幕或封面

跳过字幕下载：

```bash
rvd BV1xx411c7mD --skip-subtitle
```

跳过封面下载：

```bash
rvd BV1xx411c7mD --skip-cover
```

跳过混流（保留分离的视频和音频文件）：

```bash
rvd BV1xx411c7mD --skip-mux
```

### 指定下载线程数

```bash
rvd BV1xx411c7mD -t 8
```

### 启用详细日志

```bash
rvd BV1xx411c7mD -v
```

### 指定 FFmpeg 路径

如果 FFmpeg 不在系统 PATH 中：

```bash
rvd BV1xx411c7mD --ffmpeg-path /path/to/ffmpeg
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

# 单P视频输出文件名模板
output_template = "<videoTitle>_<quality>"

# 多P视频输出文件名模板
multi_output_template = "<videoTitle>/P<pageNumberWithZero>_<pageTitle>"

# 认证信息
[auth]
cookie = "SESSDATA=your_sessdata_here"
# access_token = ""

# 外部工具路径
[paths]
ffmpeg = "/usr/local/bin/ffmpeg"
```

配置文件中的设置会被命令行参数覆盖。

### 下载番剧和课程

下载番剧（通过 ep 或 ss 链接）：

```bash
# 通过 ep 链接下载
rvd "https://www.bilibili.com/bangumi/play/ep123456"

# 通过 ss 链接下载整季
rvd "https://www.bilibili.com/bangumi/play/ss12345"

# 下载特定集数
rvd ep123456 -p "1,2,3"
```

下载课程：

```bash
rvd "https://www.bilibili.com/cheese/play/ep123456"
```

### 批量下载

下载收藏夹中的所有视频：

```bash
rvd "https://space.bilibili.com/{mid}/favlist?fid={fav_id}"
```

下载UP主空间的所有视频：

```bash
rvd "https://space.bilibili.com/{mid}"
```

下载合集：

```bash
rvd "https://www.bilibili.com/medialist/play/ml{media_id}"
```

下载系列：

```bash
rvd "https://space.bilibili.com/{mid}/channel/seriesdetail?sid={series_id}"
```

### 下载弹幕

下载 XML 格式弹幕：

```bash
rvd BV1xx411c7mD --download-danmaku --danmaku-format xml
```

下载 ASS 格式弹幕（默认）：

```bash
rvd BV1xx411c7mD --download-danmaku
```

### 使用不同 API 模式

使用 TV API（获取无水印片源）：

```bash
rvd BV1xx411c7mD --use-tv-api
```

使用 APP API：

```bash
rvd BV1xx411c7mD --use-app-api
```

使用国际版 API：

```bash
rvd BV1xx411c7mD --use-intl-api
```

## 命令行参数

```
Usage: rvd [OPTIONS] <URL>

Arguments:
  <URL>
          视频 URL（支持 bilibili BV/av/ep/ss/cheese 号及批量链接）
          示例: BV1xx411c7mD, av170001, ep123456, ss12345
                https://www.bilibili.com/video/BV1xx411c7mD
                https://www.bilibili.com/bangumi/play/ep123456
                https://space.bilibili.com/{mid}

Options:
  -q, --quality <QUALITY>
          清晰度优先级（逗号分隔）
          示例: "1080P,720P,480P"
          可用选项: 8K 超高清, 4K 超清, 1080P 60帧, 1080P 高码率, 1080P 高清, 
                   720P 60帧, 720P 高清, 480P 清晰, 360P 流畅

  -c, --codec <CODEC>
          编码格式优先级（逗号分隔）
          示例: "hevc,avc,av1"
          可用选项: avc (H.264), hevc (H.265), av1

  -o, --output <OUTPUT>
          输出文件路径或模板
          支持变量: <videoTitle>, <pageNumber>, <pageNumberWithZero>, <pageTitle>,
                   <quality>, <codec>, <uploader>, <uploaderMid>, <bvid>, <cid>, <date>

  -p, --pages <PAGES>
          选择特定分P或集数
          示例: "1" (单个), "1,2,5" (多个), "1-5" (范围), "ALL" (全部)

  -t, --threads <THREADS>
          下载线程数
          [default: 4]

  -i, --interactive
          交互式清晰度选择模式

      --cookie <COOKIE>
          用于认证的 Cookie 字符串（用于下载会员内容）

      --access-token <ACCESS_TOKEN>
          用于认证的 Access Token（用于 TV/APP API）

      --skip-subtitle
          跳过字幕下载

      --skip-cover
          跳过封面下载

      --skip-mux
          跳过混流（保留分离的视频和音频文件）

      --info-only
          仅显示视频信息，不进行下载

      --config-file <CONFIG_FILE>
          指定配置文件路径

      --ffmpeg-path <FFMPEG_PATH>
          指定 FFmpeg 可执行文件路径

      --use-tv-api
          使用 TV API 模式（获取无水印片源）

      --use-app-api
          使用 APP API 模式（支持杜比音频）

      --use-intl-api
          使用国际版 API 模式

      --download-danmaku
          下载弹幕文件

      --danmaku-format <FORMAT>
          弹幕格式（xml 或 ass）
          [default: ass]

  -v, --verbose
          启用详细日志输出

  -h, --help
          显示帮助信息

  -V, --version
          显示版本信息
```

## 技术特点

### 模块化架构

RVD 采用分层架构设计，各层职责清晰：

- **CLI 层**: 命令行参数解析和用户交互
- **应用层**: 协调各模块完成下载任务
- **平台层**: 特定视频平台的实现（可扩展）
- **核心层**: 通用的下载、混流、进度跟踪功能
- **工具层**: HTTP 客户端、配置管理、文件操作

### 可扩展设计

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

### 性能优化

- 多线程分块下载，充分利用带宽
- 流式写入，避免内存占用过大
- 智能重试机制，提高下载成功率
- 异步 I/O，提升并发性能

## 开发

### 构建项目

```bash
# 开发构建
cargo build

# 发布构建（优化）
cargo build --release
```

### 运行测试

```bash
# 运行所有测试
cargo test

# 运行特定测试
cargo test test_parse_video_info

# 显示测试输出
cargo test -- --nocapture
```

### 代码质量检查

```bash
# Clippy 检查
cargo clippy -- -D warnings

# 代码格式化
cargo fmt

# 格式检查（不修改文件）
cargo fmt -- --check
```

### 项目结构

```
src/
├── cli/           # 命令行参数解析
├── app/           # 应用协调逻辑
├── platform/      # 平台特定实现
│   ├── bilibili/  # 哔哩哔哩平台
│   └── trait.rs   # 平台接口定义
├── core/          # 核心功能
│   ├── downloader.rs  # 下载引擎
│   ├── muxer.rs       # 混流器
│   ├── progress.rs    # 进度跟踪
│   └── subtitle.rs    # 字幕处理
├── utils/         # 工具模块
│   ├── http.rs    # HTTP 客户端
│   ├── config.rs  # 配置管理
│   └── file.rs    # 文件操作
├── error.rs       # 错误类型定义
├── types.rs       # 数据结构定义
└── main.rs        # 程序入口
```

## 常见问题

### FFmpeg 相关

**Q: 提示找不到 FFmpeg？**

A: 请确保 FFmpeg 已安装并在系统 PATH 中，或使用 `--ffmpeg-path` 参数指定路径。

**Q: 如何安装 FFmpeg？**

A: 
- macOS: `brew install ffmpeg`
- Ubuntu/Debian: `sudo apt install ffmpeg`
- Windows: 从 [FFmpeg 官网](https://ffmpeg.org/download.html) 下载

### 下载相关

**Q: 下载速度慢？**

A: 可以尝试增加线程数：`rvd <url> -t 8`

**Q: 下载失败或中断？**

A: 程序会自动重试 3 次。如果仍然失败，请检查网络连接或尝试使用认证信息。

**Q: 如何下载会员视频？**

A: 需要提供有效的 Cookie（SESSDATA）：`rvd <url> --cookie "SESSDATA=..."`

### 认证相关

**Q: 如何获取 Cookie？**

A: 
1. 在浏览器中登录 bilibili.com
2. 打开开发者工具（F12）
3. 在 Application/Storage > Cookies 中找到 SESSDATA
4. 复制其值使用

**Q: Cookie 会过期吗？**

A: 是的，Cookie 有效期通常为几个月。过期后需要重新获取。

## 路线图

### v0.2.0（已完成）✅
- [x] 番剧和课程下载支持
- [x] 批量下载功能（收藏夹、UP主空间、合集、系列）
- [x] 弹幕下载（XML/ASS 格式）
- [x] TV/APP/国际版 API 支持
- [x] 章节信息提取和嵌入

### v0.3.0（计划中）
- [ ] 8K/HDR/杜比视界/杜比全景声完整支持
- [ ] Aria2c 下载引擎支持
- [ ] MP4Box 混流支持
- [ ] 二维码登录
- [ ] 性能优化和内存使用改进

### v1.0.0（长期目标）
- [ ] 完整的 BBDown 功能对等
- [ ] 多平台支持（YouTube、爱奇艺等）
- [ ] GUI 界面
- [ ] 下载队列管理
- [ ] 断点续传支持

## 贡献

欢迎提交 Issue 和 Pull Request！

在提交 PR 前，请确保：
- 代码通过 `cargo test`
- 代码通过 `cargo clippy`
- 代码已格式化 `cargo fmt`

## 许可证

MIT License

## 免责声明

本项目仅供个人学习、研究和非商业性用途。用户在使用本工具时，需自行确保遵守相关法律法规，特别是与版权相关的法律条款。开发者不对因使用本工具而产生的任何版权纠纷或法律责任承担责任。

## 致谢

- [BBDown](https://github.com/nilaoda/BBDown) - 设计思路参考
- [bilibili-API-collect](https://github.com/SocialSisterYi/bilibili-API-collect) - API 文档参考
- Rust 社区的优秀开源项目
