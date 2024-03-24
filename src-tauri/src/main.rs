// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{AppHandle, Manager, SystemTray, SystemTrayEvent, WindowEvent};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
async fn set_title(app_handle: AppHandle, title: String) {
    let result = app_handle.tray_handle().set_title(&title);

    if let Err(err) = result {
        eprintln!("Error setting tray title: {}", err);
    }
}

fn main() {
    let tray = SystemTray::new();

    tauri::Builder::default()
        .on_window_event(|event| {
            if let WindowEvent::CloseRequested { api, .. } = event.event() {
                let window = event.window();
                window.hide().unwrap();
                api.prevent_close();
            }
        })
        .system_tray(tray)
        .on_system_tray_event(|app, event| {
            if let SystemTrayEvent::LeftClick { .. } = event {
                let window = app.get_window("main").unwrap();
                window.show().unwrap();
            };
        })
        .invoke_handler(tauri::generate_handler![set_title])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
