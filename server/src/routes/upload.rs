use std::sync::Arc;
use axum::{Router, routing::post, extract::{State, Multipart, Query}, Json};
use serde::Deserialize;
use uuid::Uuid;

use crate::AppState;
use crate::auth::middleware::AuthUser;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", post(upload_file))
}

async fn upload_file(
    State(state): State<Arc<AppState>>,
    Query(params): Query<UploadParams>,
    _auth: AuthUser,
    mut multipart: Multipart,
) -> Result<Json<serde_json::Value>, (axum::http::StatusCode, Json<serde_json::Value>)> {
    while let Some(field) = multipart.next_field().await.unwrap_or(None) {
        let filename = field.file_name().unwrap_or("file").to_string();
        if field.name().unwrap_or("") != "file" {
            continue;
        }
        let data = field.bytes().await
            .map_err(|e| (axum::http::StatusCode::BAD_REQUEST, Json(serde_json::json!({ "error": e.to_string() }))))?;

        let ext = filename.rsplit('.').next()
            .filter(|value| value.len() <= 16 && value.chars().all(|c| c.is_ascii_alphanumeric()))
            .unwrap_or("bin");
        let file_id = format!("{}.{}", Uuid::new_v4(), ext);
        let storage_class = if params.storage_class.as_deref() == Some("permanent") {
            "permanent"
        } else {
            "temporary"
        };
        let key = format!("{}/{}", storage_class, file_id);

        // PaperPhoneLite deliberately keeps uploaded files on this server.
        let upload_dir = format!("{}/{}", state.config.upload_dir, storage_class);
        tokio::fs::create_dir_all(&upload_dir).await
            .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": format!("Upload directory failed: {}", e) }))))?;
        let file_path = format!("{}/{}", upload_dir, file_id);
        tokio::fs::write(&file_path, &data).await
            .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": format!("Local save failed: {}", e) }))))?;

        let url = format!("/api/files/{}", key);
        return Ok(Json(serde_json::json!({ "url": url, "key": key })));
    }

    Err((axum::http::StatusCode::BAD_REQUEST, Json(serde_json::json!({ "error": "No file uploaded" }))))
}

#[derive(Default, Deserialize)]
struct UploadParams {
    storage_class: Option<String>,
}
