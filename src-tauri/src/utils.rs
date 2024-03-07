// 窗口样式
#![allow(unused_imports)]
use tauri::{Manager, Runtime, App};
use window_shadows::set_shadow;
use window_vibrancy::{self, NSVisualEffectMaterial};

// 窗口阴影
pub fn set_window_shadow<R: Runtime>(app: &tauri::App<R>) {
    let window = app.get_window("mainWindow").unwrap();
    set_shadow(&window, true).expect("Unsupported platform!");
}

/// 背景模糊
pub fn init(app: &mut App){
    let win = app.get_window("mainWindow").unwrap();

    // 仅在 macOS 下执行
    #[cfg(target_os = "macos")]
    window_vibrancy::apply_vibrancy(&win, NSVisualEffectMaterial::FullScreenUI)
        .expect("Unsupported platform! 'apply_vibrancy' is only supported on macOS");

    // 仅在 windows 下执行
    #[cfg(target_os = "windows")]
    window_vibrancy::apply_blur(&win, Some((18, 18, 18, 125)))
        .expect("Unsupported platform! 'apply_blur' is only supported on Windows");

}