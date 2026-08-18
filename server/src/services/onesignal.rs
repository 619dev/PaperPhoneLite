use crate::config::Config;

/// Result of sending a push to OneSignal player(s).
pub enum SendResult {
    Sent,
    Failed,
}

/// Send a push notification directly via OneSignal REST API.
/// Returns the result so callers (including relay) can handle it.
pub async fn send_push(config: &Config, player_ids: &[String], title: &str, body: &str) -> SendResult {
    let (app_id, rest_key) = match (&config.onesignal_app_id, &config.onesignal_rest_key) {
        (Some(aid), Some(rk)) => (aid.clone(), rk.clone()),
        _ => return SendResult::Failed,
    };

    let client = reqwest::Client::new();
    let resp = client
        .post("https://onesignal.com/api/v1/notifications")
        .header("Authorization", format!("Basic {}", rest_key))
        .json(&serde_json::json!({
            "app_id": app_id,
            "include_subscription_ids": player_ids,
            "headings": { "en": title },
            "contents": { "en": body },
            "priority": 10,
            "ttl": 86400,
        }))
        .send()
        .await;

    match resp {
        Ok(r) => {
            let status = r.status();
            let body_text = r.text().await.unwrap_or_default();
            if status.is_success() {
                tracing::debug!("[OneSignal] ✅ Push sent ({} players): {}", player_ids.len(), &body_text[..100.min(body_text.len())]);
                SendResult::Sent
            } else {
                tracing::warn!("[OneSignal] ❌ Push failed (HTTP {}): {}", status, &body_text[..200.min(body_text.len())]);
                SendResult::Failed
            }
        }
        Err(e) => {
            tracing::error!("[OneSignal] ❌ Request error: {:?}", e);
            SendResult::Failed
        }
    }
}

/// Send push via the relay service (for self-hosted servers without OneSignal credentials).
async fn push_via_relay(config: &Config, user_id: &str, player_ids: &[String], title: &str, body: &str) {
    let (relay_url, relay_key) = match (&config.onesignal_relay_url, &config.onesignal_relay_key) {
        (Some(url), Some(key)) => (url.clone(), key.clone()),
        _ => return,
    };

    let client = reqwest::Client::new();
    let url = format!("{}/api/push-relay/onesignal", relay_url.trim_end_matches('/'));

    match client
        .post(&url)
        .json(&serde_json::json!({
            "relay_key": relay_key,
            "player_ids": player_ids,
            "title": title,
            "body": body,
        }))
        .send()
        .await
    {
        Ok(r) => {
            let status = r.status();
            if status.is_success() {
                tracing::debug!("[OneSignal-Relay] ✅ Push relayed for user {}", user_id);
            } else {
                let body_text = r.text().await.unwrap_or_default();
                tracing::warn!("[OneSignal-Relay] ❌ Relay failed (HTTP {}): {}", status, &body_text[..200.min(body_text.len())]);
            }
        }
        Err(e) => {
            tracing::error!("[OneSignal-Relay] ❌ Request error: {:?}", e);
        }
    }
}

/// Check if local OneSignal credentials are configured.
fn has_local_onesignal(config: &Config) -> bool {
    config.onesignal_app_id.is_some() && config.onesignal_rest_key.is_some()
}

/// Check if OneSignal relay is configured.
fn has_relay(config: &Config) -> bool {
    config.onesignal_relay_url.is_some() && config.onesignal_relay_key.is_some()
}

/// Send push notification to a user via OneSignal.
///
/// Strategy:
/// 1. If local OneSignal credentials are configured → send directly
/// 2. Else if relay URL is configured → forward to push relay
/// 3. Else → skip silently
pub async fn push_to_user(db: &sqlx::MySqlPool, config: &Config, user_id: &str, title: &str, body: &str) {
    if !has_local_onesignal(config) && !has_relay(config) {
        return;
    }

    let players: Vec<(String,)> = sqlx::query_as(
        "SELECT player_id FROM onesignal_players WHERE user_id = ?"
    )
    .bind(user_id)
    .fetch_all(db).await.unwrap_or_default();

    if players.is_empty() {
        tracing::debug!("[OneSignal] No players registered for user {}", user_id);
        return;
    }

    let player_ids: Vec<String> = players.into_iter().map(|(pid,)| pid).collect();

    // Strategy 1: Direct OneSignal
    if has_local_onesignal(config) {
        tracing::info!("[OneSignal] Sending push to user {} ({} devices)", user_id, player_ids.len());
        send_push(config, &player_ids, title, body).await;
        return;
    }

    // Strategy 2: Via relay
    push_via_relay(config, user_id, &player_ids, title, body).await;
}
