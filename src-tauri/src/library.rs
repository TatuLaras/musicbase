use crate::{
    database::ConnectionWrapper,
    models::{base_metadata::Song, Quality},
};

pub fn has_file(file_path: &str, db: &ConnectionWrapper) -> bool {
    db.exists(&mut Song {
        song_id: None,
        name: "".into(),
        file_path: file_path.into(),
        track: None,
        disc: None,
        duration_s: None,
        quality: Quality::Lossy,
        genre: None,
        artist: None,
        album: None,
    })
    .unwrap_or(false)
}
