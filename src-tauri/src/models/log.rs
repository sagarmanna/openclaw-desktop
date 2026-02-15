use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogRow {
  pub id: String,
  pub level: String,
  pub message: String,
  pub meta_json: Option<String>,
  pub created_at: String,
}
