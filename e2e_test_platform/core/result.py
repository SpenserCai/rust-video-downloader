"""测试结果数据结构"""
from dataclasses import dataclass, field
from pathlib import Path
from typing import List, Dict, Any


@dataclass
class TestResult:
    """测试结果"""
    
    # 测试名称
    name: str
    
    # 是否通过
    passed: bool
    
    # 执行时间（秒）
    duration: float
    
    # 标准输出
    output: str = ""
    
    # 错误输出
    error: str = ""
    
    # 产物列表
    artifacts: List[Path] = field(default_factory=list)
    
    # 验证结果列表
    validations: List[Dict[str, Any]] = field(default_factory=list)
