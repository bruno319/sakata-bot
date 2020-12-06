use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Player {
    pub id: Option<i32>,
    pub nickname: String,
    pub coins: i16,
    pub stardust: i16,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerDto {
    pub nickname: String,
    pub discord_id: u64,
}