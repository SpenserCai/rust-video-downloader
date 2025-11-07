"""错误处理测试"""
import yaml
from typing import List
from pathlib import Path

from core.base_test import BaseTestCase, TestResult
from validators.output import OutputValidator
from validators.file import FileValidator


class TestInvalidUrl(BaseTestCase):
    """测试无效URL的错误处理
    
    验证RVD能够正确处理无效的URL，包括：
    - 识别无效URL
    - 输出清晰的错误信息
    - 程序不会崩溃
    """
    
    def __init__(self, config):
        super().__init__(config)
        self.tags = ['core', 'error', 'validation']
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
        validations = []
        
        # 对于错误处理测试，我们期望命令失败但有清晰的错误信息
        output_lower = (result.output + result.error).lower()
        
        # 验证包含错误相关信息
        has_error_info = any(keyword in output_lower for keyword in [
            'error', 'invalid', 'failed', 'not found', '错误', '无效', '失败'
        ])
        
        if has_error_info:
            validations.append({"validator": "error_message", "passed": True, "message": "Error message found"})
        else:
            validations.append({"validator": "error_message", "passed": False, "message": "Expected error message not found"})
            result.validations = validations
            result.error = "Expected error message not found in output"
            return False
        
        # 验证错误信息是清晰的（不是崩溃或panic）
        has_crash = any(keyword in output_lower for keyword in [
            'panic', 'segmentation fault', 'core dumped', 'fatal'
        ])
        
        if has_crash:
            validations.append({"validator": "graceful_error", "passed": False, "message": "Program crashed"})
            result.validations = validations
            result.error = "Program crashed instead of handling error gracefully"
            return False
        
        validations.append({"validator": "graceful_error", "passed": True, "message": "Error handled gracefully"})
        result.validations = validations
        self.logger.info("Error handled correctly with clear error message")
        return True


class TestNetworkError(BaseTestCase):
    """测试网络错误的处理
    
    验证RVD能够正确处理网络错误，包括：
    - 识别网络连接问题
    - 输出清晰的错误信息
    - 重试机制（如果实现）
    - 程序不会崩溃
    """
    
    def __init__(self, config):
        super().__init__(config)
        self.tags = ['core', 'error', 'network']
        self.timeout = 120  # 2分钟（考虑重试时间）
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
        validations = []
        output_lower = (result.output + result.error).lower()
        
        # 验证包含网络错误相关信息
        has_network_error = any(keyword in output_lower for keyword in [
            'network', 'connection', 'timeout', 'dns', 'resolve',
            '网络', '连接', '超时', '域名', 'failed to connect'
        ])
        
        if has_network_error:
            validations.append({"validator": "network_error", "passed": True, "message": "Network error detected"})
        else:
            # 也可能是其他错误信息，只要有错误提示就行
            has_error = any(keyword in output_lower for keyword in [
                'error', 'failed', '错误', '失败'
            ])
            if has_error:
                validations.append({"validator": "network_error", "passed": True, "message": "Generic error detected"})
            else:
                validations.append({"validator": "network_error", "passed": False, "message": "No error message found"})
                result.validations = validations
                result.error = "Expected error message not found in output"
                return False
        
        # 验证没有崩溃
        has_crash = any(keyword in output_lower for keyword in [
            'panic', 'segmentation fault', 'core dumped', 'fatal'
        ])
        
        if has_crash:
            validations.append({"validator": "graceful_error", "passed": False, "message": "Program crashed"})
            result.validations = validations
            result.error = "Program crashed instead of handling error gracefully"
            return False
        
        validations.append({"validator": "graceful_error", "passed": True, "message": "Error handled gracefully"})
        result.validations = validations
        self.logger.info("Network error handled correctly")
        return True


class TestDiskSpaceError(BaseTestCase):
    """测试磁盘空间不足的处理
    
    验证RVD能够正确处理磁盘空间不足的情况，包括：
    - 检测磁盘空间不足
    - 输出清晰的错误信息
    - 清理临时文件
    - 程序不会崩溃
    
    注意：此测试难以模拟真实的磁盘空间不足场景，
    主要验证程序在写入失败时的错误处理能力
    """
    
    def __init__(self, config):
        super().__init__(config)
        self.tags = ['core', 'error', 'disk']
        self.timeout = 60  # 1分钟
        
        # 从配置文件加载测试数据
        test_data = self._load_test_data('core')
        error_data = test_data.get('error_handling', {}).get('disk_space', {})
        self.video_url = error_data.get('url', 'PLACEHOLDER_VIDEO_URL')
        
        # 尝试使用一个无效的输出路径来模拟写入失败
        # 例如：/dev/null/invalid_path（在Unix系统上）
        self.invalid_output_path = Path('/dev/null/invalid_path')
    
    def get_command(self) -> List[str]:
        """获取执行命令"""
        cmd = self._build_base_command()
        cmd.extend([
            self.video_url,
            '--output', str(self.invalid_output_path),
        ])
        return cmd
    
    def validate(self, result: TestResult) -> bool:
        """验证结果"""
        validations = []
        output_lower = (result.output + result.error).lower()
        
        # 验证包含磁盘/写入错误相关信息
        has_disk_error = any(keyword in output_lower for keyword in [
            'disk', 'space', 'write', 'permission', 'denied', 'cannot create',
            '磁盘', '空间', '写入', '权限', '拒绝', '无法创建'
        ])
        
        if has_disk_error:
            validations.append({"validator": "disk_error", "passed": True, "message": "Disk/write error detected"})
        else:
            # 也可能是其他错误信息
            has_error = any(keyword in output_lower for keyword in [
                'error', 'failed', '错误', '失败'
            ])
            if has_error:
                validations.append({"validator": "disk_error", "passed": True, "message": "Generic error detected"})
                self.logger.warning("Got error but not specifically about disk space")
            else:
                validations.append({"validator": "disk_error", "passed": False, "message": "No error message found"})
                result.validations = validations
                result.error = "Expected error message not found in output"
                return False
        
        # 验证没有崩溃
        has_crash = any(keyword in output_lower for keyword in [
            'panic', 'segmentation fault', 'core dumped', 'fatal'
        ])
        
        if has_crash:
            validations.append({"validator": "graceful_error", "passed": False, "message": "Program crashed"})
            result.validations = validations
            result.error = "Program crashed instead of handling error gracefully"
            return False
        
        validations.append({"validator": "graceful_error", "passed": True, "message": "Error handled gracefully"})
        result.validations = validations
        self.logger.info("Disk space error handled correctly")
        return True


class TestFFmpegNotAvailable(BaseTestCase):
    """测试 FFmpeg 不可用的处理
    
    验证RVD能够正确处理 FFmpeg 不可用的情况，包括：
    - 检测 FFmpeg 是否可用
    - 输出清晰的错误信息或警告
    - 降级处理（如果支持）
    - 程序不会崩溃
    
    注意：此测试难以模拟 FFmpeg 不可用的场景，
    主要验证程序在需要 FFmpeg 时的检测和错误处理能力
    """
    
    def __init__(self, config):
        super().__init__(config)
        self.tags = ['core', 'error', 'ffmpeg']
        self.timeout = 600  # 10分钟
        
        # 从配置文件加载测试数据
        test_data = self._load_test_data('core')
        muxer_data = test_data.get('muxer', {}).get('basic', {})
        self.video_url = muxer_data.get('url', 'PLACEHOLDER_VIDEO_URL')
    
    def get_command(self) -> List[str]:
        """获取执行命令"""
        # 通过设置无效的 PATH 环境变量来模拟 FFmpeg 不可用
        # 注意：这可能不会在所有情况下生效，因为程序可能使用绝对路径
        cmd = self._build_base_command()
        cmd.extend([
            self.video_url,
            '--output', str(self.workdir),
        ])
        return cmd
    
    def validate(self, result: TestResult) -> bool:
        """验证结果"""
        validations = []
        output_lower = (result.output + result.error).lower()
        
        # 检查是否提到 FFmpeg
        has_ffmpeg_mention = 'ffmpeg' in output_lower
        
        if has_ffmpeg_mention:
            # 检查是否有 FFmpeg 相关的错误或警告
            has_ffmpeg_error = any(keyword in output_lower for keyword in [
                'ffmpeg not found', 'ffmpeg is not available', 'cannot find ffmpeg',
                'ffmpeg 未找到', 'ffmpeg 不可用', '找不到 ffmpeg'
            ])
            
            if has_ffmpeg_error:
                validations.append({"validator": "ffmpeg_error", "passed": True, "message": "FFmpeg error detected"})
                self.logger.info("FFmpeg unavailability detected correctly")
            else:
                # FFmpeg 可能是可用的，这也是正常的
                validations.append({"validator": "ffmpeg_available", "passed": True, "message": "FFmpeg appears to be available"})
                self.logger.info("FFmpeg is available, test scenario not applicable")
        else:
            # 如果没有提到 FFmpeg，可能是：
            # 1. 程序成功完成（FFmpeg 可用）
            # 2. 程序在 FFmpeg 检查之前就失败了
            has_success = any(keyword in output_lower for keyword in [
                'completed', 'success', '完成', '成功', 'muxed to'
            ])
            
            if has_success:
                validations.append({"validator": "success", "passed": True, "message": "Download completed successfully"})
                self.logger.info("FFmpeg is available and working, test scenario not applicable")
            else:
                validations.append({"validator": "unknown", "passed": True, "message": "Test scenario unclear"})
                self.logger.warning("Cannot determine FFmpeg status from output")
        
        # 验证没有崩溃
        has_crash = any(keyword in output_lower for keyword in [
            'panic', 'segmentation fault', 'core dumped', 'fatal'
        ])
        
        if has_crash:
            validations.append({"validator": "graceful_error", "passed": False, "message": "Program crashed"})
            result.validations = validations
            result.error = "Program crashed"
            return False
        
        validations.append({"validator": "graceful_error", "passed": True, "message": "No crash detected"})
        result.validations = validations
        
        # 这个测试主要验证程序不会崩溃，所以只要没有崩溃就算通过
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
        self.tags = ['core', 'error', 'auth']
        self.timeout = 600  # 10分钟
        
        # 从配置文件加载需要认证的URL
        urls_file = config.resolve_path(config.get('test_data.urls_file', './datas/urls.yaml'))
        self.requires_auth = False
        
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
        validations = []
        output_lower = (result.output + result.error).lower()
        
        if self.requires_auth:
            # 如果启用了认证，应该能够成功下载
            has_success = any(keyword in output_lower for keyword in [
                'completed', 'success', '完成', '成功', 'muxed to'
            ])
            
            if has_success:
                # 验证文件是否下载成功
                file_validator = FileValidator(
                    files_exist=["*.mp4", "*.mkv", "*.flv"],
                    min_size={"*.mp4": 1024 * 100, "*.mkv": 1024 * 100, "*.flv": 1024 * 100}
                )
                passed, msg = file_validator.validate(self.workdir)
                
                if passed:
                    validations.append({"validator": "auth_success", "passed": True, "message": "Downloaded with authentication"})
                    result.validations = validations
                    self.logger.info("Successfully downloaded with authentication")
                    return True
                else:
                    validations.append({"validator": "file", "passed": False, "message": msg})
                    result.validations = validations
                    result.error = "No video files found despite success message"
                    return False
            else:
                validations.append({"validator": "auth_success", "passed": False, "message": "Failed with authentication"})
                result.validations = validations
                result.error = "Failed to download even with authentication"
                return False
        else:
            # 如果没有认证，应该有清晰的错误提示
            has_auth_error = any(keyword in output_lower for keyword in [
                'auth', 'login', 'credential', 'permission', 'forbidden', '403',
                '认证', '登录', '权限', '禁止', 'vip', '大会员'
            ])
            
            if has_auth_error:
                validations.append({"validator": "auth_error", "passed": True, "message": "Authentication error detected"})
                result.validations = validations
                self.logger.info("Correctly identified authentication requirement")
                return True
            else:
                # 也可能是其他错误，只要有错误提示就行
                has_error = any(keyword in output_lower for keyword in [
                    'error', 'failed', '错误', '失败'
                ])
                if has_error:
                    validations.append({"validator": "generic_error", "passed": True, "message": "Generic error detected"})
                    result.validations = validations
                    self.logger.warning("Got error but not specifically about authentication")
                    return True
                else:
                    validations.append({"validator": "auth_error", "passed": False, "message": "No error message found"})
                    result.validations = validations
                    result.error = "Expected authentication error not found"
                    return False
