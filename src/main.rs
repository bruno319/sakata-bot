use std::env;

use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

use crate::api::SakataApi;

mod command;
mod api;
pub mod model;

struct Handler {
    api: SakataApi,
}

impl Handler {
    fn new() -> Handler {
        Handler {
            api: SakataApi::new(),
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
            _ => {}
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected", ready.user.name)
    }
}

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN")
        .expect("Expected discord token in the environment");

    let mut client = Client::builder(token)
        .event_handler(Handler::new())
        .await
        .expect("Err create client");

    if let Err(e) = client.start().await {
        eprintln!("Error on starting client: {}", e)
    }
}
