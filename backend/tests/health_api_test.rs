use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use backend::{app, config::AppConfig, state::AppState};
use tower::ServiceExt;

#[tokio::test]
async fn health_endpoint_returns_ok() {
    let config = AppConfig::from_env().expect("config should load");
    let state = AppState::new(config).await.expect("state should build");
    let router = app::router(state);

    let response = router
        .oneshot(
            Request::builder()
                .uri("/health")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}
