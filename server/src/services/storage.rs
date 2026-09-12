use std::{
    collections::HashSet,
    path::{Component, Path, PathBuf},
    time::{Duration, SystemTime},
};

use crate::{config::Config, AppState};

fn safe_file_name(value: &str) -> Option<String> {
    let value = value.trim_start_matches('/');
    let value = value.strip_prefix("uploads/").unwrap_or(value);
    let value = value
        .strip_prefix("permanent/")
        .or_else(|| value.strip_prefix("temporary/"))
        .unwrap_or(value);
    let path = Path::new(value);
    if value.is_empty()
        || path
            .components()
            .any(|part| !matches!(part, Component::Normal(_)))
        || path.file_name().and_then(|name| name.to_str()) != Some(value)
    {
        return None;
    }
    Some(value.to_string())
}

fn file_name_from_url(url: &str) -> Option<String> {
    if let Some((_, tail)) = url.split_once("/api/files/") {
        return safe_file_name(tail);
    }
    safe_file_name(url)
}

async fn permanent_avatar_files(state: &AppState) -> HashSet<String> {
    sqlx::query_scalar::<_, String>(
        "SELECT avatar FROM users WHERE avatar IS NOT NULL AND avatar <> '' UNION ALL \
         SELECT avatar FROM `groups` WHERE avatar IS NOT NULL AND avatar <> ''",
    )
    .fetch_all(&state.db)
    .await
    .unwrap_or_else(|error| {
        tracing::warn!(
            "Unable to inspect avatar URLs during upload migration: {}",
            error
        );
        Vec::new()
    })
    .into_iter()
    .filter_map(|url| file_name_from_url(&url))
    .collect()
}

/// Automatically classifies files created by versions that stored every upload
/// directly in UPLOAD_DIR. Avatar files are permanent; all other legacy uploads
/// are treated as expiring chat attachments.
pub async fn migrate_legacy_uploads(state: &AppState) {
    let root = Path::new(&state.config.upload_dir);
    let permanent = permanent_avatar_files(state).await;
    let Ok(mut entries) = tokio::fs::read_dir(root).await else {
        return;
    };
    let mut migrated = 0_u64;
    while let Ok(Some(entry)) = entries.next_entry().await {
        let Ok(kind) = entry.file_type().await else {
            continue;
        };
        if !kind.is_file() {
            continue;
        }
        let name = entry.file_name().to_string_lossy().to_string();
        if name.starts_with('.') || safe_file_name(&name).is_none() {
            continue;
        }
        let class = if permanent.contains(&name) {
            "permanent"
        } else {
            "temporary"
        };
        let destination = root.join(class).join(&name);
        if destination.exists() {
            tracing::warn!(
                "Legacy upload migration skipped duplicate destination: {}",
                destination.display()
            );
            continue;
        }
        match tokio::fs::rename(entry.path(), &destination).await {
            Ok(()) => migrated += 1,
            Err(error) => tracing::warn!("Unable to migrate legacy upload {}: {}", name, error),
        }
    }
    if migrated > 0 {
        tracing::info!(
            "Classified {} legacy uploads into permanent and temporary storage",
            migrated
        );
    }
}

pub async fn cleanup_temporary_files(config: &Config) {
    if config.chat_file_retention_days == 0 {
        return;
    }
    let cutoff = SystemTime::now()
        .checked_sub(Duration::from_secs(
            config.chat_file_retention_days.saturating_mul(86_400),
        ))
        .unwrap_or(SystemTime::UNIX_EPOCH);
    let directory = PathBuf::from(&config.upload_dir).join("temporary");
    let Ok(mut entries) = tokio::fs::read_dir(&directory).await else {
        return;
    };
    let mut removed = 0_u64;
    while let Ok(Some(entry)) = entries.next_entry().await {
        let Ok(kind) = entry.file_type().await else {
            continue;
        };
        if !kind.is_file() {
            continue;
        }
        let expired = entry
            .metadata()
            .await
            .ok()
            .and_then(|metadata| metadata.modified().ok())
            .map(|modified| modified < cutoff)
            .unwrap_or(false);
        if expired && tokio::fs::remove_file(entry.path()).await.is_ok() {
            removed += 1;
        }
    }
    if removed > 0 {
        tracing::info!(
            "Temporary chat file cleanup removed {} files older than {} days",
            removed,
            config.chat_file_retention_days
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_legacy_and_classified_file_names() {
        assert_eq!(
            file_name_from_url("/api/files/uploads/a.jpg").as_deref(),
            Some("a.jpg")
        );
        assert_eq!(
            file_name_from_url("https://server/api/files/permanent/a.jpg").as_deref(),
            Some("a.jpg")
        );
        assert_eq!(
            file_name_from_url("temporary/a.jpg").as_deref(),
            Some("a.jpg")
        );
    }

    #[test]
    fn rejects_nested_and_traversal_paths() {
        assert!(safe_file_name("../secret").is_none());
        assert!(safe_file_name("nested/a.jpg").is_none());
        assert!(safe_file_name("").is_none());
    }
}
