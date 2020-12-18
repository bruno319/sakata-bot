use serde::{Deserialize, Serialize};

use crate::types::model::{Class, Domain, Rarity};

#[derive(Deserialize, Debug)]
pub struct Player {
    pub discord_id: i64,
    pub channel_id: i64,
    pub discriminator: u16,
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
    pub mal_id: u32,
    pub name: String,
    pub rarity: Rarity,
    pub class: Class,
    pub domain: Domain,
    pub overall_power: u8,
    pub image: String,
    pub quantity: u8,
}

#[derive(Serialize, Debug)]
pub struct PlayerJoinedJson {
    pub nickname: String,
    pub discord_id: u64,
    pub channel_id: u64,
    pub discriminator: u16,
}

#[derive(Deserialize, Debug)]
pub struct Party {
    pub id: i64,
    pub power: u16,
    pub cards: Vec<PlayerCard>,
}

impl From<Player> for Party {
    fn from(player: Player) -> Self {
        Party {
            id: player.discord_id,
            power: player.party_power,
            cards: player.party,
        }
    }
}

#[derive(Serialize, Debug)]
pub struct SwapCardsJson {
    pub card_out: i32,
    pub card_in: i32,
}