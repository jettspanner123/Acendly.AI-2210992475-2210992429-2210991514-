use serde::Serialize;
use crate::models::response::AstChunk::AstChunk;

#[derive(Serialize, Debug)]
pub struct AstTextTransferPayload {
    pub language: String,
    pub version: String,
    pub has_errors: bool,
    /// Indented plain-text representation of the AST as a line-by-line array
    pub ast_text: Vec<String>,
    pub chunks: Vec<AstChunk>,
}

#[derive(Serialize, Debug)]
pub struct AstTextParseResponse {
    pub success: bool,
    pub message: String,
    pub summary: Option<AstTextTransferPayload>,
}
