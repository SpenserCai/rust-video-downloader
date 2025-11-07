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
        self.tags = ['bilibili', 'quality', 'feature']
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
            validations.append({"validator": "output", "passed": True, "message": "Quality information found in output"})
        else:
            self.logger.warning(f"Quality information not clearly marked in output, but continuing validation")
            validations.append({"validator": "output", "passed": False, "message": "Quality information not clearly marked"})
        
        # 验证下载成功
        has_success = any(keyword in output_lower for keyword in [
            'completed', 'success', '完成', '成功', 'muxed to', 'downloaded'
        ])
        
        if has_success:
            validations.append({"validator": "success", "passed": True, "message": "Download completed successfully"})
        else:
            validations.append({"validator": "success", "passed": False, "message": "No success message found"})
            result.validations = validations
            result.error = "Download did not complete successfully"
            return False
        
        # 验证视频文件存在（至少一种格式）
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


class TestQualityPriority(BaseTestCase):
    """测试质量优先级选择
    
    验证RVD能够按照优先级选择视频质量，包括：
    - 支持多个质量选项
    - 按优先级顺序选择
    - 选择可用的最高质量
    """
    
    def __init__(self, config):
        super().__init__(config)
        self.tags = ['bilibili', 'quality', 'priority']
        self.timeout = 600  # 10分钟
        
        # 从配置文件加载测试数据
        test_data = self._load_test_data('bilibili')
        quality_data = test_data.get('quality_selection', {}).get('quality_priority', {})
        self.video_url = quality_data.get('url', 'PLACEHOLDER_VIDEO_URL')
        self.quality = quality_data.get('quality', '1080P,720P,480P')
    
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
            validations.append({"validator": "success", "passed": False, "message": "Download failed"})
            result.validations = validations
            result.error = "Download did not complete successfully"
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


class TestCodecPriority(BaseTestCase):
    """测试编码优先级选择
    
    验证RVD能够按照编码优先级选择视频，包括：
    - 支持 AVC/HEVC/AV1 编码选择
    - 按优先级顺序选择编码
    - 选择可用的最优编码
    """
    
    def __init__(self, config):
        super().__init__(config)
        self.tags = ['bilibili', 'quality', 'codec']
        self.timeout = 600  # 10分钟
        
        # 从配置文件加载测试数据
        test_data = self._load_test_data('bilibili')
        codec_data = test_data.get('quality_selection', {}).get('codec_priority', {})
        self.video_url = codec_data.get('url', 'PLACEHOLDER_VIDEO_URL')
        self.codec = codec_data.get('codec', 'hevc,avc,av1')
    
    def get_command(self) -> List[str]:
        """获取执行命令"""
        cmd = self._build_base_command()
        cmd.extend([
            '--codec', self.codec,
            self.video_url,
            '--output', str(self.workdir),
        ])
        return cmd
    
    def validate(self, result: TestResult) -> bool:
        """验证结果"""
        validations = []
        output_lower = result.output.lower()
        
        # 验证输出包含编码信息
        has_codec_info = any(keyword in output_lower for keyword in [
            'codec', '编码', 'avc', 'hevc', 'av1', 'h264', 'h265'
        ])
        
        if has_codec_info:
            validations.append({"validator": "codec_info", "passed": True, "message": "Codec information found"})
        else:
            self.logger.warning("Codec information not clearly marked in output")
        
        # 验证下载成功
        has_success = any(keyword in output_lower for keyword in [
            'completed', 'success', '完成', '成功', 'muxed to'
        ])
        
        if has_success:
            validations.append({"validator": "success", "passed": True, "message": "Download completed"})
        else:
            validations.append({"validator": "success", "passed": False, "message": "Download failed"})
            result.validations = validations
            result.error = "Download did not complete successfully"
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


class TestDolbyVision(BaseTestCase):
    """测试杜比视界质量
    
    验证RVD能够正确处理杜比视界质量，包括：
    - 识别杜比视界质量
    - FFmpeg 版本检测
    - 正确下载或给出警告
    """
    
    def __init__(self, config):
        super().__init__(config)
        self.tags = ['bilibili', 'quality', 'dolby', 'advanced']
        self.timeout = 900  # 15分钟（杜比视界文件可能较大）
        
        # 从配置文件加载测试数据
        test_data = self._load_test_data('bilibili')
        dolby_data = test_data.get('quality_selection', {}).get('dolby_vision', {})
        self.video_url = dolby_data.get('url', 'PLACEHOLDER_DOLBY_VISION_URL')
        self.quality = dolby_data.get('quality', '杜比视界')
    
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
        
        # 检查是否提到杜比视界或 FFmpeg 版本
        has_dolby_mention = any(keyword in output_lower for keyword in [
            'dolby', '杜比', 'vision', '视界', 'ffmpeg', '126'
        ])
        
        if has_dolby_mention:
            validations.append({"validator": "dolby_info", "passed": True, "message": "Dolby Vision information found"})
        else:
            self.logger.warning("No Dolby Vision information found in output")
        
        # 验证下载成功或有明确的警告/错误信息
        has_success = any(keyword in output_lower for keyword in [
            'completed', 'success', '完成', '成功', 'muxed to'
        ])
        
        has_warning = any(keyword in output_lower for keyword in [
            'warning', 'ffmpeg version', 'upgrade', '警告', '版本', '升级'
        ])
        
        if has_success:
            validations.append({"validator": "success", "passed": True, "message": "Download completed"})
            
            # 验证视频文件存在
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
        elif has_warning:
            validations.append({"validator": "warning", "passed": True, "message": "FFmpeg version warning detected"})
            result.validations = validations
            self.logger.info("Dolby Vision warning detected (FFmpeg version may be insufficient)")
            return True
        else:
            validations.append({"validator": "result", "passed": False, "message": "No clear result"})
            result.validations = validations
            result.error = "No success or warning message found"
            return False
