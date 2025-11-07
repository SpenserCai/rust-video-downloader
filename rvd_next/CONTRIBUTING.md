# 贡献指南

感谢你对 RVD Next 的关注！我们欢迎各种形式的贡献。

## 贡献方式

### 🐛 报告 Bug

如果你发现了 Bug，请：

1. 在 [GitHub Issues](https://github.com/SpenserCai/rust-video-downloader/issues) 搜索是否已有相关问题
2. 如果没有，创建新 Issue，包含：
   - 清晰的标题
   - 详细的问题描述
   - 复现步骤
   - 预期行为和实际行为
   - 环境信息（操作系统、Rust 版本等）
   - 相关日志（使用 `--verbose` 获取）

### 💡 建议新功能

如果你有新功能建议：

1. 在 [GitHub Discussions](https://github.com/SpenserCai/rust-video-downloader/discussions) 讨论
2. 说明功能的用途和价值
3. 如果可能，提供设计思路

### 📝 改进文档

文档改进包括：

- 修正错别字和语法错误
- 添加缺失的说明
- 改进示例代码
- 翻译文档

直接提交 Pull Request 即可。

### 💻 贡献代码

#### 开发流程

1. **Fork 仓库**
   ```bash
   # 在 GitHub 上 Fork 项目
   git clone https://github.com/YOUR_USERNAME/rust-video-downloader.git
   cd rust-video-downloader/rvd_next
   ```

2. **创建分支**
   ```bash
   git checkout -b feature/your-feature-name
   ```

3. **开发和测试**
   ```bash
   # 开发你的功能
   
   # 运行测试
   cargo test
   
   # 检查代码
   cargo clippy -- -D warnings
   cargo fmt --check
   ```

4. **提交代码**
   ```bash
   git add .
   git commit -m "feat: add your feature"
   ```
   
   提交信息格式：
   - `feat`: 新功能
   - `fix`: Bug 修复
   - `docs`: 文档更新
   - `refactor`: 代码重构
   - `test`: 测试相关
   - `chore`: 构建/工具相关

5. **推送到 GitHub**
   ```bash
   git push origin feature/your-feature-name
   ```

6. **创建 Pull Request**
   - 在 GitHub 上创建 Pull Request
   - 填写 PR 模板
   - 等待代码审查

#### 代码规范

- 遵循 Rust 官方代码风格
- 使用 `cargo fmt` 格式化代码
- 使用 `cargo clippy` 检查代码质量
- 为公共 API 编写文档注释
- 添加必要的测试

详见 [开发指南](docs/DEVELOPMENT.md)

## 添加新平台

添加新平台是最常见的贡献类型。完整步骤见 [开发指南 - 添加新平台](docs/DEVELOPMENT.md#添加新平台)

简要步骤：

1. 在 `src/platform/` 下创建新平台模块
2. 实现 `Platform` trait
3. 在 `Orchestrator` 中注册平台
4. 添加测试和文档
5. 提交 Pull Request

## Pull Request 指南

### PR 标题

使用清晰的标题，格式：`类型: 简短描述`

示例：
- `feat: add YouTube platform support`
- `fix: resolve CDN optimization issue`
- `docs: improve user guide`

### PR 描述

包含以下内容：

1. **变更说明**: 这个 PR 做了什么？
2. **动机**: 为什么需要这个变更？
3. **测试**: 如何测试这个变更？
4. **相关 Issue**: 关闭或关联的 Issue

### PR 检查清单

提交 PR 前，确保：

- [ ] 代码通过 `cargo test`
- [ ] 代码通过 `cargo clippy`
- [ ] 代码已用 `cargo fmt` 格式化
- [ ] 添加了必要的测试
- [ ] 更新了相关文档
- [ ] PR 描述清晰完整

## 代码审查

### 审查流程

1. 维护者会审查你的代码
2. 可能会提出修改建议
3. 根据反馈修改代码
4. 审查通过后合并

### 审查标准

- 代码质量和风格
- 测试覆盖率
- 文档完整性
- 性能影响
- 向后兼容性

## 社区准则

### 行为准则

- 尊重他人
- 保持友好和专业
- 接受建设性批评
- 关注项目目标

### 沟通渠道

- **GitHub Issues**: Bug 报告和功能请求
- **GitHub Discussions**: 一般讨论和问答
- **Pull Requests**: 代码审查和讨论

## 开发环境

### 前置要求

- Rust 1.70+
- FFmpeg（测试混流功能）
- Git

### 设置开发环境

```bash
# 克隆仓库
git clone https://github.com/YOUR_USERNAME/rust-video-downloader.git
cd rust-video-downloader/rvd_next

# 安装依赖
cargo build

# 运行测试
cargo test

# 运行程序
cargo run -- --help
```

详见 [开发指南 - 开发环境设置](docs/DEVELOPMENT.md#开发环境设置)

## 测试

### 运行测试

```bash
# 所有测试
cargo test

# 特定测试
cargo test test_bilibili

# 显示输出
cargo test -- --nocapture

# 集成测试
cargo test --test integration_test
```

### 编写测试

- 为新功能添加单元测试
- 为 Bug 修复添加回归测试
- 确保测试可重复运行
- 使用有意义的测试名称

## 文档

### 文档类型

- **代码文档**: Rust 文档注释
- **用户文档**: `docs/` 目录下的 Markdown 文件
- **API 文档**: 通过 `cargo doc` 生成

### 更新文档

修改代码时，同时更新：

- 代码注释
- 用户指南（如果影响用户）
- API 文档（如果是公共 API）
- CHANGELOG.md

## 发布流程

（仅维护者）

1. 更新版本号（`Cargo.toml`）
2. 更新 `CHANGELOG.md`
3. 创建 Git tag
4. 发布到 crates.io
5. 创建 GitHub Release

## 获取帮助

需要帮助？

- 查看 [开发指南](docs/DEVELOPMENT.md)
- 在 [GitHub Discussions](https://github.com/SpenserCai/rust-video-downloader/discussions) 提问
- 查看现有代码和测试

## 许可证

贡献的代码将采用项目的 MIT 许可证。

## 致谢

感谢所有贡献者！你们的贡献让 RVD Next 变得更好。

---

再次感谢你的贡献！🎉
