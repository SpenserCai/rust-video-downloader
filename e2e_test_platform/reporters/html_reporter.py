"""HTML报告器"""
from typing import List
from datetime import datetime
import html

from .base import Reporter
from core.result import TestResult


class HtmlReporter(Reporter):
    """HTML报告器"""
    
    def generate(self, results: List[TestResult]) -> str:
        """
        生成HTML报告
        
        Args:
            results: 测试结果列表
            
        Returns:
            HTML格式的报告内容
        """
        total = len(results)
        passed = sum(1 for r in results if r.passed)
        failed = total - passed
        total_duration = sum(r.duration for r in results)
        
        html_content = f"""<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <title>E2E Test Report</title>
    <style>
        body {{
            font-family: Arial, sans-serif;
            margin: 20px;
            background-color: #f5f5f5;
        }}
        .container {{
            max-width: 1200px;
            margin: 0 auto;
            background-color: white;
            padding: 20px;
            border-radius: 8px;
            box-shadow: 0 2px 4px rgba(0,0,0,0.1);
        }}
        h1 {{
            color: #333;
            border-bottom: 2px solid #4CAF50;
            padding-bottom: 10px;
        }}
        .summary {{
            background: #f0f0f0;
            padding: 15px;
            border-radius: 5px;
            margin: 20px 0;
        }}
        .summary-item {{
            display: inline-block;
            margin-right: 30px;
            font-size: 16px;
        }}
        .pass {{
            color: #4CAF50;
            font-weight: bold;
        }}
        .fail {{
            color: #f44336;
            font-weight: bold;
        }}
        .test {{
            margin: 15px 0;
            padding: 15px;
            border: 1px solid #ddd;
            border-radius: 5px;
            background-color: #fafafa;
        }}
        .test-header {{
            font-weight: bold;
            font-size: 16px;
            margin-bottom: 10px;
        }}
        .test-pass {{
            border-left: 4px solid #4CAF50;
        }}
        .test-fail {{
            border-left: 4px solid #f44336;
        }}
        .error {{
            background: #ffe0e0;
            padding: 10px;
            margin-top: 10px;
            border-radius: 3px;
            font-family: monospace;
            font-size: 12px;
            white-space: pre-wrap;
            word-wrap: break-word;
        }}
        .timestamp {{
            color: #666;
            font-size: 14px;
            margin-top: 20px;
        }}
    </style>
</head>
<body>
    <div class="container">
        <h1>E2E Test Report</h1>
        <div class="summary">
            <div class="summary-item">Total: <strong>{total}</strong></div>
            <div class="summary-item"><span class="pass">Passed: {passed}</span></div>
            <div class="summary-item"><span class="fail">Failed: {failed}</span></div>
            <div class="summary-item">Duration: <strong>{total_duration:.2f}s</strong></div>
        </div>
        <div class="tests">
"""
        
        for result in results:
            status_class = "test-pass" if result.passed else "test-fail"
            status_text = "✓ PASS" if result.passed else "✗ FAIL"
            status_color = "pass" if result.passed else "fail"
            
            html_content += f"""
            <div class="test {status_class}">
                <div class="test-header">
                    <span class="{status_color}">{status_text}</span> {html.escape(result.name)} 
                    <span style="color: #666;">({result.duration:.2f}s)</span>
                </div>
"""
            
            if not result.passed and result.error:
                html_content += f'                <div class="error">{html.escape(result.error)}</div>\n'
            
            html_content += "            </div>\n"
        
        html_content += f"""
        </div>
        <div class="timestamp">Generated at: {datetime.now().strftime('%Y-%m-%d %H:%M:%S')}</div>
    </div>
</body>
</html>
"""
        return html_content
