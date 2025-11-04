"""内容验证器"""
import re
from pathlib import Path
from typing import List, Optional, Tuple

from .base import Validator


class ContentValidator(Validator):
    """内容验证器"""
    
    def __init__(self,
                 file_pattern: str,
                 contains: Optional[List[str]] = None,
                 regex: Optional[str] = None):
        """
        初始化内容验证器
        
        Args:
            file_pattern: 文件模式（支持glob）
            contains: 内容必须包含的文本列表
            regex: 内容必须匹配的正则表达式
        """
        self.file_pattern = file_pattern
        self.contains = contains or []
        self.regex = regex
    
    def validate(self, workdir: Path) -> Tuple[bool, str]:
        """
        验证文件内容
        
        Args:
            workdir: 工作目录
            
        Returns:
            (是否通过, 错误信息)
        """
        files = list(workdir.glob(self.file_pattern))
        if not files:
            return False, f"No files match pattern: {self.file_pattern}"
        
        for file in files:
            try:
                content = file.read_text(encoding='utf-8', errors='ignore')
                
                # 检查包含
                for text in self.contains:
                    if text not in content:
                        return False, f"File {file.name} does not contain: {text}"
                
                # 检查正则
                if self.regex and not re.search(self.regex, content, re.MULTILINE | re.DOTALL):
                    return False, f"File {file.name} does not match regex: {self.regex}"
                    
            except Exception as e:
                return False, f"Failed to read file {file.name}: {str(e)}"
        
        return True, ""
