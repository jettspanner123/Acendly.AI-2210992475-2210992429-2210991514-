use tree_sitter::{Node, Parser, Tree};
use uuid::Uuid;

use crate::models::response::AstNode::{AstNode, SourceLocation, SourcePosition};
use crate::models::response::AstChunk::{AstChunk, AstChunkType};

/// Parse raw source code into a tree-sitter Tree (TypeScript grammar).
pub fn parse_to_ast(content: &str) -> Tree {
    let mut parser = Parser::new();
    let language = tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into();
    parser.set_language(&language).expect("Error loading TypeScript grammar");
    parser.parse(content, None).expect("Error parsing source code")
}

/// Convert a tree-sitter Tree into a structured AstNode tree.
pub fn build_ast_node(tree: &Tree, source: &[u8]) -> AstNode {
    let root = tree.root_node();
    convert_node(root, source, None)
}

/// Recursively convert a tree-sitter Node into an AstNode.
fn convert_node(node: Node, source: &[u8], parent_id: Option<String>) -> AstNode {
    let id = Uuid::new_v4().to_string();
    let node_type = node.kind().to_string();

    let loc = Some(SourceLocation {
        start: SourcePosition {
            line: node.start_position().row + 1,
            column: node.start_position().column,
        },
        end: SourcePosition {
            line: node.end_position().row + 1,
            column: node.end_position().column,
        },
    });

    // Extract leaf text as value
    let value = if node.child_count() == 0 {
        node.utf8_text(source).ok().map(|t| t.trim().to_string()).filter(|t| !t.is_empty())
    } else {
        None
    };

    // Extract name for named declarations (function, class, variable, etc.)
    let name = extract_name(&node, source);

    // Extract operator for binary/unary expressions
    let operator = extract_operator(&node, source);

    // Recurse into named children only (skip punctuation/syntax tokens)
    let children: Vec<AstNode> = {
        let mut cursor = node.walk();
        node.children(&mut cursor)
            .filter(|child| child.is_named())
            .map(|child| convert_node(child, source, Some(id.clone())))
            .collect()
    };

    AstNode {
        id,
        node_type,
        name,
        operator,
        value,
        children: if children.is_empty() { None } else { Some(children) },
        loc,
        parent_id,
    }
}

/// Try to extract a meaningful name from declaration nodes.
fn extract_name(node: &Node, source: &[u8]) -> Option<String> {
    let named_kinds = [
        "function_declaration",
        "function",
        "class_declaration",
        "class",
        "method_definition",
        "variable_declarator",
        "interface_declaration",
        "type_alias_declaration",
        "enum_declaration",
    ];

    if named_kinds.contains(&node.kind()) {
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            if child.kind() == "identifier" || child.kind() == "type_identifier" {
                return child.utf8_text(source).ok().map(|t| t.to_string());
            }
        }
    }
    None
}

/// Try to extract an operator from binary/unary expression nodes.
fn extract_operator(node: &Node, source: &[u8]) -> Option<String> {
    let operator_kinds = ["binary_expression", "unary_expression", "assignment_expression"];

    if operator_kinds.contains(&node.kind()) {
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            if !child.is_named() {
                // Unnamed nodes are typically operators/punctuation
                if let Ok(text) = child.utf8_text(source) {
                    let t = text.trim().to_string();
                    if !t.is_empty() {
                        return Some(t);
                    }
                }
            }
        }
    }
    None
}

/// Walk the AstNode tree and extract semantic chunks for embedding.
pub fn extract_chunks(node: &AstNode) -> Vec<AstChunk> {
    let mut chunks = Vec::new();
    collect_chunks(node, &mut chunks);
    chunks
}

fn collect_chunks(node: &AstNode, chunks: &mut Vec<AstChunk>) {
    let chunk_type = match node.node_type.as_str() {
        "function_declaration" | "function" | "arrow_function" | "method_definition" => {
            Some(AstChunkType::Function)
        }
        "class_declaration" | "class" => Some(AstChunkType::Class),
        "expression_statement" | "assignment_expression" | "call_expression" => {
            Some(AstChunkType::Expression)
        }
        "if_statement"
        | "for_statement"
        | "while_statement"
        | "return_statement"
        | "variable_declaration" => Some(AstChunkType::Statement),
        _ => None,
    };

    if let Some(chunk_type) = chunk_type {
        let name = node.name.clone();
        let text = build_human_readable_text(node);
        chunks.push(AstChunk {
            id: Uuid::new_v4().to_string(),
            chunk_type,
            name,
            text,
            ast_node_id: node.id.clone(),
        });
    }

    if let Some(children) = &node.children {
        for child in children {
            collect_chunks(child, chunks);
        }
    }
}

/// Build a human-readable description of a node for use in embeddings.
fn build_human_readable_text(node: &AstNode) -> String {
    match node.node_type.as_str() {
        "function_declaration" | "function" | "method_definition" => {
            let name = node.name.as_deref().unwrap_or("anonymous");
            format!("Function '{}' defined at line {}", name, node.loc.as_ref().map_or(0, |l| l.start.line))
        }
        "arrow_function" => {
            format!("Arrow function at line {}", node.loc.as_ref().map_or(0, |l| l.start.line))
        }
        "class_declaration" | "class" => {
            let name = node.name.as_deref().unwrap_or("anonymous");
            format!("Class '{}' defined at line {}", name, node.loc.as_ref().map_or(0, |l| l.start.line))
        }
        "if_statement" => {
            format!("Conditional (if) statement at line {}", node.loc.as_ref().map_or(0, |l| l.start.line))
        }
        "for_statement" => {
            format!("For loop at line {}", node.loc.as_ref().map_or(0, |l| l.start.line))
        }
        "while_statement" => {
            format!("While loop at line {}", node.loc.as_ref().map_or(0, |l| l.start.line))
        }
        "return_statement" => {
            format!("Return statement at line {}", node.loc.as_ref().map_or(0, |l| l.start.line))
        }
        "variable_declaration" => {
            format!("Variable declaration at line {}", node.loc.as_ref().map_or(0, |l| l.start.line))
        }
        "call_expression" => {
            format!("Function call expression at line {}", node.loc.as_ref().map_or(0, |l| l.start.line))
        }
        "expression_statement" | "assignment_expression" => {
            format!("Expression at line {}", node.loc.as_ref().map_or(0, |l| l.start.line))
        }
        _ => format!("{} node at line {}", node.node_type, node.loc.as_ref().map_or(0, |l| l.start.line)),
    }
}

/// Render a tree-sitter Tree as an indented plain-text AST.
///
/// Each line follows the pattern:
///   <indent><node_type> [<start_line>:<start_col> - <end_line>:<end_col>]  (<text>)
///
/// Named nodes are printed normally; anonymous (syntax) nodes are shown in
/// quotes so they are visually distinct.
pub fn render_ast_as_text(tree: &Tree, source: &[u8]) -> String {
    let mut output = String::new();
    render_node(tree.root_node(), source, 0, &mut output);
    output
}

fn render_node(node: Node, source: &[u8], depth: usize, output: &mut String) {
    let indent = "  ".repeat(depth);

    // Show anonymous nodes in quotes to distinguish them from named node types
    let label = if node.is_named() {
        node.kind().to_string()
    } else {
        format!("\"{}\"", node.kind())
    };

    let start = node.start_position();
    let end   = node.end_position();
    let loc   = format!("[{}:{} - {}:{}]", start.row + 1, start.column, end.row + 1, end.column);

    // For leaf nodes attach the source text so the tree is self-contained
    let leaf_text = if node.child_count() == 0 {
        node.utf8_text(source)
            .ok()
            .map(|t| {
                let trimmed = t.trim();
                if trimmed.is_empty() { String::new() } else { format!("  ({})", trimmed) }
            })
            .unwrap_or_default()
    } else {
        String::new()
    };

    output.push_str(&format!("{}{} {}{}\n", indent, label, loc, leaf_text));

    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        render_node(child, source, depth + 1, output);
    }
}
