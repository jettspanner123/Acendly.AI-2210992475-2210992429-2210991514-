import uuid

from fastapi import APIRouter
from sqlalchemy.orm import Session

from database.AlchemyEngineStore import AlchemyEngineStore
from database.entities.OP_SyntaxSummaryTBL import OP_SyntaxSummaryTBL
from database.entities.OP_SyntaxSingularCodeSummariesTBL import OP_SyntaxSingularCodeSummariesTBL
from models.request.AstEmbedRequestDTO import AstEmbedRequestDTO
from models.request.SingularCodeSummaryRequestDTO import SingularCodeSummaryRequestDTO
from models.response.BaseResponse import BaseResponse
from models.response.SyntaxSummaryResponse import SyntaxSummaryResponse
from models.response.SingularCodeSummaryResponse import SingularCodeSummaryResponse
from services.EmbeddingService import EmbeddingService

AI_ROUTER = APIRouter(prefix="/ai", tags=["AI"])
EMBEDDING_SERVICE = EmbeddingService()
ALCHEMY_ENGINE = AlchemyEngineStore.ALCHEMY_ENGINE


class AIController:

    @staticmethod
    @AI_ROUTER.get("/health", response_model=BaseResponse)
    async def health_check():
        return BaseResponse(success=True, message="AI Controller is healthy!")

    @staticmethod
    @AI_ROUTER.post("/summarise", response_model=SyntaxSummaryResponse)
    async def summarise_ast(request: AstEmbedRequestDTO):
        try:
            summary = request.summary
            chunks_as_dicts = [chunk.model_dump() for chunk in summary.chunks]

            # Step 1: generate plain-English syntax summary via qwen2.5-coder
            syntax_summary = EMBEDDING_SERVICE.generate_syntax_summary(
                ast_text=summary.ast_text,
                chunks=chunks_as_dicts,
            )

            # Step 2: embed the summary text for semantic search
            summary_embedding = EMBEDDING_SERVICE.generate_embedding(
                syntax_summary.lower()
            )[0]

            with Session(ALCHEMY_ENGINE) as db:
                new_record = OP_SyntaxSummaryTBL(
                    id=uuid.uuid4(),
                    language=summary.language,
                    version=summary.version,
                    has_errors=summary.has_errors,
                    ast_text=summary.ast_text,
                    chunks=chunks_as_dicts,
                    syntax_summary=syntax_summary,
                    syntax_summary_embedding=summary_embedding,
                )
                db.add(new_record)
                db.commit()

            return SyntaxSummaryResponse(
                success=True,
                message="Syntax summary generated and stored successfully.",
                language=summary.language,
                version=summary.version,
                has_errors=summary.has_errors,
                ast_text=summary.ast_text,
                chunks=chunks_as_dicts,
                syntax_summary=syntax_summary,
            )

        except Exception as e:
            return SyntaxSummaryResponse(
                success=False,
                message=str(e),
            )

    @staticmethod
    @AI_ROUTER.post("/summarise/symbol", response_model=SingularCodeSummaryResponse)
    async def summarise_symbol(request: SingularCodeSummaryRequestDTO):
        try:
            # Step 1: generate plain-English summary of the single code symbol
            syntax_summary = EMBEDDING_SERVICE.generate_singular_summary(request.text)

            # Step 2: embed the summary for semantic search
            summary_embedding = EMBEDDING_SERVICE.generate_embedding(
                syntax_summary.lower()
            )[0]

            with Session(ALCHEMY_ENGINE) as db:
                new_record = OP_SyntaxSingularCodeSummariesTBL(
                    id=uuid.uuid4(),
                    source_text=request.text,
                    syntax_summary=syntax_summary,
                    syntax_summary_embedding=summary_embedding,
                )
                db.add(new_record)
                db.commit()

            return SingularCodeSummaryResponse(
                success=True,
                message="Singular code symbol summary generated and stored successfully.",
                source_text=request.text,
                syntax_summary=syntax_summary,
            )

        except Exception as e:
            return SingularCodeSummaryResponse(
                success=False,
                message=str(e),
            )
