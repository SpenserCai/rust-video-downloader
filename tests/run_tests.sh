#!/bin/bash
# RVD 测试运行脚本

set -e

echo "================================"
echo "RVD 测试套件"
echo "================================"
echo ""

# 颜色定义
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# 检查配置文件
if [ ! -f "tests/rvd.toml" ]; then
    echo -e "${YELLOW}警告: tests/rvd.toml 不存在${NC}"
    echo "某些测试可能会失败。请复制 tests/rvd.toml.example 并配置。"
    echo ""
fi

# 检查 FFmpeg
if ! command -v ffmpeg &> /dev/null; then
    echo -e "${YELLOW}警告: FFmpeg 未安装${NC}"
    echo "混流相关的测试可能会失败。"
    echo ""
fi

echo "运行测试..."
echo ""

# 1. 单元测试
echo -e "${GREEN}[1/5] 运行单元测试...${NC}"
cargo test --test cli_test --quiet
cargo test --test core_chapter_test --quiet
cargo test --test core_danmaku_test --quiet
cargo test --test platform_bilibili_test --quiet
echo -e "${GREEN}✓ 单元测试完成${NC}"
echo ""

# 2. 集成测试
echo -e "${GREEN}[2/5] 运行集成测试...${NC}"
cargo test --test integration_test --quiet
echo -e "${GREEN}✓ 集成测试完成${NC}"
echo ""

# 3. 端到端测试 - 解析测试（快速）
echo -e "${GREEN}[3/5] 运行端到端解析测试...${NC}"
cargo test --test e2e_download_test test_32_1 --quiet
cargo test --test e2e_download_test test_32_2_parse --quiet
cargo test --test e2e_download_test test_32_4_parse --quiet
echo -e "${GREEN}✓ 解析测试完成${NC}"
echo ""

# 4. 端到端测试 - 下载测试（较慢，可选）
echo -e "${YELLOW}[4/5] 端到端下载测试（可能需要几分钟）...${NC}"
read -p "是否运行下载测试？(y/N) " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    cargo test --test e2e_download_test test_32_7 --quiet -- --nocapture
    cargo test --test e2e_download_test test_info_only --quiet -- --nocapture
    echo -e "${GREEN}✓ 下载测试完成${NC}"
else
    echo -e "${YELLOW}⊘ 跳过下载测试${NC}"
fi
echo ""

# 5. 显示测试统计
echo -e "${GREEN}[5/5] 测试统计${NC}"
echo "运行完整测试报告："
cargo test --test e2e_download_test -- --list | grep -c "test" || echo "0"
echo ""

echo "================================"
echo -e "${GREEN}测试完成！${NC}"
echo "================================"
echo ""
echo "提示："
echo "- 运行所有测试: cargo test"
echo "- 运行特定测试: cargo test test_name"
echo "- 显示输出: cargo test -- --nocapture"
echo "- 包括忽略的测试: cargo test -- --include-ignored"
