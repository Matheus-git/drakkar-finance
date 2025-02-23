use rusqlite::{Connection, Result};

pub fn connection() -> Result<Connection> {
    let conn: Connection = Connection::open("drakkar-finance.sqlite")?;
    Ok(conn)
}

pub fn create_tables() -> Result<()> {
    let conn: Connection = connection()?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS players (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            birthdate DATE NOT NULL,
            nationality TEXT NOT NULL
        )",
        [], 
    )?;
    
    Ok(())
}