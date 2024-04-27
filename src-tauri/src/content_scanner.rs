// List files in directory
// If file not in database, get metadata and store it there
//

use audiotags::Tag;
use walkdir::WalkDir;

use crate::{
    library,
    models::{Album, Artist, Quality, Song},
};

pub fn scan_for_new_content(dir: &str) {
    for entry in WalkDir::new(dir).into_iter() {
        if let Ok(entry) = entry {
            let path = entry.path().to_string_lossy();
            if !is_audio(&path) || library::has_file(&path) {
                continue;
            }
            let song = get_metadata(&path);
            println!("{:?}", song);
        }
    }
}

fn is_audio(file_path: &str) -> bool {
    let audio_file_extensions = [".flac", ".mp3"];

    for ext in audio_file_extensions {
        if file_path.ends_with(ext) {
            return true;
        }
    }

    false
}

// Parse file metadata into objects
fn get_metadata(file_path: &str) -> Song {
    let tag = Tag::new().read_from_path(file_path).unwrap();

    let artist = Artist {
        id: None,
        name: get_str(tag.artist()),
    };

    let album = Album {
        name: get_str(tag.album_title()),
        cover_path: String::from(""),
        year: get(tag.year()) as u16,
        tracks: get(tag.total_tracks()),
        artist: artist.clone(),
    };

    Song {
        name: get_str(tag.title()),
        track: get(tag.track_number()),
        duration_s: get(tag.duration()),
        quality: if file_path.ends_with(".flac") {
            Quality::Lossless
        } else {
            Quality::Lossy
        },
        genre: get_str(tag.genre()),
        bitrate_kbps: 0.0,
        sample_rate_khz: 0.0,
        artist,
        album,
    }
}

fn get_str(value: Option<&str>) -> String {
    value.unwrap_or("Unknown").to_string()
}

fn get<T: Default>(value: Option<T>) -> T {
    value.unwrap_or_default()
}
