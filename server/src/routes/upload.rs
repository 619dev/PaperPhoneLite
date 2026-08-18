use std::sync::Arc;
use axum::{Router, routing::post, extract::{State, Multipart}, Json};
use uuid::Uuid;

use crate::AppState;
use crate::auth::middleware::AuthUser;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", post(upload_file))
}

async fn upload_file(
    State(state): State<Arc<AppState>>,
    _auth: AuthUser,
    mut multipart: Multipart,
) -> Result<Json<serde_json::Value>, (axum::http::StatusCode, Json<serde_json::Value>)> {
    while let Some(field) = multipart.next_field().await.unwrap_or(None) {
        let filename = field.file_name().unwrap_or("file").to_string();
        let data = field.bytes().await
            .map_err(|e| (axum::http::StatusCode::BAD_REQUEST, Json(serde_json::json!({ "error": e.to_string() }))))?;

        let ext = filename.rsplit('.').next().unwrap_or("bin");
        let file_id = format!("{}.{}", Uuid::new_v4(), ext);
        let key = format!("uploads/{}", file_id);

        // PaperPhoneLite deliberately keeps uploaded files on this server.
        let upload_dir = &state.config.upload_dir;
        tokio::fs::create_dir_all(upload_dir).await.ok();
        let file_path = format!("{}/{}", upload_dir, file_id);
        tokio::fs::write(&file_path, &data).await
            .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": format!("Local save failed: {}", e) }))))?;

        let url = format!("/api/files/{}", key);
        return Ok(Json(serde_json::json!({ "url": url, "key": key })));
    }

    Err((axum::http::StatusCode::BAD_REQUEST, Json(serde_json::json!({ "error": "No file uploaded" }))))
}
