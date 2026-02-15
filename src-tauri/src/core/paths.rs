use std::path::PathBuf;
use tauri::{AppHandle, Manager};
use tauri::path::BaseDirectory;

use crate::core::errors::{AppError, AppResult};

pub fn app_data_dir(app: &AppHandle) -> AppResult<PathBuf> {
  let p = app
    .path()
    .resolve("openclaw_desktop", BaseDirectory::AppData)
    .map_err(|e| AppError::Internal(format!("resolve app data dir: {e}")))?;
  Ok(p)
}
