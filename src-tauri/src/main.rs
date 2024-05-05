// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use musicbase::{
    database::ConnectionWrapper,
    models::{Album, Artist},
};
use once_cell::sync::Lazy;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust! Skibidi", name)
}

static SAMPLE_ARTISTS: Lazy<[Artist; 4]> = Lazy::new(|| {
    [
        Artist {
            id: None,
            name: "Björk".into(),
        },
        Artist {
            id: None,
            name: "Anssi Kela".into(),
        },
        Artist {
            id: Some(123),
            name: "Radiohead".into(),
        },
        Artist {
            id: Some(123),
            name: "SQL Injector '`\"".into(),
        },
    ]
});

static SAMPLE_ALBUMS: Lazy<[Album; 3]> = Lazy::new(|| {
    [
        Album {
            id: None,
            name: "Suuria Kuvioita".into(),
            artist: Some(SAMPLE_ARTISTS[0].clone()),
            cover_path: None,
            year: Some(2003),
            total_tracks: Some(10),
            total_discs: Some(1),
        },
        Album {
            id: None,
            name: "Homogenic".into(),
            artist: Some(SAMPLE_ARTISTS[1].clone()),
            cover_path: Some("path/to/cover/".into()),
            year: Some(1997),
            total_tracks: Some(10),
            total_discs: Some(1),
        },
        Album {
            id: None,
            name: "Empty album".into(),
            artist: None,
            cover_path: None,
            year: None,
            total_tracks: None,
            total_discs: None,
        },
    ]
});
fn main() {
    // tauri::Builder::default()
    //     .invoke_handler(tauri::generate_handler![greet])
    //     .run(tauri::generate_context!())
    //     .expect("error while running tauri application");

    let db = ConnectionWrapper {
        conn: sqlite::open("test.db").expect("Connection failed"),
    };
    db.create_schema();
    for mut album in SAMPLE_ALBUMS.clone().into_iter() {
        if let Some(artist) = &mut album.artist {
            db.insert(artist).expect("Artist insert");
        }
        db.insert(&mut album).expect("Insert");
    }
}
