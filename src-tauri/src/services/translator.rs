use serde_json::{json, Value};

// Convert LLM “intent” payload to an internal “plan”.
pub fn to_plan(llm_payload: &Value) -> Value {
  let intent = llm_payload.get("intent").and_then(|v| v.as_str()).unwrap_or("chat");

  match intent {
    "system_check" => json!({
      "kind": "system_check",
      "steps": ["detect_os", "check_deps"]
    }),
    "setup_openclaw" => json!({
      "kind": "setup_openclaw",
      "steps": ["setup_openclaw"]
    }),
    "create_agent" => json!({
      "kind": "create_agent",
      "agent": {
        "name": llm_payload.get("name"),
        "role": llm_payload.get("role"),
        "goal": llm_payload.get("goal"),
        "tools": llm_payload.get("tools"),
        "config": llm_payload.get("config")
      }
    }),
    _ => json!({
      "kind": "chat",
      "steps": []
    })
  }
}
