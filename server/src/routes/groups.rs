use std::sync::Arc;
use axum::{Router, routing::{get, post, put, delete}, extract::{State, Path, Query}, Json};
use serde::Deserialize;
use uuid::Uuid;

use crate::AppState;
use crate::auth::middleware::AuthUser;

const MAX_GROUP_MEMBERS: usize = 2000;

async fn count_members(db: &sqlx::MySqlPool, group_id: &str) -> Result<usize, (axum::http::StatusCode, Json<serde_json::Value>)> {
    let (count,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM group_members WHERE group_id = ?")
        .bind(group_id)
        .fetch_one(db).await
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": e.to_string() }))))?;
    Ok(count as usize)
}
pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(list_groups).post(create_group))
        .route("/search", get(search_groups))
        .route("/{id}", get(get_group).put(update_group).delete(disband_group))
        .route("/{id}/members", post(add_members))
        .route("/{id}/members/{user_id}", delete(remove_member))
        .route("/{id}/leave", post(leave_group))
        .route("/{id}/mute", post(toggle_mute))
        .route("/{id}/invite", post(create_invite))
        .route("/join/{invite_id}", post(join_by_invite))
        .route("/{id}/auto-delete", put(update_auto_delete))
        .route("/{id}/encryption", put(toggle_encryption))
        .route("/{id}/sender-keys", post(upload_sender_keys).get(get_sender_keys))
}

#[derive(Deserialize)]
struct CreateGroupReq {
    name: String,
    member_ids: Option<Vec<String>>,
}

async fn create_group(
    State(state): State<Arc<AppState>>,
    auth: AuthUser,
    Json(body): Json<CreateGroupReq>,
) -> Result<(axum::http::StatusCode, Json<serde_json::Value>), (axum::http::StatusCode, Json<serde_json::Value>)> {
    let id = Uuid::new_v4().to_string();

    sqlx::query("INSERT INTO `groups` (id, name, owner_id) VALUES (?, ?, ?)")
        .bind(&id).bind(&body.name).bind(&auth.0.id)
        .execute(&state.db).await
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": e.to_string() }))))?;

    // Add owner as member
    sqlx::query("INSERT INTO group_members (group_id, user_id, role) VALUES (?, ?, 'owner')")
        .bind(&id).bind(&auth.0.id)
        .execute(&state.db).await.ok();

    // Add other members (enforce 2000 limit)
    if let Some(members) = &body.member_ids {
        let to_add: Vec<&String> = members.iter().filter(|m| *m != &auth.0.id).collect();
        if 1 + to_add.len() > MAX_GROUP_MEMBERS {
            return Err((axum::http::StatusCode::BAD_REQUEST, Json(serde_json::json!({ "error": format!("Group cannot exceed {} members", MAX_GROUP_MEMBERS) }))));
        }
        for member_id in to_add {
            sqlx::query("INSERT INTO group_members (group_id, user_id) VALUES (?, ?)")
                .bind(&id).bind(member_id)
                .execute(&state.db).await.ok();

            // Notify each member
            state.ws_clients.send_to_user(member_id, serde_json::json!({
                "type": "group_member_added",
                "group_id": &id,
                "group_name": &body.name,
                "added_by": &auth.0.id,
            }));
        }
    }

    Ok((axum::http::StatusCode::CREATED, Json(serde_json::json!({ "id": id, "name": body.name }))))
}

async fn list_groups(
    State(state): State<Arc<AppState>>,
    auth: AuthUser,
) -> Result<Json<serde_json::Value>, (axum::http::StatusCode, Json<serde_json::Value>)> {
    let rows: Vec<(String, String, Option<String>, String, Option<String>, i32, i8, i8)> = sqlx::query_as(
        "SELECT g.id, g.name, g.avatar, g.owner_id, g.notice, g.auto_delete, gm.muted, g.encrypted
         FROM `groups` g JOIN group_members gm ON gm.group_id = g.id
         WHERE gm.user_id = ?"
    )
    .bind(&auth.0.id)
    .fetch_all(&state.db).await
    .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": e.to_string() }))))?;

    let groups: Vec<serde_json::Value> = rows.iter().map(|(id, name, avatar, owner_id, notice, auto_del, muted, encrypted)| {
        serde_json::json!({ "id": id, "name": name, "avatar": avatar, "owner_id": owner_id, "notice": notice, "auto_delete": auto_del, "muted": *muted == 1, "encrypted": *encrypted == 1 })
    }).collect();

    Ok(Json(serde_json::json!(groups)))
}

async fn get_group(
    State(state): State<Arc<AppState>>,
    _auth: AuthUser,
    Path(id): Path<String>,
) -> Result<Json<serde_json::Value>, (axum::http::StatusCode, Json<serde_json::Value>)> {
    let group: Option<(String, String, Option<String>, String, Option<String>, i32, i8)> = sqlx::query_as(
        "SELECT id, name, avatar, owner_id, notice, auto_delete, encrypted FROM `groups` WHERE id = ?"
    )
    .bind(&id)
    .fetch_optional(&state.db).await
    .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": e.to_string() }))))?;

    let (gid, name, avatar, owner_id, notice, auto_del, encrypted) = group
        .ok_or_else(|| (axum::http::StatusCode::NOT_FOUND, Json(serde_json::json!({ "error": "Group not found" }))))?;

    // Get members
    let members: Vec<(String, String, String, Option<String>, String, i8)> = sqlx::query_as(
        "SELECT u.id, u.username, u.nickname, u.avatar, gm.role, gm.muted
         FROM group_members gm JOIN users u ON u.id = gm.user_id
         WHERE gm.group_id = ?"
    )
    .bind(&id)
    .fetch_all(&state.db).await.unwrap_or_default();

    let member_list: Vec<serde_json::Value> = members.iter().map(|(uid, username, nickname, avatar, role, muted)| {
        serde_json::json!({ "id": uid, "username": username, "nickname": nickname, "avatar": avatar, "role": role, "muted": *muted == 1 })
    }).collect();

    Ok(Json(serde_json::json!({
        "id": gid, "name": name, "avatar": avatar, "owner_id": owner_id,
        "notice": notice, "auto_delete": auto_del, "encrypted": encrypted == 1, "members": member_list
    })))
}

#[derive(Deserialize)]
struct UpdateGroupReq {
    name: Option<String>,
    avatar: Option<String>,
    notice: Option<String>,
}

async fn update_group(
    State(state): State<Arc<AppState>>,
    _auth: AuthUser,
    Path(id): Path<String>,
    Json(body): Json<UpdateGroupReq>,
) -> Result<Json<serde_json::Value>, (axum::http::StatusCode, Json<serde_json::Value>)> {
    if let Some(name) = &body.name {
        sqlx::query("UPDATE `groups` SET name = ? WHERE id = ?").bind(name).bind(&id).execute(&state.db).await.ok();
    }
    if let Some(avatar) = &body.avatar {
        sqlx::query("UPDATE `groups` SET avatar = ? WHERE id = ?").bind(avatar).bind(&id).execute(&state.db).await.ok();
    }
    if let Some(notice) = &body.notice {
        sqlx::query("UPDATE `groups` SET notice = ? WHERE id = ?").bind(notice).bind(&id).execute(&state.db).await.ok();
    }
    Ok(Json(serde_json::json!({ "ok": true })))
}

async fn disband_group(
    State(state): State<Arc<AppState>>,
    auth: AuthUser,
    Path(id): Path<String>,
) -> Result<Json<serde_json::Value>, (axum::http::StatusCode, Json<serde_json::Value>)> {
    // Only owner can disband
    let owner: Option<(String,)> = sqlx::query_as("SELECT owner_id FROM `groups` WHERE id = ?")
        .bind(&id).fetch_optional(&state.db).await.ok().flatten();
    if owner.map(|o| o.0) != Some(auth.0.id.clone()) {
        return Err((axum::http::StatusCode::FORBIDDEN, Json(serde_json::json!({ "error": "Only owner can disband" }))));
    }
    sqlx::query("DELETE FROM `groups` WHERE id = ?").bind(&id).execute(&state.db).await.ok();
    Ok(Json(serde_json::json!({ "ok": true })))
}

#[derive(Deserialize)]
struct AddMembersReq {
    user_ids: Vec<String>,
}

async fn add_members(
    State(state): State<Arc<AppState>>,
    auth: AuthUser,
    Path(group_id): Path<String>,
    Json(body): Json<AddMembersReq>,
) -> Result<Json<serde_json::Value>, (axum::http::StatusCode, Json<serde_json::Value>)> {
    let group_name: Option<(String,)> = sqlx::query_as("SELECT name FROM `groups` WHERE id = ?")
        .bind(&group_id).fetch_optional(&state.db).await.ok().flatten();
    let name = group_name.map(|g| g.0).unwrap_or_default();

    // Enforce 2000-member limit
    let current = count_members(&state.db, &group_id).await?;
    if current + body.user_ids.len() > MAX_GROUP_MEMBERS {
        return Err((axum::http::StatusCode::BAD_REQUEST, Json(serde_json::json!({ "error": format!("Group cannot exceed {} members", MAX_GROUP_MEMBERS) }))));
    }

    for uid in &body.user_ids {
        sqlx::query("INSERT IGNORE INTO group_members (group_id, user_id) VALUES (?, ?)")
            .bind(&group_id).bind(uid)
            .execute(&state.db).await.ok();

        state.ws_clients.send_to_user(uid, serde_json::json!({
            "type": "group_member_added",
            "group_id": &group_id,
            "group_name": &name,
            "added_by": &auth.0.id,
        }));
    }
    Ok(Json(serde_json::json!({ "ok": true })))
}

async fn remove_member(
    State(state): State<Arc<AppState>>,
    _auth: AuthUser,
    Path((group_id, user_id)): Path<(String, String)>,
) -> Result<Json<serde_json::Value>, (axum::http::StatusCode, Json<serde_json::Value>)> {
    sqlx::query("DELETE FROM group_members WHERE group_id = ? AND user_id = ?")
        .bind(&group_id).bind(&user_id)
        .execute(&state.db).await
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": e.to_string() }))))?;

    // Clean up sender keys for the removed member
    sqlx::query("DELETE FROM group_sender_keys WHERE group_id = ? AND user_id = ?")
        .bind(&group_id).bind(&user_id).execute(&state.db).await.ok();
    sqlx::query("DELETE FROM group_sender_key_distributions WHERE group_id = ? AND (from_id = ? OR to_id = ?)")
        .bind(&group_id).bind(&user_id).bind(&user_id).execute(&state.db).await.ok();

    // Notify remaining members to rotate their sender keys
    let encrypted: Option<(i8,)> = sqlx::query_as("SELECT encrypted FROM `groups` WHERE id = ?")
        .bind(&group_id).fetch_optional(&state.db).await.ok().flatten();
    if encrypted.map(|e| e.0).unwrap_or(0) == 1 {
        let members: Vec<(String,)> = sqlx::query_as(
            "SELECT user_id FROM group_members WHERE group_id = ?"
        ).bind(&group_id).fetch_all(&state.db).await.unwrap_or_default();
        for (mid,) in &members {
            state.ws_clients.send_to_user(mid, serde_json::json!({
                "type": "sender_key_rotate",
                "group_id": &group_id,
                "removed_user": &user_id,
            }));
        }
    }

    Ok(Json(serde_json::json!({ "ok": true })))
}

async fn leave_group(
    State(state): State<Arc<AppState>>,
    auth: AuthUser,
    Path(id): Path<String>,
) -> Result<Json<serde_json::Value>, (axum::http::StatusCode, Json<serde_json::Value>)> {
    sqlx::query("DELETE FROM group_members WHERE group_id = ? AND user_id = ?")
        .bind(&id).bind(&auth.0.id)
        .execute(&state.db).await.ok();

    // Clean up sender keys
    sqlx::query("DELETE FROM group_sender_keys WHERE group_id = ? AND user_id = ?")
        .bind(&id).bind(&auth.0.id).execute(&state.db).await.ok();
    sqlx::query("DELETE FROM group_sender_key_distributions WHERE group_id = ? AND (from_id = ? OR to_id = ?)")
        .bind(&id).bind(&auth.0.id).bind(&auth.0.id).execute(&state.db).await.ok();

    // Notify remaining members to rotate sender keys if encrypted
    let encrypted: Option<(i8,)> = sqlx::query_as("SELECT encrypted FROM `groups` WHERE id = ?")
        .bind(&id).fetch_optional(&state.db).await.ok().flatten();
    if encrypted.map(|e| e.0).unwrap_or(0) == 1 {
        let members: Vec<(String,)> = sqlx::query_as(
            "SELECT user_id FROM group_members WHERE group_id = ?"
        ).bind(&id).fetch_all(&state.db).await.unwrap_or_default();
        for (mid,) in &members {
            state.ws_clients.send_to_user(mid, serde_json::json!({
                "type": "sender_key_rotate",
                "group_id": &id,
                "removed_user": &auth.0.id,
            }));
        }
    }

    Ok(Json(serde_json::json!({ "ok": true })))
}

#[derive(Deserialize)]
struct MuteReq { muted: bool }

async fn toggle_mute(
    State(state): State<Arc<AppState>>,
    auth: AuthUser,
    Path(id): Path<String>,
    Json(body): Json<MuteReq>,
) -> Result<Json<serde_json::Value>, (axum::http::StatusCode, Json<serde_json::Value>)> {
    sqlx::query("UPDATE group_members SET muted = ? WHERE group_id = ? AND user_id = ?")
        .bind(body.muted as i8).bind(&id).bind(&auth.0.id)
        .execute(&state.db).await.ok();
    Ok(Json(serde_json::json!({ "ok": true })))
}

#[derive(Deserialize)]
struct InviteReq {
    expires_days: Option<i32>,
}

async fn create_invite(
    State(state): State<Arc<AppState>>,
    auth: AuthUser,
    Path(group_id): Path<String>,
    Json(body): Json<InviteReq>,
) -> Result<Json<serde_json::Value>, (axum::http::StatusCode, Json<serde_json::Value>)> {
    let invite_id = Uuid::new_v4().to_string();
    let days = body.expires_days.unwrap_or(7);

    sqlx::query("INSERT INTO group_invites (id, group_id, created_by, expires_at) VALUES (?, ?, ?, DATE_ADD(NOW(), INTERVAL ? DAY))")
        .bind(&invite_id).bind(&group_id).bind(&auth.0.id).bind(days)
        .execute(&state.db).await
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": e.to_string() }))))?;

    Ok(Json(serde_json::json!({ "invite_id": invite_id })))
}

async fn join_by_invite(
    State(state): State<Arc<AppState>>,
    auth: AuthUser,
    Path(invite_id): Path<String>,
) -> Result<Json<serde_json::Value>, (axum::http::StatusCode, Json<serde_json::Value>)> {
    let invite: Option<(String,)> = sqlx::query_as(
        "SELECT group_id FROM group_invites WHERE id = ? AND expires_at > NOW()"
    )
    .bind(&invite_id)
    .fetch_optional(&state.db).await
    .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": e.to_string() }))))?;

    let (group_id,) = invite
        .ok_or_else(|| (axum::http::StatusCode::NOT_FOUND, Json(serde_json::json!({ "error": "Invalid or expired invite" }))))?;

    // Enforce 2000-member limit
    let current = count_members(&state.db, &group_id).await?;
    if current >= MAX_GROUP_MEMBERS {
        return Err((axum::http::StatusCode::BAD_REQUEST, Json(serde_json::json!({ "error": format!("Group cannot exceed {} members", MAX_GROUP_MEMBERS) }))));
    }

    sqlx::query("INSERT IGNORE INTO group_members (group_id, user_id) VALUES (?, ?)")
        .bind(&group_id).bind(&auth.0.id)
        .execute(&state.db).await.ok();

    Ok(Json(serde_json::json!({ "ok": true, "group_id": group_id })))
}

#[derive(Deserialize)]
struct SearchQuery { q: String }

async fn search_groups(
    State(state): State<Arc<AppState>>,
    _auth: AuthUser,
    Query(params): Query<SearchQuery>,
) -> Result<Json<serde_json::Value>, (axum::http::StatusCode, Json<serde_json::Value>)> {
    let pattern = format!("%{}%", params.q);
    let rows: Vec<(String, String, Option<String>)> = sqlx::query_as(
        "SELECT id, name, avatar FROM `groups` WHERE name LIKE ? LIMIT 20"
    )
    .bind(&pattern)
    .fetch_all(&state.db).await.unwrap_or_default();

    let groups: Vec<serde_json::Value> = rows.iter().map(|(id, name, avatar)| {
        serde_json::json!({ "id": id, "name": name, "avatar": avatar })
    }).collect();

    Ok(Json(serde_json::json!(groups)))
}

#[derive(Deserialize)]
struct AutoDeleteReq { auto_delete: i32 }

async fn update_auto_delete(
    State(state): State<Arc<AppState>>,
    auth: AuthUser,
    Path(id): Path<String>,
    Json(body): Json<AutoDeleteReq>,
) -> Result<Json<serde_json::Value>, (axum::http::StatusCode, Json<serde_json::Value>)> {
    const ALLOWED: [i32; 5] = [0, 86400, 259200, 604800, 2592000];
    if !ALLOWED.contains(&body.auto_delete) {
        return Err((axum::http::StatusCode::BAD_REQUEST, Json(serde_json::json!({ "error": "Invalid auto_delete value" }))));
    }
    // Only group owner can change auto_delete
    let owner: Option<(String,)> = sqlx::query_as("SELECT owner_id FROM `groups` WHERE id = ?")
        .bind(&id).fetch_optional(&state.db).await.ok().flatten();
    if owner.as_ref().map(|o| o.0.as_str()) != Some(&auth.0.id) {
        return Err((axum::http::StatusCode::FORBIDDEN, Json(serde_json::json!({ "error": "Only the group owner can change auto-delete" }))));
    }
    sqlx::query("UPDATE `groups` SET auto_delete = ? WHERE id = ?")
        .bind(body.auto_delete).bind(&id)
        .execute(&state.db).await.ok();
    Ok(Json(serde_json::json!({ "ok": true })))
}

// ── Group Encryption Toggle ─────────────────────────────────────────────

#[derive(Deserialize)]
struct EncryptionReq { encrypted: bool }

async fn toggle_encryption(
    State(state): State<Arc<AppState>>,
    auth: AuthUser,
    Path(id): Path<String>,
    Json(body): Json<EncryptionReq>,
) -> Result<Json<serde_json::Value>, (axum::http::StatusCode, Json<serde_json::Value>)> {
    // Only group owner can toggle encryption
    let owner: Option<(String,)> = sqlx::query_as("SELECT owner_id FROM `groups` WHERE id = ?")
        .bind(&id).fetch_optional(&state.db).await.ok().flatten();
    if owner.as_ref().map(|o| o.0.as_str()) != Some(&auth.0.id) {
        return Err((axum::http::StatusCode::FORBIDDEN, Json(serde_json::json!({ "error": "Only the group owner can toggle encryption" }))));
    }

    // Update encryption status
    sqlx::query("UPDATE `groups` SET encrypted = ? WHERE id = ?")
        .bind(body.encrypted as i8).bind(&id)
        .execute(&state.db).await
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": e.to_string() }))))?;

    // Clear all group messages
    sqlx::query("DELETE FROM messages WHERE type = 'group' AND to_id = ?")
        .bind(&id).execute(&state.db).await.ok();

    // Clear all sender keys and distributions
    sqlx::query("DELETE FROM group_sender_keys WHERE group_id = ?")
        .bind(&id).execute(&state.db).await.ok();
    sqlx::query("DELETE FROM group_sender_key_distributions WHERE group_id = ?")
        .bind(&id).execute(&state.db).await.ok();

    // Broadcast encryption change to all group members
    let members: Vec<(String,)> = sqlx::query_as(
        "SELECT user_id FROM group_members WHERE group_id = ?"
    ).bind(&id).fetch_all(&state.db).await.unwrap_or_default();

    for (mid,) in &members {
        state.ws_clients.send_to_user(mid, serde_json::json!({
            "type": "group_encryption_changed",
            "group_id": &id,
            "encrypted": body.encrypted,
        }));
    }

    Ok(Json(serde_json::json!({ "ok": true })))
}

// ── Sender Key Management ───────────────────────────────────────────────

#[derive(Deserialize)]
struct SenderKeyUpload {
    distributions: Vec<SenderKeyDistEntry>,
    key_version: u32,
}

#[derive(Deserialize)]
struct SenderKeyDistEntry {
    to_id: String,
    encrypted_key: String,
    header: String,
}

async fn upload_sender_keys(
    State(state): State<Arc<AppState>>,
    auth: AuthUser,
    Path(group_id): Path<String>,
    Json(body): Json<SenderKeyUpload>,
) -> Result<Json<serde_json::Value>, (axum::http::StatusCode, Json<serde_json::Value>)> {
    // Verify caller is a member
    let member: Option<(String,)> = sqlx::query_as(
        "SELECT user_id FROM group_members WHERE group_id = ? AND user_id = ?"
    ).bind(&group_id).bind(&auth.0.id).fetch_optional(&state.db).await.ok().flatten();
    if member.is_none() {
        return Err((axum::http::StatusCode::FORBIDDEN, Json(serde_json::json!({ "error": "Not a group member" }))));
    }

    // Upsert sender key metadata
    sqlx::query(
        "INSERT INTO group_sender_keys (group_id, user_id, key_version) VALUES (?, ?, ?)
         ON DUPLICATE KEY UPDATE key_version = VALUES(key_version), created_at = NOW()"
    ).bind(&group_id).bind(&auth.0.id).bind(body.key_version)
    .execute(&state.db).await.ok();

    // Insert distributions for each recipient
    for dist in &body.distributions {
        sqlx::query(
            "INSERT INTO group_sender_key_distributions (group_id, from_id, to_id, encrypted_key, header, key_version) VALUES (?, ?, ?, ?, ?, ?)
             ON DUPLICATE KEY UPDATE encrypted_key = VALUES(encrypted_key), header = VALUES(header), created_at = NOW()"
        )
        .bind(&group_id).bind(&auth.0.id).bind(&dist.to_id)
        .bind(&dist.encrypted_key).bind(&dist.header).bind(body.key_version)
        .execute(&state.db).await.ok();

        // Notify recipient via WebSocket
        state.ws_clients.send_to_user(&dist.to_id, serde_json::json!({
            "type": "sender_key_distribution",
            "group_id": &group_id,
            "from_id": &auth.0.id,
            "encrypted_key": &dist.encrypted_key,
            "header": &dist.header,
            "key_version": body.key_version,
        }));
    }

    Ok(Json(serde_json::json!({ "ok": true })))
}

async fn get_sender_keys(
    State(state): State<Arc<AppState>>,
    auth: AuthUser,
    Path(group_id): Path<String>,
) -> Result<Json<serde_json::Value>, (axum::http::StatusCode, Json<serde_json::Value>)> {
    // Get all sender key distributions addressed to this user for this group
    let rows: Vec<(String, String, String, u32)> = sqlx::query_as(
        "SELECT from_id, encrypted_key, header, key_version
         FROM group_sender_key_distributions
         WHERE group_id = ? AND to_id = ?
         ORDER BY created_at DESC"
    )
    .bind(&group_id).bind(&auth.0.id)
    .fetch_all(&state.db).await.unwrap_or_default();

    let keys: Vec<serde_json::Value> = rows.iter().map(|(from_id, ek, hdr, ver)| {
        serde_json::json!({
            "from_id": from_id,
            "encrypted_key": ek,
            "header": hdr,
            "key_version": ver,
        })
    }).collect();

    Ok(Json(serde_json::json!({ "keys": keys })))
}
