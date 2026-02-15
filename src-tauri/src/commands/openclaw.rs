use tauri::{AppHandle, State};

use crate::core::app_state::AppState;
use crate::commands::system::{ensure_db_for_other, get_pool};
use crate::services::{db, openclaw_cli};

#[tauri::command]
pub fn setup_openclaw(app: AppHandle, state: State<AppState>) -> Result<serde_json::Value, String> {
  ensure_db_for_other(&app, &state).map_err(|e| e.to_string())?;
  let p = get_pool(&state).map_err(|e| e.to_string())?;

  let msg = openclaw_cli::setup_openclaw_stub().map_err(|e| e.to_string())?;
  let _ = db::insert_log(&p, "INFO", "Ran OpenClaw setup stub", None);

  Ok(serde_json::json!({
    "ok": true,
    "message": msg
  }))
}
