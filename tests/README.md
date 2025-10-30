# RVD 测试文档

本目录包含 RVD (Rust Video Downloader) 项目的所有测试用例。

## 测试结构

```
tests/
├── cli_test.rs                 # CLI参数解析单元测试
├── integration_test.rs         # 集成测试（不需要实际下载）
├── e2e_download_test.rs        # 端到端测试（需要实际下载）
├── platform_bilibili_test.rs   # Bilibili平台单元测试
├── utils_config_test.rs        # 配置管理单元测试
├── utils_file_test.rs          # 文件工具单元测试
├── test_data/                  # 测试产生的文件存放目录
└── rvd.toml                    # 测试配置文件（可选）
```

### 单元测试

1. **cli_test.rs** - CLI参数解析测试
   - 测试质量优先级解析（默认和自定义）
   - 测试编码优先级解析（默认和自定义）
   - 测试分P选择解析（单个、多个、范围、混合、ALL）
   - 覆盖率：11个测试用例

2. **utils_config_test.rs** - 配置管理测试
   - 测试配置文件加载（存在/不存在）
   - 测试有效和无效的TOML格式
   - 测试部分配置加载
   - 覆盖率：5个测试用例

3. **utils_file_test.rs** - 文件工具测试
   - 测试文件名清理（非法字符、长度限制）
   - 测试文件名模板解析（单P和多P）
   - 测试默认输出路径生成
   - 覆盖率：6个测试用例

4. **platform_bilibili_test.rs** - Bilibili平台测试
   - 测试URL识别（各种B站URL格式）
   - 测试流选择逻辑（按质量、按编码、降级）
   - 测试边界情况（无视频流、无音频流）
   - 覆盖率：7个测试用例

### 集成测试

**integration_test.rs** - 完整流程集成测试（不实际下载大文件）
- 测试视频信息解析（有/无认证）
- 测试流信息获取
- 测试字幕获取
- 测试Orchestrator创建
- 测试小文件下载
- 测试FFmpeg检查
- 测试info-only模式
- 测试多分P选择
- 测试错误处理（无效URL、网络失败）
- 测试质量和编码优先级
- 测试文件命名模板
- 测试配置加载
- 覆盖率：15个测试用例

### 端到端测试

**e2e_download_test.rs** - 完整下载流程测试（实际下载文件）
- 测试单视频完整下载
- 测试跳过混流下载
- 测试使用配置文件下载
- 测试多分P下载
- 测试错误恢复
- 测试质量降级
- 覆盖率：7个测试用例

**注意**: 端到端测试默认被 `#[ignore]` 标记，需要手动运行

## 运行测试

### 快速开始

```bash
# 运行所有单元测试和集成测试（推荐）
cargo test

# 显示测试输出
cargo test -- --nocapture
```

### 运行特定测试文件
```bash
cargo test --test cli_test              # CLI参数解析测试
cargo test --test integration_test      # 集成测试
cargo test --test platform_bilibili_test # 平台测试
cargo test --test utils_config_test     # 配置管理测试
cargo test --test utils_file_test       # 文件工具测试
```

### 运行端到端测试（实际下载文件）

**前提条件**:
1. 安装 FFmpeg: `brew install ffmpeg` (macOS) 或 `sudo apt-get install ffmpeg` (Linux)
2. 配置认证（可选）: 创建 `tests/rvd.toml` 并添加 cookie

```bash
# 运行所有端到端测试
cargo test --test e2e_download_test -- --ignored --nocapture

# 运行特定端到端测试
cargo test --test e2e_download_test test_e2e_download_single_video -- --ignored --nocapture
cargo test --test e2e_download_test test_e2e_high_quality_with_auth -- --ignored --nocapture
```

**可用的端到端测试**:
- `test_e2e_download_single_video` - 单视频完整下载
- `test_e2e_download_skip_mux` - 跳过混流测试
- `test_e2e_download_with_config` - 使用配置文件
- `test_e2e_multi_page_download` - 多分P下载
- `test_e2e_error_recovery` - 错误恢复
- `test_e2e_quality_fallback` - 质量降级
- `test_e2e_high_quality_with_auth` - 会员高清下载（需要认证）

## 测试配置

### 可选配置文件

测试使用 `tests/rvd.toml` 配置文件（可选），其中包含：
- 默认质量和编码优先级
- 测试用的认证信息（cookie）
- 线程数等配置

示例配置：
```toml
[auth]
cookie = "your_bilibili_cookie_here"
access_token = "your_access_token_here"
```

**注意**：
1. 集成测试中涉及网络请求的测试会检查配置文件是否存在
2. 如果不存在或未配置认证信息，会跳过相应测试
3. 不要将包含真实认证信息的配置文件提交到版本控制！

### 测试数据和清理

测试产生的文件保存在 `tests/test_data/` 目录（已在 .gitignore 中）。

```bash
# 查看测试产物
ls -lh tests/test_data/

# 手动清理
rm -rf tests/test_data/*
```

## 测试覆盖的需求

根据设计文档，测试覆盖了以下核心功能：

### 已测试功能
- ✅ URL解析（需求 1.1, 1.2）
- ✅ 参数解析（需求 2.1, 2.2, 5.2-5.4）
- ✅ 配置管理（需求 10.1-10.5）
- ✅ 文件名处理（需求 8.1-8.4）
- ✅ 流选择逻辑（需求 2.3）
- ✅ 视频信息解析（需求 1.2）
- ✅ 认证处理（需求 4.1-4.4）

### 测试统计
- 总测试用例数：51个
- 单元测试：29个
- 集成测试：15个
- 端到端测试：7个
- 测试通过率：100%（不包括需要手动运行的端到端测试）

## 代码质量检查

### 编译检查
```bash
cargo check --tests
```

### Clippy检查
```bash
cargo clippy --tests -- -D warnings
```

### 格式检查
```bash
cargo fmt --check
```

## 测试最佳实践

1. **隔离性**：每个测试用例独立运行，不依赖其他测试
2. **可重复性**：测试结果应该是确定的和可重复的
3. **快速性**：单元测试应该快速执行（< 1秒）
4. **清晰性**：测试名称清楚地描述测试内容
5. **覆盖性**：测试覆盖正常情况和边界情况

## Continuous Integration

在 CI 环境中，建议只运行单元测试和集成测试，跳过端到端测试：

```bash
# CI环境中的测试命令
cargo test --lib --tests
```

如果需要在 CI 中运行端到端测试：
```bash
# 确保安装了FFmpeg
cargo test --test e2e_download_test -- --ignored
```

## Test Coverage

查看测试覆盖率：

```bash
# 安装 tarpaulin
cargo install cargo-tarpaulin

# 生成覆盖率报告
cargo tarpaulin --out Html --output-dir coverage
```

## Troubleshooting

### 测试失败常见原因

1. **FFmpeg未安装**: 某些测试需要FFmpeg
   - 解决方案: 安装FFmpeg或跳过相关测试

2. **网络连接问题**: 集成测试需要访问B站API
   - 解决方案: 检查网络连接或使用代理

3. **认证失败**: 某些测试需要有效的cookie
   - 解决方案: 提供有效的认证信息或跳过相关测试

4. **磁盘空间不足**: 端到端测试需要下载文件
   - 解决方案: 清理磁盘空间或清理test_data目录

### 跳过特定测试

```bash
# 跳过需要FFmpeg的测试
cargo test -- --skip muxer

# 跳过需要网络的测试
cargo test --lib
```

## Contributing

添加新测试时，请遵循以下规范：

1. **命名规范**: 测试函数名应清晰描述测试内容
2. **文档注释**: 为复杂测试添加注释说明
3. **清理资源**: 测试结束后清理临时文件
4. **独立性**: 测试应该相互独立，不依赖执行顺序
5. **标记**: 需要网络或长时间运行的测试应标记为 `#[ignore]`

示例：
```rust
#[tokio::test]
#[ignore] // 需要网络连接
async fn test_download_feature() {
    // 测试代码
    // ...
    
    // 清理
    cleanup_test_files();
}
```

## 未来改进

1. ✅ 增加端到端测试覆盖完整下载流程
2. ✅ 增加更多边界情况和错误处理测试
3. 添加性能测试和基准测试
4. 增加mock服务器测试（避免依赖真实网络）
5. 提高测试覆盖率到80%以上
6. 添加并发下载压力测试
