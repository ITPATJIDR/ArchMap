use std::collections::HashMap;

pub struct PlaybookManager {
    path_map: HashMap<String, String>,
}

impl PlaybookManager {
    // Constructor to initialize the PlaybookManager with predefined paths
    pub fn new() -> Self {
        let mut path_map: HashMap<String, String> = HashMap::new();

        // Add playbook paths here
        path_map.insert("React".to_string(), "/home/itpat/Code/Rust/automated_infra/src-tauri/src/templates/ansible_playbooks/react.yaml".to_string());
        path_map.insert("Express".to_string(), "/home/itpat/Code/Rust/automated_infra/src-tauri/src/templates/ansible_playbooks/express.yaml".to_string());
        path_map.insert("Check_dir".to_string(), "/home/itpat/Code/Rust/automated_infra/src-tauri/src/templates/ansible_playbooks/check_and_create_dir.yaml".to_string());

        PlaybookManager { path_map }
    }

    // Method to find a specific playbook by name
    pub fn find_playbook(&self, key: &str) -> Option<&String> {
        self.path_map.get(key)
    }
}
