// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod utils;
mod templates;
mod services;

use crate::services::create_infra::check_service;
use crate::services::docker_hub::{fetch_docker_repositories, search_docker_repositories, get_docker_config_by_name};

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            check_service,
            fetch_docker_repositories,
            search_docker_repositories,
            get_docker_config_by_name
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
