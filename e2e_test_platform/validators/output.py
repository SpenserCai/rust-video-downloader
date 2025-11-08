"""输出验证器"""
import re
from typing import List, Optional, Tuple

from .base import Validator
from core.result import TestResult


class OutputValidator(Validator):
    """输出验证器"""
    
    def __init__(self,
                 contains: Optional[List[str]] = None,
                 not_contains: Optional[List[str]] = None,
                 regex: Optional[str] = None,
                 exit_code: Optional[int] = None):
        """
        初始化输出验证器
        
        Args:
            contains: 输出必须包含的文本列表
            not_contains: 输出不能包含的文本列表
            regex: 输出必须匹配的正则表达式
            exit_code: 期望的退出码
        """
        self.contains = contains or []
        self.not_contains = not_contains or []
        self.regex = regex
        self.expected_exit_code = exit_code
    
    def validate(self, result: TestResult) -> Tuple[bool, str]:
        """
        验证输出
        
        Args:
            result: 测试结果
            
        Returns:
            (是否通过, 错误信息)
        """
        output = result.output + result.error
        
        # 检查包含
        for text in self.contains:
            if text not in output:
                return False, f"Output does not contain: {text}"
        
        # 检查不包含
        for text in self.not_contains:
            if text in output:
                return False, f"Output should not contain: {text}"
        
        # 检查正则
        if self.regex and not re.search(self.regex, output, re.MULTILINE | re.DOTALL):
            return False, f"Output does not match regex: {self.regex}"
        
        return True, ""
