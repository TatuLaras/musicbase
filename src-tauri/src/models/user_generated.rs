use crate::param::AsQuery;
use serde::{Deserialize, Serialize};
use sqlite::State;

use crate::{
    database,
    param::{asc, Condition, Order},
    utils::option_as_slice,
};

use super::{ensure_valid, Retrieve, Store, StoreFull};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Playlist {
    pub playlist_id: Option<i64>,
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
        (name, desc, cover_path)
        VALUES 
        (:name, :desc, :cover_path)
        ";

        let mut statement = conn.prepare(query)?;

        statement.bind((":name", &self.name[..]))?;
        statement.bind((":desc", &self.desc[..]))?;
        statement.bind((":cover_path", option_as_slice(&self.cover_path)))?;

        database::execute_statement(&mut statement)?;
        self.playlist_id = Some(database::last_id(conn)?);
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
}

impl Retrieve for Playlist {
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
            playlist.playlist_id, playlist.name, playlist.desc, 
            playlist.cover_path, playlist.created,
            GROUP_CONCAT(t.name) AS tags

            FROM playlist

            LEFT JOIN playlist_tag AS pt
            ON pt.playlist_id = playlist.playlist_id

            LEFT JOIN tag AS t
            ON t.tag_id = pt.tag_id
            
            WHERE {}

            GROUP BY playlist.playlist_id
            ORDER BY {}
            ",
            condition.as_query(Condition::None),
            order.as_query(asc("playlist.playlist_id")),
        );
        let mut playlists: Vec<Playlist> = Vec::new();

        let mut statement = conn.prepare(query)?;

        while let Ok(State::Row) = statement.next() {
            // basically .collect() but converts to String also
            let mut tags = Vec::new();
            let tags_field = statement.read::<String, _>("tags");
            if let Ok(tags_field) = tags_field {
                for tag in tags_field.split(",") {
                    if tag.len() > 0 {
                        tags.push(tag.to_string());
                    }
                }
            }

            let playlist = Playlist {
                playlist_id: Some(statement.read::<i64, _>("playlist_id")?),
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

impl StoreFull for Playlist {
    fn insert_full(&mut self, conn: &sqlite::Connection) -> Result<(), sqlite::Error> {
        self.insert(conn)?;
        for tag in &self.tags {
            let mut tag = Tag {
                tag_id: None,
                name: tag.into(),
            };
            tag.insert(conn)?;

            let Some(tag_id) = tag.tag_id else { continue; };
            let Some(playlist_id) = self.playlist_id else { continue; };

            let mut playlist_tag = PlaylistTag {
                playlist_tag_id: None,
                tag_id,
                playlist_id,
            };
            playlist_tag.insert(conn)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct PlaylistSong {
    pub playlist_song_id: Option<i64>,
    pub song_id: i64,
    pub playlist_id: i64,
    pub ordering: i64,
    pub added: Option<String>,
}

impl Store for PlaylistSong {
    fn insert(&mut self, conn: &sqlite::Connection) -> Result<(), sqlite::Error> {
        ensure_valid(self)?;

        // Note the missing "exists check"

        let query = "INSERT INTO playlist_song
        (song_id, playlist_id, ordering) 
        VALUES 
        (:song_id, :playlist_id, :ordering) 
        ";

        let mut statement = conn.prepare(query)?;

        statement.bind((":song_id", self.song_id))?;
        statement.bind((":playlist_id", self.playlist_id))?;
        statement.bind((":ordering", self.ordering))?;

        database::execute_statement(&mut statement)?;
        self.playlist_song_id = Some(database::last_id(conn)?);
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
            self.playlist_song_id = Some(song_id);
            return Ok(true);
        }

        Ok(false)
    }

    fn is_valid(&self) -> bool {
        self.song_id > 0 && self.playlist_id > 0
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Tag {
    pub tag_id: Option<i64>,
    pub name: String,
}

impl Store for Tag {
    fn insert(&mut self, conn: &sqlite::Connection) -> Result<(), sqlite::Error> {
        ensure_valid(self)?;

        if self.exists(conn)? {
            return Ok(());
        }

        let query = "INSERT INTO tag
        (name)
        VALUES 
        (:name)
        ";

        let mut statement = conn.prepare(query)?;

        statement.bind((":name", &self.name[..]))?;

        database::execute_statement(&mut statement)?;
        self.tag_id = Some(database::last_id(conn)?);
        Ok(())
    }

    fn exists(&mut self, conn: &sqlite::Connection) -> Result<bool, sqlite::Error> {
        let query = "SELECT 
        tag_id 
        FROM tag
        WHERE name = :name
        LIMIT 1
        ";

        let mut statement = conn.prepare(query)?;

        statement.bind((":name", &self.name[..]))?;

        if let Ok(State::Row) = statement.next() {
            let tag_id = statement.read::<i64, _>(0)?;
            self.tag_id = Some(tag_id);
            return Ok(true);
        }

        Ok(false)
    }

    fn is_valid(&self) -> bool {
        self.name.len() > 0
    }
}

impl Retrieve for Tag {
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
            tag.tag_id, tag.name
            FROM tag
            WHERE {}
            ORDER BY {}
            ",
            condition.as_query(Condition::None),
            order.as_query(asc("tag.tag_id")),
        );
        let mut tags: Vec<Tag> = Vec::new();

        let mut statement = conn.prepare(query)?;

        while let Ok(State::Row) = statement.next() {
            let tag = Tag {
                tag_id: Some(statement.read::<i64, _>("tag_id")?),
                name: statement.read::<String, _>("name")?,
            };
            tags.push(tag);
        }
        Ok(tags)
    }
}

pub struct PlaylistTag {
    pub playlist_tag_id: Option<i64>,
    pub tag_id: i64,
    pub playlist_id: i64,
}

impl Store for PlaylistTag {
    fn insert(&mut self, conn: &sqlite::Connection) -> Result<(), sqlite::Error> {
        ensure_valid(self)?;

        let query = "INSERT INTO playlist_tag
        (tag_id, playlist_id)
        VALUES 
        (:tag_id, :playlist_id)
        ";

        let mut statement = conn.prepare(query)?;

        statement.bind((":tag_id", self.tag_id))?;
        statement.bind((":playlist_id", self.playlist_id))?;

        database::execute_statement(&mut statement)?;
        self.playlist_tag_id = Some(database::last_id(conn)?);
        Ok(())
    }

    fn exists(&mut self, conn: &sqlite::Connection) -> Result<bool, sqlite::Error> {
        let query = "SELECT 
        playlist_tag_id FROM playlist_tag

        WHERE tag_id = :tag_id
        AND playlist_id = :playlist_id

        LIMIT 1
        ";

        let mut statement = conn.prepare(query)?;

        statement.bind((":tag_id", self.tag_id))?;
        statement.bind((":playlist_id", self.playlist_id))?;

        if let Ok(State::Row) = statement.next() {
            let song_id = statement.read::<i64, _>(0)?;
            self.playlist_tag_id = Some(song_id);
            return Ok(true);
        }

        Ok(false)
    }

    fn is_valid(&self) -> bool {
        self.tag_id > 0 && self.playlist_id > 0
    }
}

pub struct AlbumTag {
    pub album_tag_id: Option<i64>,
    pub album_id: i64,
    pub tag_id: i64,
}

impl Store for AlbumTag {
    fn insert(&mut self, conn: &sqlite::Connection) -> Result<(), sqlite::Error> {
        ensure_valid(self)?;

        let query = "INSERT INTO album_tag
        (tag_id, album_id)
        VALUES 
        (:tag_id, :album_id)
        ";

        let mut statement = conn.prepare(query)?;

        statement.bind((":song_id", self.tag_id))?;
        statement.bind((":album_id", self.album_id))?;

        database::execute_statement(&mut statement)?;
        self.album_tag_id = Some(database::last_id(conn)?);
        Ok(())
    }

    fn exists(&mut self, conn: &sqlite::Connection) -> Result<bool, sqlite::Error> {
        let query = "SELECT 
        album_tag_id FROM album_tag

        WHERE tag_id = :tag_id
        AND album_id = :album_id

        LIMIT 1
        ";

        let mut statement = conn.prepare(query)?;

        statement.bind((":tag_id", self.tag_id))?;
        statement.bind((":album_id", self.album_id))?;

        if let Ok(State::Row) = statement.next() {
            let song_id = statement.read::<i64, _>(0)?;
            self.album_tag_id = Some(song_id);
            return Ok(true);
        }

        Ok(false)
    }

    fn is_valid(&self) -> bool {
        self.tag_id > 0 && self.album_id > 0
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Directory {
    pub directory_id: Option<i64>,
    pub path: String,
}

impl Store for Directory {
    fn insert(&mut self, conn: &sqlite::Connection) -> Result<(), sqlite::Error> {
        ensure_valid(self)?;

        let query = "INSERT INTO directory
        (path)
        VALUES 
        (:path)
        ";

        let mut statement = conn.prepare(query)?;

        statement.bind((":path", &self.path[..]))?;

        database::execute_statement(&mut statement)?;
        self.directory_id = Some(database::last_id(conn)?);
        Ok(())
    }

    fn exists(&mut self, conn: &sqlite::Connection) -> Result<bool, sqlite::Error> {
        let query = "SELECT 
        directory_id 
        FROM directory
        WHERE path = :path
        LIMIT 1
        ";

        let mut statement = conn.prepare(query)?;

        statement.bind((":path", &self.path[..]))?;

        if let Ok(State::Row) = statement.next() {
            let directory_id = statement.read::<i64, _>(0)?;
            self.directory_id = Some(directory_id);
            return Ok(true);
        }

        Ok(false)
    }

    fn is_valid(&self) -> bool {
        self.path.len() > 0
    }

    fn delete(&self, conn: &sqlite::Connection) -> Result<(), sqlite::Error> {
        let Some(directory_id) = self.directory_id else { return Ok(()); };

        let query = "DELETE FROM directory WHERE directory_id = :directory_id";
        let mut statement = conn.prepare(query)?;

        statement.bind((":directory_id", directory_id))?;

        database::execute_statement(&mut statement)?;

        Ok(())
    }
}

impl Retrieve for Directory {
    fn get_by(
        conn: &sqlite::Connection,
        condition: Condition,
        order: Order,
    ) -> Result<Vec<Self>, sqlite::Error>
    where
        Self: Sized,
    {
        let query = format!(
            "SELECT directory_id, path
            FROM directory

            WHERE {}
            ORDER BY {}
            ",
            condition.as_query(Condition::None),
            order.as_query(asc("directory.directory_id")),
        );
        let mut directories: Vec<Directory> = Vec::new();

        let mut statement = conn.prepare(query)?;

        while let Ok(State::Row) = statement.next() {
            let directory = Directory {
                directory_id: Some(statement.read::<i64, _>("directory_id")?),
                path: statement.read::<String, _>("path")?,
            };
            directories.push(directory);
        }
        Ok(directories)
    }
}
