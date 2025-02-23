use chrono::{DateTime, FixedOffset};
use std::error::Error;

use crate::repositories::player_repository::PlayerRepository;
use super::player_domain::Player;

pub struct PlayerService<T: PlayerRepository> {
    player_repository: T
}

impl<T: PlayerRepository> PlayerService<T> {
    pub fn new(player_repository: T) -> Self {
        PlayerService { player_repository }
    }

    pub fn save_player(&self, name: String, birthdate: DateTime<FixedOffset>, nationality: String) -> Result<Player, Box<dyn Error>>{
        let player_id: i32 = self.player_repository.insert_player(name.clone(), birthdate, nationality.clone())?;

        let player: Player = Player::new(player_id as u32, name, birthdate, nationality);
        
        Ok(player)
    }
    
    pub fn find_player_by_id(&self, id: u32) -> Result<Player, Box<dyn Error>> {
        let player: Player = self.player_repository.find_player_by_id(id)?;
        Ok(player)
    } 

    pub fn list_players(&self) {
        let _ = &self.player_repository.list_players();
    }

    pub fn delete_player_by_id(&self, id:u32) {
        let _ = self.player_repository.delete_player_by_id(id);
    }
}