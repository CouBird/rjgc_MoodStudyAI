use axum::{
    body::Body,
    http::{Request, StatusCode, header},
};
use backend::{app, config::AppConfig, state::AppState};
use chrono::{Duration, Utc};
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

async fn register_test_user() -> String {
    let phone = unique_phone();
    let (status, body) = json_request(
        test_app().await,
        "POST",
        "/api/v1/auth/register",
        json!({
            "phone": phone,
            "nickname": "emotion tester",
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

async fn create_session(token: &str) -> String {
    let room_name = format!("emotion-{}", unique_suffix());
    let close_at = (Utc::now() + Duration::hours(2)).to_rfc3339();
    let (status, room) = json_request(
        test_app().await,
        "POST",
        "/api/v1/rooms",
        json!({
            "name": room_name,
            "description": "emotion room",
            "capacity": 3,
            "isPrivate": false,
            "password": null,
            "closeAt": close_at
        }),
        Some(token),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{room}");
    let room_id = room["data"]["roomId"].as_str().unwrap();

    let (status, seats) = json_request(
        test_app().await,
        "GET",
        &format!("/api/v1/rooms/{room_id}/seats"),
        json!({}),
        Some(token),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{seats}");
    let seat_id = seats["data"][0]["seatId"].as_str().unwrap();

    let (status, session) = json_request(
        test_app().await,
        "POST",
        "/api/v1/study-sessions",
        json!({
            "roomId": room_id,
            "seatId": seat_id,
            "mode": "normal",
            "studyContent": "emotion study"
        }),
        Some(token),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{session}");
    session["data"]["sessionId"].as_str().unwrap().to_string()
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

    format!("129{digits}")
}

fn unique_suffix() -> String {
    uuid::Uuid::new_v4().simple().to_string()[..8].to_string()
}

#[tokio::test]
async fn user_can_submit_emotion_and_query_trends_and_stats() {
    let token = register_test_user().await;
    let session_id = create_session(&token).await;

    let (status, body) = json_request(
        test_app().await,
        "POST",
        &format!("/api/v1/study-sessions/{session_id}/emotion-records"),
        json!({
            "emotionTag": "焦虑",
            "emotionScore": 8,
            "userNote": "今天复习压力比较大"
        }),
        Some(&token),
    )
    .await;

    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(body["data"]["emotionRecord"]["emotionTag"], "焦虑");
    assert!(
        body["data"]["aiFeedback"]
            .as_str()
            .unwrap()
            .contains("任务")
    );

    let (status, _) = json_request(
        test_app().await,
        "POST",
        &format!("/api/v1/study-sessions/{session_id}/emotion-records"),
        json!({
            "emotionTag": "平静",
            "emotionScore": 5
        }),
        Some(&token),
    )
    .await;
    assert_eq!(status, StatusCode::CONFLICT);

    let today = Utc::now().date_naive();
    let (status, trends) = json_request(
        test_app().await,
        "GET",
        &format!("/api/v1/users/me/emotion-trends?period=week&date={today}"),
        json!({}),
        Some(&token),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{trends}");
    assert_eq!(trends["data"]["period"], "week");
    assert_eq!(trends["data"]["emotionMap"]["焦虑"], 4);
    assert_eq!(trends["data"]["items"][0]["emotionTag"], "焦虑");
    assert_eq!(trends["data"]["items"][0]["emotionValue"], 4);
    assert_eq!(trends["data"]["mainEmotion"], "焦虑");
    assert!(trends["data"]["summary"].as_str().unwrap().contains("焦虑"));
    assert!(
        !trends["data"]["tagDistribution"]
            .as_array()
            .unwrap()
            .is_empty()
    );

    let (status, stats) = json_request(
        test_app().await,
        "GET",
        "/api/v1/users/me/stats?period=week",
        json!({}),
        Some(&token),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{stats}");
    assert_eq!(stats["data"]["period"], "week");
    assert!(stats["data"]["totalMinutes"].as_i64().is_some());
}
