"""配置管理模块"""
import yaml
import os
from pathlib import Path
from typing import Any, Dict, Optional


class Config:
    """测试平台配置管理器"""
    
    def __init__(self, config_file: str = "config.yaml"):
        """
        初始化配置管理器
        
        Args:
            config_file: 配置文件路径（相对于平台根目录）
        """
        self.root_dir = Path(__file__).parent.parent.resolve()
        self.config_file = self.root_dir / config_file
        self._config: Dict[str, Any] = {}
        self.load()
    
    def load(self):
        """加载配置文件"""
        if self.config_file.exists():
            with open(self.config_file, 'r', encoding='utf-8') as f:
                self._config = yaml.safe_load(f) or {}
        else:
            raise FileNotFoundError(f"Config file not found: {self.config_file}")
        
        # 环境变量覆盖
        self._apply_env_overrides()
    
    def _apply_env_overrides(self):
        """应用环境变量覆盖"""
        # E2E_EXECUTABLE 覆盖 platform.executable
        if exe := os.getenv('E2E_EXECUTABLE'):
            self._config.setdefault('platform', {})['executable'] = exe
        
        # E2E_TIMEOUT 覆盖 platform.default_timeout
        if timeout := os.getenv('E2E_TIMEOUT'):
            self._config.setdefault('platform', {})['default_timeout'] = int(timeout)
        
        # E2E_WORKDIR 覆盖 platform.default_workdir
        if workdir := os.getenv('E2E_WORKDIR'):
            self._config.setdefault('platform', {})['default_workdir'] = workdir
    
    def get(self, key: str, default: Any = None) -> Any:
        """
        获取配置值，支持点号分隔的嵌套键
        
        Args:
            key: 配置键，支持点号分隔（如 'platform.executable'）
            default: 默认值
            
        Returns:
            配置值或默认值
        """
        keys = key.split('.')
        value = self._config
        for k in keys:
            if isinstance(value, dict):
                value = value.get(k)
            else:
                return default
            if value is None:
                return default
        return value
    
    def resolve_path(self, path: str) -> Path:
        """
        解析路径（相对于平台根目录）
        
        Args:
            path: 路径字符串
            
        Returns:
            解析后的绝对路径
        """
        p = Path(path)
        if p.is_absolute():
            return p
        return (self.root_dir / p).resolve()
    
    @property
    def executable(self) -> Path:
        """获取可执行程序路径"""
        return self.resolve_path(self.get('platform.executable', '../../target/release/rvd'))
    
    @property
    def config_file_path(self) -> Optional[Path]:
        """获取RVD配置文件路径"""
        if cfg := self.get('platform.config_file'):
            return self.resolve_path(cfg)
        return None
    
    @property
    def auth_file_path(self) -> Optional[Path]:
        """获取认证文件路径"""
        if auth := self.get('platform.auth_file'):
            return self.resolve_path(auth)
        return None
    
    @property
    def default_timeout(self) -> int:
        """获取默认超时时间（秒）"""
        return self.get('platform.default_timeout', 300)
    
    @property
    def default_workdir(self) -> Path:
        """获取默认工作目录"""
        return self.resolve_path(self.get('platform.default_workdir', './workdir'))
