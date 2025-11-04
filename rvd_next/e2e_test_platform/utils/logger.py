"""日志工具"""
import logging
from pathlib import Path
from typing import Optional

from core.config import Config


def setup_logging(config: Config, verbose: bool = False):
    """
    设置日志
    
    Args:
        config: 配置管理器
        verbose: 是否启用详细输出
    """
    # 获取日志级别
    if verbose:
        level = logging.DEBUG
    else:
        level_str = config.get('logging.level', 'INFO')
        level = getattr(logging, level_str, logging.INFO)
    
    # 获取日志格式
    log_format = config.get('logging.format', '%(asctime)s - %(name)s - %(levelname)s - %(message)s')
    
    # 创建处理器列表
    handlers = []
    
    # 控制台处理器
    console_handler = logging.StreamHandler()
    console_handler.setLevel(level)
    console_handler.setFormatter(logging.Formatter(log_format))
    handlers.append(console_handler)
    
    # 文件处理器
    if log_file := config.get('logging.file'):
        log_path = config.resolve_path(log_file)
        log_path.parent.mkdir(parents=True, exist_ok=True)
        
        file_handler = logging.FileHandler(log_path, encoding='utf-8')
        file_handler.setLevel(level)
        file_handler.setFormatter(logging.Formatter(log_format))
        handlers.append(file_handler)
    
    # 配置根日志记录器
    logging.basicConfig(
        level=level,
        format=log_format,
        handlers=handlers,
        force=True  # 强制重新配置
    )
    
    # 设置第三方库的日志级别
    logging.getLogger('urllib3').setLevel(logging.WARNING)
    logging.getLogger('requests').setLevel(logging.WARNING)
