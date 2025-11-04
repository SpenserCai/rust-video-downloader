"""测试加载器"""
import importlib
import inspect
import logging
from pathlib import Path
from typing import List, Optional

from .config import Config
from .base_test import BaseTestCase


class TestLoader:
    """测试加载器"""
    
    def __init__(self, config: Config):
        """
        初始化测试加载器
        
        Args:
            config: 配置管理器
        """
        self.config = config
        self.test_dir = config.root_dir / 'tests'
        self.logger = logging.getLogger(__name__)
    
    def discover_tests(self,
                       pattern: Optional[str] = None,
                       tags: Optional[List[str]] = None) -> List[BaseTestCase]:
        """
        发现测试用例
        
        Args:
            pattern: 测试名称模式（包含匹配）
            tags: 标签过滤列表
            
        Returns:
            测试用例列表
        """
        test_cases = []
        
        # 查找所有测试文件
        test_files = self.test_dir.glob('test_*.py')
        
        for test_file in test_files:
            # 模式匹配
            if pattern and pattern not in test_file.stem:
                continue
            
            # 加载模块
            module_name = f"tests.{test_file.stem}"
            try:
                module = importlib.import_module(module_name)
                
                # 查找测试类
                for name, obj in inspect.getmembers(module):
                    if (inspect.isclass(obj) and
                        issubclass(obj, BaseTestCase) and
                        obj is not BaseTestCase):
                        
                        # 创建实例
                        test_case = obj(self.config)
                        
                        # 标签过滤
                        if tags and not any(tag in test_case.tags for tag in tags):
                            continue
                        
                        test_cases.append(test_case)
                        self.logger.debug(f"Loaded test: {test_case.name}")
                        
            except Exception as e:
                self.logger.error(f"Failed to load {test_file}: {e}", exc_info=True)
        
        return test_cases
    
    def load_test(self, test_name: str) -> Optional[BaseTestCase]:
        """
        加载指定的测试用例
        
        Args:
            test_name: 测试名称
            
        Returns:
            测试用例或None
        """
        tests = self.discover_tests()
        for test in tests:
            if test.name == test_name:
                return test
        return None
    
    def list_tests(self) -> List[str]:
        """
        列出所有测试用例
        
        Returns:
            测试名称列表
        """
        tests = self.discover_tests()
        return [test.name for test in tests]
