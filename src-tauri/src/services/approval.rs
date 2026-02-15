use chrono::Utc;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;

use crate::core::errors::{AppError, AppResult};

pub fn create_approval(
  pool: &Pool<SqliteConnectionManager>,
  agent_id: &str,
  action_type: &str,
  preview_json: &str,
) -> AppResult<String> {
  let conn = pool.get().map_err(|e| AppError::Db(format!("pool get: {e}")))?;
  let id = uuid::Uuid::new_v4().to_string();
  let created_at = Utc::now().to_rfc3339();

  conn.execute(
    "INSERT INTO approvals (id, agent_id, action_type, preview_json, approved, created_at)
     VALUES (?1, ?2, ?3, ?4, 0, ?5)",
    (id.clone(), agent_id, action_type, preview_json, created_at),
  )
  .map_err(|e| AppError::Db(format!("approval insert: {e}")))?;

  Ok(id)
}

pub fn set_approval(pool: &Pool<SqliteConnectionManager>, approval_id: &str, approved: bool) -> AppResult<()> {
  let conn = pool.get().map_err(|e| AppError::Db(format!("pool get: {e}")))?;
  conn.execute(
    "UPDATE approvals SET approved=?1 WHERE id=?2",
    (if approved { 1 } else { 0 }, approval_id),
  )
  .map_err(|e| AppError::Db(format!("approval update: {e}")))?;
  Ok(())
}
