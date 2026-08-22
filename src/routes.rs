//! HTTP route handlers.

use std::sync::Arc;

use axum::{
    Json, Router,
    extract::{Query, State},
    http::StatusCode,
    routing::get,
};
use serde::{Deserialize, Serialize};

use crate::error::ApiError;

/// Shared application dependencies.
#[derive(Debug, Default)]
pub struct AppState {
    _private: (),
}

/// Returns the service's routes.
pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/health", get(health))
        .route("/api/v1/greeting", get(greeting))
}

async fn health() -> StatusCode {
    StatusCode::NO_CONTENT
}

async fn greeting(
    State(_state): State<Arc<AppState>>,
    Query(query): Query<GreetingQuery>,
) -> Result<Json<GreetingResponse>, ApiError> {
    let name = query.name.trim();
    if name.is_empty() {
        return Err(ApiError::bad_request("name must not be empty"));
    }
    if name.len() > 100 {
        return Err(ApiError::bad_request("name must be at most 100 bytes"));
    }

    Ok(Json(GreetingResponse {
        message: format!("Hello, {name}!"),
    }))
}

#[derive(Debug, Deserialize)]
struct GreetingQuery {
    name: String,
}

#[derive(Debug, Serialize)]
struct GreetingResponse {
    message: String,
}
