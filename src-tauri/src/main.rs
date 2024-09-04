// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use notify_rust::Notification;

#[tauri::command]
fn notify(_app_handle: tauri::AppHandle) {
    let _ = Notification::new()
        .summary("Time is up!")
        .body("The timer you set has finished!")
        .timeout(0)
        .image_path("file://icons/128x128.png")
        .show();
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![notify])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
