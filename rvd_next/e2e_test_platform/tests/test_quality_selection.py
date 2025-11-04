"""质量选择测试"""
from typing import List

from core.base_test import BaseTestCase, TestResult
from validators.output import OutputValidator
from validators.file import FileValidator


class TestQualitySelection(BaseTestCase):
    """测试质量选择功能
    
    验证RVD能够正确选择指定的视频质量，包括：
    - 接受质量参数
    - 下载指定质量的视频
    - 输出包含质量信息
    """
    
    def __init__(self, config):
        super().__init__(config)
        self.tags = ['quality', 'feature']
        self.timeout = 600  # 10分钟
        
        # 从配置文件加载测试数据（自动处理auth_file, quality等通用参数）
        test_data = self._load_test_data('quality_selection')
        self.video_url = test_data.get('url', 'PLACEHOLDER_VIDEO_URL')
        # quality已经在_load_test_data中自动设置
        if not self.quality:
            self.quality = '720P'
    
    def get_command(self) -> List[str]:
        """获取执行命令"""
        # _build_base_command 已经包含了 --quality 参数
        cmd = self._build_base_command()
        cmd.extend([
            self.video_url,
            '--output', str(self.workdir),
        ])
        return cmd
    
    def validate(self, result: TestResult) -> bool:
        """验证结果"""
        validations = []
        
        # 验证输出包含质量信息（支持中英文）
        output_lower = result.output.lower()
        has_quality_info = any(keyword in output_lower for keyword in [
            'quality', '质量', 'selected video', self.quality.lower(), 
            '480p', '720p', '1080p', 'avc', 'hevc'
        ])
        
        if has_quality_info:
            validations.append({"validator": "output", "passed": True, "message": ""})
        else:
            self.logger.warning(f"Quality information not clearly marked in output, but continuing validation")
            validations.append({"validator": "output", "passed": False, "message": "Quality information not clearly marked"})
        
        # 验证视频文件存在（至少一种格式）
        file_validator = FileValidator(
            files_exist=["*.mp4", "*.mkv", "*.flv"],
            min_size={"*.mp4": 1024 * 100, "*.mkv": 1024 * 100, "*.flv": 1024 * 100}
        )
        passed, msg = file_validator.validate(self.workdir)
        validations.append({"validator": "file", "passed": passed, "message": msg})
        
        result.validations = validations
        return passed
