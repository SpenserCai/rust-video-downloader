"""控制台报告器"""
from typing import List

try:
    from colorama import Fore, Style, init
    init(autoreset=True)
    COLORS_AVAILABLE = True
except ImportError:
    COLORS_AVAILABLE = False
    # 定义空的颜色类
    class Fore:
        GREEN = ''
        RED = ''
        YELLOW = ''
        RESET = ''
    
    class Style:
        RESET_ALL = ''

from .base import Reporter
from core.result import TestResult


class ConsoleReporter(Reporter):
    """控制台报告器"""
    
    def generate(self, results: List[TestResult]) -> str:
        """
        生成控制台报告
        
        Args:
            results: 测试结果列表
            
        Returns:
            报告内容
        """
        total = len(results)
        passed = sum(1 for r in results if r.passed)
        failed = total - passed
        
        output = []
        output.append("\n" + "=" * 70)
        output.append("TEST RESULTS")
        output.append("=" * 70)
        
        for result in results:
            if result.passed:
                status = f"{Fore.GREEN}✓ PASS{Style.RESET_ALL}"
            else:
                status = f"{Fore.RED}✗ FAIL{Style.RESET_ALL}"
            
            output.append(f"{status} {result.name} ({result.duration:.2f}s)")
            
            if not result.passed and result.error:
                # 显示错误信息的前几行
                error_lines = result.error.split('\n')[:5]
                for line in error_lines:
                    if line.strip():
                        output.append(f"  {Fore.RED}Error: {line}{Style.RESET_ALL}")
        
        output.append("=" * 70)
        passed_str = f"{Fore.GREEN}{passed}{Style.RESET_ALL}"
        failed_str = f"{Fore.RED}{failed}{Style.RESET_ALL}"
        output.append(f"Total: {total} | Passed: {passed_str} | Failed: {failed_str}")
        output.append("=" * 70)
        
        return "\n".join(output)
