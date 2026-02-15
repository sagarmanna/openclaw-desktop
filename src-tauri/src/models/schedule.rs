use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduleUpsertPayload {
  pub id: Option<String>,
  pub agent_id: String,
  pub cron: String,
  pub enabled: bool,
}
