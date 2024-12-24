use serde::{Deserialize, Serialize};

use crate::utils::ansible::ansible_managers::run_ansible_command;
use crate::templates::playbook_manager::PlaybookManager;

use crate::utils::docker::docker_compose_managers::generate_docker_compose;

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Service {
    pub label: String,
    pub name: String,
    pub service_type: String,
    pub config: Option<String>,
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


    let filter_service: Vec<Service> = service.iter()
    .filter(|&item| item.service_type == "services")
    .cloned()  
    .collect();

    let filter_docker_compose: Vec<Service> = service.iter()
    .filter(|&item| item.service_type == "docker-compose")
    .cloned()  
    .collect();

    // - Create Service
    create_service(filter_service, &manager, new_dir.clone());


    // - Create Docker Compose
    
    match  generate_docker_compose(filter_docker_compose, new_dir.clone()) {
        Ok(r) => println!("{:?}", r),
        Err(e) => println!("{:?}", e)
    }
    
    // - Setup services

    Ok(format!("Service list: {:?}", service))
}

fn create_service(service_list: Vec<Service>, manager: &PlaybookManager, new_dir: String) {
    for item in &service_list {
        match manager.find_playbook(&item.name) {
            Some(playbook_path) => {
                let new_project_path = format!(" dirpath={} project_name={}", new_dir, item.label.to_lowercase() );
                run_ansible_command(playbook_path.clone(), Some(new_project_path));
            }
            None => println!("Playbook for 'check_dir' not found."),
        }
    }
}
