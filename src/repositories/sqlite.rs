use rusqlite::{Connection, Result};

pub fn connection() -> Result<Connection> {
    let conn: Connection = Connection::open("drakkar-finance.sqlite")?;
    Ok(conn)
}