// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use crate::backend::start_backend;
use crate::database::sqlite_database::{
    create_database, create_table, delete_data, insert_data, select_data, update_data,
};
use crate::utils::window_style::{set_window_blur, set_window_shadow};

mod backend;
mod database;
mod utils;

use std::sync::mpsc::sync_channel;
use tauri::WindowEvent;

fn main() {
    let (tx, rx) = sync_channel(1);
    start_backend(rx);

    tauri::Builder::default()
        .setup(|app| {
            set_window_shadow(app);
            set_window_blur(app);
            Ok(())
        })
        .on_window_event(move |event| match event.event() {
            WindowEvent::Destroyed => {
                tx.send(-1).expect("[Error] 发送消息");
                println!("[Event] 应用程序已关闭，正在关闭 API...");
            }
            _ => {}
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
        .expect("[Error] 运行 Tauri 应用程序时出错");
}
