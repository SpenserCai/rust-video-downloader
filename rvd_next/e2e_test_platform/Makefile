# RVD E2E测试平台 Makefile

.PHONY: help smoke core bilibili full list clean install build

# 默认目标
help:
	@echo "RVD E2E测试平台 - 可用命令:"
	@echo ""
	@echo "  make smoke      - 运行冒烟测试 (5-10分钟)"
	@echo "  make core       - 运行核心功能测试 (15-20分钟)"
	@echo "  make bilibili   - 运行Bilibili平台测试 (20-30分钟)"
	@echo "  make full       - 运行完整测试 (30-40分钟)"
	@echo "  make parallel   - 并行运行完整测试 (加速)"
	@echo ""
	@echo "  make list       - 列出所有测试用例"
	@echo "  make list-all   - 列出所有测试（包括待实现的）"
	@echo ""
	@echo "  make install    - 安装Python依赖"
	@echo "  make build      - 编译RVD可执行文件"
	@echo "  make clean      - 清理测试产物"
	@echo ""
	@echo "  make report     - 在浏览器中打开最新的HTML报告"
	@echo ""
	@echo "高级用法:"
	@echo "  make test TESTS='TestBVVideoDownload TestQualitySelection'"
	@echo "  make test TAGS='download quality'"
	@echo ""

# 安装依赖
install:
	@echo "安装Python依赖..."
	pip install -r requirements.txt

# 编译RVD
build:
	@echo "编译RVD..."
	cd .. && cargo build --release

# 冒烟测试
smoke:
	@echo "运行冒烟测试..."
	python run_test_suite.py --suite smoke

# 核心功能测试
core:
	@echo "运行核心功能测试..."
	python run_test_suite.py --suite core

# Bilibili平台测试
bilibili:
	@echo "运行Bilibili平台测试..."
	python run_test_suite.py --suite bilibili

# 完整测试
full:
	@echo "运行完整测试..."
	python run_test_suite.py --suite full

# 并行测试
parallel:
	@echo "并行运行完整测试..."
	python run_test_suite.py --suite full --parallel --workers 4

# 列出测试
list:
	@python run_test_suite.py --list

# 列出所有测试（包括待实现的）
list-all:
	@python run_test_suite.py --list --show-disabled

# 运行指定测试
test:
ifdef TESTS
	@echo "运行指定测试: $(TESTS)"
	@python run_test_suite.py --tests $(TESTS)
else ifdef TAGS
	@echo "运行标签测试: $(TAGS)"
	@python run_test_suite.py --tags $(TAGS)
else
	@echo "错误: 请指定 TESTS 或 TAGS"
	@echo "示例: make test TESTS='TestBVVideoDownload'"
	@echo "示例: make test TAGS='download quality'"
endif

# 清理测试产物
clean:
	@echo "清理测试产物..."
	rm -rf workdir/*
	rm -rf reports/*.json reports/*.html
	find . -type d -name "__pycache__" -exec rm -rf {} + 2>/dev/null || true
	find . -type f -name "*.pyc" -delete 2>/dev/null || true
	@echo "清理完成"

# 打开HTML报告
report:
	@if [ -f reports/latest.html ]; then \
		echo "打开HTML报告..."; \
		if command -v open > /dev/null; then \
			open reports/latest.html; \
		elif command -v xdg-open > /dev/null; then \
			xdg-open reports/latest.html; \
		else \
			echo "请手动打开: reports/latest.html"; \
		fi \
	else \
		echo "错误: 报告文件不存在，请先运行测试"; \
	fi

# 详细输出的测试
verbose:
	@echo "运行冒烟测试（详细输出）..."
	python run_test_suite.py --suite smoke -v

# 快速检查（只运行最基础的测试）
quick:
	@echo "快速检查..."
	python run_test_suite.py --tests TestDefaultConfig TestBVVideoDownload

# CI/CD模式（并行+详细输出）
ci:
	@echo "CI/CD模式运行..."
	python run_test_suite.py --suite full --parallel --workers 8 -v
