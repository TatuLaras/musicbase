use chrono::{DateTime, Utc};
use sqlite::State;

use crate::{database, utils};

pub trait Store {
    // Inserts the object into the database
    //
    // Takes a mutable reference to the object and fills in the id field to the newly inserted id
    // Not the cleanest approach, but makes the code way more concise in certain situations.
    fn insert(&mut self, conn: &sqlite::Connection) -> Result<(), sqlite::Error>;
    // Returns true if an "overlapping" data point is found in the database, fills in id field of
    // to the found id
    fn exists(&mut self, conn: &sqlite::Connection) -> Result<bool, sqlite::Error>;
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
    fn insert(&mut self, conn: &sqlite::Connection) -> Result<(), sqlite::Error> {
        if self.name.len() == 0 {
            return Err(sqlite::Error {
                code: None,
                message: Some("Cannot push an Artist with an empty name field".into()),
            });
        }

        let query = "INSERT OR IGNORE INTO artist (name) VALUES (:name)";
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
        if self.name.len() == 0 {
            return Err(sqlite::Error {
                code: None,
                message: Some("Cannot push an Album with an empty name field".into()),
            });
        }

        let query = "
        INSERT OR IGNORE INTO album 
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
        // Bit messy but searches with the artist only if the album has one
        let query = format!(
            "SELECT 
            al.album_id FROM album AS al

            LEFT JOIN artist AS ar
            ON al.artist_id = ar.artist_id

            WHERE al.name = :name 
            {}
            LIMIT 1",
            if self.artist.is_some() {
                "AND ar.name = :artist_name"
            } else {
                ""
            }
        );

        let mut statement = conn.prepare(query)?;

        statement.bind((":name", &self.name[..]))?;

        // Similarly we only bind the value if the artist exists
        if let Some(artist) = &self.artist {
            statement.bind((":artist_name", &artist.name[..]))?;
        }

        if let Ok(State::Row) = statement.next() {
            let album_id = statement.read::<i64, _>(0)?;
            // Assing the found id to the mutable ref
            self.id = Some(album_id);
            return Ok(true);
        }

        Ok(false)
    }
}

#[derive(Debug, Clone)]
pub struct Song {
    pub name: String,
    pub track: Option<u16>,
    pub duration_s: Option<f64>,
    pub quality: Quality,
    pub genre: String,
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
