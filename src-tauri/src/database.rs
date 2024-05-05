use sqlite::{Connection, State};

use crate::models::{Album, Artist, Store};

pub struct ConnectionWrapper {
    pub conn: Connection,
}

pub struct User {
    pub name: String,
    pub age: u8,
}

impl ConnectionWrapper {
    pub fn create_schema(&self) {
        let query = "
        CREATE TABLE artist (
            artist_id INTEGER PRIMARY KEY,
            name TEXT NOT NULL UNIQUE
        );
        CREATE TABLE album (
            album_id INTEGER PRIMARY KEY,
            name TEXT NOT NULL UNIQUE,
            artist_id INTEGER,
            cover_path TEXT,
            year INTEGER,
            total_tracks INTEGER,
            total_discs INTEGER,
            UNIQUE (artist_id, name, year)
        );
        ";
        self.conn.execute(query).unwrap();
    }

    pub fn insert(&self, item: &mut impl Store) -> Result<(), sqlite::Error> {
        item.insert(&self.conn)
    }

    pub fn exists(&self, item: &mut impl Store) -> Result<bool, sqlite::Error> {
        item.exists(&self.conn)
    }

    pub fn get_all_artists(&self) -> Result<Vec<Artist>, sqlite::Error> {
        let query = "SELECT artist_id, name FROM artist ORDER BY artist_id ASC";
        let mut artists: Vec<Artist> = Vec::new();

        let mut statement = self.conn.prepare(query)?;

        while let Ok(State::Row) = statement.next() {
            let artist = Artist {
                id: Some(statement.read::<i64, _>("artist_id")?),
                name: statement.read::<String, _>("name")?,
            };
            artists.push(artist);
        }
        Ok(artists)
    }

    pub fn get_all_albums(&self) -> Result<Vec<Album>, sqlite::Error> {
        let query = "SELECT
        al.album_id, al.name, al.artist_id, al.cover_path,
        al.year, al.total_tracks, al.total_discs, ar.name AS artist_name
        FROM album AS al
        LEFT JOIN artist AS ar
        ON al.artist_id = ar.artist_id
        ORDER BY album_id ASC
";

        let mut albums: Vec<Album> = Vec::new();

        let mut statement = self.conn.prepare(query)?;

        while let Ok(State::Row) = statement.next() {
            let artist_id = statement.read::<Option<i64>, _>("artist_id")?;
            let artist_name = statement.read::<Option<String>, _>("artist_name")?;

            let album = Album {
                id: Some(statement.read::<i64, _>("album_id")?),
                name: statement.read::<String, _>("name")?,
                artist: if artist_id.is_some() {
                    Some(Artist {
                        id: artist_id,
                        name: artist_name.unwrap_or("".to_string()),
                    })
                } else {
                    None
                },
                cover_path: statement.read::<Option<String>, _>("cover_path")?,
                year: statement.read::<Option<i64>, _>("year")?,
                total_tracks: statement.read::<Option<i64>, _>("total_tracks")?,
                total_discs: statement.read::<Option<i64>, _>("total_discs")?,
            };
            albums.push(album);
        }
        Ok(albums)
    }
}

pub fn execute_statement(statement: &mut sqlite::Statement) -> Result<(), sqlite::Error> {
    loop {
        let result = statement.next();
        if let Ok(res) = result {
            if res == State::Done {
                break;
            }
        }
    }
    Ok(())
}

pub fn last_id(conn: &sqlite::Connection) -> Result<i64, sqlite::Error> {
    let query = "SELECT LAST_INSERT_ROWID()";

    let mut statement = conn.prepare(query)?;

    if let Ok(State::Row) = statement.next() {
        return Ok(statement.read::<i64, _>(0)?);
    }
    Err(sqlite::Error {
        code: None,
        message: Some("Error in retrieving last insert row id".into()),
    })
}
