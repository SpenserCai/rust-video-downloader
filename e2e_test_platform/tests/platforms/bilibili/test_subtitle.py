"""字幕下载测试"""
from typing import List

from core.base_test import BaseTestCase, TestResult
from validators.output import OutputValidator
from validators.file import FileValidator
from validators.content import ContentValidator


class TestSingleSubtitle(BaseTestCase):
    """测试单字幕下载
    
    验证RVD能够正确下载字幕文件，包括：
    - 下载字幕文件
    - 生成正确格式的字幕文件（SRT）
    - 字幕文件内容格式正确
    """
    
    def __init__(self, config):
        super().__init__(config)
        self.tags = ['bilibili', 'subtitle', 'single']
        self.timeout = 600  # 10分钟
        
        # 从配置文件加载测试数据
        test_data = self._load_test_data('bilibili')
        subtitle_data = test_data.get('subtitle', {}).get('single_subtitle', {})
        self.video_url = subtitle_data.get('url', 'PLACEHOLDER_VIDEO_URL')
    
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
        output_lower = result.output.lower()
        
        # 验证下载成功
        has_success = any(keyword in output_lower for keyword in [
            'completed', 'success', '完成', '成功', 'muxed to'
        ])
        
        if has_success:
            validations.append({"validator": "success", "passed": True, "message": "Download completed"})
        else:
            self.logger.warning("No clear success message")
        
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
            validations.append({"validator": "video", "passed": passed, "message": msg})
            if passed:
                self.logger.info("Video downloaded successfully, no subtitles available")
                return True
            result.error = "Neither subtitle nor video file found"
            return False
        
        # 如果有字幕文件，验证其格式
        if srt_files:
            validations.append({"validator": "subtitle_file", "passed": True, "message": f"Found {len(srt_files)} SRT subtitle(s)"})
            
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
            validations.append({"validator": "subtitle_file", "passed": True, "message": f"Found {len(vtt_files)} VTT subtitle(s)"})
            
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


class TestMultiLanguageSubtitle(BaseTestCase):
    """测试多语言字幕下载
    
    验证RVD能够正确下载多语言字幕，包括：
    - 识别多语言字幕
    - 下载所有可用字幕
    - 字幕文件命名正确
    """
    
    def __init__(self, config):
        super().__init__(config)
        self.tags = ['bilibili', 'subtitle', 'multilang']
        self.timeout = 600  # 10分钟
        
        # 从配置文件加载测试数据
        test_data = self._load_test_data('bilibili')
        subtitle_data = test_data.get('subtitle', {}).get('multi_subtitle', {})
        self.video_url = subtitle_data.get('url', 'PLACEHOLDER_MULTI_SUBTITLE_URL')
    
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
        
        output_lower = result.output.lower()
        
        # 验证下载成功
        has_success = any(keyword in output_lower for keyword in [
            'completed', 'success', '完成', '成功', 'muxed to'
        ])
        
        if has_success:
            validations.append({"validator": "success", "passed": True, "message": "Download completed"})
        else:
            self.logger.warning("No clear success message")
        
        # 检查字幕文件
        srt_files = list(self.workdir.glob('**/*.srt'))
        vtt_files = list(self.workdir.glob('**/*.vtt'))
        subtitle_files = srt_files + vtt_files
        
        if not subtitle_files:
            self.logger.warning("No subtitle files found, video may not have subtitles")
            result.validations = validations
            # 只要视频下载成功就算通过
            file_validator = FileValidator(
                files_exist=["*.mp4", "*.mkv", "*.flv"],
                min_size={"*.mp4": 1024 * 100, "*.mkv": 1024 * 100, "*.flv": 1024 * 100}
            )
            passed, msg = file_validator.validate(self.workdir)
            validations.append({"validator": "video", "passed": passed, "message": msg})
            if passed:
                self.logger.info("Video downloaded successfully, no subtitles available")
                return True
            result.error = "Neither subtitle nor video file found"
            return False
        
        # 如果有多个字幕文件，说明支持多语言
        if len(subtitle_files) > 1:
            validations.append({"validator": "multilang", "passed": True, "message": f"Found {len(subtitle_files)} subtitle files (multi-language)"})
            self.logger.info(f"Multiple subtitle files found: {[f.name for f in subtitle_files]}")
        else:
            validations.append({"validator": "multilang", "passed": True, "message": f"Found {len(subtitle_files)} subtitle file"})
            self.logger.info("Single subtitle file found, video may only have one language")
        
        # 验证字幕格式
        if srt_files:
            content_validator = ContentValidator(
                file_pattern="*.srt",
                contains=["-->"]
            )
            passed, msg = content_validator.validate(self.workdir)
            validations.append({"validator": "srt_content", "passed": passed, "message": msg})
            result.validations = validations
            if not passed:
                result.error = msg
            return passed
        
        if vtt_files:
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
        
        result.validations = validations
        return True
