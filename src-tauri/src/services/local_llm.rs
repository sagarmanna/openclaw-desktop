use serde_json::json;

// Stub local model: tiny rule-based “intent”.
pub fn local_chat(text: &str) -> serde_json::Value {
  let t = text.to_lowercase();

  if t.contains("check") && t.contains("system") {
    return json!({
      "intent": "system_check",
      "message": "Local model: I can run system checks. Say “check my system”."
    });
  }

  if t.contains("setup") && t.contains("openclaw") {
    return json!({
      "intent": "setup_openclaw",
      "message": "Local model: I can start OpenClaw setup. Say “setup openclaw”."
    });
  }

  if t.contains("create") && t.contains("agent") {
    return json!({
      "intent": "create_agent",
      "name": "Meeting Summarizer",
      "role": "Analyst",
      "goal": "Summarize meeting notes and extract action items",
      "tools": ["notes", "calendar"],
      "config": { "mode": "summary+action_items" },
      "message": "Local model: I drafted an agent config. I’ll request approval if it’s public."
    });
  }

  json!({
    "intent": "chat",
    "message": format!("Local model (stub): I received: \"{}\"", text)
  })
}
