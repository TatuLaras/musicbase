use sqlite::{Connection, State};

use crate::models::{Artist, Store};

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

";
        self.conn.execute(query).unwrap();
    }

    pub fn insert(&self, item: &impl Store) -> Result<(), sqlite::Error> {
        item.insert(&self.conn)
    }

    pub fn exists(&self, item: &impl Store) -> Result<bool, sqlite::Error> {
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
