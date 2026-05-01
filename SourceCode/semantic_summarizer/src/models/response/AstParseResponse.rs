use serde::Serialize;
use crate::models::response::AstNode::AstNode;
use crate::models::response::AstChunk::AstChunk;

#[derive(Serialize, Debug)]
pub struct AstTransferPayload {
    pub language: String,
    pub version: String,
    pub has_errors: bool,
    pub ast: AstNode,
    pub chunks: Vec<AstChunk>,
}

#[derive(Serialize, Debug)]
pub struct AstParseResponse {
    pub success: bool,
    pub message: String,
    pub summary: Option<AstTransferPayload>,
}
