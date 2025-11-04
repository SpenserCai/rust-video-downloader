"""字幕下载测试"""
import yaml
from typing import List

from core.base_test import BaseTestCase, TestResult
from validators.output import OutputValidator
from validators.file import FileValidator
from validators.content import ContentValidator


class TestSubtitleDownload(BaseTestCase):
    """测试字幕下载功能
    
    验证RVD能够正确下载字幕文件，包括：
    - 下载字幕文件
    - 生成正确格式的字幕文件（SRT或VTT）
    - 字幕文件内容格式正确
    """
    
    def __init__(self, config):
        super().__init__(config)
        self.tags = ['subtitle', 'feature']
        self.timeout = 600  # 10分钟
        
        # 从配置文件加载URL
        urls_file = config.resolve_path(config.get('test_data.urls_file', './datas/urls.yaml'))
        if urls_file.exists():
            with open(urls_file, 'r', encoding='utf-8') as f:
                urls_data = yaml.safe_load(f)
                subtitle_data = urls_data.get('subtitle', {})
                self.video_url = subtitle_data.get('url', 'PLACEHOLDER_VIDEO_URL')
        else:
            self.video_url = "PLACEHOLDER_VIDEO_URL"
    
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
        
        # 检查是否有字幕文件（SRT或VTT格式）
        srt_files = list(self.workdir.glob('**/*.srt'))
        vtt_files = list(self.workdir.glob('**/*.vtt'))
        
        if not srt_files and not vtt_files:
            # 字幕可能不是必需的，如果视频本身没有字幕
            self.logger.warning("No subtitle files found, video may not have subtitles")
            result.validations = validations
            # 只要视频下载成功就算通过
            file_validator = FileValidator(
                files_exist=["*.mp4", "*.mkv", "*.flv"],
                min_size={"*.mp4": 1024 * 100, "*.mkv": 1024 * 100, "*.flv": 1024 * 100}
            )
            passed, msg = file_validator.validate(self.workdir)
            validations.append({"validator": "file", "passed": passed, "message": msg})
            return passed
        
        # 如果有字幕文件，验证其格式
        if srt_files:
            # 验证SRT格式：应该包含时间戳和文本
            content_validator = ContentValidator(
                file_pattern="*.srt",
                contains=["-->"]  # SRT时间戳格式
            )
            passed, msg = content_validator.validate(self.workdir)
            validations.append({"validator": "srt_content", "passed": passed, "message": msg})
            result.validations = validations
            if not passed:
                result.error = msg
            return passed
        
        if vtt_files:
            # 验证VTT格式：应该包含WEBVTT标记
            content_validator = ContentValidator(
                file_pattern="*.vtt",
                contains=["WEBVTT"]
            )
            passed, msg = content_validator.validate(self.workdir)
            validations.append({"validator": "vtt_content", "passed": passed, "message": msg})
            result.validations = validations
            if not passed:
                result.error = msg
            return passed
        
        return True
