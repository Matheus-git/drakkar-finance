use chrono::{DateTime, FixedOffset};
use std::fmt;

#[derive(Debug)]
pub struct Player {
    pub id: u32,
    pub name: String,
    pub birthdate: DateTime<FixedOffset>,
    pub nationality: String
}

impl Player {
    pub fn new(id: u32, name: String, birthdate: DateTime<FixedOffset>, nationality: String) -> Self {
        Player {
            id, 
            name, 
            birthdate,
            nationality
        }
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Player {{ id: {}, name: {}, birthdate: {}, nationality: {} }}",
            self.id, self.name, self.birthdate, self.nationality
        )
    }
}