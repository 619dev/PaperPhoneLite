use std::sync::Arc;
use axum::{Router, routing::get, extract::{State, Path}, response::IntoResponse, http::{header, HeaderValue, StatusCode}};
use std::path::{Component, Path as FsPath};

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
    if FsPath::new(local_name).components().any(|part| !matches!(part, Component::Normal(_))) {
        return (StatusCode::BAD_REQUEST, "Invalid file path").into_response();
    }
    let file_path = format!("{}/{}", state.config.upload_dir, local_name);
    match tokio::fs::read(&file_path).await {
        Ok(data) => {
            let content_type = mime_guess::from_path(&file_path)
                .first_or_octet_stream()
                .to_string();
            let mut response = data.into_response();
            response.headers_mut().insert(
                header::CONTENT_TYPE,
                HeaderValue::from_str(&content_type).unwrap_or_else(|_| HeaderValue::from_static("application/octet-stream")),
            );
            response.headers_mut().insert(header::CACHE_CONTROL, HeaderValue::from_static("private, max-age=31536000, immutable"));
            response.headers_mut().insert(header::X_CONTENT_TYPE_OPTIONS, HeaderValue::from_static("nosniff"));
            response
        }
        Err(_) => (StatusCode::NOT_FOUND, "File not found").into_response(),
    }
}
