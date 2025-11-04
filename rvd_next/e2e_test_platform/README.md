# RVD E2E测试平台

模块化、可扩展的端到端测试平台，用于RVD Next项目的持续可回归测试。

## 特性

- **模块化架构**: 清晰的模块划分，易于维护和扩展
- **灵活配置**: 支持YAML配置文件和环境变量
- **强大验证**: 支持输出、文件、内容等多种验证方式
- **多种报告**: 控制台、JSON、HTML格式报告
- **并行执行**: 支持并行执行提高测试效率
- **位置无关**: 可在任何位置运行，不依赖绝对路径

## 安装

1. 确保已安装Python 3.13+

2. 安装依赖：
```bash
cd rvd_next/e2e_test_platform
pip install -r requirements.txt
```

## 配置

### 1. 编辑配置文件

编辑 `config.yaml` 设置可执行程序路径和其他选项：

```yaml
platform:
  executable: "../../target/release/rvd"  # RVD可执行程序路径
  auth_file: null                         # 认证文件路径（可选）
  default_timeout: 300                    # 默认超时时间（秒）
```

### 2. 配置认证（可选）

如果需要测试需要认证的功能（如大会员专享内容），需要配置认证文件：

1. 复制 `auth.toml.example` 为 `auth.toml`（在项目根目录）
2. 使用二维码登录获取凭证：
   ```bash
   ../../target/release/rvd --login-qrcode --config-file ../../rvd.toml
   ```
3. 或手动填入从浏览器获取的Cookie

4. 在 `config.yaml` 中配置认证文件路径：
   ```yaml
   platform:
     auth_file: "../../auth.toml"
   ```

5. 或在测试用例中单独指定：
   ```python
   def __init__(self, config):
       super().__init__(config)
       self.requires_auth = True
       self.auth_file = config.resolve_path("../../auth.toml")
   ```

### 3. 配置测试URL

编辑 `datas/urls.yaml` 设置测试URL（将占位符替换为实际URL）：

```yaml
single_video:
  url: "BV1xx411c7mD"  # 替换为实际的视频URL

batch_download:
  playlist_url: "https://space.bilibili.com/xxx/favlist?fid=xxx"
  expected_count: 5
```

### 4. 环境变量（可选）

可以通过环境变量覆盖配置：

```bash
export E2E_EXECUTABLE=/path/to/rvd
export E2E_TIMEOUT=600
export E2E_WORKDIR=/tmp/e2e_tests
```

## 使用

### 基本使用

```bash
# 运行所有测试
python run_tests.py

# 列出所有可用测试
python run_tests.py --list

# 运行指定测试
python run_tests.py TestSingleVideoDownload TestBatchDownload

# 使用模式匹配
python run_tests.py --pattern video

# 使用标签过滤
python run_tests.py --tags basic feature
```

### 高级选项

```bash
# 并行执行（4个worker）
python run_tests.py --parallel --workers 4

# 详细输出
python run_tests.py -v

# 使用自定义配置文件
python run_tests.py --config my_config.yaml
```

## 测试报告

测试完成后，报告将保存在 `reports/` 目录：

- `latest.json`: JSON格式的详细报告
- `latest.html`: HTML格式的可视化报告
- `test.log`: 测试执行日志

## 添加新测试用例

1. 在 `tests/` 目录创建新的测试文件（以 `test_` 开头）

2. 继承 `BaseTestCase` 并实现必需的方法：

```python
from core.base_test import BaseTestCase, TestResult
from typing import List

class TestMyFeature(BaseTestCase):
    """测试我的功能"""
    
    def __init__(self, config):
        super().__init__(config)
        self.tags = ['feature', 'my-tag']
        self.timeout = 300
    
    def get_command(self) -> List[str]:
        """返回要执行的命令"""
        cmd = self._build_base_command()  # 构建包含认证的基础命令
        cmd.extend([
            "my-url",
            '--output', str(self.workdir),
        ])
        return cmd
    
    def validate(self, result: TestResult) -> bool:
        """验证测试结果"""
        # 使用验证器验证输出和文件
        from validators.output import OutputValidator
        from validators.file import FileValidator
        
        # 验证输出
        output_validator = OutputValidator(contains=["success"])
        passed, msg = output_validator.validate(result)
        if not passed:
            result.error = msg
            return False
        
        # 验证文件
        file_validator = FileValidator(files_exist=["*.mp4"])
        passed, msg = file_validator.validate(self.workdir)
        if not passed:
            result.error = msg
            return False
        
        return True
```

## 扩展验证器

创建自定义验证器：

```python
from validators.base import Validator
from typing import Tuple

class MyValidator(Validator):
    def validate(self, value) -> Tuple[bool, str]:
        # 实现验证逻辑
        if my_condition:
            return True, ""
        return False, "Validation failed"
```

## 扩展报告器

创建自定义报告器：

```python
from reporters.base import Reporter
from core.result import TestResult
from typing import List

class MyReporter(Reporter):
    def generate(self, results: List[TestResult]) -> str:
        # 实现报告生成逻辑
        return "My custom report"
```

## 目录结构

```
e2e_test_platform/
├── config.yaml           # 配置文件
├── run_tests.py          # CLI入口
├── requirements.txt      # Python依赖
├── core/                 # 核心模块
│   ├── config.py         # 配置管理
│   ├── base_test.py      # 测试基类
│   ├── loader.py         # 测试加载器
│   └── runner.py         # 测试运行器
├── validators/           # 验证器
│   ├── output.py         # 输出验证器
│   ├── file.py           # 文件验证器
│   └── content.py        # 内容验证器
├── reporters/            # 报告器
│   ├── console.py        # 控制台报告器
│   ├── json_reporter.py  # JSON报告器
│   └── html_reporter.py  # HTML报告器
├── utils/                # 工具模块
├── tests/                # 测试用例
├── datas/                # 测试数据
└── reports/              # 报告输出
```

## 故障排除

### 测试失败

1. 检查 `reports/test.log` 查看详细日志
2. 使用 `-v` 参数运行以获取更多信息
3. 检查 `reports/latest.html` 查看可视化报告

### 配置问题

1. 确保 `config.yaml` 中的路径正确
2. 确保可执行程序存在且有执行权限
3. 检查环境变量是否正确设置

### URL占位符

如果看到 "PLACEHOLDER_VIDEO_URL" 错误，说明需要在 `datas/urls.yaml` 中配置实际的测试URL。

## 持续集成

在CI环境中使用：

```bash
# 设置环境变量
export E2E_EXECUTABLE=/path/to/rvd
export E2E_TIMEOUT=600

# 运行测试
python run_tests.py

# 检查退出码
if [ $? -eq 0 ]; then
    echo "All tests passed"
else
    echo "Tests failed"
    exit 1
fi
```

## 许可证

与RVD项目相同的许可证。
