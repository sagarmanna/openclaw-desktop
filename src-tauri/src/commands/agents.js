use chrono::Utc;
use tauri::{AppHandle, State};

use crate::core::{app_state::AppState, errors::{AppError, AppResult}};
use crate::commands::system::{ensure_db_for_other, get_pool};
use crate::models::agent::{Agent, AgentCreatePayload};
use crate::services::db;

fn insert_agent(pool: &r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>, payload: AgentCreatePayload) -> AppResult<Agent> {
  let conn = pool.get().map_err(|e| AppError::Db(format!("pool get: {e}")))?;
  let id = uuid::Uuid::new_v4().to_string();
  let created_at = Utc::now().to_rfc3339();

  let tools_json = serde_json::to_string(&payload.tools)
    .map_err(|e| AppError::Invalid(format!("tools json: {e}")))?;
  let config_json = serde_json::to_string(&payload.config)
    .map_err(|e| AppError::Invalid(format!("config json: {e}")))?;

  conn.execute(
    "INSERT INTO agents (id, name, role, goal, tools_json, config_json, created_at)
     VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
    (id.clone(), payload.name, payload.role, payload.goal, tools_json.clone(), config_json.clone(), created_at.clone())
  ).map_err(|e| AppError::Db(format!("agent insert: {e}")))?;

  Ok(Agent { id, name: conn.query_row("SELECT name FROM agents WHERE id=?1", [id.clone()], |r| r.get(0)).unwrap_or("".into()),
    role: conn.query_row("SELECT role FROM agents WHERE id=?1", [id.clone()], |r| r.get(0)).unwrap_or("".into()),
    goal: conn.query_row("SELECT goal FROM agents WHERE id=?1", [id.clone()], |r| r.get(0)).unwrap_or("".into()),
    tools_json, config_json, created_at })
}

fn list_agents_db(pool: &r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>) -> AppResult<Vec<Agent>> {
  let conn = pool.get().map_err(|e| AppError::Db(format!("pool get: {e}")))?;
  let mut stmt = conn.prepare(
    "SELECT id, name, role, goal, tools_json, config_json, created_at FROM agents ORDER BY created_at DESC"
  ).map_err(|e| AppError::Db(format!("agents prepare: {e}")))?;

  let rows = stmt.query_map([], |r| {
    Ok(Agent {
      id: r.get(0)?,
      name: r.get(1)?,
      role: r.get(2)?,
      goal: r.get(3)?,
      tools_json: r.get(4)?,
      config_json: r.get(5)?,
      created_at: r.get(6)?,
    })
  }).map_err(|e| AppError::Db(format!("agents query: {e}")))?;

  let mut out = Vec::new();
  for row in rows {
    out.push(row.map_err(|e| AppError::Db(format!("agents row: {e}")))?);
  }
  Ok(out)
}

#[tauri::command]
pub fn create_agent(app: AppHandle, state: State<AppState>, payload: AgentCreatePayload) -> Result<Agent, String> {
  ensure_db_for_other(&app, &state).map_err(|e| e.to_string())?;
  let p = get_pool(&state).map_err(|e| e.to_string())?;

  let agent = insert_agent(&p, payload).map_err(|e| e.to_string())?;
  let _ = db::insert_log(&p, "INFO", "Agent created", Some(&serde_json::to_string(&agent).unwrap_or_default()));
  Ok(agent)
}

#[tauri::command]
pub fn list_agents(app: AppHandle, state: State<AppState>) -> Result<Vec<Agent>, String> {
  ensure_db_for_other(&app, &state).map_err(|e| e.to_string())?;
  let p = get_pool(&state).map_err(|e| e.to_string())?;
  list_agents_db(&p).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn approve_action(app: AppHandle, state: State<AppState>, approval_id: String, approved: bool) -> Result<(), String> {
  ensure_db_for_other(&app, &state).map_err(|e| e.to_string())?;
  let p = get_pool(&state).map_err(|e| e.to_string())?;

  crate::services::approval::set_approval(&p, &approval_id, approved)
    .map_err(|e| e.to_string())?;

  let _ = db::insert_log(&p, "INFO", "Approval decision recorded", Some(&format!("{{\"id\":\"{}\",\"approved\":{}}}", approval_id, approved)));
  Ok(())
}
