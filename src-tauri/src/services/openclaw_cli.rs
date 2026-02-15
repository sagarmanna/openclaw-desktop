// Stub wrapper around OpenClaw CLI.
// Later: download/install CLI, run it with safe allowlist, stream logs, etc.

use crate::core::errors::{AppResult};

pub fn setup_openclaw_stub() -> AppResult<String> {
  Ok("OpenClaw setup stub complete (replace with real installer).".to_string())
}
