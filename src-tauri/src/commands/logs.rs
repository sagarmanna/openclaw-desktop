use tauri::{AppHandle, State};

use crate::core::app_state::AppState;
use crate::commands::system::{ensure_db_for_other, get_pool};
use crate::models::log::LogRow;
use crate::services::db;

#[tauri::command]
pub fn list_logs(app: AppHandle, state: State<AppState>, limit: Option<i64>) -> Result<Vec<LogRow>, String> {
  ensure_db_for_other(&app, &state).map_err(|e| e.to_string())?;
  let p = get_pool(&state).map_err(|e| e.to_string())?;
  db::list_logs(&p, limit.unwrap_or(50)).map_err(|e| e.to_string())
}
