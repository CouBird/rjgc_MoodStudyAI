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

async fn register_test_user(prefix: &str) -> String {
    let phone = unique_phone(prefix);
    let (status, body) = json_request(
        test_app().await,
        "POST",
        "/api/v1/auth/register",
        json!({
            "phone": phone,
            "nickname": "checkin tester",
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

async fn create_room_and_start_session(token: &str) -> String {
    let room_name = format!("checkin-{}", unique_suffix());
    let close_at = (Utc::now() + Duration::hours(2)).to_rfc3339();
    let (status, room) = json_request(
        test_app().await,
        "POST",
        "/api/v1/rooms",
        json!({
            "name": room_name,
            "description": "checkin room",
            "capacity": 4,
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
            "studyContent": "valid checkin study"
        }),
        Some(token),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{session}");
    session["data"]["sessionId"].as_str().unwrap().to_string()
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
async fn user_can_create_makeup_checkin_and_query_calendar_detail() {
    let token = register_test_user("131").await;
    let date = (Utc::now().date_naive() - Duration::days(1)).to_string();
    let month = &date[..7];

    let (status, body) = json_request(
        test_app().await,
        "POST",
        "/api/v1/checkins",
        json!({
            "date": date,
            "totalMinutes": 45,
            "makeupReason": "昨天忘记提交",
            "summaryNote": "补充学习记录"
        }),
        Some(&token),
    )
    .await;

    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(body["data"]["totalMinutes"], 45);
    assert_eq!(body["data"]["isMakeup"], true);

    let (status, detail) = json_request(
        test_app().await,
        "GET",
        &format!("/api/v1/checkins/{date}"),
        json!({}),
        Some(&token),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{detail}");
    assert_eq!(detail["data"]["date"], date);

    let (status, calendar) = json_request(
        test_app().await,
        "GET",
        &format!("/api/v1/checkins?month={month}"),
        json!({}),
        Some(&token),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{calendar}");
    assert!(calendar["data"]["days"].as_array().unwrap().len() >= 28);

    let (status, _) = json_request(
        test_app().await,
        "POST",
        "/api/v1/checkins",
        json!({
            "date": date,
            "totalMinutes": 30,
            "makeupReason": "重复补卡"
        }),
        Some(&token),
    )
    .await;
    assert_eq!(status, StatusCode::CONFLICT);
}

#[tokio::test]
async fn streak_uses_yesterday_when_today_has_no_checkin() {
    let token = register_test_user("129").await;
    let yesterday = (Utc::now().date_naive() - Duration::days(1)).to_string();

    let (status, makeup) = json_request(
        test_app().await,
        "POST",
        "/api/v1/checkins",
        json!({
            "date": yesterday,
            "totalMinutes": 45,
            "makeupReason": "补昨天",
            "summaryNote": "今天未打卡时仍显示截至昨天的连续天数"
        }),
        Some(&token),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{makeup}");

    let (status, today_stats) = json_request(
        test_app().await,
        "GET",
        "/api/v1/users/me/stats/today",
        json!({}),
        Some(&token),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{today_stats}");
    assert_eq!(today_stats["data"]["todayCheckin"], false);
    assert_eq!(today_stats["data"]["streakDays"], 1);

    let (status, me) = json_request(
        test_app().await,
        "GET",
        "/api/v1/users/me",
        json!({}),
        Some(&token),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{me}");
    assert_eq!(me["data"]["streakDays"], 1);
}

#[tokio::test]
async fn valid_study_session_end_updates_today_checkin() {
    let token = register_test_user("130").await;
    let session_id = create_room_and_start_session(&token).await;
    let ended_at = (Utc::now() + Duration::minutes(12)).to_rfc3339();

    let (status, ended) = json_request(
        test_app().await,
        "PATCH",
        &format!("/api/v1/study-sessions/{session_id}"),
        json!({
            "status": "ended",
            "studyContent": "valid end",
            "endedAt": ended_at
        }),
        Some(&token),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{ended}");
    assert_eq!(ended["data"]["isValid"], true);
    assert!(ended["data"]["durationMinutes"].as_i64().unwrap() >= 10);

    let (status, emotion) = json_request(
        test_app().await,
        "POST",
        &format!("/api/v1/study-sessions/{session_id}/emotion-records"),
        json!({
            "emotionTag": "满足",
            "emotionScore": 8,
            "userNote": "结束学习后记录情绪"
        }),
        Some(&token),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{emotion}");

    let today = Utc::now().date_naive().to_string();
    let (status, detail) = json_request(
        test_app().await,
        "GET",
        &format!("/api/v1/checkins/{today}"),
        json!({}),
        Some(&token),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{detail}");
    assert!(detail["data"]["totalMinutes"].as_i64().unwrap() >= 10);
    assert_eq!(detail["data"]["isMakeup"], false);
    assert_eq!(detail["data"]["emotionRecord"]["emotionTag"], "满足");
    assert_eq!(
        detail["data"]["emotionRecord"]["userNote"],
        "结束学习后记录情绪"
    );
    assert!(
        detail["data"]["emotionRecord"]["aiFeedback"]
            .as_str()
            .unwrap()
            .contains("满足")
    );
}

#[tokio::test]
async fn makeup_checkin_recalculates_cached_streak_days() {
    let token = register_test_user("128").await;
    let session_id = create_room_and_start_session(&token).await;
    let ended_at = (Utc::now() + Duration::minutes(12)).to_rfc3339();

    let (status, ended) = json_request(
        test_app().await,
        "PATCH",
        &format!("/api/v1/study-sessions/{session_id}"),
        json!({
            "status": "ended",
            "studyContent": "today valid study",
            "endedAt": ended_at
        }),
        Some(&token),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{ended}");

    let (status, me) = json_request(
        test_app().await,
        "GET",
        "/api/v1/users/me",
        json!({}),
        Some(&token),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{me}");
    let user_id = me["data"]["userId"]
        .as_str()
        .unwrap()
        .parse::<i64>()
        .unwrap();

    let yesterday = (Utc::now().date_naive() - Duration::days(1)).to_string();
    let (status, makeup) = json_request(
        test_app().await,
        "POST",
        "/api/v1/checkins",
        json!({
            "date": yesterday,
            "totalMinutes": 30,
            "makeupReason": "补齐昨天",
            "summaryNote": "补卡后应刷新连续天数"
        }),
        Some(&token),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{makeup}");

    let streak_days = sqlx::query_scalar::<_, i32>("SELECT streak_days FROM users WHERE id = ?")
        .bind(user_id)
        .fetch_one(&test_pool().await)
        .await
        .unwrap();

    assert_eq!(streak_days, 2);
}
