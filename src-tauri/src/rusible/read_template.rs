use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

// Define a struct that matches your YAML structure
#[derive(Debug, Serialize, Deserialize)]
pub struct Template {
	pub service_templates: ServiceTemplates,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceTemplates {
	pub prerequisites: Vec<Step>,
	pub install_prerequisites: Vec<Step>,
	pub setup_service: Vec<Step>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Step {
    pub name: String,
    pub command: String,
    pub note: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Service {
    pub label: String,
    pub service_type: String,
}


impl Template {
    pub fn find_template(service: &Service) -> &str {
        let service_type = service.service_type.clone();         

        if service_type == "React" {
            return "/home/itpat/Code/Rust/automated_infra/src-tauri/src/templates/react.yaml";
        }else if service_type == "Express"  {
            return "/home/itpat/Code/Rust/automated_infra/src-tauri/src/templates/express.yaml";
        }else {
            return ""
        }
    }    

    pub fn load_from_file<P: AsRef<Path>>(file_path: P) -> Result<Template, Box<dyn std::error::Error>> {
        let yaml_content = fs::read_to_string(file_path)?; 
        let config: Template = serde_yaml::from_str(&yaml_content)?; 
        Ok(config)
    }
}
