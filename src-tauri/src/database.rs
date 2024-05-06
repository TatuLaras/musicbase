use sqlite::{Connection, State};

use crate::{
    models::{Album, Artist, Song, Store, StoreFull},
    utils::option_cast,
};

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
            name TEXT NOT NULL,
            artist_id INTEGER,
            cover_path TEXT,
            year INTEGER,
            total_tracks INTEGER,
            total_discs INTEGER,
            UNIQUE (artist_id, name)
        );

        CREATE TABLE song (
            song_id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            file_path TEXT NOT NULL UNIQUE,
            track INTEGER,
            disc INTEGER,
            duration_s FLOATING,
            quality INTEGER NOT NULL,
            genre TEXT,
            artist_id INTEGER,
            album_id INTEGER
        );
        ";
        self.conn.execute(query).unwrap();
    }

    pub fn insert(&self, item: &mut impl Store) -> Result<(), sqlite::Error> {
        item.insert(&self.conn)
    }

    pub fn insert_full(&self, item: &mut impl StoreFull) -> Result<(), sqlite::Error> {
        item.insert_full(&self.conn)
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

    pub fn get_all_songs(&self) -> Result<Vec<Song>, sqlite::Error> {
        let query = "SELECT

        s.song_id, s.name, s.file_path, s.track, s.disc, 
        s.duration_s, s.quality, s.genre, s.artist_id, s.album_id,

        ar.name AS artist_name,

        al.name AS album_name, al.artist_id AS album_artist_id,
        al.cover_path, al.year, al.total_tracks, al.total_discs,

        alar.name AS album_artist_name

        FROM song AS s
        
        LEFT JOIN artist AS ar
        ON ar.artist_id = s.artist_id
        
        LEFT JOIN album AS al
        ON al.album_id = s.album_id

        LEFT JOIN artist AS alar
        ON alar.artist_id = al.artist_id

        ORDER BY s.song_id ASC
        ";

        let mut songs: Vec<Song> = Vec::new();

        let mut statement = self.conn.prepare(query)?;

        while let Ok(State::Row) = statement.next() {
            let artist_id = statement.read::<Option<i64>, _>("artist_id")?;
            let artist_name = statement.read::<Option<String>, _>("artist_name")?;
            let album_id = statement.read::<Option<i64>, _>("album_id")?;
            let album_name = statement.read::<Option<String>, _>("album_name")?;
            let album_artist_id = statement.read::<Option<i64>, _>("album_artist_id")?;
            let album_artist_name = statement.read::<Option<String>, _>("album_artist_name")?;

            let song = Song {
                id: Some(statement.read::<i64, _>("song_id")?),
                name: statement.read::<String, _>("name")?,
                file_path: statement.read::<String, _>("file_path")?,
                track: option_cast::<i64, u16>(statement.read::<Option<i64>, _>("track")?),
                disc: option_cast::<i64, u16>(statement.read::<Option<i64>, _>("disc")?),
                duration_s: statement.read::<Option<f64>, _>("duration_s")?,
                quality: statement.read::<i64, _>("quality")?.into(),
                genre: statement.read::<Option<String>, _>("genre")?,
                artist: if artist_id.is_some() && artist_name.is_some() {
                    Some(Artist {
                        id: artist_id,
                        name: artist_name.unwrap_or("".into()),
                    })
                } else {
                    None
                },
                album: if album_id.is_some() && album_name.is_some() {
                    Some(Album {
                        id: album_id,
                        name: album_name.unwrap_or("".into()),
                        artist: if album_artist_id.is_some() && album_artist_name.is_some() {
                            Some(Artist {
                                id: album_artist_id,
                                name: album_artist_name.unwrap_or("".into()),
                            })
                        } else {
                            None
                        },
                        cover_path: statement.read::<Option<String>, _>("cover_path")?,
                        year: statement.read::<Option<i64>, _>("year")?,
                        total_tracks: statement.read::<Option<i64>, _>("total_tracks")?,
                        total_discs: statement.read::<Option<i64>, _>("total_discs")?,
                    })
                } else {
                    None
                },
            };
            songs.push(song);
        }
        Ok(songs)
    }
}

pub fn execute_statement(statement: &mut sqlite::Statement) -> Result<(), sqlite::Error> {
    loop {
        let result = statement.next();
        if let Ok(res) = result {
            if res == State::Done {
                break;
            }
        } else if let Err(err) = result {
            return Err(err);
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
