// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{
    fs,
    io::{Read, Write},
    os::unix::net::UnixListener,
    sync::Mutex,
    thread,
};

use musicbase::{
    audio_playback::{play_file, start_mpv_process},
    content_scanner::scan_for_new_content,
    database::{get_ordering_offset, update_cover, update_playlist, ConnectionWrapper},
    images::save_cover,
    models::{
        base_metadata::{Album, Artist, Song},
        user_generated::{Directory, Playlist, PlaylistSong, Tag},
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
                "jpg", "jpeg", "png", "gif", "bmp", "JPG", "JPEG", "PNG", "GIF", "BMP",
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
    let (cover_path, cover_path_small, cover_path_tiny) =
        save_cover(&image_data, &extension, data_dir);
    let Some(cover_path) = cover_path else { return Err(()) };
    let Some(cover_path_small) = cover_path_small else { return Err(()) };
    let Some(cover_path_tiny) = cover_path_tiny else { return Err(()) };

    // Insert into db
    if let Err(err) = update_cover(
        &db,
        id,
        playlist,
        cover_path,
        cover_path_small,
        cover_path_tiny,
    ) {
        println!("Error in select_cover, {}", err);
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
    play_file(&song.file_path[..], queue);
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
        param::asc("playlist_song.ordering"),
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
fn add_songs_to_playlist(
    song_ids: Vec<i64>,
    playlist_id: i64,
    db: State<'_, Mutex<ConnectionWrapper>>,
) -> Vec<PlaylistSong> {
    let mut playlist_songs: Vec<PlaylistSong> = Vec::new();
    let Ok(db) = db.lock() else { return playlist_songs };
    let result = get_ordering_offset(&db, playlist_id);
    let order_offset = match result {
        Ok(v) => v,
        Err(err) => {
            println!(
                "Error in main::add_songs_to_playlist, get_max_playlist_order: {}",
                err
            );
            0
        }
    };

    for (i, song_id) in song_ids.into_iter().enumerate() {
        let mut playlist_song = PlaylistSong {
            playlist_song_id: None,
            added: None,
            song_id,
            playlist_id,
            ordering: order_offset + (i as i64),
        };

        let result = insert::<PlaylistSong>(&db, &mut playlist_song);
        if let Ok(_) = result {
            playlist_songs.push(playlist_song);
        };
    }
    playlist_songs
}

//  TODO: use the tauri async commands instead of this crap
#[tauri::command]
fn scan(app_handle: AppHandle) {
    thread::spawn(move || {
        let db = get_db();

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

        app_handle
            .emit_all::<Option<()>>("scan_done", None)
            .unwrap();
    });
}

#[tauri::command]
fn edit_playlist(playlist: Playlist, db: State<'_, Mutex<ConnectionWrapper>>) {
    let Ok(db) = db.lock() else { return };
    if let Err(err) = update_playlist(&db, playlist) {
        println!("Error in command edit_playlist, {}", err);
    };
}

// Initializes a Unix domain socket listener to be used by the musicbase web server
// Basically allows us to send arbitrary tauri events to the frontend from an outside process
#[tauri::command]
fn init_ipc_socket(app_handle: AppHandle, state: State<Mutex<SocketListenerState>>) {
    let Ok(mut state) = state.lock() else { return };

    if state.running {
        return;
    }

    let socket = "/tmp/musicbasetatularassocket";
    if let Err(_) = fs::remove_file(&socket) {
        return;
    };
    let Ok(listener) = UnixListener::bind(socket) else { return };

    // Simple protocol:
    // kind;payload
    thread::spawn(move || loop {
        match listener.accept() {
            Ok((mut socket, _addr)) => {
                // Parse the received data

                let mut payload = String::new();

                if let Err(err) = socket.read_to_string(&mut payload) {
                    println!("Error when receiving ipc socket connection: {}", err);
                    continue;
                }

                let parts: Vec<_> = payload.split(";").collect();

                if parts.len() != 2 {
                    println!("Ignoring malformed ipc data");
                    continue;
                }

                let kind = parts[0];
                let payload = parts[1];

                match kind {
                    // Ask for the application data directory
                    "datadir" => {
                        let Some(data_dir) = app_handle.path_resolver().app_data_dir() else { continue };
                        let Some(data_dir) = data_dir.to_str() else { continue };
                        let _ = socket.write_all(data_dir.as_bytes());
                    }
                    // Just forward it as an tauri event to the frontend
                    _ => app_handle.emit_all(kind, payload).unwrap(),
                };
            }
            Err(e) => println!("accept function failed: {:?}", e),
        }
    });

    // on success...
    state.running = true;
}

fn get_db() -> ConnectionWrapper {
    ConnectionWrapper {
        conn: sqlite::open("/home/tatu/test.db").expect("Connection failed"),
    }
}

pub struct SocketListenerState {
    running: bool,
}

fn main() {
    let db = get_db();
    let _ = db.create_schema();
    start_mpv_process().unwrap();

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
            get_artist_albums,
            create_playlist,
            get_playlist,
            get_playlist_songs,
            delete_directory,
            select_cover,
            create_tag,
            add_songs_to_playlist,
            edit_playlist,
            init_ipc_socket,
        ])
        .setup(|app| {
            app.manage(Mutex::new(db));
            app.manage(Mutex::new(SocketListenerState { running: false }));
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
