use std::sync::Arc;
use axum::{extract::State, routing::post, Json, Router};
use serde::Deserialize;
use crate::{services::apns::{send_push, SendResult}, AppState};

pub fn router() -> Router<Arc<AppState>> {
    Router::new().route("/apns", post(relay_apns))
}

#[derive(Deserialize)]
struct RelayApnsReq {
    relay_key: String,
    tokens: Vec<String>,
    title: String,
    body: String,
}

async fn relay_apns(
    State(state): State<Arc<AppState>>,
    Json(body): Json<RelayApnsReq>,
) -> Result<Json<serde_json::Value>, (axum::http::StatusCode, Json<serde_json::Value>)> {
    let secret = match &state.config.apns_relay_secret {
        Some(value) if !value.is_empty() => value,
        _ => return Err((axum::http::StatusCode::FORBIDDEN, Json(serde_json::json!({"error": "Push relay not enabled"})))),
    };
    if &body.relay_key != secret {
        return Err((axum::http::StatusCode::UNAUTHORIZED, Json(serde_json::json!({"error": "Invalid relay key"}))));
    }
    if state.config.apns_bundle_id.is_none() || state.config.apns_team_id.is_none()
        || state.config.apns_key_id.is_none() || state.config.apns_private_key.is_none() {
        return Err((axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "APNS credentials not configured"}))));
    }

    let mut sent = 0u32;
    let mut stale_tokens = Vec::new();
    for token in &body.tokens {
        match send_push(&state.config, token, &body.title, &body.body).await {
            SendResult::Sent => sent += 1,
            SendResult::StaleToken => stale_tokens.push(token.clone()),
            SendResult::Failed => {}
        }
    }
    Ok(Json(serde_json::json!({"ok": true, "sent": sent, "stale_tokens": stale_tokens})))
}
