# RVD E2E 测试平台

模块化、可扩展的端到端测试平台，用于 RVD Next 项目的持续可回归测试。

## 目录

- [快速开始](#快速开始)
- [测试套件](#测试套件)
- [使用方法](#使用方法)
- [测试状态](#测试状态)
- [配置说明](#配置说明)
- [开发指南](#开发指南)
- [故障排查](#故障排查)

## 快速开始

### 1. 环境准备

```bash
# 安装Python依赖
cd rvd_next/e2e_test_platform
pip install -r requirements.txt

# 编译RVD可执行文件
cd ..
cargo build --release
```

### 2. 运行测试

```bash
# 快速验证（冒烟测试，5-10分钟）
make smoke

# 或使用Python脚本
python run_test_suite.py --suite smoke

# 查看所有可用命令
make help
```

### 3. 查看报告

```bash
# 在浏览器中打开HTML报告
make report

# 或手动打开
open reports/latest.html  # macOS
xdg-open reports/latest.html  # Linux
```

## 测试套件

### 预定义套件

| 套件 | 测试数 | 时间 | 用途 | 命令 |
|------|--------|------|------|------|
| **smoke** | 5 | 5-10分钟 | 快速验证核心功能 | `make smoke` |
| **core** | 12 | 15-20分钟 | 测试所有核心模块 | `make core` |
| **bilibili** | 16 | 25-35分钟 | 测试Bilibili平台功能 | `make bilibili` |
| **full** | 28 | 40-50分钟 | 完整回归测试 | `make full` |

### 测试覆盖

<details>
<summary><b>核心功能测试（12个）</b></summary>

#### 配置管理（3个）
- ✅ TestDefaultConfig - 默认配置
- ✅ TestCustomConfig - 自定义配置
- ✅ TestCLIPriority - CLI参数优先级

#### 下载器（2个）
- ✅ TestBuiltinDownloader - 内置下载器
- ✅ TestDownloadProgress - 下载进度

#### 混流器（4个）
- ✅ TestBasicMux - 基础混流
- ✅ TestSubtitleEmbed - 字幕嵌入
- ✅ TestChapterEmbed - 章节嵌入
- ✅ TestSkipMux - 跳过混流

#### HTTP客户端（2个）
- ✅ TestDefaultUserAgent - 默认User-Agent
- ✅ TestCustomUserAgentCLI - 自定义User-Agent

#### 错误处理（2个）
- ✅ TestInvalidUrl - 无效URL处理
- ✅ TestNetworkError - 网络错误处理

</details>

<details>
<summary><b>Bilibili平台测试（16个）</b></summary>

#### 基础下载（4个）
- ✅ TestBVVideoDownload - BV号视频
- ✅ TestMultiPageVideoDownload - 多P视频
- ✅ TestBangumiEPDownload - 番剧EP号
- ✅ TestBangumiSSDownload - 番剧SS号

#### 批量下载（2个）
- ✅ TestFavoritesBatchDownload - 收藏夹批量下载
- ✅ TestUserSpaceBatchDownload - UP主空间

#### 质量选择（4个）
- ✅ TestQualitySelection - 质量选择
- ✅ TestQualityPriority - 质量优先级
- ✅ TestCodecPriority - 编码优先级
- ✅ TestDolbyVision - 杜比视界（需要大会员）

#### 弹幕（3个）
- ✅ TestXMLDanmaku - XML弹幕
- ✅ TestASSDanmaku - ASS弹幕
- ✅ TestDanmakuDecompression - 弹幕解压

#### 字幕（1个）
- ✅ TestSingleSubtitle - 单字幕

#### 章节（2个）
- ✅ TestNormalVideoChapters - 普通视频章节
- ✅ TestChapterEmbedding - 章节嵌入

</details>

## 使用方法

### 使用Makefile（推荐）

```bash
# 查看所有命令
make help

# 运行测试
make smoke      # 冒烟测试
make core       # 核心功能测试
make bilibili   # Bilibili平台测试
make full       # 完整测试
make parallel   # 并行运行完整测试

# 其他命令
make list       # 列出所有测试
make clean      # 清理测试产物
make report     # 打开HTML报告
```

### 使用Python脚本

```bash
# 运行测试套件
python run_test_suite.py --suite smoke
python run_test_suite.py --suite core
python run_test_suite.py --suite bilibili
python run_test_suite.py --suite full

# 运行指定测试
python run_test_suite.py --tests TestBVVideoDownload TestQualitySelection

# 根据标签运行
python run_test_suite.py --tags download quality

# 并行执行
python run_test_suite.py --suite full --parallel --workers 4

# 详细输出
python run_test_suite.py --suite smoke -v

# 列出所有测试
python run_test_suite.py --list
python run_test_suite.py --list --show-disabled
```

### 常用场景

<details>
<summary><b>场景1: 快速验证功能</b></summary>

```bash
# 刚拉取代码或修改了核心功能
make smoke

# 预期输出:
# ✓ TestDefaultConfig - 通过
# ✓ TestBuiltinDownloader - 通过
# ✓ TestBasicMux - 通过
# ✓ TestBVVideoDownload - 通过
# ✓ TestQualitySelection - 通过
# 
# 5/5 测试通过 (用时: 8分钟)
```

</details>

<details>
<summary><b>场景2: 开发特定功能</b></summary>

```bash
# 开发下载器功能
python run_test_suite.py --tags download

# 开发Bilibili功能
python run_test_suite.py --tags bilibili

# 开发质量选择功能
python run_test_suite.py --tags quality
```

</details>

<details>
<summary><b>场景3: 提交代码前检查</b></summary>

```bash
# 串行执行（稳定）
make full

# 并行执行（快速，推荐）
make parallel

# 查看报告
make report
```

</details>

<details>
<summary><b>场景4: 调试测试失败</b></summary>

```bash
# 1. 运行失败的测试并启用详细输出
python run_test_suite.py --tests TestBVVideoDownload -v

# 2. 查看工作目录
ls -la workdir/TestBVVideoDownload/

# 3. 查看HTML报告
make report

# 4. 使用原始测试工具进行更底层的调试
python run_tests.py TestBVVideoDownload -v
```

</details>

## 测试状态

### 总体概况

| 类别 | 已启用 | 待实现 | 总计 | 覆盖率 |
|------|--------|--------|------|--------|
| 核心功能 | 12 | 0 | 12 | 100% |
| Bilibili平台 | 16 | 0 | 16 | 100% |
| **总计** | **28** | **0** | **28** | **100%** |

### 功能覆盖矩阵

<details>
<summary><b>查看详细覆盖情况</b></summary>

#### 核心功能模块

| 模块 | 功能 | 状态 |
|------|------|------|
| 配置管理 | 默认配置 | ✅ |
| | 自定义配置 | ✅ |
| | CLI优先级 | ✅ |
| 下载器 | 内置下载器 | ✅ |
| | 下载进度 | ✅ |
| | aria2c下载器 | ⚠️ 待实现 |
| 混流器 | 基础混流 | ✅ |
| | 字幕嵌入 | ✅ |
| | 章节嵌入 | ✅ |
| | 跳过混流 | ✅ |
| HTTP客户端 | 默认UA | ✅ |
| | 自定义UA(CLI) | ✅ |
| | 自定义UA(配置) | ⚠️ 待实现 |
| 错误处理 | 无效URL | ✅ |
| | 网络错误 | ✅ |

#### Bilibili平台功能

| 模块 | 功能 | 状态 |
|------|------|------|
| 基础下载 | BV号视频 | ✅ |
| | AV号视频 | ⚠️ 需配置URL |
| | 多P视频 | ✅ |
| | 番剧EP | ✅ |
| | 番剧SS | ✅ |
| 批量下载 | 收藏夹 | ✅ |
| | UP主空间 | ✅ |
| | 合集 | ⚠️ 需配置URL |
| 质量选择 | 质量选择 | ✅ |
| | 质量优先级 | ✅ |
| | 编码优先级 | ✅ |
| | 杜比视界 | ✅ (需大会员) |
| 弹幕 | XML格式 | ✅ |
| | ASS格式 | ✅ |
| | 解压缩 | ✅ |
| 字幕 | 单字幕 | ✅ |
| | 多语言字幕 | ⚠️ 需配置URL |
| 章节 | 普通视频 | ✅ |
| | 番剧章节 | ⚠️ 需配置URL |
| | 章节嵌入 | ✅ |

**图例**: ✅ 已启用 | ⚠️ 待实现/需配置

</details>

## 配置说明

### 基础配置

编辑 `config.yaml` 设置基本选项：

```yaml
platform:
  executable: "../target/release/rvd"  # RVD可执行文件路径
  default_timeout: 300                 # 默认超时时间（秒）
  default_workdir: "./workdir"         # 工作目录

execution:
  parallel: false                      # 是否并行执行
  max_workers: 4                       # 最大并行数

reporting:
  console: true                        # 控制台报告
  json: true                           # JSON报告
  html: true                           # HTML报告
  output_dir: "./reports"              # 报告输出目录
```

### 测试URL配置

编辑 `datas/urls.yaml` 配置测试URL：

```yaml
# 核心测试
core:
  downloader:
    builtin:
      url: "https://www.bilibili.com/video/BV16vpizyEpY"

# Bilibili测试
bilibili:
  basic_download:
    bv_video:
      url: "https://www.bilibili.com/video/BV16vpizyEpY"
    bangumi_ss:
      url: "https://www.bilibili.com/bangumi/play/ss47794"
```

### 认证配置（可选）

<details>
<summary><b>配置大会员认证</b></summary>

如需测试需要认证的功能（如大会员专享内容）：

1. 使用二维码登录获取凭证：
   ```bash
   ../target/release/rvd --login-qrcode --config-file ../rvd.toml
   ```

2. 或手动创建 `cfg/auth.toml`：
   ```toml
   [bilibili]
   cookie = "your_cookie_here"
   ```

3. 在 `datas/urls.yaml` 中指定认证文件：
   ```yaml
   bilibili:
     quality_selection:
       dolby_vision:
         url: "https://..."
         auth_file: "./cfg/auth.toml"
   ```

</details>

## 开发指南

### 添加新测试用例

<details>
<summary><b>步骤1: 编写测试用例</b></summary>

在 `tests/` 目录创建测试文件：

```python
# tests/platforms/bilibili/test_new_feature.py
from typing import List
from core.base_test import BaseTestCase, TestResult
from validators.file import FileValidator

class TestNewFeature(BaseTestCase):
    """测试新功能"""
    
    def __init__(self, config):
        super().__init__(config)
        self.tags = ['bilibili', 'new-feature']
        self.timeout = 600
        
        # 从urls.yaml加载测试数据
        test_data = self._load_test_data('bilibili')
        feature_data = test_data.get('new_feature', {})
        self.video_url = feature_data.get('url', 'PLACEHOLDER_URL')
    
    def get_command(self) -> List[str]:
        cmd = self._build_base_command()
        cmd.extend([
            self.video_url,
            '--new-feature-flag',
            '--output', str(self.workdir),
        ])
        return cmd
    
    def validate(self, result: TestResult) -> bool:
        validations = []
        
        # 验证输出
        output_lower = result.output.lower()
        has_success = 'success' in output_lower
        
        if has_success:
            validations.append({
                "validator": "output",
                "passed": True,
                "message": "Success message found"
            })
        else:
            validations.append({
                "validator": "output",
                "passed": False,
                "message": "No success message"
            })
            result.validations = validations
            result.error = "No success message in output"
            return False
        
        # 验证文件
        file_validator = FileValidator(
            files_exist=["*.mp4"],
            min_size={"*.mp4": 1024 * 100}
        )
        passed, msg = file_validator.validate(self.workdir)
        validations.append({
            "validator": "file",
            "passed": passed,
            "message": msg
        })
        
        result.validations = validations
        if not passed:
            result.error = msg
        
        return passed
```

</details>

<details>
<summary><b>步骤2: 配置测试URL</b></summary>

在 `datas/urls.yaml` 中添加：

```yaml
bilibili:
  new_feature:
    url: "https://www.bilibili.com/video/BVxxxxxxxxx"
    description: "新功能测试视频"
```

</details>

<details>
<summary><b>步骤3: 注册测试</b></summary>

在 `run_test_suite.py` 的 `ENABLED_TESTS` 中注册：

```python
ENABLED_TESTS = {
    'bilibili': {
        # ... 其他测试 ...
        'TestNewFeature': {
            'module': 'tests.platforms.bilibili.test_new_feature',
            'class': 'TestNewFeature',
            'description': '测试新功能',
            'tags': ['bilibili', 'new-feature'],
        },
    },
}
```

</details>

<details>
<summary><b>步骤4: 运行测试</b></summary>

```bash
# 运行新测试
python run_test_suite.py --tests TestNewFeature -v

# 或运行相关标签的测试
python run_test_suite.py --tags new-feature
```

</details>

### 目录结构

```
e2e_test_platform/
├── README.md                 # 本文档
├── Makefile                  # 便捷命令
├── config.yaml               # 平台配置
├── run_tests.py              # 原始测试启动器
├── run_test_suite.py         # 测试套件启动器 ⭐
├── requirements.txt          # Python依赖
├── core/                     # 测试框架核心
│   ├── base_test.py          # 测试基类
│   ├── config.py             # 配置管理
│   ├── loader.py             # 测试加载器
│   ├── runner.py             # 测试运行器
│   └── result.py             # 结果定义
├── tests/                    # 测试用例
│   ├── core/                 # 核心功能测试
│   └── platforms/            # 平台测试
│       └── bilibili/         # Bilibili平台测试
├── validators/               # 验证器
│   ├── output.py             # 输出验证
│   ├── file.py               # 文件验证
│   └── content.py            # 内容验证
├── reporters/                # 报告生成器
│   ├── console.py            # 控制台报告
│   ├── json_reporter.py      # JSON报告
│   └── html_reporter.py      # HTML报告
├── datas/                    # 测试数据
│   └── urls.yaml             # 测试URL配置
├── cfg/                      # 配置文件（可选）
│   ├── rvd.toml              # RVD配置
│   └── auth.toml             # 认证配置
├── workdir/                  # 测试工作目录
└── reports/                  # 测试报告输出
    ├── latest.json           # JSON报告
    ├── latest.html           # HTML报告
    └── test.log              # 测试日志
```

## 故障排查

### 常见问题

<details>
<summary><b>Q: 测试失败怎么办？</b></summary>

1. 查看控制台输出的错误信息
2. 检查 `reports/latest.html` 获取详细报告
3. 使用 `-v` 参数运行以获取详细日志
4. 检查 `workdir/` 目录下的测试产物

```bash
# 运行失败的测试并启用详细输出
python run_test_suite.py --tests TestBVVideoDownload -v

# 查看工作目录
ls -la workdir/TestBVVideoDownload/

# 查看HTML报告
make report
```

</details>

<details>
<summary><b>Q: URL配置错误怎么办？</b></summary>

1. 检查 `datas/urls.yaml` 中的URL是否有效
2. 确保URL格式正确（包含完整的协议和域名）
3. 对于需要认证的内容，确保配置了 `auth_file`

```bash
# 检查URL配置
cat datas/urls.yaml | grep -A 5 "bv_video"

# 手动测试URL
../target/release/rvd "https://www.bilibili.com/video/BV16vpizyEpY" --info-only
```

</details>

<details>
<summary><b>Q: 如何跳过某些测试？</b></summary>

1. 使用标签过滤：`--tags` 只运行特定标签的测试
2. 指定测试名称：`--tests` 只运行指定的测试
3. 将测试从 `ENABLED_TESTS` 移到 `DISABLED_TESTS`（在 `run_test_suite.py` 中）

```bash
# 只运行下载相关测试
python run_test_suite.py --tags download

# 只运行指定测试
python run_test_suite.py --tests TestBVVideoDownload TestQualitySelection
```

</details>

<details>
<summary><b>Q: 并行测试时出现问题？</b></summary>

1. 减少工作线程数：`--workers 2`
2. 关闭并行模式，串行运行
3. 检查是否有资源竞争（如端口占用）

```bash
# 减少并行度
python run_test_suite.py --suite full --parallel --workers 2

# 串行运行
python run_test_suite.py --suite full
```

</details>

### 快速诊断

| 问题 | 命令 |
|------|------|
| 测试失败 | `python run_test_suite.py --tests <TestName> -v` |
| URL配置错误 | `cat datas/urls.yaml` |
| 可执行文件问题 | `ls -la ../target/release/rvd` |
| 依赖问题 | `pip list \| grep -E "pyyaml\|requests"` |
| 工作目录问题 | `ls -la workdir/` |
| 报告问题 | `ls -la reports/` |

## 最佳实践

### 开发工作流

```bash
# 1. 开发新功能前 - 运行冒烟测试
make smoke

# 2. 开发过程中 - 运行相关测试
python run_test_suite.py --tags download

# 3. 提交代码前 - 运行完整测试
make parallel

# 4. 查看报告
make report
```

### CI/CD集成

```yaml
# .github/workflows/test.yml
name: E2E Tests

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      
      - name: Setup Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      
      - name: Build RVD
        run: |
          cd rvd_next
          cargo build --release
      
      - name: Setup Python
        uses: actions/setup-python@v2
        with:
          python-version: '3.9'
      
      - name: Install Dependencies
        run: |
          cd rvd_next/e2e_test_platform
          pip install -r requirements.txt
      
      - name: Run Tests
        run: |
          cd rvd_next/e2e_test_platform
          python run_test_suite.py --suite full --parallel --workers 4 -v
      
      - name: Upload Reports
        if: always()
        uses: actions/upload-artifact@v2
        with:
          name: test-reports
          path: rvd_next/e2e_test_platform/reports/
```

## 工具对比

### run_test_suite.py vs run_tests.py

| 特性 | run_test_suite.py | run_tests.py |
|------|-------------------|--------------|
| 测试注册表 | ✅ 内置，清晰管理 | ❌ 需要自动发现 |
| 测试套件 | ✅ 预定义4个套件 | ❌ 无 |
| 状态管理 | ✅ 区分启用/禁用 | ❌ 无 |
| 易用性 | ✅ 高，命令简洁 | ⚠️ 中等 |
| 灵活性 | ✅ 高，多种执行方式 | ✅ 高 |
| 适用场景 | 日常开发、CI/CD | 调试、自定义测试 |

**建议**：
- 日常开发使用 `run_test_suite.py`
- 调试特定问题时使用 `run_tests.py`
- 两者可以共存，互为补充

## 常用命令速查

```bash
# 快速验证
make smoke                                    # 冒烟测试（5-10分钟）

# 完整测试
make full                                     # 完整测试（30-40分钟）
make parallel                                 # 并行完整测试（15-20分钟）

# 特定模块
make core                                     # 核心功能测试
make bilibili                                 # Bilibili平台测试

# 自定义运行
python run_test_suite.py --tests <TestName>  # 运行指定测试
python run_test_suite.py --tags <tag>        # 根据标签运行
python run_test_suite.py --suite <suite> -v  # 详细输出

# 查看和清理
make list                                     # 列出测试
make report                                   # 查看报告
make clean                                    # 清理产物

# 开发辅助
make install                                  # 安装依赖
make build                                    # 编译RVD
make quick                                    # 快速检查
```

## 许可证

与 RVD 项目保持一致。

---

**更新日期**: 2024-11-07  
**版本**: 2.0.0  
**维护者**: RVD Contributors
