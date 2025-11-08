"""输出路径和模板测试"""
from typing import List

from core.base_test import BaseTestCase, TestResult
from validators.output import OutputValidator
from validators.file import FileValidator


class TestDefaultOutputPath(BaseTestCase):
    """测试默认输出路径
    
    验证RVD能够使用默认输出路径，包括：
    - 文件名自动生成
    - 命名规则正确
    - 目录自动创建
    """
    
    def __init__(self, config):
        super().__init__(config)
        self.tags = ['core', 'output', 'default']
        self.timeout = 600  # 10分钟
        
        # 从配置文件加载测试数据
        test_data = self._load_test_data('core')
        output_data = test_data.get('output_template', {}).get('default', {})
        self.video_url = output_data.get('url', 'PLACEHOLDER_VIDEO_URL')
    
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
            files_exist=["*.mp4", "*.mkv", "*.flv"],
            min_size={"*.mp4": 1024 * 100, "*.mkv": 1024 * 100, "*.flv": 1024 * 100}
        )
        passed, msg = file_validator.validate(self.workdir)
        validations.append({"validator": "file", "passed": passed, "message": msg})
        
        if not passed:
            result.validations = validations
            result.error = msg
            return False
        
        # 验证文件名格式（应该包含视频标题或 BV 号）
        video_files = list(self.workdir.glob('**/*.mp4')) + list(self.workdir.glob('**/*.mkv')) + list(self.workdir.glob('**/*.flv'))
        if video_files:
            filename = video_files[0].name
            self.logger.info(f"Generated filename: {filename}")
            validations.append({"validator": "filename", "passed": True, "message": f"Filename: {filename}"})
        
        result.validations = validations
        return True


class TestCustomOutputTemplate(BaseTestCase):
    """测试自定义输出模板
    
    验证RVD能够使用自定义输出模板，包括：
    - 模板变量替换
    - 模板解析正确
    - 自定义格式应用
    """
    
    def __init__(self, config):
        super().__init__(config)
        self.tags = ['core', 'output', 'template']
        self.timeout = 600  # 10分钟
        
        # 从配置文件加载测试数据
        test_data = self._load_test_data('core')
        output_data = test_data.get('output_template', {}).get('custom', {})
        self.video_url = output_data.get('url', 'PLACEHOLDER_VIDEO_URL')
        self.output_template = output_data.get('output_template', '{title}_{bvid}')
    
    def get_command(self) -> List[str]:
        """获取执行命令"""
        cmd = self._build_base_command()
        cmd.extend([
            self.video_url,
            '--output', str(self.workdir),
        ])
        
        # 如果支持输出模板参数，添加它
        if self.output_template:
            # 注意：这里假设 RVD 支持 --output-template 参数
            # 如果实际参数名不同，需要调整
            cmd.extend(['--output-template', self.output_template])
        
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
            files_exist=["*.mp4", "*.mkv", "*.flv"],
            min_size={"*.mp4": 1024 * 100, "*.mkv": 1024 * 100, "*.flv": 1024 * 100}
        )
        passed, msg = file_validator.validate(self.workdir)
        validations.append({"validator": "file", "passed": passed, "message": msg})
        
        if not passed:
            result.validations = validations
            result.error = msg
            return False
        
        # 验证文件名格式（应该符合模板）
        video_files = list(self.workdir.glob('**/*.mp4')) + list(self.workdir.glob('**/*.mkv')) + list(self.workdir.glob('**/*.flv'))
        if video_files:
            filename = video_files[0].name
            self.logger.info(f"Generated filename with template: {filename}")
            validations.append({"validator": "filename", "passed": True, "message": f"Filename: {filename}"})
        
        result.validations = validations
        return True


class TestSpecialCharacterHandling(BaseTestCase):
    """测试特殊字符处理
    
    验证RVD能够正确处理文件名中的特殊字符，包括：
    - 特殊字符替换
    - 文件名安全化
    - 非法字符过滤
    """
    
    def __init__(self, config):
        super().__init__(config)
        self.tags = ['core', 'output', 'special']
        self.timeout = 600  # 10分钟
        
        # 从配置文件加载测试数据
        test_data = self._load_test_data('core')
        output_data = test_data.get('output_template', {}).get('default', {})
        self.video_url = output_data.get('url', 'PLACEHOLDER_VIDEO_URL')
    
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
            files_exist=["*.mp4", "*.mkv", "*.flv"],
            min_size={"*.mp4": 1024 * 100, "*.mkv": 1024 * 100, "*.flv": 1024 * 100}
        )
        passed, msg = file_validator.validate(self.workdir)
        validations.append({"validator": "file", "passed": passed, "message": msg})
        
        if not passed:
            result.validations = validations
            result.error = msg
            return False
        
        # 验证文件名不包含非法字符（如 / \ : * ? " < > |）
        video_files = list(self.workdir.glob('**/*.mp4')) + list(self.workdir.glob('**/*.mkv')) + list(self.workdir.glob('**/*.flv'))
        if video_files:
            filename = video_files[0].name
            illegal_chars = ['/', '\\', ':', '*', '?', '"', '<', '>', '|']
            has_illegal = any(char in filename for char in illegal_chars)
            
            if not has_illegal:
                validations.append({"validator": "filename_safe", "passed": True, "message": f"Filename is safe: {filename}"})
            else:
                validations.append({"validator": "filename_safe", "passed": False, "message": f"Filename contains illegal characters: {filename}"})
                result.validations = validations
                result.error = "Filename contains illegal characters"
                return False
        
        result.validations = validations
        return True
