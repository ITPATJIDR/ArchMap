use serde::{Deserialize, Serialize};

use crate::utils::ansible::ansible_managers::run_ansible_command;
use crate::templates::playbook_manager::PlaybookManager;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Service {
    pub label: String,
    pub service_type: String,
}


#[tauri::command]
pub fn check_service(service: Vec<Service>, path: Vec<String>) -> Result<String, String> {
    if service.is_empty() {
        return Ok("No services provided.".to_string());
    }

    if path.is_empty() {
        return Ok("No path provided.".to_string());
    }

    let manager = PlaybookManager::new();

    let new_dir = format!("{}/{}", path[0], "test_automated");

    // TODO
    // - Create a new directory and check if it exists
        // - Craft command to check dir exists
    match manager.find_playbook("Check_dir") {
        Some(playbook_path) => {
            let check_dir_command = format!(" directory_path={}", new_dir);
            run_ansible_command(playbook_path.clone(), Some(check_dir_command));
        }
        None => println!("Playbook for 'check_dir' not found."),
    }

    // - Find Templates
    for item in &service {
        match manager.find_playbook(&item.service_type) {
            Some(playbook_path) => {
                let new_project_path = format!("{}/{}", new_dir, item.label);
                run_ansible_command(playbook_path.clone(), Some(new_project_path));
            }
            None => println!("Playbook for 'check_dir' not found."),
        }
    }
    
    // - Setup services

    Ok(format!("Service list: {:?}", service))
}

