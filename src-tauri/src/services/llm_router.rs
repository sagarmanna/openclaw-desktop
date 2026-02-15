use serde_json::Value;

use crate::models::settings::Settings;
use crate::services::{external_llm, local_llm};

pub fn route(settings: &Settings, text: &str) -> Value {
  if let Some(k) = settings.user_llm_key.as_ref().filter(|s| !s.trim().is_empty()) {
    return external_llm::external_chat(&settings.user_llm_provider, k, text);
  }
  local_llm::local_chat(text)
}
