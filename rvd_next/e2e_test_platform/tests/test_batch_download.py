"""批量下载测试"""
import yaml
from typing import List

from core.base_test import BaseTestCase, TestResult
from validators.output import OutputValidator
from validators.file import FileValidator


class TestBatchDownload(BaseTestCase):
    """测试批量下载功能
    
    验证RVD能够正确下载批量视频（播放列表、收藏夹等），包括：
    - 识别批量URL
    - 下载多个视频
    - 输出包含批量信息
    """
    
    def __init__(self, config):
        super().__init__(config)
        self.tags = ['batch', 'video']
        self.timeout = 1800  # 30分钟
        
        # 从配置文件加载URL
        urls_file = config.resolve_path(config.get('test_data.urls_file', './datas/urls.yaml'))
        if urls_file.exists():
            with open(urls_file, 'r', encoding='utf-8') as f:
                urls_data = yaml.safe_load(f)
                batch_data = urls_data.get('batch_download', {})
                self.playlist_url = batch_data.get('playlist_url', 'PLACEHOLDER_PLAYLIST_URL')
                self.expected_count = batch_data.get('expected_count', 3)
        else:
            self.playlist_url = "PLACEHOLDER_PLAYLIST_URL"
            self.expected_count = 3
    
    def get_command(self) -> List[str]:
        """获取执行命令"""
        cmd = self._build_base_command()
        cmd.extend([
            self.playlist_url,
            '--output', str(self.workdir),
        ])
        return cmd
    
    def validate(self, result: TestResult) -> bool:
        """验证结果"""
        # 验证输出包含批量下载信息
        if "批量" not in result.output and "batch" not in result.output.lower() and "playlist" not in result.output.lower():
            result.error = "Output does not contain batch download information"
            return False
        
        # 验证文件数量
        video_files = list(self.workdir.glob('**/*.mp4')) + list(self.workdir.glob('**/*.mkv')) + list(self.workdir.glob('**/*.flv'))
        if len(video_files) < self.expected_count:
            result.error = f"Expected at least {self.expected_count} videos, got {len(video_files)}"
            return False
        
        self.logger.info(f"Downloaded {len(video_files)} videos")
        return True
