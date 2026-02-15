use std::{fs, path::PathBuf};

use chrono::Utc;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::OptionalExtension;

use crate::core::errors::{AppError, AppResult};
use crate::models::{settings::Settings, log::LogRow};

const MIGRATION_001: &str = include_str!("../migrations/001_init.sql");

pub fn init_pool(db_path: PathBuf) -> AppResult<Pool<SqliteConnectionManager>> {
  if let Some(parent) = db_path.parent() {
    fs::create_dir_all(parent)
      .map_err(|e| AppError::Internal(format!("create db dir: {e}")))?;
  }

  let manager = SqliteConnectionManager::file(db_path);
  let pool = Pool::new(manager).map_err(|e| AppError::Db(format!("pool: {e}")))?;

  // migrate
  {
    let conn = pool.get().map_err(|e| AppError::Db(format!("pool get: {e}")))?;
    conn.execute_batch(MIGRATION_001)
      .map_err(|e| AppError::Db(format!("migrate: {e}")))?;

    // ensure settings row exists
    let existing: Option<i64> = conn
      .query_row("SELECT id FROM settings WHERE id=1", [], |r| r.get(0))
      .optional()
      .map_err(|e| AppError::Db(format!("settings select: {e}")))?;

    if existing.is_none() {
      let s = Settings::default();
      conn.execute(
        "INSERT INTO settings (id, user_llm_provider, user_llm_key, sandbox_enabled)
         VALUES (1, ?1, ?2, ?3)",
        (
          s.user_llm_provider,
          s.user_llm_key,
          if s.sandbox_enabled { 1 } else { 0 },
        ),
      )
      .map_err(|e| AppError::Db(format!("settings insert: {e}")))?;
    }
  }

  Ok(pool)
}

pub fn load_settings(pool: &Pool<SqliteConnectionManager>) -> AppResult<Settings> {
  let conn = pool.get().map_err(|e| AppError::Db(format!("pool get: {e}")))?;
  let (provider, key, sandbox): (String, Option<String>, i64) = conn
    .query_row(
      "SELECT user_llm_provider, user_llm_key, sandbox_enabled FROM settings WHERE id=1",
      [],
      |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)),
    )
    .map_err(|e| AppError::Db(format!("settings read: {e}")))?;

  Ok(Settings {
    user_llm_provider: provider,
    user_llm_key: key,
    sandbox_enabled: sandbox != 0,
  })
}

pub fn save_settings(pool: &Pool<SqliteConnectionManager>, s: Settings) -> AppResult<()> {
  let conn = pool.get().map_err(|e| AppError::Db(format!("pool get: {e}")))?;
  conn
    .execute(
      "UPDATE settings
       SET user_llm_provider=?1, user_llm_key=?2, sandbox_enabled=?3
       WHERE id=1",
      (
        s.user_llm_provider,
        s.user_llm_key,
        if s.sandbox_enabled { 1 } else { 0 },
      ),
    )
    .map_err(|e| AppError::Db(format!("settings update: {e}")))?;
  Ok(())
}

pub fn insert_log(pool: &Pool<SqliteConnectionManager>, level: &str, message: &str, meta: Option<&str>) -> AppResult<()> {
  let conn = pool.get().map_err(|e| AppError::Db(format!("pool get: {e}")))?;
  let id = uuid::Uuid::new_v4().to_string();
  let created_at = Utc::now().to_rfc3339();
  conn.execute(
    "INSERT INTO logs (id, level, message, meta_json, created_at) VALUES (?1, ?2, ?3, ?4, ?5)",
    (id, level, message, meta.map(|s| s.to_string()), created_at),
  ).map_err(|e| AppError::Db(format!("log insert: {e}")))?;
  Ok(())
}

pub fn list_logs(pool: &Pool<SqliteConnectionManager>, limit: i64) -> AppResult<Vec<LogRow>> {
  let conn = pool.get().map_err(|e| AppError::Db(format!("pool get: {e}")))?;
  let mut stmt = conn
    .prepare("SELECT id, level, message, meta_json, created_at FROM logs ORDER BY created_at DESC LIMIT ?1")
    .map_err(|e| AppError::Db(format!("logs prepare: {e}")))?;

  let rows = stmt
    .query_map([limit], |r| {
      Ok(LogRow {
        id: r.get(0)?,
        level: r.get(1)?,
        message: r.get(2)?,
        meta_json: r.get(3)?,
        created_at: r.get(4)?,
      })
    })
    .map_err(|e| AppError::Db(format!("logs query: {e}")))?;

  let mut out = Vec::new();
  for row in rows {
    out.push(row.map_err(|e| AppError::Db(format!("logs row: {e}")))?);
  }
  Ok(out)
}
