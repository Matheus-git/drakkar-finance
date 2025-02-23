pub mod domain;
pub mod repositories;
pub mod utils;

use crate::repositories::player_repository::SqlitePlayerRepository;
use crate::repositories::sqlite::create_tables;
use crate::domain::player_service::PlayerService;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    create_tables()?;

    let player_service = PlayerService::new(SqlitePlayerRepository);

    // ⚠️ Uncomment the lines below to execute specific actions such as listing, saving, finding, or deleting a player.


    player_service.list_players();

    // let player =  player_service.save_player(
    //     "Matheus Silva".to_string(), 
    //     utils::date_utils::parse_str_to_date("11-11-2011 00:00:00 -0300")?,  
    //     "Brasileiro".to_string()
    // )?;

    // let player = player_service.find_player_by_id(1)?;

    // player_service.delete_player_by_id(12);

    // println!("{}", player);

    Ok(())
}