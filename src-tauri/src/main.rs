// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{fs, sync::Mutex, thread};

use musicbase::{
    audio_playback::{play_file, replace_with, seek_to},
    content_scanner::scan_for_new_content,
    database::{update_field, ConnectionWrapper},
    images::save_cover,
    models::{
        base_metadata::{Album, Artist, Song},
        user_generated::{Directory, Playlist, Tag},
        Retrieve, Store, StoreFull,
    },
    param::{self, eq, Order},
};
use tauri::{api::dialog, AppHandle, Manager, State};

fn vec_result<T>(res: Result<Vec<T>, sqlite::Error>) -> Vec<T> {
    match res {
        Ok(a) => a,
        Err(error) => {
            println!("Error: {}", error.message.unwrap_or("".into()));
            vec![]
        }
    }
}

fn get_all<T: Retrieve>(db: &ConnectionWrapper) -> Vec<T> {
    vec_result(db.get_all::<T>(Order::Default))
}

fn get_by<T: Retrieve>(db: &ConnectionWrapper, field: &str, value: &str, order: Order) -> Vec<T> {
    vec_result(db.get_by::<T>(eq(field, value), order))
}

fn get_one_by<T: Retrieve + Clone>(db: &ConnectionWrapper, field: &str, value: &str) -> Option<T> {
    let result = get_by::<T>(db, field, value, Order::Default);
    if result.len() == 0 {
        None
    } else {
        Some(result[0].clone())
    }
}

fn insert<T: Store>(db: &ConnectionWrapper, obj: &mut T) -> Result<(), sqlite::Error> {
    match db.insert(obj) {
        Ok(_) => Ok(()),
        Err(err) => {
            println!("Error in main::insert, {}", err);
            return Err(err);
        }
    }
}

fn insert_full<T: StoreFull>(db: &ConnectionWrapper, obj: &mut T) -> Result<(), sqlite::Error> {
    match db.insert_full(obj) {
        Ok(_) => Ok(()),
        Err(err) => {
            println!("Error in main::insert_full, {}", err);
            return Err(err);
        }
    }
}

#[tauri::command]
fn get_all_albums(db: State<'_, Mutex<ConnectionWrapper>>) -> Vec<Album> {
    get_all::<Album>(&db.lock().unwrap())
}

#[tauri::command]
fn get_all_artists(db: State<'_, Mutex<ConnectionWrapper>>) -> Vec<Artist> {
    get_all::<Artist>(&db.lock().unwrap())
}

#[tauri::command]
fn get_all_playlists(db: State<'_, Mutex<ConnectionWrapper>>) -> Vec<Playlist> {
    get_all::<Playlist>(&db.lock().unwrap())
}

#[tauri::command]
fn get_all_tags(db: State<'_, Mutex<ConnectionWrapper>>) -> Vec<Tag> {
    get_all::<Tag>(&db.lock().unwrap())
}

#[tauri::command]
fn get_all_directories(db: State<'_, Mutex<ConnectionWrapper>>) -> Vec<Directory> {
    get_all::<Directory>(&db.lock().unwrap())
}

#[tauri::command]
async fn select_directory() -> bool {
    // Directory picker
    let path = dialog::blocking::FileDialogBuilder::new()
        .set_title("Select music directory")
        .pick_folder();

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

//  TODO: Some kind of image scaling / optimization
//  For example don't load the full res images for gridviews etc.
#[tauri::command]
async fn select_cover(
    id: i64,
    playlist: bool,
    db: State<'_, Mutex<ConnectionWrapper>>,
    app_handle: AppHandle,
) -> Result<(), ()> {
    let Ok(db) = db.lock() else { return Err(()) };

    // Get application data directory
    let Some(data_dir) = app_handle.path_resolver().app_data_dir() else { return Err(()) };
    let Some(data_dir) = data_dir.to_str() else { return Err(()) };

    // File picker
    let path = dialog::blocking::FileDialogBuilder::new()
        .add_filter(
            "filter",
            &[
                "jpg", "jpeg", "png", "gif", "bmp", "svg", "JPG", "JPEG", "PNG", "GIF", "BMP",
                "SVG",
            ],
        )
        .set_title("Select cover")
        .pick_file();

    let Some(path) = path else { return Err(()); };

    let Some(extension) = path.extension() else { return Err(()) };
    let Ok(extension) = extension.to_os_string().into_string() else { return Err(()) };

    // Read image file
    let Ok(image_data) = fs::read(path) else { return Err(()) };

    // Save image
    let Some(cover_path) = save_cover(&image_data, &extension, data_dir) else { return Err(()) };

    // Insert into db
    if let Err(err) = update_field(
        &db,
        if playlist { "playlist" } else { "album" },
        "cover_path",
        &cover_path[..],
        if playlist { "playlist_id" } else { "album_id" },
        id,
    ) {
        println!("Error in main::select_cover, {}", err);
        return Err(());
    }
    Ok(())
}

#[tauri::command]
fn delete_directory(directory_id: i64, db: State<'_, Mutex<ConnectionWrapper>>) {
    let dir = Directory {
        directory_id: Some(directory_id),
        path: "".into(),
    };
    let res = dir.delete(&db.lock().unwrap().conn);
    if let Err(err) = res {
        println!("{}", err);
    }
}

#[tauri::command]
fn play_song(db: State<'_, Mutex<ConnectionWrapper>>, song_id: i64, queue: bool) {
    let song = get_one_by::<Song>(
        &db.lock().unwrap(),
        "song.song_id",
        &song_id.to_string()[..],
    );
    let Some(song) = song else { return; };
    thread::spawn(move || {
        play_file(&song.file_path[..], queue);
    });
}

#[tauri::command]
fn replace_song(db: State<'_, Mutex<ConnectionWrapper>>, song_id: i64) {
    let song = get_one_by::<Song>(
        &db.lock().unwrap(),
        "song.song_id",
        &song_id.to_string()[..],
    );
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
fn get_artist_albums(db: State<'_, Mutex<ConnectionWrapper>>, artist_id: i64) -> Vec<Album> {
    get_by(
        &db.lock().unwrap(),
        "album.artist_id",
        &artist_id.to_string()[..],
        Order::Default,
    )
}

#[tauri::command]
fn get_album(db: State<'_, Mutex<ConnectionWrapper>>, album_id: i64) -> Option<Album> {
    get_one_by::<Album>(
        &db.lock().unwrap(),
        "album.album_id",
        &album_id.to_string()[..],
    )
}

#[tauri::command]
fn get_album_songs(db: State<'_, Mutex<ConnectionWrapper>>, album_id: i64) -> Vec<Song> {
    get_by::<Song>(
        &db.lock().unwrap(),
        "album.album_id",
        &album_id.to_string()[..],
        param::asc("song.disc, song.track"),
    )
}

#[tauri::command]
fn get_playlist(db: State<'_, Mutex<ConnectionWrapper>>, playlist_id: i64) -> Option<Playlist> {
    get_one_by::<Playlist>(
        &db.lock().unwrap(),
        "playlist.playlist_id",
        &playlist_id.to_string()[..],
    )
}

#[tauri::command]
fn get_playlist_songs(db: State<'_, Mutex<ConnectionWrapper>>, playlist_id: i64) -> Vec<Song> {
    get_by::<Song>(
        &db.lock().unwrap(),
        "playlist_song.playlist_id",
        &playlist_id.to_string()[..],
        param::asc("song.disc, song.track"),
    )
}

#[tauri::command]
fn create_tag(name: String, db: State<'_, Mutex<ConnectionWrapper>>) -> Option<Tag> {
    let mut tag = Tag { tag_id: None, name };
    let result = insert(&db.lock().unwrap(), &mut tag);
    if let Ok(_) = result {
        return Some(tag);
    };
    None
}

#[tauri::command]
fn create_playlist(name: String, db: State<'_, Mutex<ConnectionWrapper>>) -> Option<Playlist> {
    let mut playlist = Playlist {
        playlist_id: None,
        name,
        desc: "".into(),
        cover_path: None,
        created: None,
        tags: Vec::new(),
    };
    let result = insert_full(&db.lock().unwrap(), &mut playlist);
    if let Ok(_) = result {
        return Some(playlist);
    };
    None
}

#[tauri::command]
fn scan(app_handle: AppHandle, db: State<'_, Mutex<ConnectionWrapper>>) {
    let db = db.lock().unwrap();

    let Some(data_dir) = app_handle.path_resolver().app_data_dir() else { return; };
    let Some(data_dir) = data_dir.to_str() else { return; };

    let res = db.get_all::<Directory>(Order::Default);
    let directories = match res {
        Ok(v) => v,
        Err(err) => {
            println!("{}", err);
            return;
        }
    };

    for directory in directories {
        println!("Scanning {}", directory.path);
        if let Err(err) = scan_for_new_content(&directory.path, &db, data_dir) {
            println!(
                "Error in command scan: {}",
                err.message.unwrap_or("".into())
            );
        }
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
            get_artist_albums,
            create_playlist,
            get_playlist,
            get_playlist_songs,
            delete_directory,
            select_cover,
            create_tag,
        ])
        .setup(|app| {
            app.manage(Mutex::new(db));
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
