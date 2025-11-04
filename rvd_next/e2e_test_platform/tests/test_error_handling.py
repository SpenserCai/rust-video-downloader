"""错误处理测试"""
import yaml
from typing import List
from pathlib import Path

from core.base_test import BaseTestCase, TestResult
from validators.output import OutputValidator


class TestInvalidUrl(BaseTestCase):
    """测试无效URL的错误处理
    
    验证RVD能够正确处理无效的URL，包括：
    - 识别无效URL
    - 输出清晰的错误信息
    - 程序不会崩溃
    """
    
    def __init__(self, config):
        super().__init__(config)
        self.tags = ['error', 'validation']
        self.timeout = 60  # 1分钟
        self.invalid_url = "https://invalid-url-that-does-not-exist.com/video/12345"
    
    def get_command(self) -> List[str]:
        """获取执行命令"""
        cmd = self._build_base_command()
        cmd.extend([
            self.invalid_url,
            '--output', str(self.workdir),
        ])
        return cmd
    
    def validate(self, result: TestResult) -> bool:
        """验证结果"""
        # 对于错误处理测试，我们期望命令失败但有清晰的错误信息
        output_lower = (result.output + result.error).lower()
        
        # 验证包含错误相关信息
        has_error_info = any(keyword in output_lower for keyword in [
            'error', 'invalid', 'failed', 'not found', '错误', '无效', '失败'
        ])
        
        if not has_error_info:
            result.error = "Expected error message not found in output"
            return False
        
        # 验证错误信息是清晰的（不是崩溃或panic）
        has_crash = any(keyword in output_lower for keyword in [
            'panic', 'segmentation fault', 'core dumped', 'fatal'
        ])
        
        if has_crash:
            result.error = "Program crashed instead of handling error gracefully"
            return False
        
        self.logger.info("Error handled correctly with clear error message")
        return True


class TestNetworkError(BaseTestCase):
    """测试网络错误的处理
    
    验证RVD能够正确处理网络错误，包括：
    - 识别网络连接问题
    - 输出清晰的错误信息
    - 程序不会崩溃
    """
    
    def __init__(self, config):
        super().__init__(config)
        self.tags = ['error', 'network']
        self.timeout = 60  # 1分钟
        # 使用一个不存在的域名
        self.invalid_domain_url = "https://this-domain-definitely-does-not-exist-12345.com/video"
    
    def get_command(self) -> List[str]:
        """获取执行命令"""
        cmd = self._build_base_command()
        cmd.extend([
            self.invalid_domain_url,
            '--output', str(self.workdir),
        ])
        return cmd
    
    def validate(self, result: TestResult) -> bool:
        """验证结果"""
        output_lower = (result.output + result.error).lower()
        
        # 验证包含网络错误相关信息
        has_network_error = any(keyword in output_lower for keyword in [
            'network', 'connection', 'timeout', 'dns', 'resolve',
            '网络', '连接', '超时', '域名'
        ])
        
        if not has_network_error:
            # 也可能是其他错误信息，只要有错误提示就行
            has_error = any(keyword in output_lower for keyword in [
                'error', 'failed', '错误', '失败'
            ])
            if not has_error:
                result.error = "Expected error message not found in output"
                return False
        
        # 验证没有崩溃
        has_crash = any(keyword in output_lower for keyword in [
            'panic', 'segmentation fault', 'core dumped', 'fatal'
        ])
        
        if has_crash:
            result.error = "Program crashed instead of handling error gracefully"
            return False
        
        self.logger.info("Network error handled correctly")
        return True


class TestAuthRequired(BaseTestCase):
    """测试需要认证的内容
    
    验证RVD能够正确处理需要认证的内容，包括：
    - 识别需要认证的内容
    - 在没有认证时给出清晰提示
    - 在有认证时能够正常下载
    """
    
    def __init__(self, config):
        super().__init__(config)
        self.tags = ['error', 'auth']
        self.timeout = 600  # 10分钟
        
        # 从配置文件加载需要认证的URL
        urls_file = config.resolve_path(config.get('test_data.urls_file', './datas/urls.yaml'))
        if urls_file.exists():
            with open(urls_file, 'r', encoding='utf-8') as f:
                urls_data = yaml.safe_load(f)
                auth_data = urls_data.get('auth_required', {})
                self.video_url = auth_data.get('url', 'PLACEHOLDER_AUTH_REQUIRED_URL')
                # 如果配置了认证文件，则启用认证
                if auth_data.get('use_auth', False):
                    self.requires_auth = True
                    auth_file_path = auth_data.get('auth_file')
                    if auth_file_path:
                        self.auth_file = config.resolve_path(auth_file_path)
        else:
            self.video_url = "PLACEHOLDER_AUTH_REQUIRED_URL"
    
    def get_command(self) -> List[str]:
        """获取执行命令"""
        cmd = self._build_base_command()
        cmd.extend([
            self.video_url,
            '--output', str(self.workdir),
        ])
        return cmd
    
    def validate(self, result: TestResult) -> bool:
        """验证结果"""
        output_lower = (result.output + result.error).lower()
        
        if self.requires_auth:
            # 如果启用了认证，应该能够成功下载
            has_success = any(keyword in output_lower for keyword in [
                'completed', 'success', '完成', '成功'
            ])
            
            if has_success:
                # 验证文件是否下载成功
                video_files = list(self.workdir.glob('**/*.mp4')) + \
                             list(self.workdir.glob('**/*.mkv')) + \
                             list(self.workdir.glob('**/*.flv'))
                if video_files:
                    self.logger.info("Successfully downloaded with authentication")
                    return True
                else:
                    result.error = "No video files found despite success message"
                    return False
            else:
                result.error = "Failed to download even with authentication"
                return False
        else:
            # 如果没有认证，应该有清晰的错误提示
            has_auth_error = any(keyword in output_lower for keyword in [
                'auth', 'login', 'credential', 'permission', 'forbidden',
                '认证', '登录', '权限', '禁止'
            ])
            
            if has_auth_error:
                self.logger.info("Correctly identified authentication requirement")
                return True
            else:
                # 也可能是其他错误，只要有错误提示就行
                has_error = any(keyword in output_lower for keyword in [
                    'error', 'failed', '错误', '失败'
                ])
                if has_error:
                    self.logger.warning("Got error but not specifically about authentication")
                    return True
                else:
                    result.error = "Expected authentication error not found"
                    return False
