# Release Guide

本文档说明如何使用 GitHub Actions 发布新版本。

## 发布流程

### 1. 准备发布

在发布前，确保：

- [ ] 所有功能已完成并测试通过
- [ ] 更新 `Cargo.toml` 中的版本号
- [ ] 更新 `CHANGELOG.md`，记录本版本的变更
- [ ] 更新 `README.md`（如有必要）
- [ ] 提交所有更改到主分支

### 2. 触发 Release Action

1. 进入 GitHub 仓库页面
2. 点击 **Actions** 标签
3. 在左侧选择 **Build and Release** 工作流
4. 点击右上角的 **Run workflow** 按钮
5. 填写参数：
   - **version**: 版本号（如 `v0.1.0`）
   - **build_windows**: 是否构建 Windows 版本
   - **build_linux**: 是否构建 Linux 版本
   - **build_macos**: 是否构建 macOS 版本
   - **publish_release**: 是否发布到 GitHub Release
6. 点击 **Run workflow** 开始构建

### 3. 构建产物

工作流会为每个平台生成以下文件：

**Windows:**
- `rvd-{version}-x86_64-pc-windows-msvc.zip`

**Linux:**
- `rvd-{version}-x86_64-unknown-linux-musl.tar.gz` (x86_64, static)

**macOS:**
- `rvd-{version}-x86_64-apple-darwin.tar.gz` (Intel)
- `rvd-{version}-aarch64-apple-darwin.tar.gz` (Apple Silicon)

### 4. 测试构建

如果 `publish_release` 设置为 `false`：

- 构建产物会作为 Artifacts 上传
- 可以从 Actions 页面下载测试
- 测试通过后，再次运行工作流并启用 `publish_release`

### 5. 发布到 Release

如果 `publish_release` 设置为 `true`：

- 自动创建 GitHub Release
- 上传所有构建产物
- 生成 SHA256 校验和文件
- 使用预定义的 Release 说明

### 6. 发布后

发布完成后：

1. 检查 Release 页面，确认所有文件已上传
2. 编辑 Release 说明，添加详细的更新内容
3. 如果是正式版本，取消 "Pre-release" 标记
4. 在社交媒体或相关渠道宣布新版本

## 版本号规范

遵循 [Semantic Versioning](https://semver.org/)：

- **MAJOR** (主版本号): 不兼容的 API 变更
- **MINOR** (次版本号): 向后兼容的功能新增
- **PATCH** (修订号): 向后兼容的问题修正

示例：
- `v0.1.0` - 初始版本
- `v0.2.0` - 新增功能
- `v0.2.1` - 修复 bug
- `v1.0.0` - 第一个稳定版本

## 测试发布

在正式发布前，建议先进行测试发布：

1. 使用测试版本号（如 `v0.1.0-beta.1`）
2. 仅构建一个平台进行快速测试
3. 不启用 `publish_release`
4. 从 Artifacts 下载并测试
5. 确认无误后再进行正式发布

## 故障排除

### 构建失败

- 检查 Rust 代码是否有编译错误
- 查看 Actions 日志获取详细错误信息
- 确保所有依赖项都已正确配置

### 发布失败

- 确保有足够的权限（需要 `contents: write`）
- 检查版本号是否已存在
- 确认 `GITHUB_TOKEN` 可用

### 跨平台编译问题

- macOS: 需要在 macOS runner 上构建
- Windows: 使用 MSVC 工具链
- Linux: 使用 musl 静态编译，确保跨发行版兼容性

## 手动发布（备用方案）

如果 GitHub Actions 不可用，可以手动构建和发布：

```bash
# 构建所有平台（需要相应的工具链）
cargo build --release --target x86_64-pc-windows-msvc
cargo build --release --target x86_64-unknown-linux-musl
cargo build --release --target x86_64-apple-darwin
cargo build --release --target aarch64-apple-darwin

# 创建压缩包
# Windows
zip rvd-v0.1.0-x86_64-pc-windows-msvc.zip target/x86_64-pc-windows-msvc/release/rvd.exe

# Linux/macOS
tar czf rvd-v0.1.0-x86_64-unknown-linux-musl.tar.gz -C target/x86_64-unknown-linux-musl/release rvd

# 生成校验和
sha256sum rvd-*.{zip,tar.gz} > SHA256SUMS

# 手动创建 GitHub Release 并上传文件
```

## Linux 静态编译说明

本项目使用 **musl** 进行 Linux 静态编译，而不是 glibc 动态链接。这样做的好处：

- ✅ **完全可移植**: 可以在任何 x86_64 Linux 发行版上运行
- ✅ **无依赖问题**: 不会出现 `GLIBC_X.XX not found` 错误
- ✅ **单一二进制**: 不需要安装额外的运行时库
- ✅ **跨发行版兼容**: 在 Ubuntu、Debian、CentOS、Alpine 等系统上都能运行

如果需要在本地构建 musl 版本：

```bash
# 安装 musl 工具
sudo apt-get install musl-tools  # Ubuntu/Debian
# 或
brew install filosottile/musl-cross/musl-cross  # macOS

# 添加 musl 目标
rustup target add x86_64-unknown-linux-musl

# 构建
cargo build --release --target x86_64-unknown-linux-musl
```

## 注意事项

1. **版本号一致性**: 确保 `Cargo.toml`、`CHANGELOG.md` 和 Release 版本号一致
2. **测试充分**: 发布前务必进行充分测试
3. **文档更新**: 确保文档与代码同步
4. **向后兼容**: 尽量保持 API 的向后兼容性
5. **安全性**: 不要在 Release 说明中包含敏感信息
6. **静态编译**: Linux 版本使用 musl 静态编译，确保最大兼容性
