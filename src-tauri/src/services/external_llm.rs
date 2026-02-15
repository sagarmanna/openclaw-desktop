use serde_json::json;

// Stub external model: in real version, call OpenAI/Claude using user key.
pub fn external_chat(provider: &str, _key: &str, text: &str) -> serde_json::Value {
  json!({
    "intent": "chat",
    "message": format!("External model ({provider}, stub): You said: \"{text}\"")
  })
}
