// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use crate::database::sqlite_database::{
    create_database, create_table, delete_data, insert_data, select_data, update_data
};
use crate::utils::window_style::{set_window_blur, set_window_shadow};

mod database;
mod utils;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            set_window_shadow(app);
            set_window_blur(app);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            create_database,
            create_table,
            insert_data,
            select_data,
            delete_data,
            update_data
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
