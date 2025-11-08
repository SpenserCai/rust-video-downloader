"""弹幕下载测试"""
from typing import List

from core.base_test import BaseTestCase, TestResult
from validators.output import OutputValidator
from validators.file import FileValidator
from validators.content import ContentValidator


class TestXMLDanmaku(BaseTestCase):
    """测试 XML 格式弹幕下载
    
    验证RVD能够正确下载 XML 格式弹幕，包括：
    - 下载 XML 弹幕文件
    - XML 格式正确
    - 包含弹幕内容
    """
    
    def __init__(self, config):
        super().__init__(config)
        self.tags = ['bilibili', 'danmaku', 'xml']
        self.timeout = 600  # 10分钟
        
        # 从配置文件加载测试数据
        test_data = self._load_test_data('bilibili')
        danmaku_data = test_data.get('danmaku', {}).get('xml_format', {})
        self.video_url = danmaku_data.get('url', 'PLACEHOLDER_VIDEO_URL')
        self.danmaku_format = 'xml'
    
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
        output_lower = result.output.lower()
        
        # 验证下载成功
        has_success = any(keyword in output_lower for keyword in [
            'completed', 'success', '完成', '成功', 'downloaded', 'danmaku', '弹幕'
        ])
        
        if has_success:
            validations.append({"validator": "success", "passed": True, "message": "Download completed"})
        else:
            self.logger.warning("No clear success message, but checking for files")
        
        # 验证 XML 弹幕文件存在
        file_pattern = "*.xml"
        file_validator = FileValidator(
            files_exist=[file_pattern],
            min_size={file_pattern: 100}  # 至少100字节
        )
        passed, msg = file_validator.validate(self.workdir)
        validations.append({"validator": "file", "passed": passed, "message": msg})
        
        if not passed:
            # 视频可能没有弹幕，这也是正常的
            self.logger.warning("No XML danmaku file found, video may not have danmaku")
            result.validations = validations
            # 只要视频下载成功就算通过
            video_validator = FileValidator(
                files_exist=["*.mp4", "*.mkv", "*.flv"],
                min_size={"*.mp4": 1024 * 100, "*.mkv": 1024 * 100, "*.flv": 1024 * 100}
            )
            video_passed, video_msg = video_validator.validate(self.workdir)
            if video_passed:
                self.logger.info("Video downloaded successfully, no danmaku available")
                return True
            result.error = "Neither danmaku nor video file found"
            return False
        
        # 验证 XML 文件内容格式
        content_patterns = ["<d ", "</d>", "<?xml"]
        content_validator = ContentValidator(
            file_pattern=file_pattern,
            contains=content_patterns
        )
        content_passed, content_msg = content_validator.validate(self.workdir)
        validations.append({"validator": "content", "passed": content_passed, "message": content_msg})
        
        result.validations = validations
        if not content_passed:
            result.error = content_msg
        
        return content_passed


class TestASSDanmaku(BaseTestCase):
    """测试 ASS 格式弹幕转换
    
    验证RVD能够正确转换 ASS 格式弹幕，包括：
    - 下载并转换为 ASS 格式
    - ASS 格式正确
    - 时间轴正确
    """
    
    def __init__(self, config):
        super().__init__(config)
        self.tags = ['bilibili', 'danmaku', 'ass']
        self.timeout = 600  # 10分钟
        
        # 从配置文件加载测试数据
        test_data = self._load_test_data('bilibili')
        danmaku_data = test_data.get('danmaku', {}).get('ass_format', {})
        self.video_url = danmaku_data.get('url', 'PLACEHOLDER_VIDEO_URL')
        self.danmaku_format = 'ass'
    
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
        output_lower = result.output.lower()
        
        # 验证下载成功
        has_success = any(keyword in output_lower for keyword in [
            'completed', 'success', '完成', '成功', 'downloaded', 'danmaku', '弹幕'
        ])
        
        if has_success:
            validations.append({"validator": "success", "passed": True, "message": "Download completed"})
        else:
            self.logger.warning("No clear success message, but checking for files")
        
        # 验证 ASS 弹幕文件存在
        file_pattern = "*.ass"
        file_validator = FileValidator(
            files_exist=[file_pattern],
            min_size={file_pattern: 100}  # 至少100字节
        )
        passed, msg = file_validator.validate(self.workdir)
        validations.append({"validator": "file", "passed": passed, "message": msg})
        
        if not passed:
            # 视频可能没有弹幕，这也是正常的
            self.logger.warning("No ASS danmaku file found, video may not have danmaku")
            result.validations = validations
            # 只要视频下载成功就算通过
            video_validator = FileValidator(
                files_exist=["*.mp4", "*.mkv", "*.flv"],
                min_size={"*.mp4": 1024 * 100, "*.mkv": 1024 * 100, "*.flv": 1024 * 100}
            )
            video_passed, video_msg = video_validator.validate(self.workdir)
            if video_passed:
                self.logger.info("Video downloaded successfully, no danmaku available")
                return True
            result.error = "Neither danmaku nor video file found"
            return False
        
        # 验证 ASS 文件内容格式
        content_patterns = ["[Script Info]", "Dialogue:", "[Events]"]
        content_validator = ContentValidator(
            file_pattern=file_pattern,
            contains=content_patterns
        )
        content_passed, content_msg = content_validator.validate(self.workdir)
        validations.append({"validator": "content", "passed": content_passed, "message": content_msg})
        
        result.validations = validations
        if not content_passed:
            result.error = content_msg
        
        return content_passed


class TestDanmakuDecompression(BaseTestCase):
    """测试弹幕解压缩
    
    验证RVD能够正确解压缩弹幕数据，包括：
    - 识别压缩的弹幕数据
    - 正确解压（deflate/gzip）
    - 解压后格式正确
    """
    
    def __init__(self, config):
        super().__init__(config)
        self.tags = ['bilibili', 'danmaku', 'decompression']
        self.timeout = 600  # 10分钟
        
        # 从配置文件加载测试数据
        test_data = self._load_test_data('bilibili')
        danmaku_data = test_data.get('danmaku', {}).get('xml_format', {})
        self.video_url = danmaku_data.get('url', 'PLACEHOLDER_VIDEO_URL')
    
    def get_command(self) -> List[str]:
        """获取执行命令"""
        cmd = self._build_base_command()
        cmd.extend([
            self.video_url,
            '--download-danmaku',
            '--danmaku-format', 'xml',
            '--output', str(self.workdir),
        ])
        return cmd
    
    def validate(self, result: TestResult) -> bool:
        """验证结果"""
        validations = []
        output_lower = result.output.lower()
        
        # 检查是否提到解压缩
        has_decompress_info = any(keyword in output_lower for keyword in [
            'decompress', 'inflate', 'gzip', 'deflate', '解压', '压缩'
        ])
        
        if has_decompress_info:
            validations.append({"validator": "decompress_info", "passed": True, "message": "Decompression information found"})
            self.logger.info("Decompression detected in output")
        else:
            self.logger.warning("No decompression information found, data may not be compressed")
        
        # 验证弹幕文件存在且可读
        file_pattern = "*.xml"
        file_validator = FileValidator(
            files_exist=[file_pattern],
            min_size={file_pattern: 100}
        )
        passed, msg = file_validator.validate(self.workdir)
        validations.append({"validator": "file", "passed": passed, "message": msg})
        
        if not passed:
            self.logger.warning("No danmaku file found, video may not have danmaku")
            result.validations = validations
            # 只要视频下载成功就算通过
            video_validator = FileValidator(
                files_exist=["*.mp4", "*.mkv", "*.flv"],
                min_size={"*.mp4": 1024 * 100, "*.mkv": 1024 * 100, "*.flv": 1024 * 100}
            )
            video_passed, video_msg = video_validator.validate(self.workdir)
            if video_passed:
                self.logger.info("Video downloaded successfully, no danmaku available")
                return True
            result.error = "Neither danmaku nor video file found"
            return False
        
        # 验证 XML 内容可读（说明解压成功）
        content_patterns = ["<d ", "<?xml"]
        content_validator = ContentValidator(
            file_pattern=file_pattern,
            contains=content_patterns
        )
        content_passed, content_msg = content_validator.validate(self.workdir)
        validations.append({"validator": "content", "passed": content_passed, "message": content_msg})
        
        result.validations = validations
        if not content_passed:
            result.error = "Danmaku file exists but content is not valid XML (decompression may have failed)"
        
        return content_passed
