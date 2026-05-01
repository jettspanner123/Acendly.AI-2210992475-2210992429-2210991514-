use axum::{
    extract::{rejection::JsonRejection, FromRequest, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::post,
    Json, Router,
};
use std::sync::Arc;

use crate::constants::ast_const::AstConstants;
use crate::models::request::AstParseRequest::AstParseRequest;
use crate::models::response::AstParseResponse::AstParseResponse;
use crate::models::response::AstTextParseResponse::AstTextParseResponse;
use crate::services::ast_service::ASTService;

pub struct AppState {
    pub ast_service: ASTService,
}

pub fn ast_router(state: Arc<AppState>) -> Router {
    Router::new()
        .route(AstConstants::PARSE_ROUTE, post(parse_handler))
        .route(AstConstants::PARSE_TEXT_ROUTE, post(parse_text_handler))
        .with_state(state)
}

/// POST /ast/parse
async fn parse_handler(
    State(state): State<Arc<AppState>>,
    payload: Result<Json<AstParseRequest>, JsonRejection>,
) -> Response {
    match payload {
        Ok(Json(request)) => {
            let response = state.ast_service.parse(request);
            (StatusCode::OK, Json(response)).into_response()
        }
        Err(_) => {
            let error_response = AstParseResponse {
                success: false,
                message: "Bad Json Body".to_string(),
                summary: None,
            };
            (StatusCode::BAD_REQUEST, Json(error_response)).into_response()
        }
    }
}

/// POST /ast/parse/text
async fn parse_text_handler(
    State(state): State<Arc<AppState>>,
    payload: Result<Json<AstParseRequest>, JsonRejection>,
) -> Response {
    match payload {
        Ok(Json(request)) => {
            let response = state.ast_service.parse_text(request);
            (StatusCode::OK, Json(response)).into_response()
        }
        Err(_) => {
            let error_response = AstTextParseResponse {
                success: false,
                message: "Bad Json Body".to_string(),
                summary: None,
            };
            (StatusCode::BAD_REQUEST, Json(error_response)).into_response()
        }
    }
}
