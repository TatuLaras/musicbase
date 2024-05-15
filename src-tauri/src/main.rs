// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use musicbase::{
    content_scanner::scan_for_new_content,
    database::ConnectionWrapper,
    models::{
        base_metadata::{Album, Artist},
        user_generated::Playlist,
        Store,
    },
    param::Order,
};
use tauri::AppHandle;

fn get_all<T: Store>() -> Vec<T> {
    let db = get_db();
    match db.get_all::<T>(Order::Default) {
        Ok(a) => a,
        Err(error) => {
            println!("Error: {}", error.message.unwrap_or("".into()));
            vec![]
        }
    }
}

#[tauri::command]
fn get_all_albums() -> Vec<Album> {
    get_all::<Album>()
}

#[tauri::command]
fn get_all_artists() -> Vec<Artist> {
    get_all::<Artist>()
}

#[tauri::command]
fn get_all_playlists() -> Vec<Playlist> {
    get_all::<Playlist>()
}

#[tauri::command]
fn scan(app_handle: AppHandle) {
    let db = get_db();
    let Some(data_dir) = app_handle.path_resolver().app_data_dir() else { return; };
    let Some(data_dir) = data_dir.to_str() else { return; };
    if let Err(error) = scan_for_new_content("/home/tatu/Music/", &db, Some(data_dir)) {
        println!(
            "Error in command scan: {}",
            error.message.unwrap_or("".into())
        );
    }
}

fn get_db() -> ConnectionWrapper {
    let db = ConnectionWrapper {
        conn: sqlite::open("test.db").expect("Connection failed"),
    };

    let _ = db.create_schema();
    db
}

fn main() {
    let db = get_db();
    let _ = db.create_schema();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_all_albums,
            scan,
            get_all_artists,
            get_all_playlists,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
