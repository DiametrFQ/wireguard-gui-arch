use std::{fs, process::Command };
use dirs::home_dir;

const SAVE_CONFIGS_DIRECTORY: &str = ".config/wireguard";

#[tauri::command]
fn save_config_file(file_name: String, content: String) -> Result<(), String> {
    let config_path = home_dir()
        .ok_or("Failed to get home directory")?
        .join(SAVE_CONFIGS_DIRECTORY);
    
    fs::create_dir_all(&config_path).map_err(|e| e.to_string())?;
    let file_path = config_path.join(file_name);
    fs::write(&file_path, content).map_err(|e| e.to_string())?;
    
    Ok(())
}

#[tauri::command]
fn get_active_tunnels() -> Vec<String> {
    let output = Command::new("wg")
        .arg("show")
        .arg("interfaces")
        .output()
        .expect("Failed to execute wg show interfaces");

    let stdout = String::from_utf8_lossy(&output.stdout);
    
    stdout
        .trim()
        .split_whitespace()
        .map(|s| s.to_string())
        .collect()
}

#[tauri::command]
fn list_tunnels() -> Result<Vec<String>, String> {
    let config_path = home_dir()
        .ok_or("Failed to get home directory")?
        .join(SAVE_CONFIGS_DIRECTORY);
    
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
        .join(SAVE_CONFIGS_DIRECTORY)
        .join(file_name);

    let status = Command::new("wg-quick")
        .arg(action)
        .arg(config_path)
        .status()
        .map_err(|e| e.to_string())?;
    
    if status.success() {
        Ok(())
    } else {
        Err("Failed to toggle tunnel".into())
    }
}

#[tauri::command]
fn read_config_file(file_name: String) -> Result<String, String> {
    let config_path = home_dir()
        .ok_or("Failed to get home directory")?
        .join(".config/wireguard")
        .join(file_name);

    fs::read_to_string(&config_path).map_err(|e| e.to_string())
}

#[tauri::command]
fn delete_config_file(file_name: String) -> Result<(), String> {
    let config_path = home_dir()
        .ok_or("Failed to get home directory")?
        .join(".config/wireguard")
        .join(file_name);

    fs::remove_file(&config_path).map_err(|e| e.to_string())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            save_config_file, 
            list_tunnels, 
            toggle_tunnel, 
            read_config_file, 
            delete_config_file,
            get_active_tunnels
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}