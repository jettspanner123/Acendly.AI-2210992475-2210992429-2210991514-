import datetime
import uuid

from pgvector.sqlalchemy import Vector
from sqlalchemy import String, DateTime, Boolean, JSON, Text
from sqlalchemy.orm import Mapped, mapped_column
from sqlalchemy.sql import func

from database.entities.DBase import DBase


class OP_SyntaxSummaryTBL(DBase):
    __tablename__ = "op_syntax_summaries"

    def __str__(self):
        return (
            f"OP_SyntaxSummaryTBL("
            f"id={self.id}, "
            f"language={self.language}, "
            f"version={self.version}, "
            f"has_errors={self.has_errors}, "
            f"created_at={self.created_at}"
            f")"
        )

    id: Mapped[uuid.UUID] = mapped_column(primary_key=True, default=uuid.uuid4())

    # Metadata from the parse response
    language: Mapped[str] = mapped_column(String(100), nullable=False)
    version: Mapped[str] = mapped_column(String(50), nullable=False)
    has_errors: Mapped[bool] = mapped_column(Boolean, nullable=False, default=False)

    # Raw ast_text array stored as-is
    ast_text: Mapped[list] = mapped_column(JSON, nullable=False)

    # Chunks array stored as-is
    chunks: Mapped[list] = mapped_column(JSON, nullable=False)

    # LLM-generated plain-English summary of what the code does
    syntax_summary: Mapped[str] = mapped_column(Text, nullable=False)

    # Vector embedding of the syntax summary for semantic search
    syntax_summary_embedding: Mapped[list] = mapped_column(Vector(768), nullable=False)

    created_at: Mapped[datetime.datetime] = mapped_column(DateTime, server_default=func.now())
    updated_at: Mapped[datetime.datetime] = mapped_column(DateTime, default=func.now(), onupdate=func.now())
