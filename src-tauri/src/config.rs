use std::fs;

use tauri::{ipc::CommandArg, path::PathResolver, AppHandle, Manager};

#[tauri::command]
pub fn get_config(app_handle: AppHandle) -> String {
    let mut app_data_dir = app_handle.path().app_data_dir().unwrap();
    app_data_dir.push("snowmetr.config.json");
    // println!("{}", app_data_dir.to_str().unwrap_or("?"));
    let config = fs::read_to_string(app_data_dir).unwrap_or("{ \"snowflakes\": [] }".to_string());

    config
}

#[tauri::command]
pub fn read_file(app_handle: AppHandle, file: &str) -> String {
    // println!("{}", app_data_dir.to_str().unwrap_or("?"));
    let res = fs::read_to_string(file).unwrap_or("EMPTY".to_string());

    res
}
