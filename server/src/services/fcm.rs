use crate::config::Config;
use jsonwebtoken::{encode, EncodingKey, Header, Algorithm};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

/// Cached OAuth2 access token for FCM
static FCM_TOKEN_CACHE: std::sync::LazyLock<Mutex<Option<(String, u64)>>> =
    std::sync::LazyLock::new(|| Mutex::new(None));

#[derive(Serialize)]
struct JwtClaims {
    iss: String,
    scope: String,
    aud: String,
    iat: u64,
    exp: u64,
}

#[derive(Deserialize)]
struct TokenResponse {
    access_token: String,
    expires_in: u64,
}

/// Get a valid OAuth2 access token for FCM, using cache when possible.
async fn get_access_token(config: &Config) -> Option<String> {
    let (client_email, private_key_raw) = match (&config.fcm_client_email, &config.fcm_private_key) {
        (Some(email), Some(key)) => (email.clone(), key.clone()),
        _ => return None,
    };

    // Normalize escaped newlines in PEM key (common in env vars)
    let private_key = private_key_raw.replace("\\n", "\n");

    // Check cache
    {
        let cache = FCM_TOKEN_CACHE.lock().ok()?;
        if let Some((token, expires_at)) = cache.as_ref() {
            let now = SystemTime::now().duration_since(UNIX_EPOCH).ok()?.as_secs();
            if now < *expires_at - 60 {
                return Some(token.clone());
            }
        }
    }

    // Generate JWT
    let now = SystemTime::now().duration_since(UNIX_EPOCH).ok()?.as_secs();
    let claims = JwtClaims {
        iss: client_email,
        scope: "https://www.googleapis.com/auth/firebase.messaging".to_string(),
        aud: "https://oauth2.googleapis.com/token".to_string(),
        iat: now,
        exp: now + 3600,
    };

    let header = Header::new(Algorithm::RS256);
    let key = EncodingKey::from_rsa_pem(private_key.as_bytes()).ok()?;
    let jwt = encode(&header, &claims, &key).ok()?;

    // Exchange JWT for access token
    let client = reqwest::Client::new();
    let resp = client
        .post("https://oauth2.googleapis.com/token")
        .form(&[
            ("grant_type", "urn:ietf:params:oauth:grant-type:jwt-bearer"),
            ("assertion", &jwt),
        ])
        .send()
        .await
        .ok()?;

    let token_resp: TokenResponse = resp.json().await.ok()?;
    let expires_at = now + token_resp.expires_in;

    // Cache the token
    if let Ok(mut cache) = FCM_TOKEN_CACHE.lock() {
        *cache = Some((token_resp.access_token.clone(), expires_at));
    }

    Some(token_resp.access_token)
}

/// Result of sending a push to a single FCM token.
pub enum SendResult {
    Sent,
    StaleToken,
    Failed,
}

/// Send a push notification to a single FCM device token directly.
/// Returns the result so callers can handle stale tokens.
pub async fn send_push(config: &Config, fcm_token: &str, title: &str, body: &str) -> SendResult {
    let project_id = match &config.fcm_project_id {
        Some(pid) => pid.clone(),
        None => return SendResult::Failed,
    };

    let access_token = match get_access_token(config).await {
        Some(t) => t,
        None => return SendResult::Failed,
    };

    let client = reqwest::Client::new();
    let url = format!(
        "https://fcm.googleapis.com/v1/projects/{}/messages:send",
        project_id
    );

    let payload = serde_json::json!({
        "message": {
            "token": fcm_token,
            "notification": {
                "title": title,
                "body": body,
            },
            "android": {
                "priority": "high",
                "notification": {
                    "sound": "default",
                    "channel_id": "paperphonelite_messages",
                }
            },
            "data": {
                "type": "message",
                "click_action": "FLUTTER_NOTIFICATION_CLICK",
            }
        }
    });

    match client
        .post(&url)
        .bearer_auth(&access_token)
        .json(&payload)
        .send()
        .await
    {
        Ok(r) => {
            let status = r.status();
            let body_text = r.text().await.unwrap_or_default();
            if status.is_success() {
                SendResult::Sent
            } else if status.as_u16() == 404 || body_text.contains("NOT_FOUND") || body_text.contains("UNREGISTERED") {
                SendResult::StaleToken
            } else {
                tracing::warn!("[FCM] ❌ Push failed (HTTP {}): {}", status, &body_text[..200.min(body_text.len())]);
                SendResult::Failed
            }
        }
        Err(e) => {
            tracing::error!("[FCM] ❌ Request error: {:?}", e);
            SendResult::Failed
        }
    }
}

/// Response from the push relay service.
#[derive(Deserialize)]
struct RelayResponse {
    #[allow(dead_code)]
    ok: bool,
    stale_tokens: Option<Vec<String>>,
}

/// Send push via the relay service (for self-hosted servers without FCM credentials).
async fn push_via_relay(db: &sqlx::MySqlPool, config: &Config, user_id: &str, fcm_tokens: &[(String,)], title: &str, body: &str) {
    let (relay_url, relay_key) = match (&config.fcm_relay_url, &config.fcm_relay_key) {
        (Some(url), Some(key)) => (url.clone(), key.clone()),
        _ => return,
    };

    let tokens: Vec<&str> = fcm_tokens.iter().map(|(t,)| t.as_str()).collect();

    let client = reqwest::Client::new();
    let url = format!("{}/api/push-relay/fcm", relay_url.trim_end_matches('/'));

    match client
        .post(&url)
        .json(&serde_json::json!({
            "relay_key": relay_key,
            "tokens": tokens,
            "title": title,
            "body": body,
        }))
        .send()
        .await
    {
        Ok(r) => {
            let status = r.status();
            if status.is_success() {
                // Clean up stale tokens returned by relay
                if let Ok(resp) = r.json::<RelayResponse>().await {
                    if let Some(stale) = resp.stale_tokens {
                        for stale_token in &stale {
                            let _ = sqlx::query(
                                "DELETE FROM fcm_tokens WHERE user_id = ? AND fcm_token = ?"
                            )
                            .bind(user_id).bind(stale_token)
                            .execute(db).await;
                        }
                        if !stale.is_empty() {
                            tracing::info!("[FCM-Relay] Cleaned {} stale tokens for user {}", stale.len(), user_id);
                        }
                    }
                }
                tracing::debug!("[FCM-Relay] ✅ Push relayed for user {}", user_id);
            } else {
                let body_text = r.text().await.unwrap_or_default();
                tracing::warn!("[FCM-Relay] ❌ Relay failed (HTTP {}): {}", status, &body_text[..200.min(body_text.len())]);
            }
        }
        Err(e) => {
            tracing::error!("[FCM-Relay] ❌ Request error: {:?}", e);
        }
    }
}

/// Check if local FCM credentials are configured.
fn has_local_fcm(config: &Config) -> bool {
    config.fcm_project_id.is_some()
        && config.fcm_client_email.is_some()
        && config.fcm_private_key.is_some()
}

/// Check if FCM relay is configured.
fn has_relay(config: &Config) -> bool {
    config.fcm_relay_url.is_some() && config.fcm_relay_key.is_some()
}

/// Send push notification to a user via Firebase Cloud Messaging (HTTP v1 API).
///
/// Strategy:
/// 1. If local FCM credentials are configured → send directly to Google
/// 2. Else if relay URL is configured → forward to push relay
/// 3. Else → skip silently
pub async fn push_to_user(db: &sqlx::MySqlPool, config: &Config, user_id: &str, title: &str, body: &str) {
    if !has_local_fcm(config) && !has_relay(config) {
        return;
    }

    let tokens: Vec<(String,)> = sqlx::query_as(
        "SELECT fcm_token FROM fcm_tokens WHERE user_id = ?"
    )
    .bind(user_id)
    .fetch_all(db).await.unwrap_or_default();

    if tokens.is_empty() { return; }

    // Strategy 1: Direct FCM
    if has_local_fcm(config) {
        for (token,) in &tokens {
            match send_push(config, token, title, body).await {
                SendResult::Sent => {
                    tracing::debug!("[FCM] ✅ Push sent to user {}", user_id);
                }
                SendResult::StaleToken => {
                    tracing::info!("[FCM] Removing stale token for user {}", user_id);
                    let _ = sqlx::query(
                        "DELETE FROM fcm_tokens WHERE user_id = ? AND fcm_token = ?"
                    )
                    .bind(user_id).bind(token)
                    .execute(db).await;
                }
                SendResult::Failed => {}
            }
        }
        return;
    }

    // Strategy 2: Via relay
    push_via_relay(db, config, user_id, &tokens, title, body).await;
}
