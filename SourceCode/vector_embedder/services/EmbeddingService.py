from typing import List
import ollama
from constants.EmbeddingCON import EmbeddingCON

class EmbeddingService:

    def generate_embedding(self, text) -> List[List[float]]:
        response = ollama.embed(
            model=EmbeddingCON.EMBEDDING_MODEL,
            input=text
        )
        return response.embeddings
