"""Bilibili 批量下载测试"""
from typing import List

from core.base_test import BaseTestCase, TestResult
from validators.output import OutputValidator
from validators.file import FileValidator


class TestFavoritesBatchDownload(BaseTestCase):
    """测试收藏夹批量下载
    
    验证RVD能够正确批量下载收藏夹视频，包括：
    - 收藏夹识别
    - 所有视频下载
    - 批量处理
    """
    
    def __init__(self, config):
        super().__init__(config)
        self.tags = ['bilibili', 'batch', 'download', 'favorites']
        self.timeout = 1200  # 20分钟
        
        # 从配置文件加载测试数据
        test_data = self._load_test_data('bilibili')
        batch_data = test_data.get('batch_download', {}).get('favorites', {})
        self.video_url = batch_data.get('url', 'PLACEHOLDER_FAVORITES_URL')
        self.batch_limit = batch_data.get('batch_limit', 5)
    
    def get_command(self) -> List[str]:
        """获取执行命令"""
        cmd = self._build_base_command()
        cmd.extend([
            self.video_url,
            '--batch-limit', str(self.batch_limit),
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
            'completed', 'success', '完成', '成功', 'downloaded'
        ])
        
        if has_success:
            validations.append({"validator": "output", "passed": True, "message": "Success message found"})
        else:
            validations.append({"validator": "output", "passed": False, "message": "No success message found"})
            result.validations = validations
            result.error = "No success message in output"
            return False
        
        # 验证视频文件存在（应该有多个）
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


class TestUserSpaceBatchDownload(BaseTestCase):
    """测试 UP 主空间批量下载
    
    验证RVD能够正确批量下载 UP 主空间视频，包括：
    - WBI 签名机制
    - 分页逻辑
    - 批量处理
    """
    
    def __init__(self, config):
        super().__init__(config)
        self.tags = ['bilibili', 'batch', 'download', 'user-space', 'wbi']
        self.timeout = 1200  # 20分钟
        
        # 从配置文件加载测试数据
        test_data = self._load_test_data('bilibili')
        batch_data = test_data.get('batch_download', {}).get('user_space', {})
        self.video_url = batch_data.get('url', 'PLACEHOLDER_USER_SPACE_URL')
        self.batch_limit = batch_data.get('batch_limit', 5)
    
    def get_command(self) -> List[str]:
        """获取执行命令"""
        cmd = self._build_base_command()
        cmd.extend([
            self.video_url,
            '--batch-limit', str(self.batch_limit),
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
            'completed', 'success', '完成', '成功', 'downloaded'
        ])
        
        if has_success:
            validations.append({"validator": "output", "passed": True, "message": "Success message found"})
        else:
            validations.append({"validator": "output", "passed": False, "message": "No success message found"})
            result.validations = validations
            result.error = "No success message in output"
            return False
        
        # 验证视频文件存在
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
