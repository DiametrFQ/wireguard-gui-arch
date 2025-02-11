// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::process::Command;
use std::fs;
use std::io::{self, Write};
use dirs::home_dir;

fn get_wireguard_dir() -> Option<std::path::PathBuf> {
    home_dir().map(|home| home.join(".wireguard"))
}


#[tauri::command]
fn add_tunnel(config_content: String, tunnel_name: String) -> Result<(), String> {
    // Получаем домашний каталог пользователя
    let home_dir = home_dir().ok_or_else(|| "Failed to get home directory".to_string())?;
    
    // Формируем путь для конфигурации туннеля в домашнем каталоге
    let dest_path = home_dir.join(format!(".config/wireguard/{}.conf", tunnel_name));

    // Пытаемся записать конфигурацию в файл
    if let Err(e) = fs::write(&dest_path, config_content) {
        return Err(format!("Failed to write config: {}", e));
    }

    // Запускаем туннель с помощью wg-quick
    let output = std::process::Command::new("wg-quick")
        .arg("up")
        .arg(dest_path.to_str().unwrap())
        .output()
        .map_err(|e| format!("Failed to start tunnel: {}", e))?;

    if !output.status.success() {
        return Err("Failed to start tunnel".to_string());
    }

    Ok(())
}

#[tauri::command]
fn start_tunnel(interface: String) -> Result<String, String> {
    let output = Command::new("wg-quick")
        .arg("up")
        .arg(&interface)
        .output()
        .map_err(|e| format!("Failed to execute wg-quick: {}", e))?;
    
    if output.status.success() {
        Ok(format!("Tunnel {} started successfully", interface))
    } else {
        Err(format!("Error starting tunnel: {}", String::from_utf8_lossy(&output.stderr)))
    }
}

#[tauri::command]
fn stop_tunnel(interface: String) -> Result<String, String> {
    let output = Command::new("wg-quick")
        .arg("down")
        .arg(&interface)
        .output()
        .map_err(|e| format!("Failed to execute wg-quick: {}", e))?;
    
    if output.status.success() {
        Ok(format!("Tunnel {} stopped successfully", interface))
    } else {
        Err(format!("Error stopping tunnel: {}", String::from_utf8_lossy(&output.stderr)))
    }
}

#[tauri::command]
fn get_tunnel_status(interface: &str) -> Result<String, String> {
    let output = Command::new("wg")
        .arg("show")
        .arg(interface)
        .output()
        .map_err(|e| e.to_string())?;

    if !output.status.success() {
        return Err("Failed to get tunnel status".into());
    }

    let status = String::from_utf8_lossy(&output.stdout).to_string();
    Ok(status)
}

#[tauri::command]
fn list_available_tunnels() -> Result<Vec<String>, String> {
    let output = Command::new("ls")
        .arg("/etc/wireguard")
        .output()
        .map_err(|e| format!("Failed to list WireGuard configurations: {}", e))?;
    
    if output.status.success() {
        let files = String::from_utf8_lossy(&output.stdout)
            .lines()
            .filter(|f| f.ends_with(".conf"))
            .map(|f| f.trim_end_matches(".conf").to_string())
            .collect();
        Ok(files)
    } else {
        Err(format!("Error listing tunnels: {}", String::from_utf8_lossy(&output.stderr)))
    }
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![start_tunnel, stop_tunnel,add_tunnel])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    wireguard_ui_lib::run()
}

