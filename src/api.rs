use std::env;
use crate::model::{PlayerDto, Player};
use reqwest::Error;

pub struct SakataApi {
    api_url: String,
    client: reqwest::Client
}

impl SakataApi {
    pub fn new() -> SakataApi {
        SakataApi {
            api_url: env::var("SAKATA_API_URL").unwrap(),
            client: reqwest::Client::new()
        }
    }
    pub async fn save_player(&self, player: PlayerDto) -> Result<Player, Error> {
        self.client
            .post(&format!("{}/players", self.api_url))
            .json(&player)
            .send()
            .await?
            .json::<Player>()
            .await
    }
}