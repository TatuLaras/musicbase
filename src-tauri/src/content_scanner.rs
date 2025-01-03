use audiotags::{Picture, Tag};
use walkdir::WalkDir;

use crate::{
    content_library,
    database::ConnectionWrapper,
    fs_utils::mime_type_to_extension,
    images::save_cover,
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
    image_cache_dir: &str,
) -> Result<(), sqlite::Error> {
    // Loop over files in a directory recursively
    for entry in WalkDir::new(dir).into_iter() {
        let Ok(entry) = entry else { continue; };
        let path = entry.path().to_string_lossy();

        // Skip over non-audio files and already existing ones
        if !is_audio(&path) || content_library::has_file(&path, db) {
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
    image_cache_dir: &str,
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
            cover_path_small: None,
            cover_path_tiny: None,
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
            if let Some(image) = tag.album_cover() {
                (
                    album.cover_path,
                    album.cover_path_small,
                    album.cover_path_tiny,
                ) = save_cover(
                    image.data,
                    mime_type_to_extension(image.mime_type),
                    image_cache_dir,
                );
            }
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
