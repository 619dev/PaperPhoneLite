use std::sync::Arc;
use axum::{Router, routing::post, extract::State, Json};
use serde::Deserialize;

use crate::AppState;
use crate::services::apns::{send_push as apns_send_push, SendResult as ApnsSendResult};
use crate::services::fcm::{send_push as fcm_send_push, SendResult as FcmSendResult};
use crate::services::onesignal::{send_push as onesignal_send_push};

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/apns", post(relay_apns))
        .route("/fcm", post(relay_fcm))
        .route("/onesignal", post(relay_onesignal))
}

// ── APNS Relay ──────────────────────────────────────────────────

#[derive(Deserialize)]
struct RelayApnsReq {
    relay_key: String,
    tokens: Vec<String>,
    title: String,
    body: String,
}

/// Push relay endpoint for APNS.
///
/// Self-hosted PaperPhoneLite servers call this endpoint to send iOS push
/// notifications through the relay's APNS credentials.
///
/// Authentication: The request must include a `relay_key` matching
/// the relay server's `APNS_RELAY_SECRET` environment variable.
///
/// Response includes `stale_tokens` so the caller can clean up its DB.
async fn relay_apns(
    State(state): State<Arc<AppState>>,
    Json(body): Json<RelayApnsReq>,
) -> Result<Json<serde_json::Value>, (axum::http::StatusCode, Json<serde_json::Value>)> {
    // Validate relay secret is configured on this server
    let secret = match &state.config.apns_relay_secret {
        Some(s) if !s.is_empty() => s.clone(),
        _ => {
            return Err((
                axum::http::StatusCode::FORBIDDEN,
                Json(serde_json::json!({"error": "Push relay not enabled on this server"})),
            ));
        }
    };

    // Validate the caller's key
    if body.relay_key != secret {
        return Err((
            axum::http::StatusCode::UNAUTHORIZED,
            Json(serde_json::json!({"error": "Invalid relay key"})),
        ));
    }

    // Ensure this server has APNS credentials configured
    if state.config.apns_bundle_id.is_none()
        || state.config.apns_team_id.is_none()
        || state.config.apns_key_id.is_none()
        || state.config.apns_private_key.is_none()
    {
        return Err((
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": "APNS credentials not configured on relay server"})),
        ));
    }

    if body.tokens.is_empty() {
        return Ok(Json(serde_json::json!({"ok": true, "sent": 0, "stale_tokens": []})));
    }

    let mut sent = 0u32;
    let mut stale_tokens: Vec<String> = Vec::new();

    for device_token in &body.tokens {
        match apns_send_push(&state.config, device_token, &body.title, &body.body).await {
            ApnsSendResult::Sent => {
                sent += 1;
            }
            ApnsSendResult::StaleToken => {
                stale_tokens.push(device_token.clone());
            }
            ApnsSendResult::Failed => {}
        }
    }

    tracing::info!(
        "[APNS-Relay] Processed {} tokens: {} sent, {} stale",
        body.tokens.len(), sent, stale_tokens.len()
    );

    Ok(Json(serde_json::json!({
        "ok": true,
        "sent": sent,
        "stale_tokens": stale_tokens,
    })))
}

// ── FCM Relay ───────────────────────────────────────────────────

#[derive(Deserialize)]
struct RelayFcmReq {
    relay_key: String,
    tokens: Vec<String>,
    title: String,
    body: String,
}

/// Push relay endpoint for FCM.
///
/// Self-hosted PaperPhoneLite servers call this endpoint to send Android push
/// notifications through the relay's Firebase credentials.
///
/// Authentication: The request must include a `relay_key` matching
/// the relay server's `FCM_RELAY_SECRET` environment variable.
///
/// Response includes `stale_tokens` so the caller can clean up its DB.
async fn relay_fcm(
    State(state): State<Arc<AppState>>,
    Json(body): Json<RelayFcmReq>,
) -> Result<Json<serde_json::Value>, (axum::http::StatusCode, Json<serde_json::Value>)> {
    // Validate relay secret is configured on this server
    let secret = match &state.config.fcm_relay_secret {
        Some(s) if !s.is_empty() => s.clone(),
        _ => {
            return Err((
                axum::http::StatusCode::FORBIDDEN,
                Json(serde_json::json!({"error": "FCM relay not enabled on this server"})),
            ));
        }
    };

    // Validate the caller's key
    if body.relay_key != secret {
        return Err((
            axum::http::StatusCode::UNAUTHORIZED,
            Json(serde_json::json!({"error": "Invalid relay key"})),
        ));
    }

    // Ensure this server has FCM credentials configured
    if state.config.fcm_project_id.is_none()
        || state.config.fcm_client_email.is_none()
        || state.config.fcm_private_key.is_none()
    {
        return Err((
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": "FCM credentials not configured on relay server"})),
        ));
    }

    if body.tokens.is_empty() {
        return Ok(Json(serde_json::json!({"ok": true, "sent": 0, "stale_tokens": []})));
    }

    let mut sent = 0u32;
    let mut stale_tokens: Vec<String> = Vec::new();

    for fcm_token in &body.tokens {
        match fcm_send_push(&state.config, fcm_token, &body.title, &body.body).await {
            FcmSendResult::Sent => {
                sent += 1;
            }
            FcmSendResult::StaleToken => {
                stale_tokens.push(fcm_token.clone());
            }
            FcmSendResult::Failed => {}
        }
    }

    tracing::info!(
        "[FCM-Relay] Processed {} tokens: {} sent, {} stale",
        body.tokens.len(), sent, stale_tokens.len()
    );

    Ok(Json(serde_json::json!({
        "ok": true,
        "sent": sent,
        "stale_tokens": stale_tokens,
    })))
}

// ── OneSignal Relay ─────────────────────────────────────────────

#[derive(Deserialize)]
struct RelayOneSignalReq {
    relay_key: String,
    player_ids: Vec<String>,
    title: String,
    body: String,
}

/// Push relay endpoint for OneSignal.
///
/// Self-hosted PaperPhoneLite servers call this endpoint to send push
/// notifications through the relay's OneSignal credentials.
///
/// Authentication: The request must include a `relay_key` matching
/// the relay server's `ONESIGNAL_RELAY_SECRET` environment variable.
async fn relay_onesignal(
    State(state): State<Arc<AppState>>,
    Json(body): Json<RelayOneSignalReq>,
) -> Result<Json<serde_json::Value>, (axum::http::StatusCode, Json<serde_json::Value>)> {
    // Validate relay secret is configured on this server
    let secret = match &state.config.onesignal_relay_secret {
        Some(s) if !s.is_empty() => s.clone(),
        _ => {
            return Err((
                axum::http::StatusCode::FORBIDDEN,
                Json(serde_json::json!({"error": "OneSignal relay not enabled on this server"})),
            ));
        }
    };

    // Validate the caller's key
    if body.relay_key != secret {
        return Err((
            axum::http::StatusCode::UNAUTHORIZED,
            Json(serde_json::json!({"error": "Invalid relay key"})),
        ));
    }

    // Ensure this server has OneSignal credentials configured
    if state.config.onesignal_app_id.is_none()
        || state.config.onesignal_rest_key.is_none()
    {
        return Err((
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": "OneSignal credentials not configured on relay server"})),
        ));
    }

    if body.player_ids.is_empty() {
        return Ok(Json(serde_json::json!({"ok": true, "sent": 0})));
    }

    let result = onesignal_send_push(&state.config, &body.player_ids, &body.title, &body.body).await;

    let sent = matches!(result, crate::services::onesignal::SendResult::Sent);

    tracing::info!(
        "[OneSignal-Relay] Processed {} players: success={}",
        body.player_ids.len(), sent
    );

    Ok(Json(serde_json::json!({
        "ok": true,
        "sent": if sent { body.player_ids.len() } else { 0 },
    })))
}
