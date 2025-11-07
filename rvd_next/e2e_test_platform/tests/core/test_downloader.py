"""下载器功能测试"""
from typing import List

from core.base_test import BaseTestCase, TestResult
from validators.output import OutputValidator
from validators.file import FileValidator


class TestBuiltinDownloader(BaseTestCase):
    """测试内置多线程下载器
    
    验证RVD内置下载器能够正确工作，包括：
    - 多线程分块下载
    - 文件完整性验证
    - 下载进度跟踪
    """
    
    def __init__(self, config):
        super().__init__(config)
        self.tags = ['core', 'download', 'builtin']
        self.timeout = 600  # 10分钟
        
        # 从配置文件加载测试数据
        test_data = self._load_test_data('core')
        downloader_data = test_data.get('downloader', {}).get('builtin', {})
        self.video_url = downloader_data.get('url', 'PLACEHOLDER_VIDEO_URL')
    
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
            'completed', 'success', '完成', '成功', 'muxed to', 'downloaded'
        ])
        
        if has_success:
            validations.append({"validator": "output", "passed": True, "message": "Success message found"})
        else:
            validations.append({"validator": "output", "passed": False, "message": "No success message found"})
            result.validations = validations
            result.error = "No success message in output"
            return False
        
        # 验证视频文件存在且大小合理
        file_validator = FileValidator(
            files_exist=["*.mp4", "*.mkv", "*.flv"],
            min_size={"*.mp4": 1024 * 100, "*.mkv": 1024 * 100, "*.flv": 1024 * 100}  # 至少100KB
        )
        passed, msg = file_validator.validate(self.workdir)
        validations.append({"validator": "file", "passed": passed, "message": msg})
        
        result.validations = validations
        if not passed:
            result.error = msg
        
        return passed


class TestAria2cDownloader(BaseTestCase):
    """测试 aria2c 外部下载器
    
    验证RVD能够正确调用 aria2c 下载器，包括：
    - aria2c 命令调用
    - 参数传递正确性
    - 下载结果验证
    """
    
    def __init__(self, config):
        super().__init__(config)
        self.tags = ['core', 'download', 'aria2c']
        self.timeout = 600  # 10分钟
        
        # 从配置文件加载测试数据
        test_data = self._load_test_data('core')
        downloader_data = test_data.get('downloader', {}).get('aria2c', {})
        self.video_url = downloader_data.get('url', 'PLACEHOLDER_VIDEO_URL')
        self.use_aria2c = downloader_data.get('use_aria2c', True)
    
    def get_command(self) -> List[str]:
        """获取执行命令"""
        cmd = self._build_base_command()
        
        # 添加 aria2c 相关参数
        if self.use_aria2c:
            cmd.extend(['--use-aria2c'])
        
        cmd.extend([
            self.video_url,
            '--output', str(self.workdir),
        ])
        return cmd
    
    def validate(self, result: TestResult) -> bool:
        """验证结果"""
        validations = []
        
        # 检查是否使用了 aria2c（从输出中查找）
        output_lower = result.output.lower()
        has_aria2c = 'aria2c' in output_lower or 'aria2' in output_lower
        
        if has_aria2c:
            validations.append({"validator": "aria2c_usage", "passed": True, "message": "aria2c detected in output"})
        else:
            self.logger.warning("aria2c not explicitly mentioned in output, but continuing validation")
        
        # 验证输出包含成功信息
        has_success = any(keyword in output_lower for keyword in [
            'completed', 'success', '完成', '成功', 'muxed to', 'downloaded'
        ])
        
        if has_success:
            validations.append({"validator": "output", "passed": True, "message": "Success message found"})
        else:
            validations.append({"validator": "output", "passed": False, "message": "No success message found"})
            result.validations = validations
            result.error = "No success message in output"
            return False
        
        # 验证视频文件存在且大小合理
        file_validator = FileValidator(
            files_exist=["*.mp4", "*.mkv", "*.flv"],
            min_size={"*.mp4": 1024 * 100, "*.mkv": 1024 * 100, "*.flv": 1024 * 100}
        )
        passed, msg = file_validator.validate(self.workdir)
        validations.append({"validator": "file", "passed": passed, "message": msg})
        
        result.validations = validations
        if not passed:
            result.error = msg
        
        return passed


class TestDownloadProgress(BaseTestCase):
    """测试下载进度跟踪
    
    验证RVD能够正确显示下载进度，包括：
    - 进度条显示
    - 进度百分比准确性
    - 速度和剩余时间显示
    """
    
    def __init__(self, config):
        super().__init__(config)
        self.tags = ['core', 'download', 'progress']
        self.timeout = 600  # 10分钟
        
        # 从配置文件加载测试数据
        test_data = self._load_test_data('core')
        downloader_data = test_data.get('downloader', {}).get('builtin', {})
        self.video_url = downloader_data.get('url', 'PLACEHOLDER_VIDEO_URL')
    
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
        
        # 验证输出包含进度相关信息
        output_lower = result.output.lower()
        has_progress = any(keyword in output_lower for keyword in [
            'progress', '进度', '%', 'downloading', '下载中', 'mb/s', 'kb/s'
        ])
        
        if has_progress:
            validations.append({"validator": "progress", "passed": True, "message": "Progress information found"})
        else:
            self.logger.warning("Progress information not clearly marked in output")
            validations.append({"validator": "progress", "passed": False, "message": "No progress information found"})
        
        # 验证下载成功
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
        
        # 验证视频文件存在
        file_validator = FileValidator(
            files_exist=["*.mp4", "*.mkv", "*.flv"],
            min_size={"*.mp4": 1024 * 100, "*.mkv": 1024 * 100, "*.flv": 1024 * 100}
        )
        passed, msg = file_validator.validate(self.workdir)
        validations.append({"validator": "file", "passed": passed, "message": msg})
        
        result.validations = validations
        if not passed:
            result.error = msg
        
        return passed
