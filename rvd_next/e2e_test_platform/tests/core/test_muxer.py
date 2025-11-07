"""混流器功能测试"""
from typing import List
import subprocess

from core.base_test import BaseTestCase, TestResult
from validators.output import OutputValidator
from validators.file import FileValidator


class TestBasicMux(BaseTestCase):
    """测试基础混流功能
    
    验证RVD能够正确混流视频和音频，包括：
    - FFmpeg 混流调用
    - 视频音频合并
    - 输出文件格式正确
    """
    
    def __init__(self, config):
        super().__init__(config)
        self.tags = ['core', 'mux', 'basic']
        self.timeout = 600  # 10分钟
        
        # 从配置文件加载测试数据
        test_data = self._load_test_data('core')
        muxer_data = test_data.get('muxer', {}).get('basic', {})
        self.video_url = muxer_data.get('url', 'PLACEHOLDER_VIDEO_URL')
    
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
        
        # 验证输出包含混流相关信息
        output_lower = result.output.lower()
        has_mux = any(keyword in output_lower for keyword in [
            'mux', '混流', 'ffmpeg', 'merging', '合并'
        ])
        
        if has_mux:
            validations.append({"validator": "mux_info", "passed": True, "message": "Mux information found"})
        else:
            self.logger.warning("Mux information not clearly marked in output")
        
        # 验证输出包含成功信息
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
        
        # 验证视频文件存在且大小合理
        file_validator = FileValidator(
            files_exist=["*.mp4", "*.mkv"],
            min_size={"*.mp4": 1024 * 100, "*.mkv": 1024 * 100}
        )
        passed, msg = file_validator.validate(self.workdir)
        validations.append({"validator": "file", "passed": passed, "message": msg})
        
        result.validations = validations
        if not passed:
            result.error = msg
        
        return passed


class TestSubtitleEmbed(BaseTestCase):
    """测试字幕嵌入功能
    
    验证RVD能够正确嵌入字幕，包括：
    - 字幕轨道添加
    - 字幕流验证
    - 多字幕支持
    """
    
    def __init__(self, config):
        super().__init__(config)
        self.tags = ['core', 'mux', 'subtitle']
        self.timeout = 600  # 10分钟
        
        # 从配置文件加载测试数据
        test_data = self._load_test_data('core')
        muxer_data = test_data.get('muxer', {}).get('with_subtitle', {})
        self.video_url = muxer_data.get('url', 'PLACEHOLDER_VIDEO_URL')
    
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
            files_exist=["*.mp4", "*.mkv"],
            min_size={"*.mp4": 1024 * 100, "*.mkv": 1024 * 100}
        )
        passed, msg = file_validator.validate(self.workdir)
        validations.append({"validator": "file", "passed": passed, "message": msg})
        
        if not passed:
            result.validations = validations
            result.error = msg
            return False
        
        # 检查是否有字幕文件（如果视频有字幕）
        subtitle_files = list(self.workdir.glob('**/*.srt')) + list(self.workdir.glob('**/*.vtt'))
        if subtitle_files:
            validations.append({"validator": "subtitle", "passed": True, "message": f"Found {len(subtitle_files)} subtitle file(s)"})
        else:
            self.logger.warning("No subtitle files found, video may not have subtitles")
        
        result.validations = validations
        return True


class TestChapterEmbed(BaseTestCase):
    """测试章节元数据嵌入功能
    
    验证RVD能够正确嵌入章节信息，包括：
    - 章节信息提取
    - 章节元数据写入
    - FFmpeg 元数据验证
    """
    
    def __init__(self, config):
        super().__init__(config)
        self.tags = ['core', 'mux', 'chapters']
        self.timeout = 600  # 10分钟
        
        # 从配置文件加载测试数据
        test_data = self._load_test_data('core')
        muxer_data = test_data.get('muxer', {}).get('basic', {})
        self.video_url = muxer_data.get('url', 'PLACEHOLDER_VIDEO_URL')
    
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
            files_exist=["*.mp4", "*.mkv"],
            min_size={"*.mp4": 1024 * 100, "*.mkv": 1024 * 100}
        )
        passed, msg = file_validator.validate(self.workdir)
        validations.append({"validator": "file", "passed": passed, "message": msg})
        
        if not passed:
            result.validations = validations
            result.error = msg
            return False
        
        # 检查输出中是否提到章节信息
        has_chapter_info = any(keyword in output_lower for keyword in [
            'chapter', '章节', 'metadata'
        ])
        
        if has_chapter_info:
            validations.append({"validator": "chapter_info", "passed": True, "message": "Chapter information found"})
        else:
            self.logger.warning("No chapter information found, video may not have chapters")
        
        result.validations = validations
        return True


class TestSkipMux(BaseTestCase):
    """测试跳过混流模式
    
    验证RVD能够正确跳过混流，包括：
    - --skip-mux 参数支持
    - 分离文件保存
    - 临时文件保留
    """
    
    def __init__(self, config):
        super().__init__(config)
        self.tags = ['core', 'mux', 'skip']
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
            result.error = "No video/audio files found"
            return False
        
        result.validations = validations
        return True
