"""测试用例基类"""
from abc import ABC, abstractmethod
from pathlib import Path
from typing import List, Optional, Dict, Any
import subprocess
import time
import logging

from .result import TestResult
from .config import Config


class BaseTestCase(ABC):
    """测试用例基类"""
    
    def __init__(self, config: Config):
        """
        初始化测试用例
        
        Args:
            config: 配置管理器
        """
        self.config = config
        self.name = self.__class__.__name__
        self.description = self.__doc__ or ""
        self.tags: List[str] = []
        self.timeout = config.default_timeout
        self.workdir: Optional[Path] = None
        self.logger = logging.getLogger(self.name)
        self._setup_workdir()
    
    def _setup_workdir(self):
        """设置工作目录"""
        base_workdir = self.config.default_workdir
        self.workdir = base_workdir / self.name
        self.workdir.mkdir(parents=True, exist_ok=True)
        self.logger.debug(f"Work directory: {self.workdir}")
    
    @abstractmethod
    def get_command(self) -> List[str]:
        """
        获取执行命令（子类必须实现）
        
        Returns:
            命令列表
        """
        pass
    
    @abstractmethod
    def validate(self, result: TestResult) -> bool:
        """
        验证测试结果（子类必须实现）
        
        Args:
            result: 测试结果
            
        Returns:
            是否通过验证
        """
        pass
    
    def setup(self):
        """测试前置操作（子类可选实现）"""
        pass
    
    def teardown(self):
        """测试后置操作（子类可选实现）"""
        pass
    
    def run(self) -> TestResult:
        """
        执行测试
        
        Returns:
            测试结果
        """
        start_time = time.time()
        result = TestResult(name=self.name, passed=False, duration=0.0)
        
        try:
            self.logger.info(f"Starting test: {self.name}")
            
            # 前置操作
            self.setup()
            
            # 执行命令
            cmd = self.get_command()
            self.logger.debug(f"Command: {' '.join(str(c) for c in cmd)}")
            self.logger.debug(f"Working directory: {self.workdir}")
            
            process = subprocess.run(
                cmd,
                cwd=self.workdir,
                capture_output=True,
                text=True,
                timeout=self.timeout,
                env=self._get_env()
            )
            
            result.output = process.stdout
            result.error = process.stderr
            
            self.logger.debug(f"Exit code: {process.returncode}")
            if process.returncode != 0:
                self.logger.warning(f"Non-zero exit code: {process.returncode}")
            
            # 收集产物
            result.artifacts = self._collect_artifacts()
            self.logger.debug(f"Collected {len(result.artifacts)} artifacts")
            
            # 验证结果
            result.passed = self.validate(result)
            
            if result.passed:
                self.logger.info(f"Test passed: {self.name}")
            else:
                self.logger.error(f"Test failed: {self.name}")
            
        except subprocess.TimeoutExpired:
            result.error = f"Test timed out after {self.timeout} seconds"
            result.passed = False
            self.logger.error(result.error)
        except Exception as e:
            result.error = f"Test execution failed: {str(e)}"
            result.passed = False
            self.logger.error(result.error, exc_info=True)
        finally:
            # 后置操作
            try:
                self.teardown()
            except Exception as e:
                error_msg = f"\nTeardown failed: {str(e)}"
                result.error += error_msg
                self.logger.error(error_msg, exc_info=True)
            
            result.duration = time.time() - start_time
            self.logger.info(f"Test completed in {result.duration:.2f}s")
        
        return result
    
    def _get_env(self) -> Dict[str, str]:
        """
        获取环境变量
        
        Returns:
            环境变量字典
        """
        import os
        env = os.environ.copy()
        env.update(self.config.get('platform.env', {}))
        return env
    
    def _collect_artifacts(self) -> List[Path]:
        """
        收集测试产物
        
        Returns:
            产物路径列表
        """
        if not self.workdir:
            return []
        
        artifacts = []
        for item in self.workdir.rglob('*'):
            if item.is_file():
                artifacts.append(item)
        
        return artifacts
