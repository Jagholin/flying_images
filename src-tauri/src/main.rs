#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod commands;
mod state;
mod request;

use commands::*;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![check_directory, create_workspace, open_workspace, get_csrf_token])
        .manage(state::state::ManagedState::new())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
