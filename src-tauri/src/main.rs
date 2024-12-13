// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod create_infra;
mod rusible;
mod utils;
mod templates;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![create_infra::check_service])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
