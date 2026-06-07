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
            "nickname": "stats tester",
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
    let room_name = format!("stats-{}", unique_suffix());
    let close_at = (Utc::now() + Duration::hours(2)).to_rfc3339();
    let (status, room) = json_request(
        test_app().await,
        "POST",
        "/api/v1/rooms",
        json!({
            "name": room_name,
            "description": "stats room",
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
            "studyContent": "stats study"
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

    format!("132{digits}")
}

fn unique_suffix() -> String {
    uuid::Uuid::new_v4().simple().to_string()[..8].to_string()
}

#[tokio::test]
async fn user_can_fetch_today_home_stats() {
    let token = register_test_user().await;

    let (status, body) = json_request(
        test_app().await,
        "GET",
        "/api/v1/users/me/stats/today",
        json!({}),
        Some(&token),
    )
    .await;

    assert_eq!(status, StatusCode::OK, "{body}");
    assert!(body["data"]["todayMinutes"].as_i64().is_some());
    assert!(body["data"]["todayHours"].as_f64().is_some());
    assert!(body["data"]["streakDays"].as_i64().is_some());
    assert!(body["data"]["todayCheckin"].as_bool().is_some());
    assert!(body["data"]["validCheckin"].as_bool().is_some());
    assert!(body["data"].get("latestEmotion").is_some());
    assert!(body["data"].get("mood").is_some());
}

#[tokio::test]
async fn user_can_fetch_documented_week_stats() {
    let token = register_test_user().await;
    let session_id = create_room_and_start_session(&token).await;
    let ended_at = (Utc::now() + Duration::minutes(12)).to_rfc3339();

    let (status, ended) = json_request(
        test_app().await,
        "PATCH",
        &format!("/api/v1/study-sessions/{session_id}"),
        json!({
            "status": "ended",
            "studyContent": "stats end",
            "endedAt": ended_at
        }),
        Some(&token),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{ended}");
    assert_eq!(ended["data"]["isValid"], true);

    let (status, emotion) = json_request(
        test_app().await,
        "POST",
        &format!("/api/v1/study-sessions/{session_id}/emotion-records"),
        json!({
            "emotionTag": "平静",
            "emotionScore": 7,
            "userNote": "stats test"
        }),
        Some(&token),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{emotion}");

    let today = Utc::now().date_naive();
    let (status, stats) = json_request(
        test_app().await,
        "GET",
        &format!("/api/v1/users/me/stats?period=week&date={today}"),
        json!({}),
        Some(&token),
    )
    .await;

    assert_eq!(status, StatusCode::OK, "{stats}");
    assert_eq!(stats["data"]["period"], "week");
    assert_eq!(stats["data"]["studyTrend"].as_array().unwrap().len(), 7);
    assert!(stats["data"]["totalMinutes"].as_i64().unwrap() >= 10);
    assert!(stats["data"]["totalHours"].as_f64().unwrap() > 0.0);
    assert_eq!(stats["data"]["previousTotalMinutes"], 0);
    assert!(stats["data"]["previousTotalHours"].as_f64().is_some());
    assert!(stats["data"]["totalMinutesChange"].as_i64().unwrap() >= 10);
    assert_eq!(stats["data"]["totalHoursGrowthPercent"], 100.0);
    assert!(stats["data"]["averageDailyHours"].as_f64().is_some());
    assert_eq!(stats["data"]["previousAverageDailyMinutes"], 0);
    assert!(stats["data"]["averageDailyMinutesChange"].as_i64().unwrap() >= 1);
    assert_eq!(stats["data"]["averageDailyHoursGrowthPercent"], 100.0);
    assert!(stats["data"]["studyDays"].as_i64().unwrap() >= 1);
    assert_eq!(stats["data"]["previousStudyDays"], 0);
    assert!(stats["data"]["studyDaysChange"].as_i64().unwrap() >= 1);
    assert_eq!(stats["data"]["studyDaysGrowthPercent"], 100.0);
    assert!(stats["data"]["streakDays"].as_i64().unwrap() >= 1);
    assert!(stats["data"]["validSessionCount"].as_i64().unwrap() >= 1);
    assert_eq!(stats["data"]["previousValidSessionCount"], 0);
    assert!(stats["data"]["validSessionCountChange"].as_i64().unwrap() >= 1);
    assert_eq!(stats["data"]["validSessionCountGrowthPercent"], 100.0);
    assert!(stats["data"]["checkinCount"].as_i64().unwrap() >= 1);
    assert_eq!(stats["data"]["previousCheckinCount"], 0);
    assert!(stats["data"]["checkinCountChange"].as_i64().unwrap() >= 1);
    assert_eq!(stats["data"]["checkinCountGrowthPercent"], 100.0);
    assert!(!stats["data"]["emotionTrend"].as_array().unwrap().is_empty());
    assert_eq!(stats["data"]["mainEmotion"], "平静");
    assert!(stats["data"]["summary"].as_str().unwrap().contains("平静"));
    assert!(stats["data"]["trends"].as_array().is_some());
}
