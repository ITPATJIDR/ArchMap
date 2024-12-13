use std::fs;
use std::path::{Path, PathBuf};
use std::io::{Error, ErrorKind};

pub struct DirectoryManager {
    base_path: PathBuf,
}

impl DirectoryManager {
    pub fn new(base_path: String) -> Self {
        DirectoryManager { base_path: PathBuf::from(base_path) }
    }

    pub fn create_directory(&self, dir_name: &str) -> Result<(), Error> {
        let path = Path::new(&self.base_path).join(dir_name);
        
        if path.exists() {
            return Err(Error::new(
                ErrorKind::AlreadyExists,
                format!("Directory '{}' already exists", dir_name)
            ));
        }

        fs::create_dir_all(&path)?;
        println!("Created directory: {}", path.display());
        Ok(())
    }

    pub fn directory_exists(&self, dir_name: &str) -> bool {
        let path = Path::new(&self.base_path).join(dir_name);
        path.exists() && path.is_dir()
    }

    pub fn remove_directory(&self, dir_name: &str) -> Result<(), Error> {
        let path = Path::new(&self.base_path).join(dir_name);
        
        if !path.exists() {
            return Err(Error::new(
                ErrorKind::NotFound,
                format!("Directory '{}' does not exist", dir_name)
            ));
        }

        fs::remove_dir_all(&path)?;
        println!("Removed directory: {}", path.display());
        Ok(())
    }

    pub fn get_current_path(&self) -> String {
        self.base_path.to_string_lossy().into_owned()
    }

    pub fn change_current_path(&mut self, new_path: &str) -> Result<(), Error> {
        let new_base_path = PathBuf::from(new_path);

        if !new_base_path.exists() {
            return Err(Error::new(
                ErrorKind::NotFound,
                format!("Path '{}' does not exist", new_path)
            ));
        }

        if !new_base_path.is_dir() {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                format!("Path '{}' is not a directory", new_path)
            ));
        }

        self.base_path = new_base_path;
        Ok(())
    }
}
