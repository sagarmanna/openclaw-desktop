use tauri::{AppHandle, State};

use crate::core::app_state::AppState;
use crate::commands::system::{ensure_db_for_other, get_pool};
use crate::services::{db, llm_router, translator};

#[tauri::command]
pub fn chat_intent(app: AppHandle, state: State<AppState>, text: String) -> Result<serde_json::Value, String> {
  ensure_db_for_other(&app, &state).map_err(|e| e.to_string())?;
  let p = get_pool(&state).map_err(|e| e.to_string())?;

  let settings = db::load_settings(&p).map_err(|e| e.to_string())?;
  let payload = llm_router::route(&settings, &text);
  let plan = translator::to_plan(&payload);

  // Example: if plan implies “public action”, require approval (stub rule)
  // Here: if user says "post" or "publish" we request approval.
  let lower = text.to_lowercase();
  if lower.contains("post") || lower.contains("publish") {
    let preview = serde_json::json!({
      "action": "post_public_content",
      "draft": payload.get("message").cloned().unwrap_or(serde_json::Value::String(text.clone()))
    })
    .to_string();

    // create approval row
    let approval_id = crate::services::approval::create_approval(&p, "unknown_agent", "public_post", &preview)
      .map_err(|e| e.to_string())?;

    let _ = db::insert_log(&p, "INFO", "Approval created", Some(&preview));

    return Ok(serde_json::json!({
      "approval_required": true,
      "approval_id": approval_id,
      "action_type": "public_post",
      "preview_json": preview,
      "message": "This looks like a public action. Please approve or reject in the approval card."
    }));
  }

  // Handle plan steps for some intents
  if plan.get("kind").and_then(|v| v.as_str()) == Some("system_check") {
    let os = crate::commands::system::detect_os();
    let deps = crate::commands::system::check_deps();
    let msg = format!(
      "System check:\n• OS: {} ({})\n• Deps: {}",
      os.get("os").unwrap_or(&serde_json::Value::String("?".into())),
      os.get("arch").unwrap_or(&serde_json::Value::String("?".into())),
      serde_json::to_string_pretty(&deps).unwrap_or_default()
    );
    let _ = db::insert_log(&p, "INFO", "System check run", None);
    return Ok(serde_json::json!({ "message": msg }));
  }

  if plan.get("kind").and_then(|v| v.as_str()) == Some("setup_openclaw") {
    let _ = db::insert_log(&p, "INFO", "User requested OpenClaw setup", None);
    return Ok(serde_json::json!({
      "message": "Say “setup openclaw” (or click) — or type it again and I will run the setup command."
    }));
  }

  Ok(serde_json::json!({
    "message": payload.get("message").cloned().unwrap_or(serde_json::Value::String("OK".into()))
  }))
}
