use std::process::Command;
use std::env;


pub struct CliCommand {
    path: String,
}

impl CliCommand {

    pub fn new(path: String) -> Self {
        if let Err(e) = env::set_current_dir(&path) {
            panic!("Failed to change directory to {}: {}", path, e);
        }

        CliCommand {
            path: path,
        }
    }

    pub fn execute_command(&self, command: String) -> bool {
        // Change working directory programmatically
        if command.starts_with("cd ") {
            let path = command[3..].trim(); 
            if let Err(e) = env::set_current_dir(path) {
                eprintln!("Failed to change directory to {}: {}", path, e);
                return false;
            }
            println!("Changed directory to {}", path);
            return true;
        }

        match Command::new("sh")
            .arg("-c")
            .arg(command)
            .output()
        {
            Ok(output) => {
                println!("Command executed successfully.");
                println!("Stdout: {}", String::from_utf8_lossy(&output.stdout));
                true
            }
            Err(e) => {
                eprintln!("Failed to execute command: {}", e);
                false
            }
        }
    }


    pub fn get_current_path(&self) -> String {
        match Command::new("pwd").output() {
            Ok(output) => String::from_utf8_lossy(&output.stdout).trim().to_string(),
            Err(_) => panic!("Can't get current path"),
        }
    }

}
