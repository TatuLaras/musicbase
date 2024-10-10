use serde::Serialize;

use crate::param::{Condition, Order};

pub mod base_metadata;
pub mod user_generated;

// Helpers

pub fn err(message: &str) -> Result<(), sqlite::Error> {
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

// A trait that implements database (SQLite) insertion and retrieval for the object
pub trait Store {
    // Inserts the object into the database
    //
    // Takes a mutable reference to the object and fills in the id field to the newly inserted id
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

    fn delete(&self, _conn: &sqlite::Connection) -> Result<(), sqlite::Error> {
        todo!()
    }
}

pub trait StoreFull {
    // Inserts the object and all contained objects into the db
    // Fills in the id field similarly to insert of the trait Store
    fn insert_full(&mut self, conn: &sqlite::Connection) -> Result<(), sqlite::Error>;
}

pub trait Retrieve {
    // Returns a vector of all items of a given type.
    // Defined in terms of get_by, no need to define this manually.
    fn get_all(conn: &sqlite::Connection, order: Order) -> Result<Vec<Self>, sqlite::Error>
    where
        Self: Sized,
    {
        Self::get_by(conn, Condition::None, order)
    }

    // Takes a condition and returns all objects of the type that match that condition
    fn get_by(
        _conn: &sqlite::Connection,
        _condition: Condition,
        _order: Order,
    ) -> Result<Vec<Self>, sqlite::Error>
    where
        Self: Sized;
}

#[derive(Debug, Clone, PartialEq, Copy, Serialize)]
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
