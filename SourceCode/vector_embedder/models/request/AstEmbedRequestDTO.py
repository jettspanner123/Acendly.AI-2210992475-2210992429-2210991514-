from typing import List
from pydantic import BaseModel


class AstChunkDTO(BaseModel):
    id: str
    type: str
    text: str
    ast_node_id: str


class AstSummaryDTO(BaseModel):
    language: str
    version: str
    has_errors: bool
    ast_text: List[str]
    chunks: List[AstChunkDTO]


class AstEmbedRequestDTO(BaseModel):
    success: bool
    message: str
    summary: AstSummaryDTO
