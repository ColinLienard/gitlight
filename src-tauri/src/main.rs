#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod commands;

use std::sync::Arc;
use tokio::sync::Notify;

#[derive(Clone, Default)]
pub struct Model {
  pub notifiers: Notifiers,
}

#[derive(Clone, Default)]
pub struct Notifiers {
  pub github_auth_ready: Arc<Notify>,
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
      commands::github_auth_flow,
      commands::github_auth_ready,
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
