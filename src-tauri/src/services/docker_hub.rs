
use reqwest;
use serde::Deserialize;
use tauri::command;

use crate::templates::service_configs_manager::ServiceConfigManager;

#[derive(Deserialize, Debug)]
struct Repository {
    name: String,
}

#[derive(Deserialize, Debug)]
struct ApiResponse {
    results: Vec<Repository>,
}

#[derive(Deserialize, Debug)]
struct SearchRepository {
    repo_name: String,
}

#[derive(Deserialize, Debug)]
struct SearchApiResponse {
    results: Vec<SearchRepository>,
}


#[command]
pub async fn fetch_docker_repositories(page: u32, page_size: u32) -> Result<Vec<String>, String> {
    let url = format!(
        "https://hub.docker.com/v2/repositories/library/?page={}&page_size={}",
        page, page_size
    );

    let response: ApiResponse = reqwest::get(&url)
        .await
        .map_err(|e| e.to_string())?
        .json()
        .await
        .map_err(|e| e.to_string())?;

    Ok(response.results.into_iter().map(|repo| repo.name).collect())
}

#[command]
pub async fn search_docker_repositories(query: String, page: u32, page_size: u32) -> Result<Vec<String>, String> {
    if query.is_empty() {
        return Err("Query cannot be empty".to_string());
    }

    let url = format!(
        "https://hub.docker.com/v2/search/repositories/?page={}&page_size={}&query={}",
        page, page_size, query
    );

    let response: SearchApiResponse = reqwest::get(&url)
        .await
        .map_err(|e| e.to_string())?
        .json()
        .await
        .map_err(|e| e.to_string())?;

    // Transform the repository names into the desired format
    let repo_names: Vec<String> = response
        .results
        .into_iter()
        .map(|repo| {
            repo.repo_name.split('?').last().unwrap_or(&repo.repo_name).to_string()
        })
        .collect();

    Ok(repo_names)
}


#[command]
pub async fn get_docker_config_by_name(service_name: &str) -> Result<String, String> {
    let base_dir = std::env::current_dir().map_err(|_| "Failed to get current directory")?;
    let service_config_path = base_dir.join("src/templates/docker_composes/service_configs.json");

    // Initialize ServiceConfigManager
    let config_manager = ServiceConfigManager::new(service_config_path)
        .map_err(|err| format!("Failed to initialize ServiceConfigManager: {}", err))?;

    // Find the service config by name
    match config_manager.find_service_config_by_name(service_name) {
        Some(service_config) => {
            serde_json::to_string(&service_config)
                .map_err(|err| format!("Failed to serialize service config: {}", err))
        }
        None => {
            let default_service_config = serde_json::json!({
                "ports": ["8000:8000"],
                "restart": "always",
            });

            serde_json::to_string(&default_service_config)
                .map_err(|err| format!("Failed to serialize default service config: {}", err))
        }
    }
}
