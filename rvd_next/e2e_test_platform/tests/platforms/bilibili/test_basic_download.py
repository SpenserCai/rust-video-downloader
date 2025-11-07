"""Bilibili 基础下载测试"""
from typing import List

from core.base_test import BaseTestCase, TestResult
from validators.output import OutputValidator
from validators.file import FileValidator


class TestBVVideoDownload(BaseTestCase):
    """测试 BV 号视频下载
    
    验证RVD能够正确下载 BV 号视频，包括：
    - BV 号解析
    - 视频下载
    - 元数据提取
    """
    
    def __init__(self, config):
        super().__init__(config)
        self.tags = ['bilibili', 'download', 'basic', 'bv']
        self.timeout = 600  # 10分钟
        
        # 从配置文件加载测试数据（使用嵌套路径，自动处理 quality 等参数）
        test_data = self._load_test_data('bilibili.basic_download.bv_video')
        self.video_url = test_data.get('url', 'PLACEHOLDER_VIDEO_URL')
    
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


class TestAVVideoDownload(BaseTestCase):
    """测试 AV 号视频下载
    
    验证RVD能够正确下载 AV 号视频，包括：
    - AV 号解析
    - 视频下载
    - AV/BV 转换
    """
    
    def __init__(self, config):
        super().__init__(config)
        self.tags = ['bilibili', 'download', 'basic', 'av']
        self.timeout = 600  # 10分钟
        
        # 从配置文件加载测试数据（使用嵌套路径，自动处理 quality 等参数）
        test_data = self._load_test_data('bilibili.basic_download.av_video')
        self.video_url = test_data.get('url', 'PLACEHOLDER_AV_VIDEO_URL')
    
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
        
        # 检查 URL 是否为占位符
        if self.video_url.startswith('PLACEHOLDER_'):
            self.logger.warning(f"URL is placeholder: {self.video_url}, skipping test")
            result.validations = validations
            result.error = "URL not configured"
            return False
        
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


class TestMultiPageVideoDownload(BaseTestCase):
    """测试多 P 视频下载
    
    验证RVD能够正确下载多 P 视频，包括：
    - 分 P 识别
    - 选择性下载
    - 批量处理
    """
    
    def __init__(self, config):
        super().__init__(config)
        self.tags = ['bilibili', 'download', 'basic', 'multi-page']
        self.timeout = 900  # 15分钟（多 P 可能需要更长时间）
        
        # 从配置文件加载测试数据（使用嵌套路径，自动处理 quality 等参数）
        test_data = self._load_test_data('bilibili.basic_download.multi_page')
        self.video_url = test_data.get('url', 'PLACEHOLDER_MULTI_PAGE_URL')
    
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
        
        # 检查 URL 是否为占位符
        if self.video_url.startswith('PLACEHOLDER_'):
            self.logger.warning(f"URL is placeholder: {self.video_url}, skipping test")
            result.validations = validations
            result.error = "URL not configured"
            return False
        
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
        
        # 验证视频文件存在（可能有多个）
        video_files = list(self.workdir.glob('**/*.mp4')) + list(self.workdir.glob('**/*.mkv')) + list(self.workdir.glob('**/*.flv'))
        
        if video_files:
            validations.append({"validator": "file", "passed": True, "message": f"Found {len(video_files)} video file(s)"})
        else:
            validations.append({"validator": "file", "passed": False, "message": "No video files found"})
            result.validations = validations
            result.error = "No video files found"
            return False
        
        result.validations = validations
        return True


class TestBangumiEPDownload(BaseTestCase):
    """测试番剧 EP 号下载
    
    验证RVD能够正确下载番剧 EP，包括：
    - EP 号解析
    - 番剧 API 调用
    - 元数据处理
    """
    
    def __init__(self, config):
        super().__init__(config)
        self.tags = ['bilibili', 'download', 'basic', 'bangumi', 'ep']
        self.timeout = 600  # 10分钟
        
        # 从配置文件加载测试数据（使用嵌套路径，自动处理 quality 等参数）
        test_data = self._load_test_data('bilibili.basic_download.bangumi_ep')
        self.video_url = test_data.get('url', 'PLACEHOLDER_BANGUMI_EP_URL')
    
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
        
        # 检查 URL 是否为占位符
        if self.video_url.startswith('PLACEHOLDER_'):
            self.logger.warning(f"URL is placeholder: {self.video_url}, skipping test")
            result.validations = validations
            result.error = "URL not configured"
            return False
        
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


class TestBangumiSSDownload(BaseTestCase):
    """测试番剧 SS 号下载
    
    验证RVD能够正确下载番剧 SS（整季），包括：
    - SS 号解析
    - 整季识别
    - 批量处理
    """
    
    def __init__(self, config):
        super().__init__(config)
        self.tags = ['bilibili', 'download', 'basic', 'bangumi', 'ss']
        self.timeout = 900  # 15分钟（整季可能需要更长时间）
        
        # 从配置文件加载测试数据（使用嵌套路径，自动处理 quality 等参数）
        test_data = self._load_test_data('bilibili.basic_download.bangumi_ss')
        self.video_url = test_data.get('url', 'PLACEHOLDER_BANGUMI_SS_URL')
    
    def get_command(self) -> List[str]:
        """获取执行命令"""
        cmd = self._build_base_command()
        cmd.extend([
            self.video_url,
            '--batch-limit', '3',  # 限制下载数量以加快测试
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
        
        # 验证视频文件存在（可能有多个）
        video_files = list(self.workdir.glob('**/*.mp4')) + list(self.workdir.glob('**/*.mkv')) + list(self.workdir.glob('**/*.flv'))
        
        if video_files:
            validations.append({"validator": "file", "passed": True, "message": f"Found {len(video_files)} video file(s)"})
        else:
            validations.append({"validator": "file", "passed": False, "message": "No video files found"})
            result.validations = validations
            result.error = "No video files found"
            return False
        
        result.validations = validations
        return True
