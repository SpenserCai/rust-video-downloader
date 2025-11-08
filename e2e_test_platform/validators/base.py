"""验证器基类"""
from abc import ABC, abstractmethod
from typing import Any, Tuple


class Validator(ABC):
    """验证器基类"""
    
    @abstractmethod
    def validate(self, value: Any) -> Tuple[bool, str]:
        """
        验证值
        
        Args:
            value: 要验证的值
            
        Returns:
            (是否通过, 错误信息)
        """
        pass
