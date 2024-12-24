use std::fs::File;
use std::path::Path;
use serde_json::Value;

use crate::services::create_infra::Service;

#[tauri::command]
pub fn generate_docker_compose(services: Vec<Service>, folder_path: String) -> Result<String, String> {
    // Initialize the docker-compose structure
    let mut docker_compose = serde_json::map::Map::new();

    // Add version and services key
    docker_compose.insert("version".to_string(), Value::String("3.8".to_string()));
    let mut service_map = serde_json::map::Map::new();

    // Iterate through the services and add them
    for service in services {
        let mut service_config = serde_json::map::Map::new();
        
        // Parse the config if it exists
        if let Some(config_str) = service.config {
            let config: Value = serde_json::from_str(&config_str)
                .map_err(|e| format!("Error parsing config for {}: {}", service.label, e))?;
            
            if let Some(ports) = config.get("ports") {
                service_config.insert("ports".to_string(), ports.clone());
            }
            if let Some(restart) = config.get("restart") {
                service_config.insert("restart".to_string(), restart.clone());
            }
            if let Some(image) = config.get("image") {
                service_config.insert("image".to_string(), image.clone());
            }
            if let Some(volumes) = config.get("volumes") {
                service_config.insert("volumes".to_string(), volumes.clone());
            }
            if let Some(environment) = config.get("environment") {
                service_config.insert("environment".to_string(), environment.clone());
            }
            if let Some(command) = config.get("command") {
                service_config.insert("command".to_string(), command.clone());
            }
        }

        // If no config provided, insert an empty object (or handle as needed)
        if service_config.is_empty() {
            service_config.insert("image".to_string(), Value::String("default_image".to_string()));
        }

        // Add the service to the services map
        service_map.insert(service.name.clone(), Value::Object(service_config));
    }

    docker_compose.insert("services".to_string(), Value::Object(service_map));

    // Write to the file
    let path = Path::new(&folder_path).join("docker-compose.yml");
    let file = File::create(&path).map_err(|e| format!("Error creating file: {}", e))?;
    serde_yaml::to_writer(file, &docker_compose).map_err(|e| format!("Error writing to file: {}", e))?;

    Ok(format!("docker-compose.yml created at: {}", path.display()))
}
