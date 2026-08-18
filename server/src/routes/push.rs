use std::sync::Arc;
use axum::{extract::State, routing::{get, post}, Json, Router};
use serde::Deserialize;
use sha2::{Digest, Sha256};
use crate::{auth::middleware::AuthUser, AppState};

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/status", get(push_status))
        .route("/ntfy", post(register_ntfy))
        .route("/ntfy-topic", get(get_ntfy_topic))
        .route("/apns", post(register_apns))
}

async fn push_status(State(state): State<Arc<AppState>>, auth: AuthUser) -> Json<serde_json::Value> {
    let ntfy: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM ntfy_subscriptions WHERE user_id = ?")
        .bind(&auth.0.id).fetch_one(&state.db).await.unwrap_or((0,));
    let apns: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM apns_tokens WHERE user_id = ?")
        .bind(&auth.0.id).fetch_one(&state.db).await.unwrap_or((0,));
    Json(serde_json::json!({
        "ntfy_configured": true,
        "ntfy_base_url": &state.config.ntfy_base_url,
        "apns_configured": state.config.apns_team_id.is_some() && state.config.apns_key_id.is_some()
            && state.config.apns_private_key.is_some() && state.config.apns_bundle_id.is_some(),
        "apns_relay_configured": state.config.apns_relay_url.is_some() && state.config.apns_relay_key.is_some(),
        "user_ntfy_subscriptions": ntfy.0,
        "user_apns_tokens": apns.0,
    }))
}

#[derive(Deserialize)]
struct NtfyReq { ntfy_topic: String, platform: Option<String> }

async fn register_ntfy(State(state): State<Arc<AppState>>, auth: AuthUser, Json(body): Json<NtfyReq>) -> Json<serde_json::Value> {
    sqlx::query("INSERT INTO ntfy_subscriptions (user_id, ntfy_topic, platform) VALUES (?, ?, ?) ON DUPLICATE KEY UPDATE platform = VALUES(platform)")
        .bind(&auth.0.id).bind(&body.ntfy_topic).bind(&body.platform).execute(&state.db).await.ok();
    Json(serde_json::json!({ "ok": true }))
}

async fn get_ntfy_topic(State(state): State<Arc<AppState>>, auth: AuthUser) -> Json<serde_json::Value> {
    let mut hasher = Sha256::new();
    hasher.update(auth.0.id.as_bytes());
    let topic = format!("pp-{}", hex::encode(&hasher.finalize()[..8]));
    Json(serde_json::json!({
        "ntfy_topic": topic,
        "ntfy_url": format!("{}/{}", state.config.ntfy_base_url.trim_end_matches('/'), topic),
    }))
}

#[derive(Deserialize)]
struct ApnsReq { apns_token: String, platform: Option<String> }

async fn register_apns(State(state): State<Arc<AppState>>, auth: AuthUser, Json(body): Json<ApnsReq>)
    -> Result<Json<serde_json::Value>, (axum::http::StatusCode, Json<serde_json::Value>)> {
    let token: String = body.apns_token.chars().filter(|c| c.is_ascii_hexdigit()).collect::<String>().to_lowercase();
    if token.len() != 64 {
        return Err((axum::http::StatusCode::BAD_REQUEST, Json(serde_json::json!({ "error": "Invalid APNS token: must be 64 hex characters" }))));
    }
    sqlx::query("INSERT INTO apns_tokens (user_id, apns_token, platform) VALUES (?, ?, ?) ON DUPLICATE KEY UPDATE platform = VALUES(platform), updated_at = NOW()")
        .bind(&auth.0.id).bind(&token).bind(&body.platform).execute(&state.db).await.ok();
    Ok(Json(serde_json::json!({ "ok": true })))
}
