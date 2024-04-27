use chrono::{DateTime, Utc};
use sqlite::State;

use crate::database;

pub trait Store {
    // Inserts the object into the database
    fn insert(&self, conn: &sqlite::Connection) -> Result<(), sqlite::Error>;
    // Returns true if an "overlapping" data point is found in the database
    fn exists(&self, conn: &sqlite::Connection) -> Result<bool, sqlite::Error>;
}

#[derive(Debug, Clone)]
pub enum Quality {
    Lossless,
    Lossy,
}

#[derive(Debug, Clone)]
pub struct Artist {
    pub id: Option<i64>,
    pub name: String,
}

impl Store for Artist {
    fn insert(&self, conn: &sqlite::Connection) -> Result<(), sqlite::Error> {
        if self.name.len() == 0 {
            return Err(sqlite::Error {
                code: None,
                message: Some("Cannot push an Artist with an empty name field".into()),
            });
        }

        let query = "INSERT INTO artist (name) VALUES (:name)";
        let mut statement = conn.prepare(query)?;

        let name = &self.name[..];
        statement.bind((":name", name))?;

        database::execute_statement(&mut statement)
    }

    fn exists(&self, conn: &sqlite::Connection) -> Result<bool, sqlite::Error> {
        let query = "SELECT EXISTS(SELECT 1 FROM artist WHERE name = :name LIMIT 1);";

        let mut statement = conn.prepare(query)?;

        let name = &self.name[..];
        statement.bind((":name", name))?;

        if let Ok(State::Row) = statement.next() {
            let exists = statement.read::<i64, _>(0)?;
            return Ok(exists == 1);
        }

        Ok(false)
    }
}

#[derive(Debug, Clone)]
pub struct Album {
    pub name: String,
    pub artist: Option<Artist>,
    pub cover_path: Option<String>,
    pub year: Option<u16>,
    pub tracks: Option<u16>,
}

impl Store for Album {
    fn insert(&self, conn: &sqlite::Connection) -> Result<(), sqlite::Error> {
        todo!()
    }

    fn exists(&self, conn: &sqlite::Connection) -> Result<bool, sqlite::Error> {
        todo!()
    }
}

#[derive(Debug, Clone)]
pub struct Song {
    pub name: String,
    pub track: u16,
    pub duration_s: f64,
    pub quality: Quality,
    pub genre: String,
    pub bitrate_kbps: f32,
    pub sample_rate_khz: f32,
    pub artist: Artist,
    pub album: Album,
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
