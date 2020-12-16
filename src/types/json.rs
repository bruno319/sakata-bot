use serde::{Deserialize, Serialize};

use crate::types::model::{Class, Domain, Rarity};

#[derive(Deserialize, Debug)]
pub struct Player {
    pub discord_id: i64,
    pub nickname: String,
    pub coins: i16,
    pub stardust: i16,
    pub party_power: u16,
    pub party: Vec<PlayerCard>,
}

#[derive(Deserialize, Debug)]
pub struct PlayerCard {
    pub base_card_id: u32,
    pub player_card_id: String,
    pub name: String,
    pub rarity: Rarity,
    pub class: Class,
    pub domain: Domain,
    pub overall_power: u8,
    pub image: String,
    pub quantity: u8,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerJoinedJson {
    pub nickname: String,
    pub discord_id: u64,
}