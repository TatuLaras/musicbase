use audiotags::Tag;
use walkdir::WalkDir;

use crate::{
    database::ConnectionWrapper,
    library,
    models::{Album, Artist, Quality, Song},
    utils::IntoOption,
};

// Scans a given directory and commits music metadata to database
pub fn scan_for_new_content(dir: &str, db: &ConnectionWrapper) -> Result<(), sqlite::Error> {
    for entry in WalkDir::new(dir).into_iter() {
        if let Ok(entry) = entry {
            let path = entry.path().to_string_lossy();
            if !is_audio(&path) || library::has_file(&path, db) {
                continue;
            }
            let mut song = get_metadata(&path);
            db.insert_full(&mut song)?;
        }
    }

    Ok(())
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

    let artist_name = tag.artist();
    let artist = if let Some(name) = artist_name {
        Some(Artist {
            id: None,
            name: name.into(),
        })
    } else {
        None
    };

    let album_artist_name = tag.album_artist();
    let album_artist = if let Some(name) = album_artist_name {
        Some(Artist {
            id: None,
            name: name.into(),
        })
    } else {
        artist.clone()
    };

    let album_name = tag.album_title();
    let album = if let Some(name) = album_name {
        Some(Album {
            id: None,
            name: name.into(),
            cover_path: Some(String::from("")),
            year: tag.year().into_option(),
            total_tracks: tag.total_tracks().into_option(),
            total_discs: tag.total_discs().into_option(),
            artist: album_artist,
        })
    } else {
        None
    };

    Song {
        id: None,
        name: get_str(tag.title()),
        track: tag.track_number(),
        duration_s: tag.duration(),
        quality: if file_path.ends_with(".flac") {
            Quality::Lossless
        } else {
            Quality::Lossy
        },
        genre: tag.genre().into_option(),
        artist,
        album,
        disc: tag.disc_number(),
        file_path: file_path.into(),
    }
}

fn get_str(value: Option<&str>) -> String {
    value.unwrap_or("Unknown").to_string()
}
