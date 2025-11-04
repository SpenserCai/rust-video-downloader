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
        
        # 认证相关配置
        self.requires_auth = False  # 子类可以设置为True表示需要认证
        self.auth_file: Optional[Path] = None  # 子类可以指定特定的认证文件
        
        self._setup_workdir()
    
    def _setup_workdir(self):
        """设置工作目录"""
        base_workdir = self.config.default_workdir
        self.workdir = base_workdir / self.name
        self.workdir.mkdir(parents=True, exist_ok=True)
        self.logger.debug(f"Work directory: {self.workdir}")
    
    def _get_auth_file(self) -> Optional[Path]:
        """
        获取认证文件路径
        
        优先级：测试用例指定 > 全局配置 > None
        
        Returns:
            认证文件路径，如果不需要认证则返回None
        """
        if not self.requires_auth:
            return None
        
        # 优先使用测试用例指定的认证文件
        if self.auth_file:
            return self.auth_file
        
        # 其次使用全局配置的认证文件
        return self.config.auth_file_path
    
    @abstractmethod
    def get_command(self) -> List[str]:
        """
        获取执行命令（子类必须实现）
        
        子类实现时应该调用 _build_base_command() 来构建包含认证的基础命令
        
        Returns:
            命令列表
        """
        pass
    
    def _build_base_command(self) -> List[str]:
        """
        构建基础命令（包含可执行程序和认证参数）
        
        Returns:
            基础命令列表
        """
        cmd = [str(self.config.executable)]
        
        # 如果需要认证，添加--config-file参数
        auth_file = self._get_auth_file()
        if auth_file:
            self.logger.debug(f"Using auth file: {auth_file}")
            cmd.extend(['--config-file', str(auth_file)])
        
        return cmd
    
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
