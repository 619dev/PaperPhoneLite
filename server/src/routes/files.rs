use std::sync::Arc;
use axum::{Router, routing::get, extract::{State, Path}, response::IntoResponse, http::{header, StatusCode}};

use crate::AppState;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/{*path}", get(proxy_file))
}

async fn proxy_file(
    State(state): State<Arc<AppState>>,
    Path(key): Path<String>,
) -> impl IntoResponse {
    // Serve only from the server-local upload directory.
    // key may be "uploads/{uuid}.ext" — strip the "uploads/" prefix since upload_dir IS the uploads folder
    let local_name = key.strip_prefix("uploads/").unwrap_or(&key);
    let file_path = format!("{}/{}", state.config.upload_dir, local_name);
    match tokio::fs::read(&file_path).await {
        Ok(data) => {
            let content_type = mime_guess::from_path(&file_path)
                .first_or_octet_stream()
                .to_string();
            (
                StatusCode::OK,
                [(header::CONTENT_TYPE, content_type), (header::CACHE_CONTROL, "public, max-age=31536000".to_string())],
                data,
            ).into_response()
        }
        Err(_) => (StatusCode::NOT_FOUND, "File not found").into_response(),
    }
}
