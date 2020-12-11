use std::env;
use std::sync::Arc;

use reqwest::Error;
use serenity::prelude::TypeMapKey;

use crate::types::{Player, PlayerCard, PlayerDto};

pub struct SakataApi {
    api_url: String,
    client: reqwest::Client,
}

impl TypeMapKey for SakataApi {
    type Value = Arc<SakataApi>;
}

impl SakataApi {
    pub fn new() -> SakataApi {
        SakataApi {
            api_url: env::var("SAKATA_API_URL").unwrap(),
            client: reqwest::Client::new(),
        }
    }

    pub async fn save_player(&self, player: PlayerDto) -> Result<Player, Error> {
        self.client
            .post(&format!("{}/players", self.api_url))
            .json(&player)
            .send()
            .await?
            .json()
            .await
    }

    pub async fn buy_common_card(&self, discord_id: u64) -> Result<PlayerCard, Error> {
        self.client
            .get(&format!("{}/players/{}/common-card", self.api_url, discord_id))
            .send()
            .await?
            .json()
            .await
    }

    pub async fn buy_star_card(&self, discord_id: u64) -> Result<PlayerCard, Error> {
        self.client
            .get(&format!("{}/players/{}/star-card", self.api_url, discord_id))
            .send()
            .await?
            .json()
            .await
    }
}