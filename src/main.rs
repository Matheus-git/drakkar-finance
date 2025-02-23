pub mod domain;
pub mod repositories;
pub mod utils;

use crate::repositories::player_repository::{list_players, delete_player_by_id,  create_tables, find_player_by_id };

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // create_tables();

    // Player::new(
    //     "Matheus Silva".to_string(),
    //     parse_str_to_date("18-07-1995 00:00:00 -0300")?,
    //     "Brasileiro".to_string(),
    // );

    delete_player_by_id(5);

    list_players();
    Ok(())
}