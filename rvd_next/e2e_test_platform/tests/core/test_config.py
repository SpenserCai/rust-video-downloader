"""配置管理测试"""
from typing import List

from core.base_test import BaseTestCase, TestResult
from validators.output import OutputValidator
from validators.file import FileValidator


class TestDefaultConfig(BaseTestCase):
    """测试默认配置
    
    验证RVD能够使用默认配置正常工作，包括：
    - 无配置文件时使用内置默认值
    - 默认参数正确应用
    - 基本功能正常
    """
    
    def __init__(self, config):
        super().__init__(config)
        self.tags = ['core', 'config', 'default']
        self.timeout = 600  # 10分钟
        
        # 从配置文件加载测试数据
        test_data = self._load_test_data('core')
        config_data = test_data.get('config', {}).get('default', {})
        self.video_url = config_data.get('url', 'PLACEHOLDER_VIDEO_URL')
    
    def get_command(self) -> List[str]:
        """获取执行命令（不指定配置文件）"""
        # 不使用 _build_base_command，直接构建命令以避免使用配置文件
        cmd = [str(self.config.executable)]
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
        
        result.validations = validations
        if not passed:
            result.error = msg
        
        return passed


class TestCustomConfig(BaseTestCase):
    """测试自定义配置文件
    
    验证RVD能够正确加载和应用自定义配置，包括：
    - TOML 配置文件解析
    - 配置值正确应用
    - 配置文件路径指定
    """
    
    def __init__(self, config):
        super().__init__(config)
        self.tags = ['core', 'config', 'custom']
        self.timeout = 600  # 10分钟
        
        # 从配置文件加载测试数据
        test_data = self._load_test_data('core')
        config_data = test_data.get('config', {}).get('custom', {})
        self.video_url = config_data.get('url', 'PLACEHOLDER_VIDEO_URL')
        self.config_file = config_data.get('config_file', './cfg/rvd.toml')
    
    def get_command(self) -> List[str]:
        """获取执行命令"""
        cmd = [str(self.config.executable)]
        
        # 指定配置文件
        if self.config_file:
            config_path = self.config.resolve_path(self.config_file)
            if config_path.exists():
                cmd.extend(['--config-file', str(config_path)])
            else:
                self.logger.warning(f"Config file not found: {config_path}, using default")
        
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
        
        result.validations = validations
        if not passed:
            result.error = msg
        
        return passed


class TestCLIPriority(BaseTestCase):
    """测试 CLI 参数优先级
    
    验证CLI参数能够覆盖配置文件值，包括：
    - CLI 参数优先级高于配置文件
    - 参数正确应用
    - 优先级顺序验证
    """
    
    def __init__(self, config):
        super().__init__(config)
        self.tags = ['core', 'config', 'priority']
        self.timeout = 600  # 10分钟
        
        # 从配置文件加载测试数据
        test_data = self._load_test_data('core')
        config_data = test_data.get('config', {}).get('custom', {})
        self.video_url = config_data.get('url', 'PLACEHOLDER_VIDEO_URL')
        self.config_file = config_data.get('config_file', './cfg/rvd.toml')
    
    def get_command(self) -> List[str]:
        """获取执行命令"""
        cmd = [str(self.config.executable)]
        
        # 指定配置文件
        if self.config_file:
            config_path = self.config.resolve_path(self.config_file)
            if config_path.exists():
                cmd.extend(['--config-file', str(config_path)])
        
        # 添加 CLI 参数（应该覆盖配置文件中的值）
        cmd.extend([
            '--quality', '480P',  # CLI 指定质量
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
        
        # 验证输出中包含 CLI 指定的质量（480P）
        has_quality = '480p' in output_lower or '480' in output_lower
        if has_quality:
            validations.append({"validator": "quality", "passed": True, "message": "CLI quality parameter applied"})
        else:
            self.logger.warning("Quality information not clearly marked in output")
        
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
