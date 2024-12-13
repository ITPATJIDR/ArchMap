use std::{path::PathBuf, process::Command};

pub fn run_ansible_command(playbook_path: PathBuf, extra_vars: Option<String>) {
    let base_dir = std::env::current_dir().expect("Failed to get current directory");
    let ansible_path = base_dir.join("src/utils/ansible/ansible_env/bin/ansible-playbook"); // Path to Ansible binary

    let mut command = Command::new(ansible_path);
    command.arg(playbook_path);

    // Add --extra-vars if provided
    if let Some(vars) = extra_vars {
        command.arg("--extra-vars").arg(vars);
    }

    match command.output() {
        Ok(output) => {
            println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
            println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
        }
        Err(e) => {
            eprintln!("Failed to execute playbook: {}", e);
        }
    }
}

