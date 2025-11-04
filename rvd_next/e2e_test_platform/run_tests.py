#!/usr/bin/env python3
"""E2E测试平台CLI入口"""
import argparse
import sys
import logging
from pathlib import Path

# 添加项目根目录到路径
sys.path.insert(0, str(Path(__file__).parent))

from core.config import Config
from core.loader import TestLoader
from core.runner import TestRunner
from reporters.console import ConsoleReporter
from reporters.json_reporter import JsonReporter
from reporters.html_reporter import HtmlReporter
from utils.logger import setup_logging


def main():
    """主函数"""
    parser = argparse.ArgumentParser(
        description='RVD E2E Test Platform',
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Examples:
  # Run all tests
  python run_tests.py
  
  # Run specific tests
  python run_tests.py TestSingleVideoDownload TestBatchDownload
  
  # Run tests matching pattern
  python run_tests.py --pattern video
  
  # Run tests with specific tags
  python run_tests.py --tags basic feature
  
  # List available tests
  python run_tests.py --list
  
  # Run tests in parallel
  python run_tests.py --parallel --workers 4
        """
    )
    
    parser.add_argument(
        'tests',
        nargs='*',
        help='Test names to run (empty for all)'
    )
    parser.add_argument(
        '-p', '--pattern',
        help='Test name pattern to match'
    )
    parser.add_argument(
        '-t', '--tags',
        nargs='+',
        help='Filter by tags'
    )
    parser.add_argument(
        '-l', '--list',
        action='store_true',
        help='List available tests'
    )
    parser.add_argument(
        '--parallel',
        action='store_true',
        help='Run tests in parallel'
    )
    parser.add_argument(
        '--workers',
        type=int,
        default=4,
        help='Number of parallel workers (default: 4)'
    )
    parser.add_argument(
        '-v', '--verbose',
        action='store_true',
        help='Verbose output'
    )
    parser.add_argument(
        '--config',
        default='config.yaml',
        help='Config file path (default: config.yaml)'
    )
    
    args = parser.parse_args()
    
    try:
        # 加载配置
        config = Config(args.config)
        
        # 设置日志
        setup_logging(config, args.verbose)
        
        logger = logging.getLogger(__name__)
        logger.info("E2E Test Platform started")
        
        # 创建加载器
        loader = TestLoader(config)
        
        # 列出测试
        if args.list:
            tests = loader.list_tests()
            print(f"\nAvailable tests ({len(tests)}):")
            for test in sorted(tests):
                print(f"  - {test}")
            print()
            return 0
        
        # 加载测试用例
        if args.tests:
            # 加载指定的测试
            test_cases = []
            for test_name in args.tests:
                test = loader.load_test(test_name)
                if test:
                    test_cases.append(test)
                else:
                    logger.warning(f"Test not found: {test_name}")
        else:
            # 发现所有测试
            test_cases = loader.discover_tests(pattern=args.pattern, tags=args.tags)
        
        if not test_cases:
            logger.error("No tests to run")
            return 1
        
        logger.info(f"Running {len(test_cases)} tests...")
        
        # 运行测试
        runner = TestRunner(config)
        results = runner.run_tests(
            test_cases,
            parallel=args.parallel or config.get('execution.parallel', False),
            max_workers=args.workers
        )
        
        # 生成报告
        output_dir = config.resolve_path(config.get('reporting.output_dir', './reports'))
        output_dir.mkdir(parents=True, exist_ok=True)
        
        # 控制台报告
        if config.get('reporting.console', True):
            console_reporter = ConsoleReporter()
            print(console_reporter.generate(results))
        
        # JSON报告
        if config.get('reporting.json', True):
            json_reporter = JsonReporter()
            json_path = output_dir / 'latest.json'
            json_path.write_text(json_reporter.generate(results), encoding='utf-8')
            logger.info(f"JSON report saved to: {json_path}")
        
        # HTML报告
        if config.get('reporting.html', True):
            html_reporter = HtmlReporter()
            html_path = output_dir / 'latest.html'
            html_path.write_text(html_reporter.generate(results), encoding='utf-8')
            logger.info(f"HTML report saved to: {html_path}")
        
        # 返回退出码
        failed = sum(1 for r in results if not r.passed)
        if failed == 0:
            logger.info("All tests passed!")
            return 0
        else:
            logger.error(f"{failed} test(s) failed")
            return 1
            
    except FileNotFoundError as e:
        print(f"Error: {e}", file=sys.stderr)
        return 1
    except Exception as e:
        print(f"Unexpected error: {e}", file=sys.stderr)
        if args.verbose:
            import traceback
            traceback.print_exc()
        return 1


if __name__ == '__main__':
    sys.exit(main())
