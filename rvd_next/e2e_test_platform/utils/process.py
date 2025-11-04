"""进程执行工具"""
import subprocess
from pathlib import Path
from typing import List, Optional, Dict
import logging


logger = logging.getLogger(__name__)


def run_command(
    cmd: List[str],
    cwd: Optional[Path] = None,
    timeout: Optional[int] = None,
    env: Optional[Dict[str, str]] = None,
    capture_output: bool = True
) -> subprocess.CompletedProcess:
    """
    执行命令
    
    Args:
        cmd: 命令列表
        cwd: 工作目录
        timeout: 超时时间（秒）
        env: 环境变量
        capture_output: 是否捕获输出
        
    Returns:
        完成的进程对象
        
    Raises:
        subprocess.TimeoutExpired: 超时
        subprocess.CalledProcessError: 命令执行失败
    """
    logger.debug(f"Running command: {' '.join(str(c) for c in cmd)}")
    if cwd:
        logger.debug(f"Working directory: {cwd}")
    
    result = subprocess.run(
        cmd,
        cwd=cwd,
        timeout=timeout,
        env=env,
        capture_output=capture_output,
        text=True
    )
    
    logger.debug(f"Command completed with exit code: {result.returncode}")
    
    return result
