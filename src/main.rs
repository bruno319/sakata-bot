use std::env;

use log::*;
use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

use crate::api::SakataApi;
use crate::s3::AwsS3Client;

mod command;
mod api;
mod s3;
pub mod types;

struct Handler {
    api: SakataApi,
    s3: AwsS3Client,
}

impl Handler {
    fn new() -> Handler {
        Handler {
            api: SakataApi::new(),
            s3: AwsS3Client::new(),
        }
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        let content = msg.content.clone();
        let mut args = content.split_whitespace();
        let cmd = args.next().unwrap_or_default();
        match cmd {
            "!join" => command::join::execute(ctx, msg, &self.api).await,
            "!card" => command::card::execute(ctx, msg, &self.api, &self.s3).await,
            "!starcard" => command::starcard::execute(ctx, msg, &self.api, &self.s3).await,
            _ => {}
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        info!("{} is connected", ready.user.name)
    }
}

#[tokio::main]
async fn main() {
    std::env::set_var("RUST_LOG", "");
    env_logger::init();

    let token = env::var("DISCORD_TOKEN")
        .expect("Expected discord token in the environment");

    let mut client = Client::builder(token)
        .event_handler(Handler::new())
        .await
        .expect("Err create client");

    if let Err(e) = client.start().await {
        error!("Error on starting client: {}", e)
    }
}
