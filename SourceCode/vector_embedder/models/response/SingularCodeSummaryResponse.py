from typing import Optional
from models.response.BaseResponse import BaseResponse


class SingularCodeSummaryResponse(BaseResponse):
    source_text: Optional[str] = None
    syntax_summary: Optional[str] = None
