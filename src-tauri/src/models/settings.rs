use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
  pub user_llm_provider: String,
  pub user_llm_key: Option<String>,
  pub sandbox_enabled: bool,
}

impl Default for Settings {
  fn default() -> Self {
    Self {
      user_llm_provider: "openai".to_string(),
      user_llm_key: None,
      sandbox_enabled: true,
    }
  }
}
