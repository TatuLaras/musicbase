use sqlite::State;

use crate::{
    database,
    param::{asc, AsQuery, Condition, Order},
    utils::{self, option_as_slice, option_cast, IntoOption},
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
    // This method returning true for an object blocks the insert-method if it would violate a
    // UNIQUE-constraint otherwise. In other cases without an UNIQUE-constraint, such as with PlaylistSong,
    // exists can return true without an insert operation being blocked by it.
    fn exists(&mut self, conn: &sqlite::Connection) -> Result<bool, sqlite::Error>;

    // Objects where this method returns false cannot be stored into the database, use this for
    // validation
    fn is_valid(&self) -> bool;

    // Returns a vector of all items of a given type
    // Implementing this is optional, the default implementation returns an empty vector of type
    // Self
    fn get_all(_conn: &sqlite::Connection, _order: Order) -> Result<Vec<Self>, sqlite::Error>
    where
        Self: Sized,
    {
        Ok(Vec::new())
    }

    // Takes a condition and returns all objects of the type that match that condition
    fn get_by(
        _conn: &sqlite::Connection,
        _condition: Condition,
        _order: Order,
    ) -> Result<Vec<Self>, sqlite::Error>
    where
        Self: Sized,
    {
        Ok(Vec::new())
    }

    // Somewhat of an implementation detail, should not be used externally (expects a specific set
    // of fields to be there in the query). This exists to avoid code repetition.
    // Still associated with this trait due to it being the most conventient way to group
    // this functionality at the moment.
    fn __get_by_query(_conn: &sqlite::Connection, _query: &str) -> Result<Vec<Self>, sqlite::Error>
    where
        Self: Sized,
    {
        Ok(Vec::new())
    }

    //  TODO: exists_by method, which works similarly to get_by but only returns if a row exists
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

#[derive(Debug, Clone, PartialEq)]
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

    fn get_all(conn: &sqlite::Connection, order: Order) -> Result<Vec<Self>, sqlite::Error>
    where
        Self: Sized,
    {
        let query = format!(
            "SELECT artist_id, name FROM artist ORDER BY {}",
            order.as_query(asc("artist_id"))
        );
        Self::__get_by_query(conn, &query)
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
            "SELECT artist_id, name FROM artist WHERE {} ORDER BY {}",
            condition.as_query(Condition::None),
            order.as_query(asc("artist_id")),
        );
        Self::__get_by_query(conn, &query)
    }

    fn __get_by_query(conn: &sqlite::Connection, query: &str) -> Result<Vec<Self>, sqlite::Error>
    where
        Self: Sized,
    {
        let mut artists: Vec<Artist> = Vec::new();

        let mut statement = conn.prepare(query)?;

        while let Ok(State::Row) = statement.next() {
            let artist = Artist {
                id: Some(statement.read::<i64, _>("artist_id")?),
                name: statement.read::<String, _>("name")?,
            };
            artists.push(artist);
        }
        Ok(artists)
    }
}

impl StoreFull for Artist {
    fn insert_full(&mut self, conn: &sqlite::Connection) -> Result<(), sqlite::Error> {
        self.insert(conn)
    }
}

#[derive(Debug, Clone, PartialEq)]
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

        let query = "INSERT INTO album 
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

    fn get_all(conn: &sqlite::Connection, order: Order) -> Result<Vec<Self>, sqlite::Error>
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

            ORDER BY {}
            ",
            order.as_query(asc("album.album_id"))
        );
        Self::__get_by_query(conn, &query)
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
        Self::__get_by_query(conn, &query)
    }

    fn __get_by_query(conn: &sqlite::Connection, query: &str) -> Result<Vec<Self>, sqlite::Error>
    where
        Self: Sized,
    {
        let mut albums: Vec<Album> = Vec::new();

        let mut statement = conn.prepare(query)?;

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

impl StoreFull for Album {
    fn insert_full(&mut self, conn: &sqlite::Connection) -> Result<(), sqlite::Error> {
        if let Some(artist) = &mut self.artist {
            artist.insert_full(conn)?;
        }
        self.insert(conn)
    }
}

#[derive(Debug, Clone, PartialEq)]
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

    fn get_all(conn: &sqlite::Connection, order: Order) -> Result<Vec<Self>, sqlite::Error>
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

            ORDER BY {}
            ",
            order.as_query(asc("song.song_id"))
        );
        Self::__get_by_query(conn, &query)
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
        Self::__get_by_query(conn, &query)
    }

    fn __get_by_query(conn: &sqlite::Connection, query: &str) -> Result<Vec<Self>, sqlite::Error>
    where
        Self: Sized,
    {
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

#[derive(Debug, Clone, PartialEq)]
pub struct Playlist {
    pub id: Option<i64>,
    pub name: String,
    pub desc: String,
    pub cover_path: Option<String>,
    pub created: Option<String>,
    pub tags: Vec<String>,
}

impl Store for Playlist {
    fn insert(&mut self, conn: &sqlite::Connection) -> Result<(), sqlite::Error> {
        ensure_valid(self)?;

        if self.exists(conn)? {
            return Ok(());
        }

        let query = "INSERT INTO playlist  
        (name, desc, cover_path, tags)
        VALUES 
        (:name, :desc, :cover_path, :tags)
        ";

        let mut statement = conn.prepare(query)?;

        statement.bind((":name", &self.name[..]))?;
        statement.bind((":desc", &self.desc[..]))?;
        statement.bind((":cover_path", option_as_slice(&self.cover_path)))?;
        statement.bind((":tags", &self.tags.join(",")[..]))?;

        database::execute_statement(&mut statement)?;
        self.id = Some(database::last_id(conn)?);
        Ok(())
    }

    fn exists(&mut self, _conn: &sqlite::Connection) -> Result<bool, sqlite::Error> {
        // The user can create multiple playlists with all the same data, which is expected
        // behaviour
        Ok(false)
    }

    fn is_valid(&self) -> bool {
        self.name.len() > 0
    }

    fn get_all(conn: &sqlite::Connection, order: Order) -> Result<Vec<Self>, sqlite::Error>
    where
        Self: Sized,
    {
        let query = format!(
            "SELECT

            playlist_id, name, desc, cover_path, created, tags
            FROM playlist
            ORDER BY {}
            ",
            order.as_query(asc("playlist_id"))
        );
        Self::__get_by_query(conn, &query)
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

            playlist_id, name, desc, cover_path, created, tags
            FROM playlist
            WHERE {}
            ORDER BY {}
            ",
            condition.as_query(Condition::None),
            order.as_query(asc("playlist_id")),
        );
        Self::__get_by_query(conn, &query)
    }

    fn __get_by_query(conn: &sqlite::Connection, query: &str) -> Result<Vec<Self>, sqlite::Error>
    where
        Self: Sized,
    {
        let mut playlists: Vec<Playlist> = Vec::new();

        let mut statement = conn.prepare(query)?;

        while let Ok(State::Row) = statement.next() {
            // basically .collect() but converts to String also
            let mut tags = Vec::new();
            for tag in statement.read::<String, _>("tags")?.split(",") {
                if tag.len() > 0 {
                    tags.push(tag.to_string());
                }
            }

            let playlist = Playlist {
                id: Some(statement.read::<i64, _>("playlist_id")?),
                name: statement.read::<String, _>("name")?,
                desc: statement.read::<String, _>("desc")?,
                cover_path: statement.read::<Option<String>, _>("cover_path")?,
                created: statement.read::<Option<String>, _>("created")?,
                tags,
            };
            playlists.push(playlist);
        }
        Ok(playlists)
    }
}

pub struct PlaylistSong {
    pub id: Option<i64>,
    pub song_id: i64,
    pub playlist_id: i64,
}

impl Store for PlaylistSong {
    fn insert(&mut self, conn: &sqlite::Connection) -> Result<(), sqlite::Error> {
        ensure_valid(self)?;

        // Note the missing "exists check"

        let query = "INSERT INTO playlist_song
        (song_id, playlist_id)
        VALUES 
        (:song_id, :playlist_id)
        ";

        let mut statement = conn.prepare(query)?;

        statement.bind((":song_id", self.song_id))?;
        statement.bind((":playlist_id", self.playlist_id))?;

        database::execute_statement(&mut statement)?;
        self.id = Some(database::last_id(conn)?);
        Ok(())
    }

    fn exists(&mut self, conn: &sqlite::Connection) -> Result<bool, sqlite::Error> {
        let query = "SELECT 
        playlist_song_id FROM playlist_song

        WHERE song_id = :song_id
        AND playlist_id = :playlist_id

        LIMIT 1
        ";

        let mut statement = conn.prepare(query)?;

        statement.bind((":song_id", self.song_id))?;
        statement.bind((":playlist_id", self.playlist_id))?;

        if let Ok(State::Row) = statement.next() {
            let song_id = statement.read::<i64, _>(0)?;
            self.id = Some(song_id);
            return Ok(true);
        }

        Ok(false)
    }

    fn is_valid(&self) -> bool {
        self.song_id > 0 && self.playlist_id > 0
    }
}
