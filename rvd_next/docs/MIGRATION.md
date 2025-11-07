# 从旧版本迁移到 RVD Next

本指南帮助你从旧版 RVD 迁移到 RVD Next。

## 主要变化

### 架构变化

| 方面 | 旧版本 | RVD Next |
|------|--------|----------|
| 架构 | 单体架构，Bilibili 专用 | 模块化架构，多平台支持 |
| 平台支持 | 仅 Bilibili | Bilibili（已实现），YouTube/抖音（计划中） |
| 代码组织 | 功能混合 | 清晰的分层架构 |
| 扩展性 | 难以扩展 | 基于 Trait 的插件系统 |

### 功能对比

| 功能 | 旧版本 | RVD Next | 说明 |
|------|--------|----------|------|
| 基础下载 | ✅ | ✅ | 完全兼容 |
| 批量下载 | ✅ | ✅ | 新增流式分页 |
| 认证登录 | ✅ | ✅ | 完全兼容 |
| 多 API 模式 | ✅ | ✅ | 完全兼容 |
| 字幕下载 | ✅ | ✅ | 完全兼容 |
| 弹幕下载 | ✅ | ✅ | 完全兼容 |
| 章节支持 | ✅ | ✅ | 完全兼容 |
| CDN 优化 | ❌ | ✅ | 新功能 |
| 流式批量下载 | ❌ | ✅ | 新功能 |
| 平台注册表 | ❌ | ✅ | 新功能 |

## 命令行参数变化

### 保持不变的参数

以下参数在两个版本中完全相同：

```bash
# 基础参数
--output, -o
--pages, -p
--quality-priority
--codec-priority
--threads
--info-only
--verbose

# 认证参数
--cookie
--access-token

# API 参数
--api-mode

# 下载选项
--use-aria2c
--aria2c-path
--aria2c-args
--skip-mux
--skip-subtitle
--skip-cover
--download-danmaku
--danmaku-format

# 混流参数
--ffmpeg-path
--use-mp4box

# 批量下载
--batch-limit
--max-videos
```

### 新增参数

RVD Next 新增的参数：

```bash
# 配置文件
--config-file <PATH>        # 指定配置文件路径

# 交互模式
--interactive               # 交互式选择清晰度

# 用户代理
--user-agent <UA>           # 自定义 User-Agent
```

### 移除的参数

无。所有旧版本的参数都保留了。

## 配置文件迁移

### 旧版本配置

旧版本使用 `rvd.toml`：

```toml
# 旧版本配置示例
[auth]
cookie = "SESSDATA=xxx"

[download]
threads = 8
```

### RVD Next 配置

RVD Next 完全兼容旧版本配置，并新增了更多选项：

```toml
# RVD Next 配置（向后兼容）
[auth]
cookie = "SESSDATA=xxx"

[download]
threads = 8

# 新增配置
[platforms.bilibili.cdn]
enabled = true
backup_hosts = ["upos-sz-mirrorcos.bilivideo.com"]
```

**迁移步骤**：

1. 复制旧的 `rvd.toml` 到 `rvd_next/` 目录
2. 可选：添加新的配置项（见 [CONFIGURATION.md](CONFIGURATION.md)）
3. 无需修改现有配置

## 认证信息迁移

### Cookie 认证

认证方式完全相同，无需迁移：

```bash
# 旧版本
rvd --cookie "SESSDATA=xxx" <URL>

# RVD Next（相同）
rvd --cookie "SESSDATA=xxx" <URL>
```

### 认证文件

如果你使用 `auth.toml` 存储认证信息：

1. 复制 `auth.toml` 到 RVD Next 目录
2. 使用 `--config-file auth.toml` 参数

## 使用场景迁移

### 场景 1: 下载单个视频

```bash
# 旧版本
rvd https://www.bilibili.com/video/BV1xx411c7mD

# RVD Next（相同）
rvd https://www.bilibili.com/video/BV1xx411c7mD
```

### 场景 2: 下载收藏夹

```bash
# 旧版本
rvd "https://space.bilibili.com/123456/favlist?fid=789"

# RVD Next（相同，但性能更好）
rvd "https://space.bilibili.com/123456/favlist?fid=789"
```

**改进**：RVD Next 使用流式分页，可以处理更大的收藏夹而不会内存溢出。

### 场景 3: 批量下载 UP 主视频

```bash
# 旧版本
rvd "https://space.bilibili.com/123456/video" --max-videos 50

# RVD Next（相同）
rvd "https://space.bilibili.com/123456/video" --max-videos 50
```

### 场景 4: 使用 TV API

```bash
# 旧版本
rvd <URL> --api-mode tv

# RVD Next（相同）
rvd <URL> --api-mode tv
```

### 场景 5: 下载杜比视界

```bash
# 旧版本
rvd <URL> --api-mode tv --quality-priority "8K,4K"

# RVD Next（相同）
rvd <URL> --api-mode tv --quality-priority "8K,4K"
```

## 新功能使用

### CDN 优化

RVD Next 新增了 CDN 优化功能，可以自动检测和替换不稳定的 CDN 节点。

**配置方式**：

```toml
[platforms.bilibili.cdn]
enabled = true
backup_hosts = [
    "upos-sz-mirrorcos.bilivideo.com",
    "upos-sz-mirrorhw.bilivideo.com"
]
```

**效果**：
- 自动检测 PCDN（不稳定节点）
- 替换为备用 CDN
- 提高下载速度和稳定性

### 交互式清晰度选择

```bash
# 交互式选择清晰度
rvd <URL> --interactive
```

会显示可用的清晰度列表供你选择。

### 自定义 User-Agent

```bash
# 命令行参数
rvd <URL> --user-agent "Custom UA"

# 或在配置文件中
[http]
user_agent = "Custom UA"
```

## 性能对比

### 批量下载性能

| 场景 | 旧版本 | RVD Next | 改进 |
|------|--------|----------|------|
| 1000 个视频的收藏夹 | 可能内存溢出 | 流式处理，稳定 | ✅ |
| 下载速度 | 基准 | 相同或更快 | ✅ |
| CDN 稳定性 | 依赖运气 | 自动优化 | ✅ |

### 内存使用

| 场景 | 旧版本 | RVD Next |
|------|--------|----------|
| 单视频下载 | ~50MB | ~50MB |
| 100 个视频批量下载 | ~200MB | ~80MB |
| 1000 个视频批量下载 | 可能 OOM | ~100MB |

## 故障排除

### 问题 1: 找不到配置文件

**症状**：RVD Next 没有加载配置文件

**解决方案**：
1. 检查配置文件路径
2. 使用 `--config-file` 明确指定
3. 使用 `--verbose` 查看加载的配置

### 问题 2: 认证失败

**症状**：提示需要登录

**解决方案**：
1. 检查 Cookie 是否正确
2. 尝试重新登录：`rvd login --mode qrcode`
3. 确保 Cookie 包含 `SESSDATA` 和 `bili_jct`

### 问题 3: 下载速度慢

**症状**：下载速度比旧版本慢

**解决方案**：
1. 启用 CDN 优化（见上文）
2. 使用 Aria2c：`--use-aria2c`
3. 增加线程数：`--threads 16`

### 问题 4: 批量下载中断

**症状**：批量下载时程序崩溃

**解决方案**：
1. 使用 `--max-videos` 限制数量
2. 检查磁盘空间
3. 使用 `--verbose` 查看详细日志

## 回退到旧版本

如果遇到问题需要回退：

```bash
# 切换到旧版本目录
cd ../src

# 使用旧版本
cargo run -- <URL>
```

## 并行使用

你可以同时保留两个版本：

```bash
# 旧版本
alias rvd-old='cd ~/rvd/src && cargo run --'

# 新版本
alias rvd='cd ~/rvd/rvd_next && cargo run --'
```

## 迁移检查清单

- [ ] 备份旧版本的配置文件
- [ ] 复制 `rvd.toml` 到 RVD Next 目录
- [ ] 复制 `auth.toml`（如果有）
- [ ] 测试基本下载功能
- [ ] 测试批量下载功能
- [ ] 测试认证功能
- [ ] 配置 CDN 优化（可选）
- [ ] 更新脚本和别名

## 获取帮助

如果在迁移过程中遇到问题：

1. 查看 [用户指南](USER_GUIDE.md)
2. 查看 [配置文件文档](CONFIGURATION.md)
3. 提交 GitHub Issue
4. 在 GitHub Discussions 提问

## 反馈

我们欢迎你的反馈！如果你在迁移过程中遇到任何问题或有改进建议，请：

- 提交 Issue: https://github.com/SpenserCai/rust-video-downloader/issues
- 参与讨论: https://github.com/SpenserCai/rust-video-downloader/discussions

## 总结

RVD Next 在保持完全向后兼容的同时，提供了更好的架构和新功能。迁移过程简单，大多数情况下无需修改任何配置或脚本。

**推荐迁移时机**：
- ✅ 需要下载大型收藏夹（1000+ 视频）
- ✅ 遇到 CDN 不稳定问题
- ✅ 希望使用新功能（交互式选择、自定义 UA 等）
- ✅ 为未来的多平台支持做准备

**可以继续使用旧版本的情况**：
- 当前功能完全满足需求
- 不需要新功能
- 等待 RVD Next 更加成熟
