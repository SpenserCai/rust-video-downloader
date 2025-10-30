# RVD 测试文档

本目录包含 Rust Video Downloader (RVD) 的所有测试用例。

## 测试结构

```
tests/
├── README.md                    # 本文档
├── rvd.toml.example            # 测试配置文件示例
├── rvd.toml                    # 实际测试配置（需自行创建，不提交到版本控制）
├── test_data/                  # 测试输出目录
│   ├── bangumi/               # 番剧测试输出
│   ├── danmaku_xml/           # XML弹幕测试输出
│   ├── danmaku_ass/           # ASS弹幕测试输出
│   ├── chapters/              # 章节测试输出
│   ├── tv_api/                # TV API测试输出
│   ├── non_interactive/       # 非交互式测试输出
│   ├── complete/              # 完整下载测试输出
│   ├── multi_page/            # 多分P测试输出
│   └── info_only/             # Info-only模式测试输出
├── cli_test.rs                # CLI参数解析测试
├── core_chapter_test.rs       # 章节功能单元测试
├── core_danmaku_test.rs       # 弹幕功能单元测试
├── platform_bilibili_test.rs  # Bilibili平台单元测试
├── integration_test.rs        # 集成测试
└── e2e_download_test.rs       # 端到端下载测试（Task 32.1-32.9）
```

## 测试类型

### 1. 单元测试

#### CLI 测试 (`cli_test.rs`)
- 测试命令行参数解析
- 测试分P选择逻辑
- 测试参数验证

#### 章节测试 (`core_chapter_test.rs`)
- 测试章节数据结构
- 测试章节信息的序列化/反序列化

#### 弹幕测试 (`core_danmaku_test.rs`)
- 测试弹幕格式枚举
- 测试弹幕数据结构

#### Bilibili 平台测试 (`platform_bilibili_test.rs`)
- 测试 URL 识别和解析
- 测试流选择逻辑
- 测试不同 API 模式
- 测试各种 URL 格式（BV/av/ep/ss/cheese/收藏夹/空间/合集/系列）

### 2. 集成测试 (`integration_test.rs`)

测试各个模块的集成：
- 视频信息解析（带/不带认证）
- 流信息获取
- 字幕获取
- Orchestrator 创建
- 小文件下载
- FFmpeg 检查
- Info-only 模式
- 多分P选择
- 错误处理
- 质量和编码优先级
- 文件命名模板
- 配置文件加载

### 3. 端到端测试 (`e2e_download_test.rs`)

覆盖 Task 32.1-32.9 的所有场景：

#### Task 32.1: VideoType 枚举扩展
- ✅ `test_32_1_parse_bvid_url` - BV号解析（使用认证信息）
- ✅ `test_32_1_parse_avid_url` - av号解析（使用认证信息）
- ✅ `test_32_1_parse_multi_page_video` - 多分P视频解析（使用认证信息）

#### Task 32.2: 番剧和课程信息获取
- ✅ `test_32_2_parse_bangumi_by_ep` - 番剧ep链接解析（需要认证）
- ✅ `test_32_2_parse_bangumi_by_ss` - 番剧ss链接解析（需要认证）
- ✅ `test_32_2_download_bangumi_single_episode` - 下载番剧单集（需要认证）

#### Task 32.3: 批量下载 - 收藏夹
- ⚠️ `test_32_3_parse_favorite_list` - 收藏夹解析（需要有效的收藏夹ID）

#### Task 32.4: 批量下载 - UP主空间
- ✅ `test_32_4_parse_space_videos` - UP主空间视频解析（使用认证信息）

#### Task 32.5: 批量下载 - 合集和系列
- ⚠️ `test_32_5_parse_media_list` - 合集解析（需要有效的合集ID）
- ⚠️ `test_32_5_parse_series_list` - 系列解析（需要有效的系列ID）

#### Task 32.6: TV/APP/国际版API支持
- 🔕 `test_32_6_tv_api_mode` - TV API模式（已忽略）
- 🔕 `test_32_6_app_api_mode` - APP API模式（已忽略）
- 🔕 `test_32_6_download_with_tv_api` - TV API下载（已忽略）

#### Task 32.7: 弹幕下载功能
- ✅ `test_32_7_download_danmaku_xml` - XML格式弹幕下载
- ✅ `test_32_7_download_danmaku_ass` - ASS格式弹幕下载

#### Task 32.8: 章节信息提取和嵌入
- ✅ `test_32_8_fetch_chapters` - 章节信息提取
- ✅ `test_32_8_download_with_chapters` - 下载并嵌入章节

#### Task 32.9: 交互式模式
- ✅ `test_32_9_interactive_mode_disabled` - 非交互式模式（自动选择）

#### 综合测试
- ✅ `test_complete_download_workflow` - 完整下载流程
- ✅ `test_multi_page_download` - 多分P下载
- ✅ `test_info_only_mode` - Info-only模式
- ✅ `test_cleanup_old_files` - 清理旧文件

## 配置测试环境

### 1. 创建配置文件

复制示例配置文件并填入你的认证信息：

```bash
cp tests/rvd.toml.example tests/rvd.toml
```

编辑 `tests/rvd.toml`：

```toml
# 默认清晰度优先级
default_quality = ["1080P 高清", "720P 高清", "480P 清晰"]

# 默认编码格式优先级
default_codec = ["hevc", "avc"]

# 下载线程数
thread_count = 4

# 认证信息（可选，用于下载会员内容）
[auth]
# 从浏览器获取的 SESSDATA cookie
cookie = "SESSDATA=your_sessdata_here"

# TV/APP API 的 access_token（可选）
# access_token = "your_access_token_here"

# 路径配置
[paths]
# FFmpeg 路径（如果不在系统 PATH 中）
# ffmpeg = "/usr/local/bin/ffmpeg"
```

### 2. 获取 Cookie

要测试需要认证的功能（如会员视频、收藏夹等），需要获取 B站的 SESSDATA cookie：

1. 在浏览器中登录 bilibili.com
2. 打开开发者工具（F12）
3. 切换到 Application/存储 标签
4. 在 Cookies 中找到 `SESSDATA`
5. 复制其值到配置文件中

**注意**: 所有测试都会尝试使用配置文件中的认证信息（如果存在）。即使是公开视频的测试，使用认证信息也可以获得更好的测试覆盖率和更稳定的结果。

### 3. 安装 FFmpeg

大部分测试需要 FFmpeg 进行混流：

**macOS:**
```bash
brew install ffmpeg
```

**Ubuntu/Debian:**
```bash
sudo apt install ffmpeg
```

**Windows:**
从 [FFmpeg官网](https://ffmpeg.org/download.html) 下载并添加到 PATH

## 运行测试

### 运行所有测试

```bash
cargo test
```

### 运行特定测试文件

```bash
# 运行单元测试
cargo test --test cli_test
cargo test --test platform_bilibili_test

# 运行集成测试
cargo test --test integration_test

# 运行端到端测试
cargo test --test e2e_download_test
```

### 运行特定测试用例

```bash
# 运行特定的测试函数
cargo test test_32_1_parse_bvid_url

# 运行包含特定关键字的测试
cargo test danmaku
```

### 运行被忽略的测试

某些测试（如 TV/APP API）默认被忽略，需要特殊配置才能运行：

```bash
# 运行所有测试，包括被忽略的
cargo test -- --ignored

# 运行所有测试（包括正常和被忽略的）
cargo test -- --include-ignored
```

### 显示测试输出

```bash
# 显示 println! 输出
cargo test -- --nocapture

# 显示详细输出
cargo test -- --nocapture --test-threads=1
```

## 测试超时设置

端到端测试设置了 5 分钟（300秒）的超时时间。如果下载超时，测试会合理停止并标记为通过（只要功能正常）。

可以在 `e2e_download_test.rs` 中修改 `TEST_TIMEOUT_SECS` 常量来调整超时时间。

## 测试数据清理

测试会在 `tests/test_data/` 目录下生成输出文件。可以手动清理：

```bash
rm -rf tests/test_data/*
```

或运行清理测试：

```bash
cargo test test_cleanup_old_files
```

该测试会自动删除超过 1 小时的旧文件。

## 测试用的 URL

### 有效的测试 URL

以下是测试中使用的真实有效的 B站链接：

#### 普通视频
- `BV1qt4y1X7TW` - BBDown 官方示例视频
- `BV1At41167aj` - 多分P视频示例
- `BV1xx411c7mD` - 短视频（快速测试）
- `BV1uv411q7Mv` - 有弹幕和字幕的视频
- `av170001` - av号示例

#### 番剧
- `https://www.bilibili.com/bangumi/play/ss28341` - 鬼灭之刃第一季
- `https://www.bilibili.com/bangumi/play/ss33073` - BBDown 文档示例
- `https://www.bilibili.com/bangumi/play/ep394750` - 具体某一集

#### UP主空间
- `https://space.bilibili.com/1` - B站官方账号
- `https://space.bilibili.com/546195` - 某个活跃UP主

### 需要替换的 URL

以下测试需要你提供真实的 ID：

- **收藏夹**: 需要你自己的收藏夹 ID 和 mid
- **合集**: 需要有效的合集 ID
- **系列**: 需要有效的 mid 和系列 ID

## 常见问题

### 1. 测试失败：无法找到 FFmpeg

**解决方案**: 安装 FFmpeg 或在配置文件中指定路径

### 2. 测试失败：认证错误

**解决方案**: 检查 `tests/rvd.toml` 中的 cookie 是否有效

### 3. 测试超时

**解决方案**: 
- 检查网络连接
- 增加 `TEST_TIMEOUT_SECS` 的值
- 使用 `--test-threads=1` 减少并发

### 4. 番剧/会员视频无法访问

**解决方案**: 
- 确保配置了有效的 cookie
- 确保账号有相应的权限（大会员等）
- 某些番剧可能有地区限制

### 5. 收藏夹/合集测试失败

**解决方案**: 
- 这些测试需要有效的 ID
- 替换测试代码中的示例 ID 为真实的 ID
- 确保配置了认证信息

## 持续集成

在 CI 环境中运行测试时：

```bash
# 只运行不需要认证的测试
cargo test --test cli_test
cargo test --test core_chapter_test
cargo test --test core_danmaku_test
cargo test --test platform_bilibili_test

# 跳过需要下载的端到端测试
cargo test --test integration_test -- --skip download
```

## 贡献测试

添加新测试时请遵循以下规范：

1. **命名规范**: 使用描述性的测试函数名
2. **文档注释**: 为测试添加清晰的注释说明测试目的
3. **错误处理**: 使用友好的错误消息
4. **清理**: 测试后清理临时文件
5. **超时**: 为长时间运行的测试设置合理的超时
6. **忽略标记**: 对需要特殊配置的测试添加 `#[ignore]`

## 测试覆盖率

查看测试覆盖率（需要安装 tarpaulin）：

```bash
cargo install cargo-tarpaulin
cargo tarpaulin --out Html
```

## 参考资料

- [Rust 测试文档](https://doc.rust-lang.org/book/ch11-00-testing.html)
- [Tokio 测试指南](https://tokio.rs/tokio/topics/testing)
- [BBDown 项目](https://github.com/nilaoda/BBDown)
- [Bilibili API 文档](https://github.com/SocialSisterYi/bilibili-API-collect)
