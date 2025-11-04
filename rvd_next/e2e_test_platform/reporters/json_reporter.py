"""JSON报告器"""
import json
from datetime import datetime
from typing import List

from .base import Reporter
from core.result import TestResult


class JsonReporter(Reporter):
    """JSON报告器"""
    
    def generate(self, results: List[TestResult]) -> str:
        """
        生成JSON报告
        
        Args:
            results: 测试结果列表
            
        Returns:
            JSON格式的报告内容
        """
        report = {
            "timestamp": datetime.now().isoformat(),
            "summary": {
                "total": len(results),
                "passed": sum(1 for r in results if r.passed),
                "failed": sum(1 for r in results if not r.passed),
                "duration": sum(r.duration for r in results)
            },
            "tests": [
                {
                    "name": r.name,
                    "passed": r.passed,
                    "duration": r.duration,
                    "output": r.output,
                    "error": r.error,
                    "artifacts": [str(a) for a in r.artifacts],
                    "validations": r.validations
                }
                for r in results
            ]
        }
        return json.dumps(report, indent=2, ensure_ascii=False)
