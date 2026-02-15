use chrono::Utc;
use tauri::{AppHandle, State};

use crate::core::{app_state::AppState, errors::{AppError, AppResult}};
use crate::commands::system::{ensure_db_for_other, get_pool};
use crate::models::schedule::ScheduleUpsertPayload;
use crate::services::db;

fn upsert(pool: &r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>, payload: ScheduleUpsertPayload) -> AppResult<String> {
  let conn = pool.get().map_err(|e| AppError::Db(format!("pool get: {e}")))?;
  let id = payload.id.unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
  let _created_at = Utc::now().to_rfc3339();

  conn.execute(
    "INSERT INTO schedules (id, agent_id, cron, enabled)
     VALUES (?1, ?2, ?3, ?4)
     ON CONFLICT(id) DO UPDATE SET agent_id=excluded.agent_id, cron=excluded.cron, enabled=excluded.enabled",
    (&id, payload.agent_id, payload.cron, if payload.enabled { 1 } else { 0 })
  ).map_err(|e| AppError::Db(format!("schedule upsert: {e}")))?;

  Ok(id)
}

#[tauri::command]
pub fn upsert_schedule(app: AppHandle, state: State<AppState>, payload: ScheduleUpsertPayload) -> Result<serde_json::Value, String> {
  ensure_db_for_other(&app, &state).map_err(|e| e.to_string())?;
  let p = get_pool(&state).map_err(|e| e.to_string())?;

  let id = upsert(&p, payload).map_err(|e| e.to_string())?;
  let _ = db::insert_log(&p, "INFO", "Schedule upserted", Some(&format!("{{\"id\":\"{}\"}}", id)));

  Ok(serde_json::json!({ "ok": true, "id": id }))
}
