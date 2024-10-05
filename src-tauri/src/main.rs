// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::time::Duration;

use musicbase::{
    audio_playback::{play_file, replace_with, seek_to},
    content_scanner::scan_for_new_content,
    database::ConnectionWrapper,
    models::{
        base_metadata::{Album, Artist, Song},
        user_generated::{Directory, Playlist, Tag},
        Retrieve,
    },
    param::{self, eq, Order},
};
use tauri::{api::dialog, AppHandle};

fn vec_result<T>(res: Result<Vec<T>, sqlite::Error>) -> Vec<T> {
    match res {
        Ok(a) => a,
        Err(error) => {
            println!("Error: {}", error.message.unwrap_or("".into()));
            vec![]
        }
    }
}

fn get_all<T: Retrieve>() -> Vec<T> {
    let db = get_db();
    vec_result(db.get_all::<T>(Order::Default))
}

fn get_by<T: Retrieve>(field: &str, value: &str, order: Order) -> Vec<T> {
    let db = get_db();
    vec_result(db.get_by::<T>(eq(field, value), order))
}

fn get_one_by<T: Retrieve + Clone>(field: &str, value: &str) -> Option<T> {
    let result = get_by::<T>(field, value, Order::Default);
    if result.len() == 0 {
        None
    } else {
        Some(result[0].clone())
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
fn get_all_tags() -> Vec<Tag> {
    get_all::<Tag>()
}

#[tauri::command]
fn get_all_directories() -> Vec<Directory> {
    get_all::<Directory>()
}

#[tauri::command]
async fn select_directory() -> bool {
    // Directory picker
    let path = dialog::blocking::FileDialogBuilder::new().pick_folder();

    let Some(path) = path else { return false; };
    let Ok(path) = path.into_os_string().into_string() else { return false; };

    let mut directory = Directory {
        directory_id: None,
        path,
    };

    // Insert into db
    let db = get_db();
    let res = db.insert(&mut directory);
    if let Err(_) = res {
        return false;
    }

    true
}

#[tauri::command]
fn play_song(song_id: i64, queue: bool) {
    let song = get_one_by::<Song>("song.song_id", &song_id.to_string()[..]);
    let Some(song) = song else { return; };

    unsafe {
        play_file(&song.file_path[..], queue);
    }
}

#[tauri::command]
fn replace_song(song_id: i64) {
    let song = get_one_by::<Song>("song.song_id", &song_id.to_string()[..]);
    let Some(song) = song else { return; };

    unsafe {
        replace_with(&song.file_path[..]);
    }
}

#[tauri::command]
fn seek(millisecs: u64) {
    unsafe {
        seek_to(millisecs);
    }
}

#[tauri::command]
fn get_album(album_id: i64) -> Option<Album> {
    get_one_by::<Album>("album.album_id", &album_id.to_string()[..])
}

#[tauri::command]
fn get_album_songs(album_id: i64) -> Vec<Song> {
    get_by::<Song>(
        "album.album_id",
        &album_id.to_string()[..],
        param::asc("song.disc, song.track"),
    )
}

#[tauri::command]
fn scan(app_handle: AppHandle) {
    let db = get_db();

    let Some(data_dir) = app_handle.path_resolver().app_data_dir() else { return; };
    let Some(data_dir) = data_dir.to_str() else { return; };

    // TODO: Iterate over a list of user folders from db
    if let Err(error) = scan_for_new_content("/home/tatu/Music/", &db, Some(data_dir)) {
        println!(
            "Error in command scan: {}",
            error.message.unwrap_or("".into())
        );
    }
}

fn get_db() -> ConnectionWrapper {
    ConnectionWrapper {
        conn: sqlite::open("/home/tatu/test.db").expect("Connection failed"),
    }
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
            get_all_tags,
            get_album,
            get_album_songs,
            get_all_directories,
            select_directory,
            play_song,
            replace_song,
            seek,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
