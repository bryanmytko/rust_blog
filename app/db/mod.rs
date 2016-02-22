use std::path::Path;
use rusqlite::Connection;

pub fn connection() -> Connection {

    let sqlite_path = Path::new(::DB_NAME);
    let conn = Connection::open(sqlite_path).unwrap();

    conn
}
