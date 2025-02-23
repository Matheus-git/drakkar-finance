use chrono::{DateTime, FixedOffset};
use crate::repositories::player_repository::insert_player;

#[derive(Debug)]
pub struct Player {
    pub id: u32,
    pub name: String,
    pub birthdate: DateTime<FixedOffset>,
    pub nationality: String
}

impl Player {
    pub fn new(name: String, birthdate: DateTime<FixedOffset>, nationality: String) ->  Result<Player, rusqlite::Error> {

        let player: Player = insert_player(name, birthdate, nationality)?;
        
        Ok(player)
    }
}
