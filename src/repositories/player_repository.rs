use rusqlite::{Connection, Result};
use chrono::{DateTime, FixedOffset};

use crate::domain::player_domain::Player;
use crate::utils::date_utils::now_utc_with_timezone;
use super::sqlite::connection;

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

    conn.execute(
        "CREATE TABLE IF NOT EXISTS contracts (
            id INTEGER PRIMARY KEY,
            player_id INTEGER NOT NULL,
            start_date DATE NOT NULL,
            end_date DATE NOT NULL,
            salary REAL NOT NULL,
            bonus REAL,
            release_clause REAL,
            FOREIGN KEY (player_id) REFERENCES players(id)
        )",
        [], 
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS transactions (
            id INTEGER PRIMARY KEY,
            player_id INTEGER NOT NULL,
            type TEXT NOT NULL CHECK(type IN ('purchase', 'sale', 'renewal')),
            value REAL NOT NULL,
            date DATE NOT NULL,
            FOREIGN KEY (player_id) REFERENCES players(id)
        )",
        [], 
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS alerts (
            id INTEGER PRIMARY KEY,
            player_id INTEGER NOT NULL,
            message TEXT NOT NULL,
            alert_date DATE NOT NULL,
            FOREIGN KEY (player_id) REFERENCES players(id)
        )",
        [], 
    )?;

    println!("Tables created successfully!");
    Ok(())
}

pub fn insert_player(name: String, birthdate: DateTime<FixedOffset>, nationality: String) -> Result<Player> {
    let conn: Connection = connection().map_err(|e| {
        eprintln!("Erro ao conectar ao banco de dados: {}", e);
        e
    })?;

    conn.execute(
        "INSERT INTO players (name, birthdate, nationality) VALUES (?1, ?2, ?3)",
        [name, birthdate.to_rfc3339(), nationality],
    ).map_err(|e| {
        eprintln!("Erro ao inserir jogador: {}", e);
        e
    })?;

    let id = conn.last_insert_rowid() as i32;

    // Consulta o jogador inserido
    let player = conn.query_row(
        "SELECT id, name, birthdate, nationality FROM players WHERE id = ?1",
        [id],
        |row| {
            Ok(Player {
                id: row.get(0)?,
                name: row.get(1)?,
                birthdate: DateTime::parse_from_rfc3339(&row.get::<_, String>(2)?).map_err(|e| {
                    eprintln!("Erro ao converter data: {}", e);
                    rusqlite::Error::InvalidQuery
                })?,
                nationality: row.get(3)?,
            })
        },
    )?;

    println!("Usuário inserido com sucesso!");
    Ok(player)
}

pub fn find_player_by_id(id: u32) -> Result<Player> {
    let conn: Connection = connection()?;
    let player = conn.query_row(
        "SELECT id, name, birthdate, nationality FROM players WHERE id = ?1",
        [id],
        |row| {
            Ok(Player {
                id: row.get(0)?,
                name: row.get(1)?,
                birthdate: DateTime::parse_from_rfc3339(&row.get::<_, String>(2)?).map_err(|e| {
                    eprintln!("Erro ao converter data: {}", e);
                    rusqlite::Error::InvalidQuery
                })?,
                nationality: row.get(3)?,
            })
        },
    )?;
    println!("Get player {:?}", player);

    Ok(player)
}

pub fn list_players() -> Result<()> {
    let conn: Connection = connection()?;

    let mut stmt = conn.prepare("SELECT id, name, birthdate, nationality FROM players").map_err(|e| {
        eprintln!("Erro: {:?}", e);
        e
    })?;
    let usuarios = stmt.query_map([], |row| {
        println!("Usuário");
        Ok(Player {
            id: row.get(0)?,
            name: row.get(1)?,
            birthdate: now_utc_with_timezone(),
            nationality: row.get(3)?,
        })
    }).map_err(|e| {
        eprintln!("Erro ao executar consulta: {:?}", e);
        e
    })?;

    let mut found = false; 

    for usuario in usuarios {
        found = true;
        match usuario {
            Ok(player) => println!("Usuário: {:?}", player),
            Err(e) => eprintln!("Erro ao processar o usuário: {:?}", e),
        }
    }

    if !found {
        println!("Usuários nao encontrados");
    }

    Ok(())
}

pub fn delete_player_by_id(id:u32) -> Result<()>{
    let conn: Connection = connection()?;

    conn.execute(
        "DELETE FROM players WHERE ID = ?1",
        [id], 
    )?;

    Ok(())
}
