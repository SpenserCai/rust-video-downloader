"""报告器基类"""
from abc import ABC, abstractmethod
from typing import List

from core.result import TestResult


class Reporter(ABC):
    """报告器基类"""
    
    @abstractmethod
    def generate(self, results: List[TestResult]) -> str:
        """
        生成报告
        
        Args:
            results: 测试结果列表
            
        Returns:
            报告内容
        """
        pass
