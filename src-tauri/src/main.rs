use std::{fs, process::Command };

use dirs::home_dir;

#[tauri::command]
fn save_config_file(file_name: String, content: String) -> Result<(), String> {
    let config_path = home_dir()
        .ok_or("Failed to get home directory")?
        .join(".config/wireguard");
    
    fs::create_dir_all(&config_path).map_err(|e| e.to_string())?;
    let file_path = config_path.join(file_name);
    fs::write(&file_path, content).map_err(|e| e.to_string())?;
    
    Ok(())
}

#[tauri::command]
fn list_tunnels() -> Result<Vec<String>, String> {
    let config_path = home_dir()
        .ok_or("Failed to get home directory")?
        .join(".config/wireguard");
    
    let files = fs::read_dir(config_path)
        .map_err(|e| e.to_string())?
        .filter_map(|entry| entry.ok())
        .filter_map(|entry| entry.file_name().into_string().ok())
        .collect();
    
    Ok(files)
}

#[tauri::command]
fn toggle_tunnel(file_name: String, action: String) -> Result<(), String> {
    let config_path = home_dir()
        .ok_or("Failed to get home directory")?
        .join(".config/wireguard/");

    let status = Command::new("wg-quick")
        .arg(action)
        .arg(format!("{}{}", config_path.display(), file_name))
        .status()
        .map_err(|e| e.to_string())?;
    
    if status.success() {
        Ok(())
    } else {
        Err("Failed to toggle tunnel".into())
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            save_config_file, list_tunnels, toggle_tunnel
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
