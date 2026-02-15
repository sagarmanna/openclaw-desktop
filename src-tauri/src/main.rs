#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod core;
mod services;
mod models;

use core::app_state::AppState;

fn main() {
  tauri::Builder::default()
    .manage(AppState::new())
    .invoke_handler(tauri::generate_handler![
      commands::system::detect_os,
      commands::system::check_deps,
      commands::openclaw::setup_openclaw,
      commands::llm::chat_intent,

      commands::system::load_settings,
      commands::system::save_settings,

      commands::agents::create_agent,
      commands::agents::list_agents,
      commands::agents::approve_action,

      commands::scheduler::upsert_schedule,
      commands::logs::list_logs,
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
