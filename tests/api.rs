//! End-to-end checks of the public HTTP interface.

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use http_body_util::BodyExt;
use rust_starter::{app, config::Config};
use tower::ServiceExt;

fn config() -> Config {
    Config {
        bind_addr: "127.0.0.1:0".parse().expect("valid test address"),
        allow_any_origin: false,
    }
}

#[tokio::test]
async fn health_is_ready() {
    let response = app(&config())
        .oneshot(
            Request::builder()
                .uri("/health")
                .body(Body::empty())
                .expect("valid request"),
        )
        .await
        .expect("router should respond");
    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}

#[tokio::test]
async fn greeting_returns_json() {
    let response = app(&config())
        .oneshot(
            Request::builder()
                .uri("/api/v1/greeting?name=Ada")
                .body(Body::empty())
                .expect("valid request"),
        )
        .await
        .expect("router should respond");
    assert_eq!(response.status(), StatusCode::OK);

    let body = response
        .into_body()
        .collect()
        .await
        .expect("body should be readable")
        .to_bytes();
    assert_eq!(body.as_ref(), br#"{"message":"Hello, Ada!"}"#);
}

#[tokio::test]
async fn greeting_rejects_empty_name() {
    let response = app(&config())
        .oneshot(
            Request::builder()
                .uri("/api/v1/greeting?name=%20%20")
                .body(Body::empty())
                .expect("valid request"),
        )
        .await
        .expect("router should respond");
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}
