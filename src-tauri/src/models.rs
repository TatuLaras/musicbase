use chrono::{DateTime, Utc};
use sqlite::State;

use crate::{
    database,
    utils::{self, option_as_slice, IntoOption},
};

fn err(message: &str) -> Result<(), sqlite::Error> {
    Err(sqlite::Error {
        code: None,
        message: Some(message.to_string()),
    })
}

fn ensure_valid(object: &impl Store) -> Result<(), sqlite::Error> {
    if object.is_valid() {
        return Ok(());
    }
    err("Object not valid")
}

pub trait Store {
    // Inserts the object into the database
    //
    // Takes a mutable reference to the object and fills in the id field to the newly inserted id
    // Not the cleanest approach, but makes the code way more concise in certain situations.
    fn insert(&mut self, conn: &sqlite::Connection) -> Result<(), sqlite::Error>;
    // Returns true if an "overlapping" data point is found in the database, fills in id field of
    // to the found id
    fn exists(&mut self, conn: &sqlite::Connection) -> Result<bool, sqlite::Error>;
    fn is_valid(&self) -> bool;
}

pub trait StoreFull {
    // Inserts the object and all contained objects into the db
    // Fills in the id field similarly
    fn insert_full(&mut self, conn: &sqlite::Connection) -> Result<(), sqlite::Error>;
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum Quality {
    Lossless,
    Lossy,
}

impl From<i64> for Quality {
    fn from(value: i64) -> Self {
        match value {
            0 => Quality::Lossless,
            _ => Quality::Lossy,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Artist {
    pub id: Option<i64>,
    pub name: String,
}

impl Store for Artist {
    fn insert(&mut self, conn: &sqlite::Connection) -> Result<(), sqlite::Error> {
        ensure_valid(self)?;

        if self.exists(conn)? {
            return Ok(());
        }

        let query = "INSERT INTO artist (name) VALUES (:name)";
        let mut statement = conn.prepare(query)?;

        let name = &self.name[..];
        statement.bind((":name", name))?;

        database::execute_statement(&mut statement)?;
        self.id = Some(database::last_id(conn)?);
        Ok(())
    }

    fn exists(&mut self, conn: &sqlite::Connection) -> Result<bool, sqlite::Error> {
        let query = "SELECT artist_id FROM artist WHERE name = :name LIMIT 1";

        let mut statement = conn.prepare(query)?;

        let name = &self.name[..];
        statement.bind((":name", name))?;

        if let Ok(State::Row) = statement.next() {
            let artist_id = statement.read::<i64, _>(0)?;
            self.id = Some(artist_id);
            return Ok(true);
        }

        Ok(false)
    }

    fn is_valid(&self) -> bool {
        self.name.len() > 0
    }
}

impl StoreFull for Artist {
    fn insert_full(&mut self, conn: &sqlite::Connection) -> Result<(), sqlite::Error> {
        self.insert(conn)
    }
}

#[derive(Debug, Clone)]
pub struct Album {
    pub id: Option<i64>,
    pub name: String,
    pub artist: Option<Artist>,
    pub cover_path: Option<String>,
    pub year: Option<i64>,
    pub total_tracks: Option<i64>,
    pub total_discs: Option<i64>,
}

impl Store for Album {
    fn insert(&mut self, conn: &sqlite::Connection) -> Result<(), sqlite::Error> {
        ensure_valid(self)?;

        if self.exists(conn)? {
            return Ok(());
        }

        let query = "
        INSERT INTO album 
        (name, artist_id, cover_path, year, total_tracks, total_discs) 
        VALUES 
        (:name, :artist_id, :cover_path, :year, :total_tracks, :total_discs)
        ";

        let mut statement = conn.prepare(query)?;

        let mut artist_id: Option<i64> = None;
        if let Some(artist) = &self.artist {
            artist_id = artist.id
        }

        statement.bind((":name", &self.name[..]))?;
        // Assumption: sqlite rust handles Option::None as NULL
        statement.bind((":artist_id", artist_id))?;
        statement.bind((":cover_path", utils::option_as_slice(&self.cover_path)))?;
        statement.bind((":year", self.year))?;
        statement.bind((":total_tracks", self.total_tracks))?;
        statement.bind((":total_discs", self.total_discs))?;

        database::execute_statement(&mut statement)?;
        self.id = Some(database::last_id(conn)?);
        Ok(())
    }

    fn exists(&mut self, conn: &sqlite::Connection) -> Result<bool, sqlite::Error> {
        ensure_valid(self)?;

        let artist_id: Option<i64> = if let Some(artist) = &self.artist {
            if let Some(artist_id) = artist.id {
                Some(artist_id)
            } else {
                None
            }
        } else {
            None
        };

        // Bit messy but searches with the artist only if the album has one
        let query = format!(
            "SELECT 
            album_id FROM album

            WHERE name = :name 
            {}
            LIMIT 1",
            if artist_id.is_some() {
                "AND artist_id = :artist_id"
            } else {
                ""
            }
        );

        let mut statement = conn.prepare(query)?;

        statement.bind((":name", &self.name[..]))?;

        if let Some(id) = artist_id {
            statement.bind((":artist_id", id))?;
        }

        if let Ok(State::Row) = statement.next() {
            let album_id = statement.read::<i64, _>(0)?;
            // Assing the found id to the mutable ref
            self.id = Some(album_id);
            return Ok(true);
        }

        Ok(false)
    }

    fn is_valid(&self) -> bool {
        self.name.len() > 0
    }
}

impl StoreFull for Album {
    fn insert_full(&mut self, conn: &sqlite::Connection) -> Result<(), sqlite::Error> {
        if let Some(artist) = &mut self.artist {
            artist.insert_full(conn)?;
        }
        self.insert(conn)
    }
}

#[derive(Debug, Clone)]
pub struct Song {
    pub id: Option<i64>,
    pub name: String,
    pub file_path: String,
    pub track: Option<u16>,
    pub disc: Option<u16>,
    pub duration_s: Option<f64>,
    pub quality: Quality,
    pub genre: Option<String>,
    pub artist: Option<Artist>,
    pub album: Option<Album>,
}

impl Store for Song {
    fn insert(&mut self, conn: &sqlite::Connection) -> Result<(), sqlite::Error> {
        ensure_valid(self)?;

        if self.exists(conn)? {
            return Ok(());
        }

        let query = "INSERT INTO song
        (name, file_path, track, disc, duration_s, quality, genre, artist_id, album_id)
        VALUES
        (:name, :file_path, :track, :disc, :duration_s, :quality, :genre, :artist_id, :album_id)
        ";

        let mut statement = conn.prepare(query)?;

        let artist_id = if let Some(artist) = &self.artist {
            artist.id
        } else {
            None
        };

        let album_id = if let Some(album) = &self.album {
            album.id
        } else {
            None
        };

        statement.bind((":name", &self.name[..]))?;
        statement.bind((":file_path", &self.file_path[..]))?;
        statement.bind((":track", self.track.into_option()))?;
        statement.bind((":disc", self.disc.into_option()))?;
        statement.bind((":duration_s", self.duration_s))?;
        statement.bind((":quality", self.quality as i64))?;
        statement.bind((":genre", option_as_slice(&self.genre)))?;
        statement.bind((":artist_id", artist_id))?;
        statement.bind((":album_id", album_id))?;

        database::execute_statement(&mut statement)?;
        self.id = Some(database::last_id(conn)?);
        Ok(())
    }

    fn exists(&mut self, conn: &sqlite::Connection) -> Result<bool, sqlite::Error> {
        let query = "SELECT 
        s.song_id FROM song AS s

        WHERE s.file_path = :file_path
        LIMIT 1
        ";

        let mut statement = conn.prepare(query)?;

        statement.bind((":file_path", &self.file_path[..]))?;

        if let Ok(State::Row) = statement.next() {
            let song_id = statement.read::<i64, _>(0)?;
            // Assing the found id to the mutable ref
            self.id = Some(song_id);
            return Ok(true);
        }

        Ok(false)
    }

    fn is_valid(&self) -> bool {
        self.name.len() > 0 && self.file_path.len() > 0
    }
}

impl StoreFull for Song {
    fn insert_full(&mut self, conn: &sqlite::Connection) -> Result<(), sqlite::Error> {
        if let Some(album) = &mut self.album {
            album.insert_full(conn)?;
        }

        if let Some(artist) = &mut self.artist {
            artist.insert_full(conn)?;
        }

        self.insert(conn)
    }
}

#[derive(Debug, Clone)]
pub struct Playlist {
    pub name: String,
    pub description: String,
    pub cover_path: String,
    pub created: DateTime<Utc>,
    pub songs: Vec<Song>,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone)]
pub enum Setting {
    Text(String),
    Toggle(bool),
}
