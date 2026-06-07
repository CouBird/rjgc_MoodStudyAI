use axum::{
    body::Body,
    http::{Request, StatusCode, header},
};
use backend::{app, config::AppConfig, state::AppState};
use chrono::{Duration, Utc};
use http_body_util::BodyExt;
use serde_json::{Value, json};
use sqlx::MySqlPool;
use tower::ServiceExt;

async fn test_app() -> axum::Router {
    let config = AppConfig::from_env().expect("config should load");
    let state = AppState::new(config).await.expect("state should build");
    app::router(state)
}

async fn test_pool() -> MySqlPool {
    let config = AppConfig::from_env().expect("config should load");
    let url = config
        .database
        .url
        .expect("DATABASE_URL should be configured for integration tests");
    MySqlPool::connect(&url).await.expect("db should connect")
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

async fn register_test_user() -> String {
    let phone = unique_phone();
    let (status, body) = json_request(
        test_app().await,
        "POST",
        "/api/v1/auth/register",
        json!({
            "phone": phone,
            "nickname": "room tester",
            "password": "abc123456",
            "confirmPassword": "abc123456",
            "agreeTerms": true,
            "agreePrivacy": true
        }),
        None,
    )
    .await;

    assert_eq!(status, StatusCode::OK, "{body}");
    body["data"]["token"].as_str().unwrap().to_string()
}

fn unique_phone() -> String {
    let digits: String = uuid::Uuid::new_v4()
        .simple()
        .to_string()
        .chars()
        .filter(|ch| ch.is_ascii_hexdigit())
        .map(|ch| ((ch as u8) % 10 + b'0') as char)
        .take(8)
        .collect();

    format!("136{digits}")
}

fn unique_suffix() -> String {
    uuid::Uuid::new_v4().simple().to_string()[..8].to_string()
}

#[tokio::test]
async fn user_can_list_rooms() {
    let token = register_test_user().await;

    let (status, body) = json_request(
        test_app().await,
        "GET",
        "/api/v1/rooms?page=1&pageSize=10",
        json!({}),
        Some(&token),
    )
    .await;

    assert_eq!(status, StatusCode::OK, "{body}");
    assert!(body["data"]["items"].as_array().unwrap().len() >= 2);
    assert!(body["data"]["total"].as_u64().unwrap() >= 2);
}

#[tokio::test]
async fn user_can_create_room_and_fetch_detail_with_initialized_seats() {
    let token = register_test_user().await;
    let room_name = format!("room-{}", unique_suffix());
    let close_at = (Utc::now() + Duration::hours(2)).to_rfc3339();

    let (status, body) = json_request(
        test_app().await,
        "POST",
        "/api/v1/rooms",
        json!({
            "name": room_name,
            "description": "integration room",
            "capacity": 6,
            "isPrivate": false,
            "password": null,
            "closeAt": close_at
        }),
        Some(&token),
    )
    .await;

    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(body["data"]["name"], room_name);
    assert_eq!(body["data"]["capacity"], 6);
    assert_eq!(body["data"]["currentMembers"], 0);
    let room_id = body["data"]["roomId"].as_str().unwrap();

    let (status, body) = json_request(
        test_app().await,
        "GET",
        &format!("/api/v1/rooms/{room_id}"),
        json!({}),
        Some(&token),
    )
    .await;

    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(body["data"]["room"]["name"], room_name);
    assert_eq!(body["data"]["seats"].as_array().unwrap().len(), 6);
    assert_eq!(body["data"]["seats"][0]["seatCode"], "A01");
    assert_eq!(body["data"]["seats"][0]["status"], "available");
    assert_eq!(body["data"]["members"].as_array().unwrap().len(), 0);

    let (status, body) = json_request(
        test_app().await,
        "GET",
        &format!("/api/v1/rooms/{room_id}/seats"),
        json!({}),
        Some(&token),
    )
    .await;

    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(body["data"].as_array().unwrap().len(), 6);
    assert_eq!(body["data"][0]["seatCode"], "A01");
    assert_eq!(body["data"][0]["status"], "available");
}

#[tokio::test]
async fn rooms_list_filters_expired_open_rooms_by_default() {
    let token = register_test_user().await;
    let room_name = format!("expired-{}", unique_suffix());
    let close_at = (Utc::now() + Duration::hours(2)).to_rfc3339();

    let (status, body) = json_request(
        test_app().await,
        "POST",
        "/api/v1/rooms",
        json!({
            "name": room_name,
            "description": "expired room",
            "capacity": 3,
            "isPrivate": false,
            "password": null,
            "closeAt": close_at
        }),
        Some(&token),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");
    let room_id = body["data"]["roomId"].as_str().unwrap();

    sqlx::query("UPDATE study_rooms SET close_at = DATE_SUB(NOW(), INTERVAL 1 HOUR) WHERE id = ?")
        .bind(room_id)
        .execute(&test_pool().await)
        .await
        .unwrap();

    let (status, body) = json_request(
        test_app().await,
        "GET",
        &format!("/api/v1/rooms?keyword={room_name}&page=1&pageSize=10"),
        json!({}),
        Some(&token),
    )
    .await;

    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(body["data"]["total"], 0);
    assert!(body["data"]["items"].as_array().unwrap().is_empty());
}

#[tokio::test]
async fn creating_room_with_duplicate_name_returns_conflict() {
    let token = register_test_user().await;
    let room_name = format!("dup-{}", unique_suffix());
    let close_at = (Utc::now() + Duration::hours(2)).to_rfc3339();
    let payload = json!({
        "name": room_name,
        "description": "duplicate room",
        "capacity": 3,
        "isPrivate": false,
        "password": null,
        "closeAt": close_at
    });

    let (status, body) = json_request(
        test_app().await,
        "POST",
        "/api/v1/rooms",
        payload.clone(),
        Some(&token),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");

    let (status, _) = json_request(
        test_app().await,
        "POST",
        "/api/v1/rooms",
        payload,
        Some(&token),
    )
    .await;
    assert_eq!(status, StatusCode::CONFLICT);
}

#[tokio::test]
async fn unauthenticated_room_access_returns_unauthorized() {
    let (status, _) = json_request(test_app().await, "GET", "/api/v1/rooms", json!({}), None).await;

    assert_eq!(status, StatusCode::UNAUTHORIZED);
}
