use crate::database::ConnectionWrapper;

pub fn get_mock_db() -> ConnectionWrapper {
    let db = ConnectionWrapper {
        conn: sqlite::open(":memory:").expect("Connection failed"),
    };
    db.create_schema().unwrap();
    db
}
