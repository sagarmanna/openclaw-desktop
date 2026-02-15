use std::sync::Mutex;

use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;

#[derive(Default)]
pub struct AppState {
  // Initialized lazily when first command runs
  pub db_pool: Mutex<Option<Pool<SqliteConnectionManager>>>,
}

impl AppState {
  pub fn new() -> Self {
    Self::default()
  }
}
