//! HTTP API components for this service.

pub mod config;
pub mod error;
pub mod routes;

use std::sync::Arc;

use axum::{
    Router,
    http::{HeaderName, Method},
};
use tower_http::{
    cors::{Any, CorsLayer},
    request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer},
    trace::TraceLayer,
};

use crate::{config::Config, routes::AppState};

/// Builds the application's HTTP router.
pub fn app(config: &Config) -> Router {
    let state = Arc::new(AppState::default());
    let request_id = HeaderName::from_static("x-request-id");

    let cors = if config.allow_any_origin {
        CorsLayer::new()
            .allow_origin(Any)
            .allow_methods([Method::GET])
    } else {
        CorsLayer::new().allow_methods([Method::GET])
    };

    Router::new()
        .merge(routes::router())
        .with_state(state)
        // Layers are applied bottom-up: CORS must be nearest the router because
        // it needs the router's default response body type.
        .layer(cors)
        .layer(SetRequestIdLayer::new(request_id.clone(), MakeRequestUuid))
        .layer(PropagateRequestIdLayer::new(request_id))
        .layer(TraceLayer::new_for_http())
}
