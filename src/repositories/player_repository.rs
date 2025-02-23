use rusqlite::{Connection, Result};
use chrono::{DateTime, FixedOffset};
use std::error::Error;

use crate::domain::player_domain::Player;
use super::sqlite::connection;

pub trait PlayerRepository {
    fn list_players(&self ) -> Result<()>;
    fn insert_player(&self, name: String, birthdate: DateTime<FixedOffset>, nationality: String) -> Result<i32, Box<dyn Error>>;
    fn find_player_by_id(&self, id: u32) -> Result<Player>;
    fn delete_player_by_id(&self, id:u32) -> Result<()>;
}

pub struct SqlitePlayerRepository;

impl PlayerRepository for SqlitePlayerRepository {    
    fn insert_player(&self, name: String, birthdate: DateTime<FixedOffset>, nationality: String) -> Result<i32, Box<dyn Error>> {
        let conn: Connection = connection().map_err(|e| {
            eprintln!("Error: {}", e);
            e
        })?;
    
        conn.execute(
            "INSERT INTO players (name, birthdate, nationality) VALUES (?1, ?2, ?3)",
            [name, birthdate.to_rfc3339(), nationality],
        ).map_err(|e| {
            eprintln!("Error: {}", e);
            e
        })?;
    
        let id: i32 = conn.last_insert_rowid() as i32;
    
        Ok(id)
    }
    
    fn find_player_by_id(&self, id: u32) -> Result<Player> {
        let conn: Connection = connection()?;
        let player = conn.query_row(
            "SELECT id, name, birthdate, nationality FROM players WHERE id = ?1",
            [id],
            |row| {
                Ok(Player {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    birthdate: DateTime::parse_from_rfc3339(&row.get::<_, String>(2)?).map_err(|e| {
                        eprintln!("Error: {}", e);
                        rusqlite::Error::InvalidQuery
                    })?,
                    nationality: row.get(3)?,
                })
            },
        )?;
    
        Ok(player)
    }
    
    fn list_players(&self, ) -> Result<()> {
        let conn: Connection = connection()?;
    
        let mut stmt = conn.prepare("SELECT id, name, birthdate, nationality FROM players")?;
        let usuarios = stmt.query_map([], |row| {
            Ok(Player {
                id: row.get(0)?,
                name: row.get(1)?,
                birthdate:  DateTime::parse_from_rfc3339(&row.get::<_, String>(2)?).map_err(|e| {
                        eprintln!("Error: {}", e);
                        rusqlite::Error::InvalidQuery
                    })?,
                nationality: row.get(3)?,
            })
        })?;
        
        for usuario in usuarios {
            match usuario {
                Ok(player) => println!("{}", player),
                Err(e) => eprintln!("Error: {:?}", e),
            }
        }
    
        Ok(())
    }
    
    fn delete_player_by_id(&self, id:u32) -> Result<()>{
        let conn: Connection = connection()?;
    
        conn.execute(
            "DELETE FROM players WHERE ID = ?1",
            [id], 
        ).map_err(|e| {
            eprintln!("Error: {}", e);
            e
        })?;
    
        Ok(())
    }
}


