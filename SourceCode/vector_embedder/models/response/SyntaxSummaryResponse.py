from typing import List, Optional
from models.response.BaseResponse import BaseResponse


class SyntaxSummaryResponse(BaseResponse):
    language: Optional[str] = None
    version: Optional[str] = None
    has_errors: Optional[bool] = None
    ast_text: Optional[List[str]] = None
    chunks: Optional[list] = None
    syntax_summary: Optional[str] = None
