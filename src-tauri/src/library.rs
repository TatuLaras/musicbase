use crate::{
    database::ConnectionWrapper,
    models::{Quality, Song},
};

pub fn has_file(file_path: &str, db: &ConnectionWrapper) -> bool {
    db.exists(&mut Song {
        id: None,
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
