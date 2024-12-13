use std::collections::HashMap;
use std::path::PathBuf;

pub struct PlaybookManager {
    path_map: HashMap<String, PathBuf>,
}

impl PlaybookManager {
    // Constructor to initialize the PlaybookManager with predefined paths
    pub fn new() -> Self {
        let mut path_map: HashMap<String, PathBuf> = HashMap::new();
        let base_dir = std::env::current_dir().expect("Failed to get current directory");
        

        // Add playbook paths here
        
        path_map.insert("React".to_string(),base_dir.join("src/templates/ansible_playbooks/react.yaml"));
        path_map.insert("Express".to_string(),base_dir.join("src/templates/ansible_playbooks/express.yaml"));
        path_map.insert("Check_dir".to_string(),base_dir.join("src/templates/ansible_playbooks/check_and_create_dir.yaml"));

        PlaybookManager { path_map }
    }

    // Method to find a specific playbook by name
    pub fn find_playbook(&self, key: &str) -> Option<&PathBuf> {
        self.path_map.get(key)
    }
}
