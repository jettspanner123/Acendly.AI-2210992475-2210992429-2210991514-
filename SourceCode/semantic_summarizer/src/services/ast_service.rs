use crate::helpers::ASTServiceHelper::{build_ast_node, extract_chunks, parse_to_ast, render_ast_as_text};
use crate::models::request::AstParseRequest::AstParseRequest;
use crate::models::response::AstParseResponse::{AstParseResponse, AstTransferPayload};
use crate::models::response::AstTextParseResponse::{AstTextParseResponse, AstTextTransferPayload};
use crate::constants::ast_const::AstConstants;

pub struct ASTService {}

impl ASTService {
    pub fn new() -> Self {
        Self {}
    }

    pub fn parse(&self, request: AstParseRequest) -> AstParseResponse {
        let tree = parse_to_ast(&request.source_code);

        let has_errors = tree.root_node().has_error();

        let source_bytes = request.source_code.as_bytes();
        let ast_node = build_ast_node(&tree, source_bytes);
        let chunks = extract_chunks(&ast_node);

        let message = if has_errors {
            "AST parsed with syntax errors. Some nodes may be incomplete or missing.".to_string()
        } else {
            "AST parsed successfully.".to_string()
        };

        let payload = AstTransferPayload {
            language: request.language,
            version: AstConstants::SCHEMA_VERSION.to_string(),
            has_errors,
            ast: ast_node,
            chunks,
        };

        AstParseResponse {
            success: true,
            message,
            summary: Some(payload),
        }
    }

    pub fn parse_text(&self, request: AstParseRequest) -> AstTextParseResponse {
        let tree = parse_to_ast(&request.source_code);

        let has_errors = tree.root_node().has_error();

        let source_bytes = request.source_code.as_bytes();
        let ast_text = render_ast_as_text(&tree, source_bytes)
            .lines()
            .filter(|l| !l.is_empty())
            .map(|l| l.to_string())
            .collect::<Vec<String>>();

        // Reuse the same chunk extraction via the structured AST node
        let ast_node = build_ast_node(&tree, source_bytes);
        let chunks = extract_chunks(&ast_node);

        let message = if has_errors {
            "AST parsed with syntax errors. Some nodes may be incomplete or missing.".to_string()
        } else {
            "AST parsed successfully.".to_string()
        };

        let payload = AstTextTransferPayload {
            language: request.language,
            version: AstConstants::SCHEMA_VERSION.to_string(),
            has_errors,
            ast_text,
            chunks,
        };

        AstTextParseResponse {
            success: true,
            message,
            summary: Some(payload),
        }
    }
}
