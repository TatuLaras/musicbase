use sqlite::{BindableWithIndex, Connection, State};

use crate::{
    models::{user_generated::Playlist, Retrieve, Store, StoreFull},
    param::{Condition, Order},
};

pub struct ConnectionWrapper {
    pub conn: Connection,
}

impl ConnectionWrapper {
    pub fn create_schema(&self) -> Result<(), sqlite::Error> {
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
            cover_path_small TEXT,
            cover_path_tiny TEXT,
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

        CREATE TABLE playlist (
            playlist_id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            desc TEXT NOT NULL,
            cover_path TEXT,
            cover_path_small TEXT,
            cover_path_tiny TEXT,
            created TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        );

        CREATE TABLE playlist_song (
            playlist_song_id INTEGER PRIMARY KEY,
            song_id INTEGER NOT NULL,
            playlist_id INTEGER NOT NULL,
            added TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            ordering INTEGER DEFAULT 0
        );

        CREATE TABLE tag (
            tag_id INTEGER PRIMARY KEY,
            name TEXT NOT NULL UNIQUE
        );

        CREATE TABLE playlist_tag (
            playlist_tag_id INTEGER PRIMARY KEY,
            playlist_id INTEGER NOT NULL,
            tag_id INTEGER NOT NULL,
            added TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
            UNIQUE (playlist_id, tag_id)
        );

        CREATE TABLE album_tag (
            album_tag_id INTEGER PRIMARY KEY,
            album_id INTEGER NOT NULL,
            tag_id INTEGER NOT NULL,
            added TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            UNIQUE (album_id, tag_id)
        );

        CREATE TABLE directory (
            directory_id INTEGER PRIMARY KEY,
            path TEXT NOT NULL UNIQUE
        );

        ";

        self.conn.execute(query)
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

    pub fn delete(&self, item: &mut impl Store) -> Result<(), sqlite::Error> {
        item.delete(&self.conn)
    }

    pub fn get_all<T: Retrieve>(&self, order: Order) -> Result<Vec<T>, sqlite::Error> {
        T::get_all(&self.conn, order)
    }

    pub fn get_by<T: Retrieve>(
        &self,
        condition: Condition,
        order: Order,
    ) -> Result<Vec<T>, sqlite::Error> {
        T::get_by(&self.conn, condition, order)
    }

    pub fn last_id(&self) -> Result<i64, sqlite::Error> {
        last_id(&self.conn)
    }
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

// Helper to run an update query
pub fn update_field<T>(
    db: &ConnectionWrapper,
    table_name: &str,
    field_name: &str,
    field_value: T,
    id_name: &str,
    id_value: i64,
) -> Result<(), sqlite::Error>
where
    T: Sized + BindableWithIndex,
{
    let query = format!(
        "UPDATE {table} SET {field} = :{field} WHERE {id} = :{id}",
        table = table_name,
        id = id_name,
        field = field_name,
    );
    let mut statement = db.conn.prepare(query)?;
    statement.bind((1, field_value))?;
    statement.bind((2, id_value))?;
    execute_statement(&mut statement)?;
    Ok(())
}

pub fn get_ordering_offset(db: &ConnectionWrapper, playlist_id: i64) -> Result<i64, sqlite::Error> {
    let query = "SELECT MAX(ordering) AS res FROM playlist_song WHERE playlist_id = :playlist_id";
    let mut statement = db.conn.prepare(query)?;
    statement.bind((":playlist_id", playlist_id))?;

    if let Ok(State::Row) = statement.next() {
        return Ok(statement.read::<i64, _>("res")?);
    }

    Ok(0)
}

pub fn update_playlist(db: &ConnectionWrapper, playlist: Playlist) -> Result<(), sqlite::Error> {
    let query = "UPDATE playlist SET name = :name, desc = :desc WHERE playlist_id = :id";
    let mut statement = db.conn.prepare(query)?;

    statement.bind((":name", &playlist.name[..]))?;
    statement.bind((":desc", &playlist.desc[..]))?;
    statement.bind((":id", playlist.playlist_id))?;
    execute_statement(&mut statement)?;

    Ok(())
}

pub fn update_cover(
    db: &ConnectionWrapper,
    id: i64,
    playlist: bool,
    cover_path: String,
    cover_path_small: String,
    cover_path_tiny: String,
) -> Result<(), sqlite::Error> {
    let query = format!(
        "UPDATE {table} 
    SET cover_path = :cover_path, 
    cover_path_small = :cover_path_small,
    cover_path_tiny = :cover_path_tiny
    WHERE {id_name} = :id",
        table = if playlist { "playlist" } else { "album" },
        id_name = if playlist { "playlist_id" } else { "album_id" },
    );
    let mut statement = db.conn.prepare(query)?;
    statement.bind((":id", id))?;
    statement.bind((":cover_path", &cover_path[..]))?;
    statement.bind((":cover_path_small", &cover_path_small[..]))?;
    statement.bind((":cover_path_tiny", &cover_path_tiny[..]))?;

    execute_statement(&mut statement)?;

    Ok(())
}
