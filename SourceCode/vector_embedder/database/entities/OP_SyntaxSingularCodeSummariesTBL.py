import datetime
import uuid

from pgvector.sqlalchemy import Vector
from sqlalchemy import DateTime, Text
from sqlalchemy.orm import Mapped, mapped_column
from sqlalchemy.sql import func

from database.entities.DBase import DBase


class OP_SyntaxSingularCodeSummariesTBL(DBase):
    __tablename__ = "op_syntax_singular_code_summaries"

    def __str__(self):
        return (
            f"OP_SyntaxSingularCodeSummariesTBL("
            f"id={self.id}, "
            f"created_at={self.created_at}"
            f")"
        )

    id: Mapped[uuid.UUID] = mapped_column(primary_key=True, default=uuid.uuid4())

    # The raw plain-text code symbol submitted by the caller
    source_text: Mapped[str] = mapped_column(Text, nullable=False)

    # LLM-generated plain-English summary of what the symbol does
    syntax_summary: Mapped[str] = mapped_column(Text, nullable=False)

    # Vector embedding of the syntax summary for semantic search
    syntax_summary_embedding: Mapped[list] = mapped_column(Vector(768), nullable=False)

    created_at: Mapped[datetime.datetime] = mapped_column(DateTime, server_default=func.now())
    updated_at: Mapped[datetime.datetime] = mapped_column(DateTime, default=func.now(), onupdate=func.now())
