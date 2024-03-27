// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{AppHandle, Manager};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
async fn open_docs(handle: tauri::AppHandle) {
  let docs_window = tauri::WebviewWindowBuilder::new(
    &handle,
    "external", /* the unique window label */
    tauri::WebviewUrl::External("https://tauri.app/".parse().unwrap())
  ).build().unwrap();
}

fn main() {
    tauri::Builder::default()
        .setup(|app|{
            let mainwindow = app.get_webview_window("main").unwrap();
            mainwindow.show();
            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![open_docs])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
