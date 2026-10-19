use crate::store::storage_paths;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct OnlineSubtitlePayload {
    pub(crate) playback_key: String,
    pub(crate) media_title: Option<String>,
    #[serde(default)]
    pub(crate) provider_id: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct DownloadOnlineSubtitlePayload {
    #[serde(default)]
    pub(crate) provider_id: String,
    pub(crate) download_id: String,
    #[serde(default)]
    pub(crate) file_id: Option<i64>,
    pub(crate) file_name: String,
    pub(crate) title: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct OnlineSubtitleSearchResult {
    pub(crate) id: String,
    pub(crate) provider_id: String,
    pub(crate) download_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) file_id: Option<i64>,
    pub(crate) title: String,
    pub(crate) file_name: String,
    pub(crate) language: String,
    pub(crate) downloads: Option<i64>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct OnlineSubtitleDownload {
    pub(crate) path: String,
    pub(crate) title: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct OnlineSubtitleCacheClearResult {
    pub(crate) removed_files: usize,
    pub(crate) removed_bytes: u64,
}

#[tauri::command]
pub(crate) async fn search_online_subtitles(
    _app: tauri::AppHandle,
    _payload: OnlineSubtitlePayload,
) -> Result<Vec<OnlineSubtitleSearchResult>, String> {
    Ok(Vec::new())
}

#[tauri::command]
pub(crate) async fn download_online_subtitle(
    _app: tauri::AppHandle,
    _payload: DownloadOnlineSubtitlePayload,
) -> Result<OnlineSubtitleDownload, String> {
    Err("Online subtitles have been disabled.".to_string())
}

#[tauri::command]
pub(crate) async fn clear_online_subtitle_cache(
    app: tauri::AppHandle,
) -> Result<OnlineSubtitleCacheClearResult, String> {
    tauri::async_runtime::spawn_blocking(move || {
        let dir = match storage_paths::app_data_dir(&app) {
            Ok(base) => base.join("online_subtitles"),
            Err(_) => {
                return Ok(OnlineSubtitleCacheClearResult {
                    removed_files: 0,
                    removed_bytes: 0,
                })
            }
        };
        let mut removed_files = 0usize;
        let mut removed_bytes = 0u64;
        if let Ok(read_dir) = std::fs::read_dir(&dir) {
            for entry in read_dir.flatten() {
                if let Ok(metadata) = entry.metadata() {
                    if metadata.is_file() {
                        if std::fs::remove_file(entry.path()).is_ok() {
                            removed_files += 1;
                            removed_bytes += metadata.len();
                        }
                    }
                }
            }
        }
        Ok(OnlineSubtitleCacheClearResult {
            removed_files,
            removed_bytes,
        })
    })
    .await
    .map_err(|error| error.to_string())?
}
