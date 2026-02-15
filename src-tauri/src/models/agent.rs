use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Agent {
  pub id: String,
  pub name: String,
  pub role: String,
  pub goal: String,
  pub tools_json: String,
  pub config_json: String,
  pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentCreatePayload {
  pub name: String,
  pub role: String,
  pub goal: String,
  pub tools: Vec<String>,
  pub config: serde_json::Value,
}
