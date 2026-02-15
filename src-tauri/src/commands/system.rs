use tauri::{AppHandle, State};

use crate::core::{app_state::AppState, errors::{AppError, AppResult}, paths};
use crate::models::settings::Settings;
use crate::services::db;

fn ensure_db(app: &AppHandle, state: &State<AppState>) -> AppResult<()> {
  let mut guard = state.db_pool.lock().map_err(|_| AppError::Internal("db_pool lock".into()))?;
  if guard.is_some() {
    return Ok(());
  }

  let dir = paths::app_data_dir(app)?;
  let db_path = dir.join("openclaw.db");
  let pool = db::init_pool(db_path)?;
  *guard = Some(pool);
  Ok(())
}

fn pool(state: &State<AppState>) -> AppResult<r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>> {
  let guard = state.db_pool.lock().map_err(|_| AppError::Internal("db_pool lock".into()))?;
  guard.clone().ok_or_else(|| AppError::Internal("db not initialized".into()))
}

#[tauri::command]
pub fn detect_os() -> serde_json::Value {
  serde_json::json!({
    "os": std::env::consts::OS,
    "arch": std::env::consts::ARCH
  })
}

#[tauri::command]
pub fn check_deps() -> serde_json::Value {
  // Stub: you’ll add real checks for python/node/openclaw/etc.
  serde_json::json!({
    "deps": [
      { "name": "node", "ok": true },
      { "name": "rust", "ok": true },
      { "name": "openclaw_cli", "ok": false, "hint": "Not installed yet. Run 'setup openclaw'." }
    ]
  })
}

#[tauri::command]
pub fn load_settings(app: AppHandle, state: State<AppState>) -> Result<Settings, String> {
  ensure_db(&app, &state).map_err(|e| e.to_string())?;
  let p = pool(&state).map_err(|e| e.to_string())?;
  db::load_settings(&p).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn save_settings(app: AppHandle, state: State<AppState>, settings: Settings) -> Result<(), String> {
  ensure_db(&app, &state).map_err(|e| e.to_string())?;
  let p = pool(&state).map_err(|e| e.to_string())?;
  db::save_settings(&p, settings).map_err(|e| e.to_string())?;
  db::insert_log(&p, "INFO", "Settings updated", None).ok();
  Ok(())
}

// Re-export helpers for other command modules
pub(crate) fn ensure_db_for_other(app: &AppHandle, state: &State<AppState>) -> AppResult<()> {
  ensure_db(app, state)
}
pub(crate) fn get_pool(state: &State<AppState>) -> AppResult<r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>> {
  pool(state)
}
