use serde::Serialize;

#[derive(Clone, Debug)]
pub(crate) struct SoiaAuthToken {
    pub payload: String,
    pub signature_hex: String,
    pub expire_at: i64,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct UpdateNotePrompt {
    pub version: String,
    pub note: String,
}

#[tauri::command]
pub(crate) fn has_available_update() -> bool {
    false
}

#[tauri::command]
pub(crate) fn should_use_embedded_update_install() -> bool {
    false
}

#[tauri::command]
pub(crate) fn consume_pending_update_note_prompt() -> Option<UpdateNotePrompt> {
    None
}

pub(crate) fn check_update(_app_handle: tauri::AppHandle) -> Option<SoiaAuthToken> {
    None
}

#[cfg(target_os = "macos")]
pub(crate) fn check_update_now(_app_handle: tauri::AppHandle) {}

pub(crate) fn run_daily_signal_ping(_app_handle: &tauri::AppHandle) -> Option<SoiaAuthToken> {
    None
}
