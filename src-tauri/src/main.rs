// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use crate::utils::{set_window_shadow, init};
use crate::database::create_database;

mod utils;
mod database;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn create_db() -> String {
    let _ = create_database();
    "Database created!".to_string()
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            set_window_shadow(app);
            init(app);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![create_db])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
