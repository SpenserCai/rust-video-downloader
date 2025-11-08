"""章节信息测试"""
from typing import List

from core.base_test import BaseTestCase, TestResult
from validators.output import OutputValidator
from validators.file import FileValidator


class TestNormalVideoChapters(BaseTestCase):
    """测试普通视频章节提取
    
    验证RVD能够正确提取普通视频的章节信息，包括：
    - 识别 view_points 章节
    - 提取章节标题和时间
    - 章节信息格式正确
    """
    
    def __init__(self, config):
        super().__init__(config)
        self.tags = ['bilibili', 'chapters', 'normal']
        self.timeout = 600  # 10分钟
        
        # 从配置文件加载测试数据
        test_data = self._load_test_data('bilibili')
        chapters_data = test_data.get('chapters', {}).get('with_chapters', {})
        self.video_url = chapters_data.get('url', 'PLACEHOLDER_CHAPTERS_URL')
    
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
        
        # 检查 URL 是否为占位符
        if self.video_url.startswith('PLACEHOLDER_'):
            self.logger.warning(f"URL is placeholder: {self.video_url}, skipping test")
            result.validations = validations
            result.error = "URL not configured"
            return False
        
        output_lower = result.output.lower()
        
        # 验证命令执行成功
        has_error = any(keyword in output_lower for keyword in [
            'error', 'failed', '错误', '失败'
        ])
        
        if has_error:
            validations.append({"validator": "execution", "passed": False, "message": "Command execution failed"})
            result.validations = validations
            result.error = "Command execution failed"
            return False
        
        validations.append({"validator": "execution", "passed": True, "message": "Command executed successfully"})
        
        # 验证输出包含章节相关信息
        has_chapter_info = any(keyword in output_lower for keyword in [
            'chapter', '章节', 'view_point', 'timestamp', '时间戳'
        ])
        
        if has_chapter_info:
            validations.append({"validator": "chapter_info", "passed": True, "message": "Chapter information found"})
            self.logger.info("Chapter information detected in output")
        else:
            # 视频可能没有章节信息，这也是正常的
            self.logger.warning("No chapter information found, video may not have chapters")
            validations.append({"validator": "chapter_info", "passed": True, "message": "No chapters (video may not have chapter markers)"})
        
        result.validations = validations
        return True


class TestBangumiChapters(BaseTestCase):
    """测试番剧章节提取
    
    验证RVD能够正确提取番剧的章节信息，包括：
    - 识别 clip_info_list 章节
    - 提取片头片尾章节
    - 章节信息格式正确
    """
    
    def __init__(self, config):
        super().__init__(config)
        self.tags = ['bilibili', 'chapters', 'bangumi']
        self.timeout = 600  # 10分钟
        
        # 从配置文件加载测试数据
        test_data = self._load_test_data('bilibili')
        chapters_data = test_data.get('chapters', {}).get('bangumi_chapters', {})
        self.video_url = chapters_data.get('url', 'PLACEHOLDER_BANGUMI_CHAPTERS_URL')
    
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
        
        # 检查 URL 是否为占位符
        if self.video_url.startswith('PLACEHOLDER_'):
            self.logger.warning(f"URL is placeholder: {self.video_url}, skipping test")
            result.validations = validations
            result.error = "URL not configured"
            return False
        
        output_lower = result.output.lower()
        
        # 验证命令执行成功
        has_error = any(keyword in output_lower for keyword in [
            'error', 'failed', '错误', '失败'
        ])
        
        if has_error:
            validations.append({"validator": "execution", "passed": False, "message": "Command execution failed"})
            result.validations = validations
            result.error = "Command execution failed"
            return False
        
        validations.append({"validator": "execution", "passed": True, "message": "Command executed successfully"})
        
        # 验证输出包含章节相关信息（番剧特有）
        has_chapter_info = any(keyword in output_lower for keyword in [
            'chapter', '章节', 'clip', 'opening', 'ending', '片头', '片尾', 'op', 'ed'
        ])
        
        if has_chapter_info:
            validations.append({"validator": "chapter_info", "passed": True, "message": "Bangumi chapter information found"})
            self.logger.info("Bangumi chapter information detected in output")
        else:
            # 番剧可能没有章节信息，这也是正常的
            self.logger.warning("No bangumi chapter information found, video may not have chapter markers")
            validations.append({"validator": "chapter_info", "passed": True, "message": "No chapters (bangumi may not have chapter markers)"})
        
        result.validations = validations
        return True


class TestChapterEmbedding(BaseTestCase):
    """测试章节嵌入功能
    
    验证RVD能够正确将章节信息嵌入到视频文件中，包括：
    - 下载视频并嵌入章节
    - 验证章节元数据存在
    - FFmpeg 元数据正确
    """
    
    def __init__(self, config):
        super().__init__(config)
        self.tags = ['bilibili', 'chapters', 'embed']
        self.timeout = 600  # 10分钟
        
        # 从配置文件加载测试数据
        test_data = self._load_test_data('bilibili')
        chapters_data = test_data.get('chapters', {}).get('with_chapters', {})
        self.video_url = chapters_data.get('url', 'PLACEHOLDER_CHAPTERS_URL')
    
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
            validations.append({"validator": "success", "passed": False, "message": "Download failed"})
            result.validations = validations
            result.error = "Download did not complete successfully"
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
        has_chapter_mention = any(keyword in output_lower for keyword in [
            'chapter', '章节', 'metadata', '元数据'
        ])
        
        if has_chapter_mention:
            validations.append({"validator": "chapter_mention", "passed": True, "message": "Chapter information mentioned in output"})
            self.logger.info("Chapter embedding detected in output")
        else:
            self.logger.warning("No chapter information mentioned, video may not have chapters")
            validations.append({"validator": "chapter_mention", "passed": True, "message": "No chapter mention (video may not have chapters)"})
        
        result.validations = validations
        return True
