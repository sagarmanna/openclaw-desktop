use crate::models::settings::Settings;

pub fn sandbox_enabled(settings: &Settings) -> bool {
  settings.sandbox_enabled
}
