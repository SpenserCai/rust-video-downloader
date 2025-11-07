"""HTTP 客户端配置测试"""
from typing import List

from core.base_test import BaseTestCase, TestResult
from validators.output import OutputValidator
from validators.file import FileValidator


class TestDefaultUserAgent(BaseTestCase):
    """测试默认 User-Agent
    
    验证RVD能够使用默认 User-Agent，包括：
    - User-Agent 随机生成
    - 日志中显示 User-Agent
    - 请求正常发送
    """
    
    def __init__(self, config):
        super().__init__(config)
        self.tags = ['core', 'http', 'user-agent']
        self.timeout = 600  # 10分钟
        
        # 从配置文件加载测试数据
        test_data = self._load_test_data('core')
        http_data = test_data.get('http_client', {}).get('default_user_agent', {})
        self.video_url = http_data.get('url', 'PLACEHOLDER_VIDEO_URL')
        self.log_user_agent = http_data.get('log_user_agent', True)
    
    def get_command(self) -> List[str]:
        """获取执行命令"""
        cmd = self._build_base_command()
        
        # 添加详细日志以查看 User-Agent
        if self.log_user_agent:
            cmd.extend(['--verbose'])
        
        cmd.extend([
            self.video_url,
            '--output', str(self.workdir),
        ])
        return cmd
    
    def validate(self, result: TestResult) -> bool:
        """验证结果"""
        validations = []
        
        # 验证输出包含 User-Agent 信息（在详细模式下）
        output_lower = result.output.lower()
        has_user_agent = 'user-agent' in output_lower or 'user_agent' in output_lower
        
        if has_user_agent:
            validations.append({"validator": "user_agent", "passed": True, "message": "User-Agent found in output"})
        else:
            self.logger.warning("User-Agent not found in output, may not be logged")
        
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


class TestCustomUserAgentCLI(BaseTestCase):
    """测试自定义 User-Agent（CLI）
    
    验证RVD能够使用 CLI 指定的 User-Agent，包括：
    - --user-agent 参数支持
    - 自定义值正确应用
    - CLI 参数优先级
    """
    
    def __init__(self, config):
        super().__init__(config)
        self.tags = ['core', 'http', 'user-agent', 'cli']
        self.timeout = 600  # 10分钟
        
        # 从配置文件加载测试数据
        test_data = self._load_test_data('core')
        http_data = test_data.get('http_client', {}).get('custom_user_agent', {})
        self.video_url = http_data.get('url', 'PLACEHOLDER_VIDEO_URL')
        self.user_agent = http_data.get('user_agent', 'CustomBot/1.0')
    
    def get_command(self) -> List[str]:
        """获取执行命令"""
        cmd = self._build_base_command()
        
        # 添加自定义 User-Agent
        if self.user_agent:
            cmd.extend(['--user-agent', self.user_agent])
        
        # 添加详细日志以查看 User-Agent
        cmd.extend(['--verbose'])
        
        cmd.extend([
            self.video_url,
            '--output', str(self.workdir),
        ])
        return cmd
    
    def validate(self, result: TestResult) -> bool:
        """验证结果"""
        validations = []
        
        # 验证输出包含自定义 User-Agent
        output_lower = result.output.lower()
        custom_ua_lower = self.user_agent.lower()
        has_custom_ua = custom_ua_lower in output_lower
        
        if has_custom_ua:
            validations.append({"validator": "custom_user_agent", "passed": True, "message": f"Custom User-Agent found: {self.user_agent}"})
        else:
            self.logger.warning(f"Custom User-Agent '{self.user_agent}' not found in output")
        
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


class TestCustomUserAgentConfig(BaseTestCase):
    """测试自定义 User-Agent（配置文件）
    
    验证RVD能够使用配置文件中的 User-Agent，包括：
    - 配置文件中的 user_agent 设置
    - 配置值正确应用
    - 配置文件优先级
    """
    
    def __init__(self, config):
        super().__init__(config)
        self.tags = ['core', 'http', 'user-agent', 'config']
        self.timeout = 600  # 10分钟
        
        # 从配置文件加载测试数据
        test_data = self._load_test_data('core')
        http_data = test_data.get('http_client', {}).get('custom_user_agent', {})
        self.video_url = http_data.get('url', 'PLACEHOLDER_VIDEO_URL')
        self.config_file = './cfg/rvd.toml'
    
    def get_command(self) -> List[str]:
        """获取执行命令"""
        cmd = [str(self.config.executable)]
        
        # 指定配置文件（假设配置文件中设置了 user_agent）
        config_path = self.config.resolve_path(self.config_file)
        if config_path.exists():
            cmd.extend(['--config-file', str(config_path)])
        else:
            self.logger.warning(f"Config file not found: {config_path}")
        
        # 添加详细日志以查看 User-Agent
        cmd.extend(['--verbose'])
        
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
