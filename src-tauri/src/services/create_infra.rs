use serde::{Deserialize, Serialize};

use crate::templates::service_configs_manager::ServiceConfig;
use crate::utils::ansible::ansible_managers::run_ansible_command;
use crate::templates::playbook_manager::PlaybookManager;

use crate::utils::docker::docker_compose_managers::docker_compose_genarate;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Service {
    pub label: String,
    pub name: String,
    pub service_type: String,
    pub config: Option<ServiceConfig>,
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
        if &item.service_type == "services"{
            match manager.find_playbook(&item.name) {
                Some(playbook_path) => {
                    let new_project_path = format!(" dirpath={} project_name={}", new_dir, item.label.to_lowercase() );
                    run_ansible_command(playbook_path.clone(), Some(new_project_path));
                }
                None => println!("Playbook for 'check_dir' not found."),
            }
        }else if &item.service_type == "docker-compose" {
            let _ = docker_compose_genarate(&item.name);
        }else {
            println!("Unknown service type")
        }
        
    }
    
    // - Setup services

    Ok(format!("Service list: {:?}", service))
}

