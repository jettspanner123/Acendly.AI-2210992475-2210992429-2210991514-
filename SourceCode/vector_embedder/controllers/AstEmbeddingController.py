import uuid

from fastapi import APIRouter
from sqlalchemy.orm import Session

from database.AlchemyEngineStore import AlchemyEngineStore
from database.entities.OP_AstEmbeddingTBL import OP_AstEmbeddingTBL
from models.request.AstEmbedRequestDTO import AstEmbedRequestDTO
from models.response.AstEmbedResponse import AstEmbedResponse
from models.response.BaseResponse import BaseResponse
from services.EmbeddingService import EmbeddingService

AST_EMBEDDING_ROUTER = APIRouter(prefix="/ast", tags=["AST Embedding"])
EMBEDDING_SERVICE = EmbeddingService()
ALCHEMY_ENGINE = AlchemyEngineStore.ALCHEMY_ENGINE


class AstEmbeddingController:

    @staticmethod
    @AST_EMBEDDING_ROUTER.get("/health", response_model=BaseResponse)
    async def health_check():
        return BaseResponse(success=True, message="AST Embedding Controller is healthy!")

    @staticmethod
    @AST_EMBEDDING_ROUTER.post("/embed", response_model=AstEmbedResponse)
    async def embed_ast(request: AstEmbedRequestDTO):
        try:
            summary = request.summary

            # Joining the Fking Text 
            joined_ast_text = "\n".join(summary.ast_text)
            joined_ast_text_lower = joined_ast_text.lower()

            # Generate the vector embedding
            embedding_vector = EMBEDDING_SERVICE.generate_embedding(joined_ast_text_lower)[0]

            # Serialize chunks to plain dicts for JSON storage
            chunks_as_dicts = [chunk.model_dump() for chunk in summary.chunks]

            with Session(ALCHEMY_ENGINE) as db:
                new_record = OP_AstEmbeddingTBL(
                    id=uuid.uuid4(),
                    language=summary.language,
                    version=summary.version,
                    has_errors=summary.has_errors,
                    ast_text=summary.ast_text,       # stored as-is (JSON array)
                    chunks=chunks_as_dicts,           # stored as-is (JSON array)
                    ast_text_embedding=embedding_vector,  # vector for semantic search
                )
                db.add(new_record)
                db.commit()

            return AstEmbedResponse(
                success=True,
                message="AST embedded and stored successfully.",
                language=summary.language,
                version=summary.version,
                has_errors=summary.has_errors,
                ast_text=summary.ast_text,
                chunks=chunks_as_dicts,
            )

        except Exception as e:
            return AstEmbedResponse(
                success=False,
                message=str(e),
            )
