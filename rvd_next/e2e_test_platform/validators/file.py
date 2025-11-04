"""文件验证器"""
from pathlib import Path
from typing import List, Optional, Dict, Tuple

from .base import Validator


class FileValidator(Validator):
    """文件验证器"""
    
    def __init__(self,
                 files_exist: Optional[List[str]] = None,
                 files_not_exist: Optional[List[str]] = None,
                 min_size: Optional[Dict[str, int]] = None,
                 max_size: Optional[Dict[str, int]] = None):
        """
        初始化文件验证器
        
        Args:
            files_exist: 必须存在的文件模式列表（支持glob）
            files_not_exist: 不能存在的文件模式列表（支持glob）
            min_size: 文件最小大小字典 {模式: 字节数}
            max_size: 文件最大大小字典 {模式: 字节数}
        """
        self.files_exist = files_exist or []
        self.files_not_exist = files_not_exist or []
        self.min_size = min_size or {}
        self.max_size = max_size or {}
    
    def validate(self, workdir: Path) -> Tuple[bool, str]:
        """
        验证文件
        
        Args:
            workdir: 工作目录
            
        Returns:
            (是否通过, 错误信息)
        """
        # 检查文件存在
        for file_pattern in self.files_exist:
            files = list(workdir.glob(file_pattern))
            if not files:
                return False, f"File not found: {file_pattern}"
        
        # 检查文件不存在
        for file_pattern in self.files_not_exist:
            files = list(workdir.glob(file_pattern))
            if files:
                return False, f"File should not exist: {file_pattern} (found: {files[0].name})"
        
        # 检查文件大小
        for file_pattern, min_bytes in self.min_size.items():
            files = list(workdir.glob(file_pattern))
            for f in files:
                if f.stat().st_size < min_bytes:
                    return False, f"File {f.name} too small: {f.stat().st_size} < {min_bytes} bytes"
        
        for file_pattern, max_bytes in self.max_size.items():
            files = list(workdir.glob(file_pattern))
            for f in files:
                if f.stat().st_size > max_bytes:
                    return False, f"File {f.name} too large: {f.stat().st_size} > {max_bytes} bytes"
        
        return True, ""
