use std::{fs, io::Write};

use audiotags::{Picture, Tag};
use walkdir::WalkDir;

use crate::{
    database::ConnectionWrapper,
    fs_utils::{get_unique_path, mime_type_to_extension},
    library,
    models::{
        base_metadata::{Album, Artist, Song},
        err, Quality,
    },
    utils::IntoOption,
};

pub struct MetadataResult<'a> {
    pub song: Song,
    pub album_cover: Option<Picture<'a>>,
}

// Scans a given directory and commits music metadata to database
pub fn scan_for_new_content(
    dir: &str,
    db: &ConnectionWrapper,
    image_cache_dir: Option<&str>,
) -> Result<(), sqlite::Error> {
    // Loop over files in a directory recursively
    for entry in WalkDir::new(dir).into_iter() {
        let Ok(entry) = entry else { continue; };
        let path = entry.path().to_string_lossy();

        // Skip over non-audio files and already existing ones
        if !is_audio(&path) || library::has_file(&path, db) {
            continue;
        }

        parse_and_save_metadata(&path, db, image_cache_dir)?;
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

// Reads the metadata tags of an audio file and saves that metadata
fn parse_and_save_metadata(
    file_path: &str,
    db: &ConnectionWrapper,
    image_cache_dir: Option<&str>,
) -> Result<(), sqlite::Error> {
    // Get metadata tags
    let Ok(tag) = Tag::new().read_from_path(file_path) else {
        return err("Could not get audio file metadata");
    };

    // Convert the tag objects into our database model objects
    let artist_name = tag.artist();
    let artist = if let Some(name) = artist_name {
        Some(Artist {
            artist_id: None,
            name: name.into(),
            artist_image_path: None,
        })
    } else {
        None
    };

    // Use album artist, if that doesn't exist use the song artist instead
    let album_artist_name = tag.album_artist();
    let album_artist = if let Some(name) = album_artist_name {
        Some(Artist {
            artist_id: None,
            name: name.into(),
            artist_image_path: None,
        })
    } else {
        artist.clone()
    };

    let album_name = tag.album_title();
    let album = if let Some(name) = album_name {
        Some(Album {
            album_id: None,
            name: name.into(),
            cover_path: None,
            year: tag.year().option_into(),
            total_tracks: tag.total_tracks().option_into(),
            total_discs: tag.total_discs().option_into(),
            artist: album_artist,
        })
    } else {
        None
    };
    let mut song = Song {
        song_id: None,
        name: get_str(tag.title()),
        track: tag.track_number(),
        duration_s: tag.duration(),
        quality: if file_path.ends_with(".flac") {
            Quality::Lossless
        } else {
            Quality::Lossy
        },
        genre: tag.genre().option_into(),
        artist,
        album,
        disc: tag.disc_number(),
        file_path: file_path.into(),
    };

    // Save cover art into app data directory
    if let Some(album) = &mut song.album {
        if !db.exists(album)? {
            album.cover_path = save_cover(tag.album_cover(), image_cache_dir);
        }
    }

    // We're done! :3
    db.insert_full(&mut song)?;
    Ok(())
}

// "Flatten" a string option into a string
fn get_str(value: Option<&str>) -> String {
    value.unwrap_or("Unknown").to_string()
}

// Write an image into the app data directory
fn save_image<'a>(picture: Picture, image_cache_dir: &str) -> Result<String, String> {
    println!("save image");
    // Create a path that does not exist already
    let extension = mime_type_to_extension(picture.mime_type);
    let path = get_unique_path(image_cache_dir, extension)?;
    println!("Path: {}", path);

    // Write file, handle errors
    let file = fs::OpenOptions::new().create(true).write(true).open(&path);

    let Ok(mut file) = file else { return Err("Couldn't open file".into()); };

    if let Ok(_) = file.write_all(&picture.data) {
        return Ok(path);
    }
    Err("Unknown error".into())
}

// Wrapper for some reason
fn save_cover(picture: Option<Picture>, image_cache_dir: Option<&str>) -> Option<String> {
    println!("Try to save cover");
    let Some(picture) = picture else { return None };
    let Some(image_cache_dir) = image_cache_dir else { return  None };
    match save_image(picture, image_cache_dir) {
        Ok(value) => Some(value),
        Err(error) => {
            println!("Error in save_cover: {}", error);
            None
        }
    }
}
