use std::env;
use std::sync::Arc;

use reqwest::Error;
use serenity::prelude::TypeMapKey;

use crate::types::json::{Party, Player, PlayerCard, PlayerJoinedJson, SwapCardsJson};

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

    pub async fn save_player(&self, player: PlayerJoinedJson) -> Result<Player, Error> {
        self.client
            .post(&format!("{}/players", self.api_url))
            .json(&player)
            .send()
            .await?
            .json()
            .await
    }

    pub async fn party(&self, discord_id: u64) -> Result<Party, Error> {
        self.client
            .get(&format!("{}/players/{}/party", self.api_url, discord_id))
            .send()
            .await?
            .json()
            .await
    }

    pub async fn swap_cards(&self, discord_id: u64, cards: SwapCardsJson) -> Result<Party, Error> {
        self.client
            .post(&format!("{}/players/{}/party/swap", self.api_url, discord_id))
            .json(&cards)
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

    pub async fn query_cards(&self, discord_id: u64, query: Vec<(String, String)>) -> Result<Vec<PlayerCard>, Error> {
        self.client
            .get(&format!("{}/players/{}/cards", self.api_url, discord_id))
            .query(&query)
            .send()
            .await?
            .json()
            .await
    }
}