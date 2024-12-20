use tauri::command;

#[command]
pub fn docker_compose_genarate(service_name: &String) -> Result<(), String> {
	println!("{:?}", service_name);
	Ok(())
}
