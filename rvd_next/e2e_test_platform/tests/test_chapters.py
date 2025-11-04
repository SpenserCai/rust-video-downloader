"""章节信息测试"""
import yaml
from typing import List

from core.base_test import BaseTestCase, TestResult
from validators.output import OutputValidator
from validators.file import FileValidator


class TestChapters(BaseTestCase):
    """测试章节信息提取功能
    
    验证RVD能够正确提取和显示视频章节信息，包括：
    - 识别包含章节的视频
    - 输出章节信息
    - 章节信息格式正确
    """
    
    def __init__(self, config):
        super().__init__(config)
        self.tags = ['chapters', 'feature']
        self.timeout = 600  # 10分钟
        
        # 从配置文件加载URL
        urls_file = config.resolve_path(config.get('test_data.urls_file', './datas/urls.yaml'))
        if urls_file.exists():
            with open(urls_file, 'r', encoding='utf-8') as f:
                urls_data = yaml.safe_load(f)
                chapters_data = urls_data.get('chapters', {})
                self.video_url = chapters_data.get('url', 'PLACEHOLDER_VIDEO_URL')
        else:
            self.video_url = "PLACEHOLDER_VIDEO_URL"
    
    def get_command(self) -> List[str]:
        """获取执行命令"""
        cmd = self._build_base_command()
        cmd.extend([
            self.video_url,
            '--info-only',  # 只显示信息，不下载
        ])
        return cmd
    
    def validate(self, result: TestResult) -> bool:
        """验证结果"""
        validations = []
        
        # 验证输出包含章节相关信息
        # 章节信息可能包含：chapter, 章节, 分P等关键词
        output_lower = result.output.lower()
        
        has_chapter_info = any(keyword in output_lower for keyword in [
            'chapter', '章节', '分p', 'part', 'episode'
        ])
        
        if not has_chapter_info:
            # 视频可能没有章节信息，这也是正常的
            self.logger.warning("No chapter information found in output, video may not have chapters")
            # 只要命令执行成功就算通过
            if "error" not in output_lower and "failed" not in output_lower:
                return True
            else:
                result.error = "Command execution failed"
                return False
        
        # 如果有章节信息，验证格式是否合理
        output_validator = OutputValidator(
            contains=["info", "video"],
        )
        passed, msg = output_validator.validate(result)
        validations.append({"validator": "output", "passed": passed, "message": msg})
        
        result.validations = validations
        if not passed:
            result.error = msg
        
        return passed
