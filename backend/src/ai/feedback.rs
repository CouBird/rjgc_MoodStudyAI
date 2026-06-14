use serde::{Deserialize, Serialize};
use std::time::Duration;

use crate::config::AiConfig;

const DEFAULT_QWEN_BASE_URL: &str = "https://dashscope.aliyuncs.com/compatible-mode/v1";
const DEFAULT_QWEN_MODEL: &str = "qwen-math-turbo";

pub struct EmotionFeedbackInput<'a> {
    pub emotion_tag: &'a str,
    pub emotion_score: i32,
    pub user_note: Option<&'a str>,
    pub study_content: Option<&'a str>,
    pub duration_minutes: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct AiFeedbackPayload {
    comfort_text: String,
    study_advice: String,
    relax_advice: String,
}

#[derive(Serialize)]
struct ChatMessage<'a> {
    role: &'a str,
    content: String,
}

#[derive(Serialize)]
struct ChatCompletionRequest<'a> {
    model: &'a str,
    messages: Vec<ChatMessage<'a>>,
    temperature: f32,
}

#[derive(Deserialize)]
struct ChatCompletionResponse {
    choices: Vec<ChatChoice>,
}

#[derive(Deserialize)]
struct ChatChoice {
    message: ChatChoiceMessage,
}

#[derive(Deserialize)]
struct ChatChoiceMessage {
    content: String,
}

pub fn generate_template_feedback(emotion_tag: &str) -> String {
    template_feedback(EmotionFeedbackInput {
        emotion_tag,
        emotion_score: 5,
        user_note: None,
        study_content: None,
        duration_minutes: 0,
    })
}

pub async fn generate_emotion_feedback(
    config: &AiConfig,
    input: EmotionFeedbackInput<'_>,
) -> String {
    match call_qwen_feedback(config, &input).await {
        Ok(payload) => payload_to_string(payload),
        Err(error) => {
            tracing::warn!(%error, "AI feedback generation failed; using template fallback");
            template_feedback(input)
        }
    }
}

async fn call_qwen_feedback(
    config: &AiConfig,
    input: &EmotionFeedbackInput<'_>,
) -> Result<AiFeedbackPayload, String> {
    let api_key = config
        .api_key
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .ok_or_else(|| "AI_API_KEY is not configured".to_string())?;
    let base_url = config
        .api_base_url
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .unwrap_or(DEFAULT_QWEN_BASE_URL)
        .trim_end_matches('/');
    let model = config
        .model
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .unwrap_or(DEFAULT_QWEN_MODEL);
    let url = format!("{base_url}/chat/completions");

    let request = ChatCompletionRequest {
        model,
        temperature: 0.7,
        messages: vec![
            ChatMessage {
                role: "system",
                content: "You are a warm Chinese study companion. Return ONLY valid JSON with keys comfortText, studyAdvice, relaxAdvice. Values must be concise Simplified Chinese strings. Do not include markdown.".to_string(),
            },
            ChatMessage {
                role: "user",
                content: build_prompt(input),
            },
        ],
    };

    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(12))
        .build()
        .map_err(|error| error.to_string())?;
    let response = client
        .post(url)
        .bearer_auth(api_key)
        .json(&request)
        .send()
        .await
        .map_err(|error| error.to_string())?;

    let status = response.status();
    let body = response.text().await.map_err(|error| error.to_string())?;
    if !status.is_success() {
        return Err(format!("Qwen API returned {status}: {body}"));
    }

    let parsed: ChatCompletionResponse =
        serde_json::from_str(&body).map_err(|error| format!("invalid Qwen response: {error}"))?;
    let content = parsed
        .choices
        .into_iter()
        .next()
        .map(|choice| choice.message.content)
        .ok_or_else(|| "Qwen response has no choices".to_string())?;

    parse_ai_content(&content)
}

fn build_prompt(input: &EmotionFeedbackInput<'_>) -> String {
    format!(
        "Study session summary:\n- emotionTag: {}\n- emotionScore: {}\n- durationMinutes: {}\n- studyContent: {}\n- userNote: {}\nPlease generate supportive feedback for a student after a study session. JSON only.",
        input.emotion_tag,
        input.emotion_score,
        input.duration_minutes,
        input.study_content.unwrap_or("not provided"),
        input.user_note.unwrap_or("not provided"),
    )
}

fn parse_ai_content(content: &str) -> Result<AiFeedbackPayload, String> {
    let trimmed = content.trim();
    if let Ok(payload) = serde_json::from_str::<AiFeedbackPayload>(trimmed) {
        return Ok(normalize_payload(payload));
    }

    let start = trimmed
        .find('{')
        .ok_or_else(|| "AI content is not JSON".to_string())?;
    let end = trimmed
        .rfind('}')
        .ok_or_else(|| "AI content is not JSON".to_string())?;
    let json_slice = &trimmed[start..=end];
    let payload = serde_json::from_str::<AiFeedbackPayload>(json_slice)
        .map_err(|error| format!("AI content JSON parse failed: {error}"))?;
    Ok(normalize_payload(payload))
}

fn normalize_payload(payload: AiFeedbackPayload) -> AiFeedbackPayload {
    let fallback = template_payload("自定义", None);
    AiFeedbackPayload {
        comfort_text: non_empty(payload.comfort_text, fallback.comfort_text),
        study_advice: non_empty(payload.study_advice, fallback.study_advice),
        relax_advice: non_empty(payload.relax_advice, fallback.relax_advice),
    }
}

fn template_feedback(input: EmotionFeedbackInput<'_>) -> String {
    payload_to_string(template_payload(input.emotion_tag, input.user_note))
}

fn template_payload(emotion_tag: &str, user_note: Option<&str>) -> AiFeedbackPayload {
    let comfort_text = match emotion_tag {
        "平静" => "今天辛苦了。你完成了一段稳定的专注学习。",
        "自豪" => "这份成就感值得被记住，你已经完成了重要的一步。",
        "满足" => "你认真投入了当下的学习，这份满足感会成为继续前进的能量。",
        "快乐" => "学习也可以是愉快的，愿你把这份轻松感带到下一次专注里。",
        "疲惫" => "可以给自己一个短暂休息，疲惫说明你已经投入了不少精力。",
        "焦虑" => "先把任务拆小，慢慢完成一件事，你不需要一次解决全部问题。",
        "难过" => "今天的坚持仍然有价值，低落的时候也可以允许自己慢一点。",
        _ => "每一种情绪都值得被接纳。感谢你今天坚持学习，也愿意记录自己的感受。",
    };
    let note_text = user_note
        .map(str::trim)
        .filter(|note| !note.is_empty())
        .map(|note| format!(" 我看到了你的记录：{note}。"))
        .unwrap_or_default();

    AiFeedbackPayload {
        comfort_text: format!("{comfort_text}{note_text}"),
        study_advice: "建议先整理一个最小任务，休息后从最容易开始的部分继续。".to_string(),
        relax_advice: "可以闭眼做几次深呼吸，或者起身活动两分钟，让注意力慢慢回稳。".to_string(),
    }
}

fn payload_to_string(payload: AiFeedbackPayload) -> String {
    serde_json::to_string(&payload).unwrap_or_else(|_| {
        r#"{"comfortText":"每一种情绪都值得被接纳。","studyAdvice":"建议先整理一个最小任务。","relaxAdvice":"可以做几次深呼吸。"}"#.to_string()
    })
}

fn non_empty(value: String, fallback: String) -> String {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        fallback
    } else {
        trimmed.to_string()
    }
}
