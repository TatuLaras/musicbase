// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use musicbase::{content_scanner::scan_for_new_content, database::ConnectionWrapper};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust! Skibidi", name)
}

fn main() {
    // tauri::Builder::default()
    //     .invoke_handler(tauri::generate_handler![greet])
    //     .run(tauri::generate_context!())
    //     .expect("error while running tauri application");

    let db = ConnectionWrapper {
        conn: sqlite::open("test.db").expect("Connection failed"),
    };
    db.create_schema();
    scan_for_new_content("test_audio/", &db).unwrap();
}
