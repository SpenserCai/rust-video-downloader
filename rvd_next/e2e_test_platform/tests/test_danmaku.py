"""弹幕下载测试"""
from typing import List

from core.base_test import BaseTestCase, TestResult
from validators.output import OutputValidator
from validators.file import FileValidator
from validators.content import ContentValidator


class TestDanmakuDownload(BaseTestCase):
    """测试弹幕下载功能
    
    验证RVD能够正确下载弹幕文件，包括：
    - 下载弹幕文件
    - 生成正确格式的弹幕文件（ASS或XML）
    - 弹幕文件内容格式正确
    """
    
    def __init__(self, config):
        super().__init__(config)
        self.tags = ['danmaku', 'feature']
        self.timeout = 600  # 10分钟
        
        # 从配置文件加载测试数据（自动处理auth_file, quality等通用参数）
        test_data = self._load_test_data('danmaku')
        self.video_url = test_data.get('url', 'PLACEHOLDER_VIDEO_URL')
        self.danmaku_format = test_data.get('format', 'ass')
    
    def get_command(self) -> List[str]:
        """获取执行命令"""
        cmd = self._build_base_command()
        cmd.extend([
            self.video_url,
            '--download-danmaku',
            '--danmaku-format', self.danmaku_format,
            '--output', str(self.workdir),
        ])
        return cmd
    
    def validate(self, result: TestResult) -> bool:
        """验证结果"""
        validations = []
        
        # 验证弹幕文件存在
        if self.danmaku_format == 'ass':
            file_pattern = "*.ass"
            content_patterns = ["[Script Info]", "Dialogue:"]
        else:  # xml
            file_pattern = "*.xml"
            content_patterns = ["<d ", "</d>"]
        
        file_validator = FileValidator(
            files_exist=[file_pattern],
            min_size={file_pattern: 100}  # 至少100字节
        )
        passed, msg = file_validator.validate(self.workdir)
        validations.append({"validator": "file", "passed": passed, "message": msg})
        
        if not passed:
            result.validations = validations
            result.error = msg
            return False
        
        # 验证弹幕文件内容格式
        content_validator = ContentValidator(
            file_pattern=file_pattern,
            contains=content_patterns
        )
        passed, msg = content_validator.validate(self.workdir)
        validations.append({"validator": "content", "passed": passed, "message": msg})
        
        result.validations = validations
        if not passed:
            result.error = msg
        
        return passed
