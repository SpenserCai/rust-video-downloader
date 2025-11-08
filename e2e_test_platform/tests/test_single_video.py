"""单视频下载测试"""
from typing import List

from core.base_test import BaseTestCase, TestResult
from validators.output import OutputValidator
from validators.file import FileValidator


class TestSingleVideoDownload(BaseTestCase):
    """测试单视频下载功能
    
    验证RVD能够正确下载单个视频文件，包括：
    - 命令执行成功
    - 输出包含成功信息
    - 生成视频文件
    - 文件大小合理
    """
    
    def __init__(self, config):
        super().__init__(config)
        self.tags = ['basic', 'video']
        self.timeout = 600  # 10分钟
        
        # 从配置文件加载测试数据（自动处理auth_file, quality等通用参数）
        test_data = self._load_test_data('single_video')
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
        
        # 验证输出包含成功信息（支持中英文）
        output_lower = result.output.lower()
        has_success = any(keyword in output_lower for keyword in [
            'completed', 'success', '完成', '成功', 'muxed to'
        ])
        
        if has_success:
            validations.append({"validator": "output", "passed": True, "message": ""})
        else:
            validations.append({"validator": "output", "passed": False, "message": "No success message found"})
            result.validations = validations
            return False
        
        # 验证视频文件存在且大小合理（至少一种格式）
        file_validator = FileValidator(
            files_exist=["*.mp4", "*.mkv", "*.flv"],
            min_size={"*.mp4": 1024 * 100, "*.mkv": 1024 * 100, "*.flv": 1024 * 100}  # 至少100KB
        )
        passed, msg = file_validator.validate(self.workdir)
        validations.append({"validator": "file", "passed": passed, "message": msg})
        
        result.validations = validations
        return passed
