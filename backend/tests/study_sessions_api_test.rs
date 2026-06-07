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
    let value = serde_json::from_slice::<Value>(&bytes).unwrap_or_else(|error| {
        panic!(
            "failed to parse response JSON for {method} {uri}, status={status}, error={error}, body={}",
            String::from_utf8_lossy(&bytes)
        )
    });

    (status, value)
}

async fn register_test_user(prefix: &str) -> String {
    let phone = unique_phone(prefix);
    let (status, body) = json_request(
        test_app().await,
        "POST",
        "/api/v1/auth/register",
        json!({
            "phone": phone,
            "nickname": "session tester",
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

async fn create_room(token: &str, capacity: i32) -> (String, String) {
    let room_name = format!("session-{}", unique_suffix());
    let close_at = (Utc::now() + Duration::hours(2)).to_rfc3339();

    let (status, body) = json_request(
        test_app().await,
        "POST",
        "/api/v1/rooms",
        json!({
            "name": room_name,
            "description": "session integration room",
            "capacity": capacity,
            "isPrivate": false,
            "password": null,
            "closeAt": close_at
        }),
        Some(token),
    )
    .await;

    assert_eq!(status, StatusCode::OK, "{body}");
    let room_id = body["data"]["roomId"].as_str().unwrap().to_string();

    let (status, body) = json_request(
        test_app().await,
        "GET",
        &format!("/api/v1/rooms/{room_id}/seats"),
        json!({}),
        Some(token),
    )
    .await;

    assert_eq!(status, StatusCode::OK, "{body}");
    let seat_id = body["data"][0]["seatId"].as_str().unwrap().to_string();

    (room_id, seat_id)
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

fn unique_suffix() -> String {
    uuid::Uuid::new_v4().simple().to_string()[..8].to_string()
}

#[tokio::test]
async fn user_can_run_study_session_lifecycle() {
    let token = register_test_user("135").await;
    let (room_id, seat_id) = create_room(&token, 4).await;

    let (status, body) = json_request(
        test_app().await,
        "POST",
        "/api/v1/study-sessions",
        json!({
            "roomId": room_id,
            "seatId": seat_id,
            "mode": "pomodoro",
            "studyContent": "复习数据结构"
        }),
        Some(&token),
    )
    .await;

    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(body["data"]["status"], "studying");
    assert_eq!(body["data"]["mode"], "pomodoro");
    assert_eq!(body["data"]["seatCode"], "A01");
    let session_id = body["data"]["sessionId"].as_str().unwrap().to_string();

    let (status, body) = json_request(
        test_app().await,
        "GET",
        "/api/v1/study-sessions/active",
        json!({}),
        Some(&token),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(body["data"]["sessionId"], session_id);

    let (status, _) = json_request(
        test_app().await,
        "POST",
        "/api/v1/study-sessions",
        json!({
            "roomId": room_id,
            "seatId": seat_id,
            "mode": "normal",
            "studyContent": "重复开始"
        }),
        Some(&token),
    )
    .await;
    assert_eq!(status, StatusCode::CONFLICT);

    let (status, body) = json_request(
        test_app().await,
        "POST",
        &format!("/api/v1/study-sessions/{session_id}/heartbeats"),
        json!({
            "clientTime": Utc::now().to_rfc3339()
        }),
        Some(&token),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert!(body["data"]["lastHeartbeatAt"].as_str().is_some());

    let (status, body) = json_request(
        test_app().await,
        "POST",
        &format!("/api/v1/study-sessions/{session_id}/breaks"),
        json!({
            "durationMinutes": 5
        }),
        Some(&token),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(body["data"]["durationMinutes"], 5);
    let break_id = body["data"]["breakId"].as_str().unwrap().to_string();

    let (status, body) = json_request(
        test_app().await,
        "PATCH",
        &format!("/api/v1/study-breaks/{break_id}"),
        json!({
            "extendMinutes": 5
        }),
        Some(&token),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(body["data"]["durationMinutes"], 10);
    assert_eq!(body["data"]["isExtended"], true);

    let (status, body) = json_request(
        test_app().await,
        "PATCH",
        &format!("/api/v1/study-sessions/{session_id}"),
        json!({
            "status": "studying",
            "studyContent": "休息后继续学习"
        }),
        Some(&token),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(body["data"]["status"], "studying");

    let (status, body) = json_request(
        test_app().await,
        "PATCH",
        &format!("/api/v1/study-sessions/{session_id}"),
        json!({
            "status": "ended",
            "studyContent": "结束学习"
        }),
        Some(&token),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(body["data"]["status"], "ended");
    assert_eq!(body["data"]["isValid"], false);

    let (status, body) = json_request(
        test_app().await,
        "GET",
        &format!("/api/v1/rooms/{room_id}/seats"),
        json!({}),
        Some(&token),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(body["data"][0]["status"], "available");

    let (status, _) = json_request(
        test_app().await,
        "PATCH",
        &format!("/api/v1/study-sessions/{session_id}"),
        json!({
            "status": "ended"
        }),
        Some(&token),
    )
    .await;
    assert_eq!(status, StatusCode::CONFLICT);
}

#[tokio::test]
async fn occupied_seat_cannot_be_used_by_another_user() {
    let owner_token = register_test_user("134").await;
    let another_token = register_test_user("133").await;
    let (room_id, seat_id) = create_room(&owner_token, 1).await;

    let (status, body) = json_request(
        test_app().await,
        "POST",
        "/api/v1/study-sessions",
        json!({
            "roomId": room_id,
            "seatId": seat_id,
            "mode": "normal",
            "studyContent": "占用座位"
        }),
        Some(&owner_token),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");

    let (status, _) = json_request(
        test_app().await,
        "POST",
        "/api/v1/study-sessions",
        json!({
            "roomId": room_id,
            "seatId": seat_id,
            "mode": "normal",
            "studyContent": "抢座"
        }),
        Some(&another_token),
    )
    .await;
    assert_eq!(status, StatusCode::CONFLICT);
}

#[tokio::test]
async fn expired_break_auto_recovers_to_studying() {
    let token = register_test_user("127").await;
    let (room_id, seat_id) = create_room(&token, 3).await;

    let (status, session) = json_request(
        test_app().await,
        "POST",
        "/api/v1/study-sessions",
        json!({
            "roomId": room_id,
            "seatId": seat_id,
            "mode": "normal",
            "studyContent": "auto recover break"
        }),
        Some(&token),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{session}");
    let session_id = session["data"]["sessionId"].as_str().unwrap().to_string();

    let (status, break_body) = json_request(
        test_app().await,
        "POST",
        &format!("/api/v1/study-sessions/{session_id}/breaks"),
        json!({
            "durationMinutes": 1
        }),
        Some(&token),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{break_body}");
    let break_id = break_body["data"]["breakId"].as_str().unwrap();

    sqlx::query(
        r#"
        UPDATE study_breaks
        SET start_time = DATE_SUB(NOW(), INTERVAL 2 MINUTE)
        WHERE id = ?
        "#,
    )
    .bind(break_id)
    .execute(&test_pool().await)
    .await
    .unwrap();

    let (status, active) = json_request(
        test_app().await,
        "GET",
        "/api/v1/study-sessions/active",
        json!({}),
        Some(&token),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{active}");
    assert_eq!(active["data"]["sessionId"], session_id);
    assert_eq!(active["data"]["status"], "studying");

    let (status, recovered) = json_request(
        test_app().await,
        "PATCH",
        &format!("/api/v1/study-sessions/{session_id}"),
        json!({
            "status": "studying"
        }),
        Some(&token),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{recovered}");
    assert_eq!(recovered["data"]["status"], "studying");
}

#[tokio::test]
async fn heartbeat_timeout_releases_seat_and_clears_active_session() {
    let token = register_test_user("126").await;
    let (room_id, seat_id) = create_room(&token, 3).await;

    let (status, session) = json_request(
        test_app().await,
        "POST",
        "/api/v1/study-sessions",
        json!({
            "roomId": room_id,
            "seatId": seat_id,
            "mode": "normal",
            "studyContent": "timeout study"
        }),
        Some(&token),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{session}");
    let session_id = session["data"]["sessionId"].as_str().unwrap().to_string();

    sqlx::query(
        r#"
        UPDATE study_sessions
        SET start_time = DATE_SUB(NOW(), INTERVAL 30 MINUTE),
            last_heartbeat_at = DATE_SUB(NOW(), INTERVAL 12 MINUTE)
        WHERE id = ?
        "#,
    )
    .bind(&session_id)
    .execute(&test_pool().await)
    .await
    .unwrap();

    let (status, seats) = json_request(
        test_app().await,
        "GET",
        &format!("/api/v1/rooms/{room_id}/seats"),
        json!({}),
        Some(&token),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{seats}");
    assert_eq!(seats["data"][0]["status"], "available");

    let (status, active) = json_request(
        test_app().await,
        "GET",
        "/api/v1/study-sessions/active",
        json!({}),
        Some(&token),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{active}");
    assert!(active["data"].is_null());

    let (status, heartbeat) = json_request(
        test_app().await,
        "POST",
        &format!("/api/v1/study-sessions/{session_id}/heartbeats"),
        json!({
            "clientTime": Utc::now().to_rfc3339()
        }),
        Some(&token),
    )
    .await;
    assert_eq!(status, StatusCode::CONFLICT, "{heartbeat}");
}
