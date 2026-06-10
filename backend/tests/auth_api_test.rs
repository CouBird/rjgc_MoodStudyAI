use axum::{
    body::Body,
    http::{Request, StatusCode, header},
};
use backend::{app, config::AppConfig, state::AppState};
use http_body_util::BodyExt;
use serde_json::{Value, json};
use tower::ServiceExt;

async fn test_app() -> axum::Router {
    let config = AppConfig::from_env().expect("config should load");
    let state = AppState::new(config).await.expect("state should build");
    app::router(state)
}

async fn json_request(
    app: axum::Router,
    method: &str,
    uri: &str,
    body: Value,
    token: Option<&str>,
) -> (StatusCode, Value) {
    let mut builder = Request::builder()
        .method(method)
        .uri(uri)
        .header(header::CONTENT_TYPE, "application/json");

    if let Some(token) = token {
        builder = builder.header(header::AUTHORIZATION, format!("Bearer {token}"));
    }

    let response = app
        .oneshot(builder.body(Body::from(body.to_string())).unwrap())
        .await
        .unwrap();

    let status = response.status();
    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let value = serde_json::from_slice::<Value>(&bytes).unwrap();

    (status, value)
}

#[tokio::test]
async fn user_can_register_login_and_fetch_me() {
    let phone = unique_phone("139");

    let (status, body) = json_request(
        test_app().await,
        "POST",
        "/api/v1/auth/register",
        json!({
            "phone": phone,
            "nickname": "测试用户",
            "password": "abc123456",
            "confirmPassword": "abc123456",
            "agreeTerms": true,
            "agreePrivacy": true
        }),
        None,
    )
    .await;

    assert_eq!(status, StatusCode::OK, "{body}");
    let token = body["data"]["token"].as_str().unwrap().to_string();
    assert_eq!(body["data"]["user"]["phone"], phone);

    let (status, body) = json_request(
        test_app().await,
        "POST",
        "/api/v1/auth/login",
        json!({
            "phone": phone,
            "password": "abc123456"
        }),
        None,
    )
    .await;

    assert_eq!(status, StatusCode::OK, "{body}");

    let (status, body) = json_request(
        test_app().await,
        "GET",
        "/api/v1/users/me",
        json!({}),
        Some(&token),
    )
    .await;

    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(body["data"]["phone"], phone);
    assert_eq!(body["data"]["profile"], Value::Null);
    assert!(body["data"]["streakDays"].as_i64().is_some());
}

#[tokio::test]
async fn duplicate_register_returns_conflict() {
    let phone = unique_phone("137");
    let payload = json!({
        "phone": phone,
        "nickname": "重复用户",
        "password": "abc123456",
        "confirmPassword": "abc123456",
        "agreeTerms": true,
        "agreePrivacy": true
    });

    let (status, body) = json_request(
        test_app().await,
        "POST",
        "/api/v1/auth/register",
        payload.clone(),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");

    let (status, _) = json_request(
        test_app().await,
        "POST",
        "/api/v1/auth/register",
        payload,
        None,
    )
    .await;
    assert_eq!(status, StatusCode::CONFLICT);
}

#[tokio::test]
async fn user_can_update_profile_and_change_password() {
    let phone = unique_phone("136");

    let (status, body) = json_request(
        test_app().await,
        "POST",
        "/api/v1/auth/register",
        json!({
            "phone": phone,
            "nickname": "资料用户",
            "password": "abc123456",
            "confirmPassword": "abc123456",
            "agreeTerms": true,
            "agreePrivacy": true
        }),
        None,
    )
    .await;

    assert_eq!(status, StatusCode::OK, "{body}");
    let token = body["data"]["token"].as_str().unwrap().to_string();

    let (status, body) = json_request(
        test_app().await,
        "PATCH",
        "/api/v1/users/me",
        json!({
            "nickname": "新的昵称",
            "profile": "考研党一枚，目标是上岸理想的大学！"
        }),
        Some(&token),
    )
    .await;

    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(body["data"]["nickname"], "新的昵称");
    assert_eq!(
        body["data"]["profile"],
        "考研党一枚，目标是上岸理想的大学！"
    );

    let (status, body) = json_request(
        test_app().await,
        "PATCH",
        "/api/v1/users/me/password",
        json!({
            "currentPassword": "abc123456",
            "newPassword": "newabc123456",
            "confirmPassword": "newabc123456"
        }),
        Some(&token),
    )
    .await;

    assert_eq!(status, StatusCode::OK, "{body}");

    let (status, _) = json_request(
        test_app().await,
        "POST",
        "/api/v1/auth/login",
        json!({
            "phone": phone,
            "password": "abc123456"
        }),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);

    let (status, body) = json_request(
        test_app().await,
        "POST",
        "/api/v1/auth/login",
        json!({
            "phone": phone,
            "password": "newabc123456"
        }),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");
}

fn unique_phone(prefix: &str) -> String {
    let digits: String = uuid::Uuid::new_v4()
        .simple()
        .to_string()
        .chars()
        .filter(|ch| ch.is_ascii_hexdigit())
        .map(|ch| ((ch as u8) % 10 + b'0') as char)
        .take(8)
        .collect();

    format!("{prefix}{digits}")
}

#[tokio::test]
async fn admin_can_login() {
    let (status, body) = json_request(
        test_app().await,
        "POST",
        "/api/v1/admin/auth/login",
        json!({
            "account": "admin",
            "password": "admin123"
        }),
        None,
    )
    .await;

    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(body["data"]["admin"]["adminName"], "admin");
    assert!(body["data"]["adminToken"].as_str().is_some());
}
