"""文件工具"""
import shutil
from pathlib import Path
import logging


logger = logging.getLogger(__name__)


def cleanup_workdir(workdir: Path, force: bool = False):
    """
    清理工作目录
    
    Args:
        workdir: 工作目录路径
        force: 是否强制删除（即使有错误）
    """
    if not workdir.exists():
        return
    
    try:
        shutil.rmtree(workdir)
        logger.debug(f"Cleaned up work directory: {workdir}")
    except Exception as e:
        if force:
            logger.warning(f"Failed to clean up {workdir}: {e}")
        else:
            raise


def format_size(size_bytes: int) -> str:
    """
    格式化文件大小
    
    Args:
        size_bytes: 字节数
        
    Returns:
        格式化的大小字符串
    """
    for unit in ['B', 'KB', 'MB', 'GB', 'TB']:
        if size_bytes < 1024.0:
            return f"{size_bytes:.2f} {unit}"
        size_bytes /= 1024.0
    return f"{size_bytes:.2f} PB"


def copy_test_data(src: Path, dst: Path):
    """
    复制测试数据
    
    Args:
        src: 源路径
        dst: 目标路径
    """
    if src.is_file():
        dst.parent.mkdir(parents=True, exist_ok=True)
        shutil.copy2(src, dst)
    elif src.is_dir():
        shutil.copytree(src, dst, dirs_exist_ok=True)
    else:
        raise ValueError(f"Invalid source path: {src}")
    
    logger.debug(f"Copied test data from {src} to {dst}")
