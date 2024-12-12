pub mod config;
pub mod windows_specific;

use tauri::{LogicalPosition, Manager, Position, WebviewEvent, Window, WindowEvent};
use windows_specific::attach;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn set_window_behind(window: &Window) {
    #[cfg(target_os = "windows")]
    windows_specific::set_window_behind(window.hwnd().unwrap());
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_window_state::Builder::new().build())
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            let app_handle = app.handle();
            let webview_window = app_handle.get_webview_window("snowmetr").unwrap();

            webview_window.set_skip_taskbar(true).unwrap();
            // webview_window.center().unwrap();
            // attach(webview_window.hwnd().unwrap()).unwrap();

            Ok(())
        })
        .on_window_event(|window, event| {
            set_window_behind(window);
        })
        .on_webview_event(|window, event| {})
        .invoke_handler(tauri::generate_handler![
            greet,
            config::get_config,
            config::read_file
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
