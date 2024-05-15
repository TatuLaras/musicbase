use crate::param::AsQuery;
use serde::Serialize;
use sqlite::State;

use crate::{
    database,
    param::{asc, Condition, Order},
    utils::{self, option_as_slice, option_cast, IntoOption},
};

use super::{ensure_valid, Quality, Store, StoreFull};

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Artist {
    pub artist_id: Option<i64>,
    pub name: String,
    pub artist_image_path: Option<String>,
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
        self.artist_id = Some(database::last_id(conn)?);
        Ok(())
    }

    fn exists(&mut self, conn: &sqlite::Connection) -> Result<bool, sqlite::Error> {
        let query = "SELECT artist_id FROM artist WHERE name = :name LIMIT 1";

        let mut statement = conn.prepare(query)?;

        let name = &self.name[..];
        statement.bind((":name", name))?;

        if let Ok(State::Row) = statement.next() {
            let artist_id = statement.read::<i64, _>(0)?;
            self.artist_id = Some(artist_id);
            return Ok(true);
        }

        Ok(false)
    }

    fn is_valid(&self) -> bool {
        self.name.len() > 0
    }

    fn get_by(
        conn: &sqlite::Connection,
        condition: Condition,
        order: Order,
    ) -> Result<Vec<Self>, sqlite::Error>
    where
        Self: Sized,
    {
        let query = format!(
            "SELECT 
            artist.artist_id, 
            artist.name, 
            album.cover_path AS artist_image_path 

            FROM artist 

            LEFT JOIN album
            ON album.artist_id = artist.artist_id

            WHERE {}

            GROUP BY artist.artist_id
            ORDER BY {}",
            condition.as_query(Condition::None),
            order.as_query(asc("artist.artist_id")),
        );
        let mut artists: Vec<Artist> = Vec::new();

        let mut statement = conn.prepare(query)?;

        while let Ok(State::Row) = statement.next() {
            let artist = Artist {
                artist_id: Some(statement.read::<i64, _>("artist_id")?),
                name: statement.read::<String, _>("name")?,
                artist_image_path: statement.read::<Option<String>, _>("artist_image_path")?,
            };
            artists.push(artist);
        }
        Ok(artists)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Album {
    pub album_id: Option<i64>,
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

        let query = "INSERT INTO album 
        (name, artist_id, cover_path, year, total_tracks, total_discs) 
        VALUES 
        (:name, :artist_id, :cover_path, :year, :total_tracks, :total_discs)
        ";

        let mut statement = conn.prepare(query)?;

        let mut artist_id: Option<i64> = None;
        if let Some(artist) = &self.artist {
            artist_id = artist.artist_id
        }

        statement.bind((":name", &self.name[..]))?;
        // Assumption: sqlite rust handles Option::None as NULL
        statement.bind((":artist_id", artist_id))?;
        statement.bind((":cover_path", utils::option_as_slice(&self.cover_path)))?;
        statement.bind((":year", self.year))?;
        statement.bind((":total_tracks", self.total_tracks))?;
        statement.bind((":total_discs", self.total_discs))?;

        database::execute_statement(&mut statement)?;
        self.album_id = Some(database::last_id(conn)?);
        Ok(())
    }

    fn exists(&mut self, conn: &sqlite::Connection) -> Result<bool, sqlite::Error> {
        ensure_valid(self)?;

        let artist_id = match &self.artist {
            Some(artist) => artist.artist_id,
            None => None,
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
            self.album_id = Some(album_id);
            return Ok(true);
        }

        Ok(false)
    }

    fn is_valid(&self) -> bool {
        self.name.len() > 0
    }

    fn get_by(
        conn: &sqlite::Connection,
        condition: Condition,
        order: Order,
    ) -> Result<Vec<Self>, sqlite::Error>
    where
        Self: Sized,
    {
        let query = format!(
            "SELECT

            album.album_id, album.name, album.artist_id, album.cover_path,
            album.year, album.total_tracks, album.total_discs, ar.name AS artist_name

            FROM album

            LEFT JOIN artist AS ar
            ON album.artist_id = ar.artist_id

            WHERE {}
            ORDER BY {}
            ",
            condition.as_query(Condition::None),
            order.as_query(asc("album.album_id")),
        );
        let mut albums: Vec<Album> = Vec::new();

        let mut statement = conn.prepare(query)?;

        while let Ok(State::Row) = statement.next() {
            let artist_id = statement.read::<Option<i64>, _>("artist_id")?;
            let artist_name = statement.read::<Option<String>, _>("artist_name")?;

            let album = Album {
                album_id: Some(statement.read::<i64, _>("album_id")?),
                name: statement.read::<String, _>("name")?,
                artist: if artist_id.is_some() {
                    Some(Artist {
                        artist_id,
                        name: artist_name.unwrap_or("".to_string()),
                        artist_image_path: None,
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

impl StoreFull for Album {
    fn insert_full(&mut self, conn: &sqlite::Connection) -> Result<(), sqlite::Error> {
        if let Some(artist) = &mut self.artist {
            artist.insert(conn)?;
        }
        self.insert(conn)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Song {
    pub song_id: Option<i64>,
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
            artist.artist_id
        } else {
            None
        };

        let album_id = if let Some(album) = &self.album {
            album.album_id
        } else {
            None
        };

        statement.bind((":name", &self.name[..]))?;
        statement.bind((":file_path", &self.file_path[..]))?;
        statement.bind((":track", self.track.option_into()))?;
        statement.bind((":disc", self.disc.option_into()))?;
        statement.bind((":duration_s", self.duration_s))?;
        statement.bind((":quality", self.quality as i64))?;
        statement.bind((":genre", option_as_slice(&self.genre)))?;
        statement.bind((":artist_id", artist_id))?;
        statement.bind((":album_id", album_id))?;

        database::execute_statement(&mut statement)?;
        self.song_id = Some(database::last_id(conn)?);
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
            self.song_id = Some(song_id);
            return Ok(true);
        }

        Ok(false)
    }

    fn is_valid(&self) -> bool {
        self.name.len() > 0 && self.file_path.len() > 0
    }

    fn get_by(
        conn: &sqlite::Connection,
        condition: Condition,
        order: Order,
    ) -> Result<Vec<Self>, sqlite::Error>
    where
        Self: Sized,
    {
        let query = format!(
            "SELECT
            song.song_id, song.name, song.file_path, song.track, song.disc, 
            song.duration_s, song.quality, song.genre, song.artist_id, song.album_id,

            artist.name AS artist_name,

            album.name AS album_name, album.artist_id AS album_artist_id,
            album.cover_path, album.year, album.total_tracks, album.total_discs,

            album_artist.name AS album_artist_name

            FROM song
            
            LEFT JOIN artist
            ON artist.artist_id = song.artist_id
            
            LEFT JOIN album
            ON album.album_id = song.album_id

            LEFT JOIN artist AS album_artist
            ON album_artist.artist_id = album.artist_id

            WHERE {}
            ORDER BY {}
            ",
            condition.as_query(Condition::None),
            order.as_query(asc("song.song_id")),
        );
        let mut songs: Vec<Song> = Vec::new();

        let mut statement = conn.prepare(query)?;

        while let Ok(State::Row) = statement.next() {
            let artist_id = statement.read::<Option<i64>, _>("artist_id")?;
            let artist_name = statement.read::<Option<String>, _>("artist_name")?;
            let album_id = statement.read::<Option<i64>, _>("album_id")?;
            let album_name = statement.read::<Option<String>, _>("album_name")?;
            let album_artist_id = statement.read::<Option<i64>, _>("album_artist_id")?;
            let album_artist_name = statement.read::<Option<String>, _>("album_artist_name")?;

            let song = Song {
                song_id: Some(statement.read::<i64, _>("song_id")?),
                name: statement.read::<String, _>("name")?,
                file_path: statement.read::<String, _>("file_path")?,
                track: option_cast::<i64, u16>(statement.read::<Option<i64>, _>("track")?),
                disc: option_cast::<i64, u16>(statement.read::<Option<i64>, _>("disc")?),
                duration_s: statement.read::<Option<f64>, _>("duration_s")?,
                quality: statement.read::<i64, _>("quality")?.into(),
                genre: statement.read::<Option<String>, _>("genre")?,
                artist: if artist_id.is_some() && artist_name.is_some() {
                    Some(Artist {
                        artist_id,
                        name: artist_name.unwrap_or("".into()),
                        artist_image_path: None,
                    })
                } else {
                    None
                },
                album: if album_id.is_some() && album_name.is_some() {
                    Some(Album {
                        album_id,
                        name: album_name.unwrap_or("".into()),
                        artist: if album_artist_id.is_some() && album_artist_name.is_some() {
                            Some(Artist {
                                artist_id: album_artist_id,
                                name: album_artist_name.unwrap_or("".into()),
                                artist_image_path: None,
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

impl StoreFull for Song {
    fn insert_full(&mut self, conn: &sqlite::Connection) -> Result<(), sqlite::Error> {
        if let Some(album) = &mut self.album {
            album.insert_full(conn)?;
        }

        if let Some(artist) = &mut self.artist {
            artist.insert(conn)?;
        }

        self.insert(conn)
    }
}
