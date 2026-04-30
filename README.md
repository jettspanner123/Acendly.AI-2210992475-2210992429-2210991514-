# ⚡ Acendly.ai

![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white&labelColor=ce422b&color=ce422b)
![Python](https://img.shields.io/badge/Python-3776AB?style=for-the-badge&logo=python&logoColor=white&labelColor=3776AB&color=3776AB)
![C#](https://img.shields.io/badge/C%23-239120?style=for-the-badge&logo=csharp&logoColor=white&labelColor=239120&color=239120)
![.NET](https://img.shields.io/badge/.NET-512BD4?style=for-the-badge&logo=dotnet&logoColor=white&labelColor=512BD4&color=512BD4)
![PostgreSQL](https://img.shields.io/badge/PostgreSQL-4169E1?style=for-the-badge&logo=postgresql&logoColor=white&labelColor=4169E1&color=4169E1)
![Entity Framework](https://img.shields.io/badge/Entity%20Framework-512BD4?style=for-the-badge&logo=dotnet&logoColor=white&labelColor=7B2FBE&color=7B2FBE)

---

# 🌟 Welcome to Acendly.ai

Acendly.ai is a multi-language, multi-service AI platform designed for rapid development, experimentation, and deployment of advanced AI and data processing workflows.

## ✨ Features
- 🦀 Rust-based AST (abstract syntax tree) / semantic summarization engine
- 🐍 Python vector embedding service
- ⚙️ .NET/C# coordinator for orchestration
- 🌐 Cross-platform support

---

## 📦 Project Structure

```
├── coordinator/           # .NET C# Orchestration Service
├── semantic_summarizer/   # Rust-based Summarizer
├── vector_embedder/       # Python Embedding Service
├── scripts/               # Helper scripts
```

---

## 🖥️ Languages & Frameworks

| Language | Main Frameworks / Libraries |
|----------|-----------------------------|
| 🦀 **Rust** | Cargo, [tree-sitter](https://tree-sitter.github.io/tree-sitter/), [tokio](https://tokio.rs/), [serde](https://serde.rs/) |
| 🐍 **Python** | [FastAPI](https://fastapi.tiangolo.com/), [SQLAlchemy](https://www.sqlalchemy.org/), [Pydantic](https://docs.pydantic.dev/), [pgvector](https://github.com/pgvector/pgvector), [Ollama](https://ollama.com/) |
| ⚙️ **C# (.NET)** | .NET 10, ASP.NET Core, OpenAPI |

---

## 🧩 Technology Usage Details

| Technology | Where it's used (path) | What it does in this repo |
|------------|-------------------------|----------------------------|
| 🦀 **Rust** | `semantic_summarizer/` | Implements the semantic summarization / AST service. |
| 🌲 **tree-sitter** | `semantic_summarizer/src/helpers/ASTServiceHelper.rs` | Parses source text into an AST (`Parser`, `Tree`, `Node`). |
| 📘 **tree-sitter-typescript** | `semantic_summarizer/src/helpers/ASTServiceHelper.rs` | Supplies the TypeScript grammar used during parsing. |
| ⚡ **tokio** | `semantic_summarizer/src/main.rs` | Provides the async runtime for the Rust service entrypoint. |
| 🔧 **serde** | `semantic_summarizer/src/models/response/BaseResponse.rs` | Derives serialization for response models. |
| 📄 **serde_json** | `semantic_summarizer/Cargo.toml` | Declared dependency (no in-code references yet). |
| 🪓 **axum** | `semantic_summarizer/Cargo.toml` | Declared dependency (no in-code references yet). |
| 🐍 **Python** | `vector_embedder/` | Implements the embedding API service. |
| 🚀 **FastAPI** | `vector_embedder/stores/RootApplicationStore.py`, `vector_embedder/controllers/*` | Defines the API application and routers/endpoints. |
| 🔍 **Pydantic** | `vector_embedder/models/request/*`, `vector_embedder/models/response/*` | Request/response DTO validation and typing. |
| 🗄️ **SQLAlchemy** | `vector_embedder/database/*`, `vector_embedder/controllers/EmbeddingController.py` | Database engine config, ORM models, and query/session usage. |
| 🧮 **pgvector** | `vector_embedder/database/entities/OP_EmbeddingTBL.py` | Vector column type used to store embeddings and perform similarity queries. |
| 🤖 **Ollama** | `vector_embedder/services/EmbeddingService.py` | Generates embeddings via `ollama.embed(...)`. |
| ⚙️ **C# / .NET 10** | `coordinator/` | Coordinator/orchestration service skeleton. |
| 🌐 **ASP.NET Core** | `coordinator/Program.cs`, `coordinator/Controllers/ASTController.cs` | Minimal API host + controller scaffolding. |
| 📖 **Microsoft.AspNetCore.OpenApi** | `coordinator/Program.cs` | Adds OpenAPI endpoints in development. |
| 📦 **Node.js (npm scripts)** | `package.json` | Runs dev helper scripts from the repo root. |
| 🔨 **Bash** | `scripts/dev-open.sh`, `scripts/dev-secrets.sh` | Local dev convenience scripts sourced by npm scripts. |

---

## 📁 Key Directories
- ⚙️ **coordinator/**: .NET C# service for orchestration
- 🦀 **semantic_summarizer/**: Rust summarization engine
- 🐍 **vector_embedder/**: Python embedding and AI services
- 🔨 **scripts/**: Utility scripts for development

---

## 🚀 Quick Start

1. **Clone the repo**
   ```bash
   git clone https://github.com/jettspanner123/Acendly.ai.git
   cd Acendly.ai
   ```
2. **Setup each service** (see respective folders for details)
3. **Run and develop!**

---

## 🤝 Contributing

We love contributions! Feel free to open issues, submit PRs, or suggest features.

---

## 🌐 Connect
- 🐙 [GitHub](https://github.com/jettspanner123/Acendly.ai)
- 💬 Issues & Discussions: Use the GitHub repo

---

> _Happy hacking!_ 🚀✨
