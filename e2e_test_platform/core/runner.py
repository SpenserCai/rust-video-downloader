"""测试运行器"""
from typing import List
from concurrent.futures import ThreadPoolExecutor, as_completed
import logging

from .config import Config
from .base_test import BaseTestCase
from .result import TestResult


class TestRunner:
    """测试运行器"""
    
    def __init__(self, config: Config):
        """
        初始化测试运行器
        
        Args:
            config: 配置管理器
        """
        self.config = config
        self.logger = logging.getLogger(__name__)
    
    def run_tests(self,
                  test_cases: List[BaseTestCase],
                  parallel: bool = False,
                  max_workers: int = 4) -> List[TestResult]:
        """
        运行测试用例
        
        Args:
            test_cases: 测试用例列表
            parallel: 是否并行执行
            max_workers: 最大并行数
            
        Returns:
            测试结果列表
        """
        if parallel:
            return self._run_parallel(test_cases, max_workers)
        else:
            return self._run_serial(test_cases)
    
    def _run_serial(self, test_cases: List[BaseTestCase]) -> List[TestResult]:
        """
        串行执行测试
        
        Args:
            test_cases: 测试用例列表
            
        Returns:
            测试结果列表
        """
        results = []
        for i, test in enumerate(test_cases, 1):
            self.logger.info(f"Running test [{i}/{len(test_cases)}]: {test.name}")
            result = test.run()
            results.append(result)
            
            if not result.passed and self.config.get('execution.stop_on_failure'):
                self.logger.warning("Stopping on failure")
                break
        
        return results
    
    def _run_parallel(self,
                      test_cases: List[BaseTestCase],
                      max_workers: int) -> List[TestResult]:
        """
        并行执行测试
        
        Args:
            test_cases: 测试用例列表
            max_workers: 最大并行数
            
        Returns:
            测试结果列表
        """
        results = []
        with ThreadPoolExecutor(max_workers=max_workers) as executor:
            future_to_test = {executor.submit(test.run): test for test in test_cases}
            
            for future in as_completed(future_to_test):
                test = future_to_test[future]
                try:
                    result = future.result()
                    results.append(result)
                    status = 'PASS' if result.passed else 'FAIL'
                    self.logger.info(f"Completed: {test.name} - {status}")
                except Exception as e:
                    self.logger.error(f"Test {test.name} raised exception: {e}", exc_info=True)
                    results.append(TestResult(
                        name=test.name,
                        passed=False,
                        duration=0.0,
                        error=str(e)
                    ))
        
        return results
