#!/usr/bin/env python3
"""
E2E测试套件启动脚本
根据已实现的功能和测试用例，提供可扩展的测试执行方案
"""
import sys
import argparse
import logging
from pathlib import Path
from typing import List, Dict, Set

# 添加项目根目录到路径
sys.path.insert(0, str(Path(__file__).parent))

from core.config import Config
from core.runner import TestRunner
from reporters.console import ConsoleReporter
from reporters.json_reporter import JsonReporter
from reporters.html_reporter import HtmlReporter
from utils.logger import setup_logging


# ============================================
# 测试用例注册表
# ============================================

# 已实现并启用的测试用例（功能已开发完成）
ENABLED_TESTS = {
    # ===== 核心功能测试 =====
    'core': {
        # 配置管理
        'TestDefaultConfig': {
            'module': 'tests.core.test_config',
            'class': 'TestDefaultConfig',
            'description': '测试默认配置',
            'tags': ['core', 'config', 'basic'],
        },
        'TestCustomConfig': {
            'module': 'tests.core.test_config',
            'class': 'TestCustomConfig',
            'description': '测试自定义配置文件',
            'tags': ['core', 'config'],
        },
        'TestCLIPriority': {
            'module': 'tests.core.test_config',
            'class': 'TestCLIPriority',
            'description': '测试CLI参数优先级',
            'tags': ['core', 'config', 'priority'],
        },
        
        # 下载器
        'TestBuiltinDownloader': {
            'module': 'tests.core.test_downloader',
            'class': 'TestBuiltinDownloader',
            'description': '测试内置多线程下载器',
            'tags': ['core', 'download', 'basic'],
        },
        'TestDownloadProgress': {
            'module': 'tests.core.test_downloader',
            'class': 'TestDownloadProgress',
            'description': '测试下载进度跟踪',
            'tags': ['core', 'download', 'progress'],
        },
        
        # 混流器
        'TestBasicMux': {
            'module': 'tests.core.test_muxer',
            'class': 'TestBasicMux',
            'description': '测试基础混流功能',
            'tags': ['core', 'mux', 'basic'],
        },
        'TestSubtitleEmbed': {
            'module': 'tests.core.test_muxer',
            'class': 'TestSubtitleEmbed',
            'description': '测试字幕嵌入',
            'tags': ['core', 'mux', 'subtitle'],
        },
        'TestChapterEmbed': {
            'module': 'tests.core.test_muxer',
            'class': 'TestChapterEmbed',
            'description': '测试章节嵌入',
            'tags': ['core', 'mux', 'chapters'],
        },
        'TestSkipMux': {
            'module': 'tests.core.test_muxer',
            'class': 'TestSkipMux',
            'description': '测试跳过混流模式',
            'tags': ['core', 'mux', 'skip'],
        },
        
        # HTTP客户端
        'TestDefaultUserAgent': {
            'module': 'tests.core.test_http_client',
            'class': 'TestDefaultUserAgent',
            'description': '测试默认User-Agent',
            'tags': ['core', 'http'],
        },
        'TestCustomUserAgentCLI': {
            'module': 'tests.core.test_http_client',
            'class': 'TestCustomUserAgentCLI',
            'description': '测试自定义User-Agent(CLI)',
            'tags': ['core', 'http', 'cli'],
        },
        
        # 错误处理
        'TestInvalidUrl': {
            'module': 'tests.core.test_error_handling',
            'class': 'TestInvalidUrl',
            'description': '测试无效URL错误处理',
            'tags': ['core', 'error', 'validation'],
        },
        'TestNetworkError': {
            'module': 'tests.core.test_error_handling',
            'class': 'TestNetworkError',
            'description': '测试网络错误处理',
            'tags': ['core', 'error', 'network'],
        },
    },
    
    # ===== Bilibili平台测试 =====
    'bilibili': {
        # 基础下载
        'TestBVVideoDownload': {
            'module': 'tests.platforms.bilibili.test_basic_download',
            'class': 'TestBVVideoDownload',
            'description': '测试BV号视频下载',
            'tags': ['bilibili', 'download', 'basic', 'bv'],
        },
        'TestMultiPageVideoDownload': {
            'module': 'tests.platforms.bilibili.test_basic_download',
            'class': 'TestMultiPageVideoDownload',
            'description': '测试多P视频下载',
            'tags': ['bilibili', 'download', 'basic', 'multi-page'],
        },
        'TestBangumiEPDownload': {
            'module': 'tests.platforms.bilibili.test_basic_download',
            'class': 'TestBangumiEPDownload',
            'description': '测试番剧EP号下载',
            'tags': ['bilibili', 'download', 'bangumi', 'ep'],
        },
        'TestBangumiSSDownload': {
            'module': 'tests.platforms.bilibili.test_basic_download',
            'class': 'TestBangumiSSDownload',
            'description': '测试番剧SS号下载',
            'tags': ['bilibili', 'download', 'bangumi', 'ss'],
        },
        
        # 批量下载
        'TestFavoritesBatchDownload': {
            'module': 'tests.platforms.bilibili.test_batch_download',
            'class': 'TestFavoritesBatchDownload',
            'description': '测试收藏夹批量下载',
            'tags': ['bilibili', 'batch', 'favorites'],
        },
        'TestUserSpaceBatchDownload': {
            'module': 'tests.platforms.bilibili.test_batch_download',
            'class': 'TestUserSpaceBatchDownload',
            'description': '测试UP主空间批量下载',
            'tags': ['bilibili', 'batch', 'wbi'],
        },
        
        # 质量选择
        'TestQualitySelection': {
            'module': 'tests.platforms.bilibili.test_quality_selection',
            'class': 'TestQualitySelection',
            'description': '测试质量选择功能',
            'tags': ['bilibili', 'quality', 'basic'],
        },
        'TestQualityPriority': {
            'module': 'tests.platforms.bilibili.test_quality_selection',
            'class': 'TestQualityPriority',
            'description': '测试质量优先级',
            'tags': ['bilibili', 'quality', 'priority'],
        },
        'TestCodecPriority': {
            'module': 'tests.platforms.bilibili.test_quality_selection',
            'class': 'TestCodecPriority',
            'description': '测试编码优先级',
            'tags': ['bilibili', 'quality', 'codec'],
        },
        'TestDolbyVision': {
            'module': 'tests.platforms.bilibili.test_quality_selection',
            'class': 'TestDolbyVision',
            'description': '测试杜比视界',
            'tags': ['bilibili', 'quality', 'dolby', 'vip'],
        },
        
        # 弹幕
        'TestXMLDanmaku': {
            'module': 'tests.platforms.bilibili.test_danmaku',
            'class': 'TestXMLDanmaku',
            'description': '测试XML弹幕下载',
            'tags': ['bilibili', 'danmaku', 'xml'],
        },
        'TestASSDanmaku': {
            'module': 'tests.platforms.bilibili.test_danmaku',
            'class': 'TestASSDanmaku',
            'description': '测试ASS弹幕转换',
            'tags': ['bilibili', 'danmaku', 'ass'],
        },
        'TestDanmakuDecompression': {
            'module': 'tests.platforms.bilibili.test_danmaku',
            'class': 'TestDanmakuDecompression',
            'description': '测试弹幕解压缩',
            'tags': ['bilibili', 'danmaku', 'decompression'],
        },
        
        # 字幕
        'TestSingleSubtitle': {
            'module': 'tests.platforms.bilibili.test_subtitle',
            'class': 'TestSingleSubtitle',
            'description': '测试单字幕下载',
            'tags': ['bilibili', 'subtitle', 'basic'],
        },
        
        # 章节
        'TestNormalVideoChapters': {
            'module': 'tests.platforms.bilibili.test_chapters',
            'class': 'TestNormalVideoChapters',
            'description': '测试普通视频章节',
            'tags': ['bilibili', 'chapters', 'normal'],
        },
        'TestChapterEmbedding': {
            'module': 'tests.platforms.bilibili.test_chapters',
            'class': 'TestChapterEmbedding',
            'description': '测试章节嵌入',
            'tags': ['bilibili', 'chapters', 'embed'],
        },
    },
}

# 待实现的测试用例（功能尚未完全开发或URL未配置）
DISABLED_TESTS = {
    'core': {
        # 'TestAria2cDownloader': {
        #     'module': 'tests.core.test_downloader',
        #     'class': 'TestAria2cDownloader',
        #     'description': '测试aria2c下载器',
        #     'tags': ['core', 'download', 'aria2c'],
        #     'reason': 'aria2c集成功能待实现，需要安装aria2c',
        # },
        # 'TestCustomUserAgentConfig': {
        #     'module': 'tests.core.test_http_client',
        #     'class': 'TestCustomUserAgentConfig',
        #     'description': '测试配置文件中的User-Agent',
        #     'tags': ['core', 'http', 'config'],
        #     'reason': '配置文件user_agent字段待实现',
        # },
        # 'TestTempFilesCleanup': {
        #     'module': 'tests.core.test_temp_files',
        #     'class': 'TestNormalCleanup',
        #     'description': '测试临时文件清理',
        #     'tags': ['core', 'temp', 'cleanup'],
        #     'reason': '临时文件管理功能待完善',
        # },
        # 'TestOutputTemplate': {
        #     'module': 'tests.core.test_output_template',
        #     'class': 'TestDefaultTemplate',
        #     'description': '测试输出模板',
        #     'tags': ['core', 'output', 'template'],
        #     'reason': '输出模板功能待实现',
        # },
        # 'TestDiskSpaceError': {
        #     'module': 'tests.core.test_error_handling',
        #     'class': 'TestDiskSpaceError',
        #     'description': '测试磁盘空间错误',
        #     'tags': ['core', 'error', 'disk'],
        #     'reason': '磁盘空间检测功能待实现',
        # },
        # 'TestFFmpegNotAvailable': {
        #     'module': 'tests.core.test_error_handling',
        #     'class': 'TestFFmpegNotAvailable',
        #     'description': '测试FFmpeg不可用',
        #     'tags': ['core', 'error', 'ffmpeg'],
        #     'reason': 'FFmpeg检测功能待完善',
        # },
        # 'TestAuthRequired': {
        #     'module': 'tests.core.test_error_handling',
        #     'class': 'TestAuthRequired',
        #     'description': '测试认证要求',
        #     'tags': ['core', 'error', 'auth'],
        #     'reason': '需要配置需要认证的测试URL',
        # },
    },
    'bilibili': {
        # 'TestAVVideoDownload': {
        #     'module': 'tests.platforms.bilibili.test_basic_download',
        #     'class': 'TestAVVideoDownload',
        #     'description': '测试AV号视频下载',
        #     'tags': ['bilibili', 'download', 'av'],
        #     'reason': '需要配置AV号测试URL',
        # },
        # 'TestMultiLanguageSubtitle': {
        #     'module': 'tests.platforms.bilibili.test_subtitle',
        #     'class': 'TestMultiLanguageSubtitle',
        #     'description': '测试多语言字幕',
        #     'tags': ['bilibili', 'subtitle', 'multilang'],
        #     'reason': '需要配置多语言字幕测试URL',
        # },
        # 'TestBangumiChapters': {
        #     'module': 'tests.platforms.bilibili.test_chapters',
        #     'class': 'TestBangumiChapters',
        #     'description': '测试番剧章节',
        #     'tags': ['bilibili', 'chapters', 'bangumi'],
        #     'reason': '需要配置番剧章节测试URL',
        # },
    },
}


# ============================================
# 测试套件定义
# ============================================

TEST_SUITES = {
    'smoke': {
        'description': '冒烟测试 - 快速验证核心功能',
        'tests': [
            'TestDefaultConfig',
            'TestBuiltinDownloader',
            'TestBasicMux',
            'TestBVVideoDownload',
            'TestQualitySelection',
        ],
    },
    'core': {
        'description': '核心功能测试 - 测试所有核心模块',
        'tests': [
            'TestDefaultConfig',
            'TestCustomConfig',
            'TestCLIPriority',
            'TestBuiltinDownloader',
            'TestDownloadProgress',
            'TestBasicMux',
            'TestSubtitleEmbed',
            'TestChapterEmbed',
            'TestSkipMux',
            'TestDefaultUserAgent',
            'TestCustomUserAgentCLI',
            'TestInvalidUrl',
            'TestNetworkError',
        ],
    },
    'bilibili': {
        'description': 'Bilibili平台测试 - 测试所有Bilibili功能',
        'tests': [
            'TestBVVideoDownload',
            'TestMultiPageVideoDownload',
            'TestBangumiEPDownload',
            'TestBangumiSSDownload',
            'TestFavoritesBatchDownload',
            'TestUserSpaceBatchDownload',
            'TestQualitySelection',
            'TestQualityPriority',
            'TestCodecPriority',
            'TestDolbyVision',
            'TestXMLDanmaku',
            'TestASSDanmaku',
            'TestDanmakuDecompression',
            'TestSingleSubtitle',
            'TestNormalVideoChapters',
            'TestChapterEmbedding',
        ],
    },
    'full': {
        'description': '完整测试 - 运行所有已启用的测试',
        'tests': 'all',  # 特殊标记，表示运行所有启用的测试
    },
}


# ============================================
# 测试加载和执行
# ============================================

def load_test_class(module_path: str, class_name: str, config: Config):
    """动态加载测试类"""
    import importlib
    try:
        module = importlib.import_module(module_path)
        test_class = getattr(module, class_name)
        return test_class(config)
    except Exception as e:
        logging.error(f"Failed to load {class_name} from {module_path}: {e}")
        return None


def get_all_enabled_tests() -> List[str]:
    """获取所有已启用的测试名称"""
    tests = []
    for category in ENABLED_TESTS.values():
        tests.extend(category.keys())
    return tests


def get_tests_by_suite(suite_name: str) -> List[str]:
    """根据套件名称获取测试列表"""
    if suite_name not in TEST_SUITES:
        return []
    
    suite = TEST_SUITES[suite_name]
    if suite['tests'] == 'all':
        return get_all_enabled_tests()
    return suite['tests']


def get_tests_by_tags(tags: List[str]) -> List[str]:
    """根据标签获取测试列表"""
    matching_tests = []
    for category in ENABLED_TESTS.values():
        for test_name, test_info in category.items():
            test_tags = test_info.get('tags', [])
            if any(tag in test_tags for tag in tags):
                matching_tests.append(test_name)
    return matching_tests


def load_tests(test_names: List[str], config: Config) -> List:
    """加载指定的测试用例"""
    test_cases = []
    for test_name in test_names:
        # 在所有类别中查找测试
        test_info = None
        for category in ENABLED_TESTS.values():
            if test_name in category:
                test_info = category[test_name]
                break
        
        if not test_info:
            logging.warning(f"Test not found: {test_name}")
            continue
        
        # 加载测试类
        test_case = load_test_class(
            test_info['module'],
            test_info['class'],
            config
        )
        
        if test_case:
            test_cases.append(test_case)
            logging.debug(f"Loaded test: {test_name}")
    
    return test_cases


def list_tests(show_disabled: bool = False):
    """列出所有测试"""
    print("\n" + "="*60)
    print("已启用的测试用例")
    print("="*60)
    
    for category_name, category_tests in ENABLED_TESTS.items():
        print(f"\n[{category_name.upper()}]")
        for test_name, test_info in category_tests.items():
            tags_str = ', '.join(test_info['tags'])
            print(f"  ✓ {test_name}")
            print(f"    描述: {test_info['description']}")
            print(f"    标签: {tags_str}")
    
    if show_disabled:
        print("\n" + "="*60)
        print("待实现的测试用例")
        print("="*60)
        
        for category_name, category_tests in DISABLED_TESTS.items():
            if not category_tests:
                continue
            print(f"\n[{category_name.upper()}]")
            for test_name, test_info in category_tests.items():
                tags_str = ', '.join(test_info['tags'])
                print(f"  ✗ {test_name}")
                print(f"    描述: {test_info['description']}")
                print(f"    标签: {tags_str}")
                print(f"    原因: {test_info.get('reason', '未说明')}")
    
    print("\n" + "="*60)
    print("测试套件")
    print("="*60)
    for suite_name, suite_info in TEST_SUITES.items():
        test_count = len(get_tests_by_suite(suite_name))
        print(f"\n  {suite_name}: {suite_info['description']}")
        print(f"    包含 {test_count} 个测试")
    
    print()


def main():
    """主函数"""
    parser = argparse.ArgumentParser(
        description='RVD E2E测试套件启动器',
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
示例:
  # 运行冒烟测试
  python run_test_suite.py --suite smoke
  
  # 运行核心功能测试
  python run_test_suite.py --suite core
  
  # 运行Bilibili平台测试
  python run_test_suite.py --suite bilibili
  
  # 运行完整测试
  python run_test_suite.py --suite full
  
  # 运行指定的测试
  python run_test_suite.py --tests TestBVVideoDownload TestQualitySelection
  
  # 根据标签运行测试
  python run_test_suite.py --tags basic download
  
  # 列出所有测试
  python run_test_suite.py --list
  
  # 列出所有测试（包括待实现的）
  python run_test_suite.py --list --show-disabled
        """
    )
    
    parser.add_argument(
        '--suite',
        choices=list(TEST_SUITES.keys()),
        help='运行指定的测试套件'
    )
    parser.add_argument(
        '--tests',
        nargs='+',
        help='运行指定的测试用例'
    )
    parser.add_argument(
        '--tags',
        nargs='+',
        help='根据标签过滤测试'
    )
    parser.add_argument(
        '--list',
        action='store_true',
        help='列出所有测试用例'
    )
    parser.add_argument(
        '--show-disabled',
        action='store_true',
        help='显示待实现的测试用例'
    )
    parser.add_argument(
        '--parallel',
        action='store_true',
        help='并行运行测试'
    )
    parser.add_argument(
        '--workers',
        type=int,
        default=4,
        help='并行工作线程数 (默认: 4)'
    )
    parser.add_argument(
        '-v', '--verbose',
        action='store_true',
        help='详细输出'
    )
    parser.add_argument(
        '--config',
        default='config.yaml',
        help='配置文件路径 (默认: config.yaml)'
    )
    
    args = parser.parse_args()
    
    # 列出测试
    if args.list:
        list_tests(args.show_disabled)
        return 0
    
    try:
        # 加载配置
        config = Config(args.config)
        
        # 设置日志
        setup_logging(config, args.verbose)
        logger = logging.getLogger(__name__)
        
        logger.info("RVD E2E测试套件启动")
        
        # 确定要运行的测试
        test_names = []
        
        if args.suite:
            test_names = get_tests_by_suite(args.suite)
            logger.info(f"运行测试套件: {args.suite}")
        elif args.tests:
            test_names = args.tests
            logger.info(f"运行指定测试: {', '.join(test_names)}")
        elif args.tags:
            test_names = get_tests_by_tags(args.tags)
            logger.info(f"根据标签运行测试: {', '.join(args.tags)}")
        else:
            # 默认运行冒烟测试
            test_names = get_tests_by_suite('smoke')
            logger.info("未指定测试，运行默认冒烟测试")
        
        if not test_names:
            logger.error("没有找到匹配的测试用例")
            return 1
        
        # 加载测试用例
        test_cases = load_tests(test_names, config)
        
        if not test_cases:
            logger.error("没有成功加载任何测试用例")
            return 1
        
        logger.info(f"共加载 {len(test_cases)} 个测试用例")
        
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
            logger.info(f"JSON报告已保存: {json_path}")
        
        # HTML报告
        if config.get('reporting.html', True):
            html_reporter = HtmlReporter()
            html_path = output_dir / 'latest.html'
            html_path.write_text(html_reporter.generate(results), encoding='utf-8')
            logger.info(f"HTML报告已保存: {html_path}")
        
        # 返回退出码
        failed = sum(1 for r in results if not r.passed)
        if failed == 0:
            logger.info("✓ 所有测试通过!")
            return 0
        else:
            logger.error(f"✗ {failed} 个测试失败")
            return 1
            
    except FileNotFoundError as e:
        print(f"错误: {e}", file=sys.stderr)
        return 1
    except Exception as e:
        print(f"意外错误: {e}", file=sys.stderr)
        if args.verbose:
            import traceback
            traceback.print_exc()
        return 1


if __name__ == '__main__':
    sys.exit(main())
