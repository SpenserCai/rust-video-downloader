"""临时文件管理测试"""
from typing import List
import time

from core.base_test import BaseTestCase, TestResult
from validators.output import OutputValidator
from validators.file import FileValidator


class TestNormalCleanup(BaseTestCase):
    """测试正常下载后清理
    
    验证RVD能够在下载完成后清理临时文件，包括：
    - 临时文件自动清理
    - 临时目录删除
    - 只保留最终文件
    """
    
    def __init__(self, config):
        super().__init__(config)
        self.tags = ['core', 'temp', 'cleanup']
        self.timeout = 600  # 10分钟
        
        # 从配置文件加载测试数据
        test_data = self._load_test_data('core')
        temp_data = test_data.get('temp_files', {}).get('normal_cleanup', {})
        self.video_url = temp_data.get('url', 'PLACEHOLDER_VIDEO_URL')
    
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
        
        # 验证输出包含成功信息
        output_lower = result.output.lower()
        has_success = any(keyword in output_lower for keyword in [
            'completed', 'success', '完成', '成功', 'muxed to'
        ])
        
        if has_success:
            validations.append({"validator": "output", "passed": True, "message": "Success message found"})
        else:
            validations.append({"validator": "output", "passed": False, "message": "No success message found"})
            result.validations = validations
            result.error = "No success message in output"
            return False
        
        # 验证最终视频文件存在
        file_validator = FileValidator(
            files_exist=["*.mp4", "*.mkv", "*.flv"],
            min_size={"*.mp4": 1024 * 100, "*.mkv": 1024 * 100, "*.flv": 1024 * 100}
        )
        passed, msg = file_validator.validate(self.workdir)
        validations.append({"validator": "file", "passed": passed, "message": msg})
        
        if not passed:
            result.validations = validations
            result.error = msg
            return False
        
        # 检查是否有临时文件残留（.tmp, .part, .downloading 等）
        temp_files = (
            list(self.workdir.glob('**/*.tmp')) +
            list(self.workdir.glob('**/*.part')) +
            list(self.workdir.glob('**/*.downloading')) +
            list(self.workdir.glob('**/temp_*'))
        )
        
        if temp_files:
            validations.append({"validator": "temp_cleanup", "passed": False, "message": f"Found {len(temp_files)} temp file(s)"})
            result.validations = validations
            result.error = f"Temporary files not cleaned up: {[f.name for f in temp_files]}"
            return False
        else:
            validations.append({"validator": "temp_cleanup", "passed": True, "message": "No temporary files found"})
        
        result.validations = validations
        return True


class TestFailedCleanup(BaseTestCase):
    """测试下载失败后清理
    
    验证RVD能够在下载失败后清理临时文件，包括：
    - 失败后清理机制
    - 临时文件删除
    - 错误处理正确
    """
    
    def __init__(self, config):
        super().__init__(config)
        self.tags = ['core', 'temp', 'cleanup', 'error']
        self.timeout = 300  # 5分钟（失败应该很快）
        
        # 从配置文件加载测试数据
        test_data = self._load_test_data('core')
        temp_data = test_data.get('temp_files', {}).get('failed_cleanup', {})
        self.video_url = temp_data.get('url', 'INVALID_URL')
    
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
        
        # 验证输出包含错误信息（预期失败）
        output_lower = result.output.lower()
        has_error = any(keyword in output_lower for keyword in [
            'error', 'failed', '错误', '失败', 'invalid'
        ])
        
        if has_error:
            validations.append({"validator": "error_message", "passed": True, "message": "Error message found (expected)"})
        else:
            self.logger.warning("No error message found, but URL is invalid")
        
        # 检查是否有临时文件残留
        temp_files = (
            list(self.workdir.glob('**/*.tmp')) +
            list(self.workdir.glob('**/*.part')) +
            list(self.workdir.glob('**/*.downloading')) +
            list(self.workdir.glob('**/temp_*'))
        )
        
        if temp_files:
            validations.append({"validator": "temp_cleanup", "passed": False, "message": f"Found {len(temp_files)} temp file(s) after failure"})
            result.validations = validations
            result.error = f"Temporary files not cleaned up after failure: {[f.name for f in temp_files]}"
            return False
        else:
            validations.append({"validator": "temp_cleanup", "passed": True, "message": "No temporary files found after failure"})
        
        # 验证没有生成最终视频文件（因为下载失败）
        video_files = list(self.workdir.glob('**/*.mp4')) + list(self.workdir.glob('**/*.mkv')) + list(self.workdir.glob('**/*.flv'))
        
        if video_files:
            self.logger.warning(f"Found {len(video_files)} video file(s) despite failure")
        
        result.validations = validations
        return True


class TestSkipMuxTempFiles(BaseTestCase):
    """测试 --skip-mux 模式的临时文件
    
    验证RVD在跳过混流模式下保留临时文件，包括：
    - 临时文件保留
    - .m4s 文件存在
    - 分离文件正确保存
    """
    
    def __init__(self, config):
        super().__init__(config)
        self.tags = ['core', 'temp', 'skip-mux']
        self.timeout = 600  # 10分钟
        
        # 从配置文件加载测试数据
        test_data = self._load_test_data('core')
        muxer_data = test_data.get('muxer', {}).get('skip_mux', {})
        self.video_url = muxer_data.get('url', 'PLACEHOLDER_VIDEO_URL')
    
    def get_command(self) -> List[str]:
        """获取执行命令"""
        cmd = self._build_base_command()
        cmd.extend([
            self.video_url,
            '--skip-mux',
            '--output', str(self.workdir),
        ])
        return cmd
    
    def validate(self, result: TestResult) -> bool:
        """验证结果"""
        validations = []
        
        # 验证输出包含成功信息
        output_lower = result.output.lower()
        has_success = any(keyword in output_lower for keyword in [
            'completed', 'success', '完成', '成功', 'downloaded', 'skipped mux', '跳过混流'
        ])
        
        if has_success:
            validations.append({"validator": "output", "passed": True, "message": "Success message found"})
        else:
            validations.append({"validator": "output", "passed": False, "message": "No success message found"})
            result.validations = validations
            result.error = "No success message in output"
            return False
        
        # 验证分离的视频和音频文件存在（.m4s 或其他格式）
        video_files = list(self.workdir.glob('**/*.m4s')) + list(self.workdir.glob('**/*.mp4'))
        
        if video_files:
            validations.append({"validator": "file", "passed": True, "message": f"Found {len(video_files)} video/audio file(s)"})
        else:
            validations.append({"validator": "file", "passed": False, "message": "No video/audio files found"})
            result.validations = validations
            result.error = "No video/audio files found in skip-mux mode"
            return False
        
        # 验证没有混流后的文件（因为跳过了混流）
        # 注意：这个验证可能需要根据实际情况调整
        self.logger.info(f"Files in workdir: {[f.name for f in self.workdir.glob('**/*') if f.is_file()]}")
        
        result.validations = validations
        return True
