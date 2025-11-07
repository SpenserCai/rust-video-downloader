# RVD Next 用户指南

本指南将帮助你快速上手 RVD Next，掌握各种使用场景和高级功能。

## 目录

- [基础使用](#基础使用)
- [认证登录](#认证登录)
- [下载选项](#下载选项)
- [批量下载](#批量下载)
- [高级功能](#高级功能)
- [配置文件](#配置文件)
- [常见问题](#常见问题)

## 基础使用

### 下载单个视频

最简单的用法，直接提供视频 URL：

```bash
rvd https://www.bilibili.com/video/BV1xx411c7mD
```

### 查看视频信息（不下载）

使用 `--info-only` 参数查看视频信息：

```bash
rvd https://www.bilibili.com/video/BV1xx411c7mD --info-only
```

输出示例：
```
📹 Video Information:
  Title: 视频标题
  Uploader: UP主名称
  Pages: 3
  Description: 视频简介...
```

### 指定输出路径

使用 `--output` 或 `-o` 参数指定输出路径：

```bash
# 指定文件名
rvd <URL> -o "我的视频.mp4"

# 指定目录（自动生成文件名）
rvd <URL> -o "downloads/"

# 使用模板变量
rvd <URL> -o "downloads/{uploader}/{title}.mp4"
```

支持的模板变量：
- `{title}`: 视频标题
- `{uploader}`: UP主名称
- `{bvid}`: BV号
- `{aid}`: AV号
- `{cid}`: CID
- `{page}`: 分P编号
- `{page_title}`: 分P标题
- `{quality}`: 清晰度
- `{codec}`: 编码格式

## 认证登录

### 为什么需要登录？

- 下载会员专享内容
- 下载 1080P 以上清晰度
- 下载付费课程
- 访问私密收藏夹

### 二维码登录（推荐）

#### Web 端登录

```bash
rvd login --mode qrcode
```

扫码后会自动保存认证信息到 `auth.toml`。

#### TV 端登录（获取更高清晰度）

```bash
rvd login --mode tv
```

TV 端登录可以获取无水印的片源，推荐使用。

### Cookie 登录

如果你已经有 Cookie，可以直接使用：

```bash
# 方式一：命令行参数
rvd --cookie "SESSDATA=xxx;bili_jct=xxx" <URL>

# 方式二：配置文件（见配置文件章节）
```

获取 Cookie 的方法：
1. 在浏览器中登录 bilibili.com
2. 打开开发者工具（F12）
3. 切换到 Application/Storage -> Cookies
4. 复制 `SESSDATA` 和 `bili_jct` 的值

### Access Token 登录

```bash
rvd --access-token "your_token" <URL>
```

## 下载选项

### 选择分P

#### 下载特定分P

```bash
# 下载第 1 分P
rvd <URL> --pages 1

# 下载第 1、3、5 分P
rvd <URL> --pages 1,3,5

# 下载第 1-5 分P
rvd <URL> --pages 1-5

# 组合使用
rvd <URL> --pages 1,3-5,7
```

#### 下载所有分P

不指定 `--pages` 参数时，默认下载所有分P。

### 选择清晰度

#### 自动选择（推荐）

使用 `--quality-priority` 指定清晰度优先级：

```bash
# 优先下载 8K，其次 4K，最后 1080P
rvd <URL> --quality-priority "8K,4K,1080P"
```

支持的清晰度：
- `8K`: 8K 超高清
- `4K`: 4K 超清
- `1080P60`: 1080P 60帧
- `1080P+`: 1080P 高码率
- `1080P`: 1080P 高清
- `720P60`: 720P 60帧
- `720P`: 720P 高清
- `480P`: 480P 清晰
- `360P`: 360P 流畅

#### 交互式选择

使用 `--interactive` 参数手动选择：

```bash
rvd <URL> --interactive
```

会显示可用的清晰度列表供你选择。

### 选择编码格式

使用 `--codec-priority` 指定编码优先级：

```bash
# 优先 AV1，其次 HEVC，最后 AVC
rvd <URL> --codec-priority "AV1,HEVC,AVC"
```

支持的编码：
- `AV1`: AV1 编码（最新，压缩率最高）
- `HEVC`: H.265 编码（高压缩率）
- `AVC`: H.264 编码（兼容性最好）

### 组合使用

```bash
rvd <URL> \
  --quality-priority "4K,1080P+" \
  --codec-priority "HEVC,AVC" \
  --pages 1-3
```

## 批量下载

### 下载收藏夹

```bash
# 下载整个收藏夹
rvd "https://space.bilibili.com/123456/favlist?fid=789"

# 限制下载数量（只下载前 10 个）
rvd "https://space.bilibili.com/123456/favlist?fid=789" --max-videos 10
```

### 下载 UP 主所有视频

```bash
# 下载 UP 主的所有投稿
rvd "https://space.bilibili.com/123456/video"

# 只下载前 20 个
rvd "https://space.bilibili.com/123456/video" --max-videos 20
```

### 下载合集

```bash
# 下载视频合集
rvd "https://space.bilibili.com/123456/channel/collectiondetail?sid=789"
```

### 下载系列

```bash
# 下载视频系列
rvd "https://space.bilibili.com/123456/channel/seriesdetail?sid=789"
```

### 下载番剧

```bash
# 使用 ep 链接
rvd "https://www.bilibili.com/bangumi/play/ep123456"

# 使用 ss 链接（下载整季）
rvd "https://www.bilibili.com/bangumi/play/ss12345"
```

### 批量下载限制

为了安全，批量下载有以下限制：

- `--batch-limit`: 硬限制，超过会报错（默认无限制）
- `--max-videos`: 软限制，只下载前 N 个视频

```bash
# 设置硬限制为 100
rvd <URL> --batch-limit 100

# 只下载前 50 个
rvd <URL> --max-videos 50
```

## 高级功能

### 使用不同的 API 模式

Bilibili 提供多种 API 接口：

```bash
# Web API（默认）
rvd <URL> --api-mode web

# TV API（无水印，需要 TV 登录）
rvd <URL> --api-mode tv

# APP API
rvd <URL> --api-mode app

# 国际版 API
rvd <URL> --api-mode international
```

### 下载字幕

```bash
# 自动下载字幕（如果有）
rvd <URL>

# 跳过字幕下载
rvd <URL> --skip-subtitle
```

字幕会自动转换为 SRT 格式并嵌入到视频中。

### 下载弹幕

```bash
# 下载弹幕（XML 格式）
rvd <URL> --download-danmaku

# 下载弹幕（ASS 格式，可用于播放器）
rvd <URL> --download-danmaku --danmaku-format ass
```

弹幕文件会保存在视频文件旁边，文件名相同但扩展名不同。

### 下载封面

```bash
# 自动下载封面（默认）
rvd <URL>

# 跳过封面下载
rvd <URL> --skip-cover
```

### 使用 Aria2c 下载

Aria2c 可以提供更快的下载速度：

```bash
# 启用 Aria2c
rvd <URL> --use-aria2c

# 指定 Aria2c 路径
rvd <URL> --use-aria2c --aria2c-path "/usr/local/bin/aria2c"

# 自定义 Aria2c 参数
rvd <URL> --use-aria2c --aria2c-args "--max-connection-per-server=16 --split=16"
```

### 多线程下载

```bash
# 使用 16 个线程下载
rvd <URL> --threads 16
```

### 跳过混流

如果你只想下载原始文件：

```bash
rvd <URL> --skip-mux
```

会生成 `.video.m4s` 和 `.audio.m4s` 文件。

### 自定义 FFmpeg 路径

```bash
rvd <URL> --ffmpeg-path "/usr/local/bin/ffmpeg"
```

### 使用 MP4Box 混流

```bash
rvd <URL> --use-mp4box
```

### 详细日志

```bash
# 启用详细日志
rvd <URL> --verbose

# 或使用环境变量
RUST_LOG=debug rvd <URL>
```

## 配置文件

### 配置文件位置

RVD Next 支持两种配置文件：

1. **全局配置**: `~/.config/rvd/config.toml`（Linux/macOS）或 `%APPDATA%\rvd\config.toml`（Windows）
2. **项目配置**: 当前目录的 `rvd.toml` 或通过 `--config-file` 指定

### 配置文件示例

创建 `config.toml`：

```toml
# HTTP 配置
[http]
user_agent = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36"
log_user_agent = true

# Aria2c 配置
[aria2c]
enabled = true
path = "/usr/bin/aria2c"
args = [
    "--max-connection-per-server=16",
    "--split=16",
    "--min-split-size=1M"
]

# 路径配置
[paths]
ffmpeg = "/usr/local/bin/ffmpeg"
output_dir = "~/Downloads/videos"

# 认证配置
[auth]
cookie = "SESSDATA=xxx;bili_jct=xxx"
# 或使用 access_token
# access_token = "your_token"

# 平台特定配置
[platforms.bilibili]
# CDN 优化
[platforms.bilibili.cdn]
backup_hosts = [
    "upos-sz-mirrorcos.bilivideo.com",
    "upos-sz-mirrorhw.bilivideo.com"
]
```

### 使用配置文件

```bash
# 使用默认配置文件
rvd <URL>

# 使用指定配置文件
rvd <URL> --config-file my-config.toml
```

### 配置优先级

```
CLI 参数 > 环境变量 > 配置文件 > 默认值
```

例如：
```bash
# 配置文件中设置了 threads = 8
# 但命令行参数会覆盖它
rvd <URL> --threads 16  # 实际使用 16 个线程
```

## 常见问题

### Q: 下载速度很慢怎么办？

A: 尝试以下方法：
1. 使用 Aria2c：`--use-aria2c`
2. 增加线程数：`--threads 16`
3. 使用 TV API：`--api-mode tv`
4. 配置 CDN 备用节点（见配置文件章节）

### Q: 提示"需要登录"怎么办？

A: 某些内容需要登录才能下载：
1. 使用二维码登录：`rvd login --mode qrcode`
2. 或提供 Cookie：`--cookie "SESSDATA=xxx"`

### Q: 下载的视频没有声音？

A: 检查以下几点：
1. 确保安装了 FFmpeg
2. 不要使用 `--skip-mux` 参数
3. 查看日志是否有混流错误

### Q: 如何下载杜比视界内容？

A: 杜比视界需要：
1. FFmpeg 5.0 或更高版本
2. 使用 TV API：`--api-mode tv`
3. 登录账号（通常需要大会员）

### Q: 批量下载时如何跳过已下载的视频？

A: 目前需要手动管理，建议：
1. 使用固定的输出目录
2. 使用模板变量生成唯一文件名
3. 下载前检查文件是否存在

### Q: 如何下载私密收藏夹？

A: 私密收藏夹需要：
1. 登录账号
2. 确保账号有权限访问该收藏夹

### Q: 支持断点续传吗？

A: 
- 内置下载器：不支持
- Aria2c：支持断点续传

### Q: 如何下载直播回放？

A: 直播回放与普通视频相同：
```bash
rvd https://www.bilibili.com/video/BV1xx411c7mD
```

### Q: 下载失败怎么办？

A: 
1. 检查网络连接
2. 使用 `--verbose` 查看详细日志
3. 尝试不同的 API 模式
4. 检查是否需要登录
5. 查看 GitHub Issues 或提交新 Issue

### Q: 如何更新 RVD？

A: 
```bash
# 如果是从源码编译
cd rvd_next
git pull
cargo build --release

# 如果是从 crates.io 安装
cargo install rvd --force
```

## 使用技巧

### 1. 创建别名

在 `.bashrc` 或 `.zshrc` 中添加：

```bash
alias rvd-hq='rvd --quality-priority "8K,4K,1080P+" --codec-priority "HEVC,AVC"'
alias rvd-fast='rvd --use-aria2c --threads 16'
```

### 2. 批量下载脚本

创建 `download.sh`：

```bash
#!/bin/bash
while IFS= read -r url; do
    rvd "$url" --output "downloads/{uploader}/{title}.mp4"
done < urls.txt
```

### 3. 定时下载

使用 cron 定时下载 UP 主的新视频：

```bash
# 每天凌晨 2 点下载
0 2 * * * /usr/local/bin/rvd "https://space.bilibili.com/123456/video" --max-videos 5
```

### 4. 下载到 NAS

```bash
rvd <URL> --output "/mnt/nas/videos/{uploader}/{title}.mp4"
```

## 下一步

- 查看 [配置文件文档](CONFIGURATION.md) 了解更多配置选项
- 查看 [架构设计文档](ARCHITECTURE.md) 了解内部实现
- 查看 [开发指南](DEVELOPMENT.md) 学习如何添加新平台

## 反馈与支持

- GitHub Issues: https://github.com/SpenserCai/rust-video-downloader/issues
- GitHub Discussions: https://github.com/SpenserCai/rust-video-downloader/discussions
