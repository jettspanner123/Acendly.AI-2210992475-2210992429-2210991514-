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

    def generate_syntax_summary(self, ast_text: List[str], chunks: List[dict]) -> str:
        # Build a structured prompt so the model focuses on what matters:
        # function signatures, inputs, outputs, and overall behaviour.
        ast_block = "\n".join(ast_text)
        chunks_block = "\n".join(
            f"- [{c.get('type', 'unknown')}] {c.get('text', '')}"
            for c in chunks
        )

        prompt = (
            "You are a code analysis assistant. "
            "Given the AST (Abstract Syntax Tree) text and semantic chunks of a source file, "
            "produce a concise technical summary that covers:\n"
            "1. What the code does overall\n"
            "2. Each function or method: its name, input parameters, and return value\n"
            "3. Any classes defined: their purpose and key methods\n"
            "4. Notable expressions or statements\n\n"
            "AST Text:\n"
            f"{ast_block}\n\n"
            "Semantic Chunks:\n"
            f"{chunks_block}\n\n"
            "Write the summary in plain English. Be concise and technical."
        )

        response = ollama.chat(
            model=EmbeddingCON.SUMMARY_MODEL,
            messages=[{"role": "user", "content": prompt}]
        )
        return response.message.content.strip()

    def generate_singular_summary(self, text: str) -> str:
        prompt = (
            "You are a code analysis assistant. "
            "Given a single code symbol (a function, method, constant, class, or similar), "
            "produce a concise technical summary that covers:\n"
            "1. What this symbol does\n"
            "2. Input parameters: name, type, and purpose of each\n"
            "3. Return value: type and what it represents\n"
            "4. Any side effects or notable behaviour\n\n"
            "Code:\n"
            f"{text}\n\n"
            "Write the summary in plain English. Be concise and technical."
        )

        response = ollama.chat(
            model=EmbeddingCON.SUMMARY_MODEL,
            messages=[{"role": "user", "content": prompt}]
        )
        return response.message.content.strip()
