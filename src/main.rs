use std::env;
use std::sync::Arc;

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
mod types;
mod embed;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        let content = msg.content.clone();
        let mut args = content.split_whitespace();
        let cmd = args.next().unwrap_or_default();
        match cmd {
            "!join" => command::join::execute(ctx, msg).await,
            "!card" => command::card::execute(ctx, msg).await,
            "!starcard" => command::starcard::execute(ctx, msg).await,
            "!party" => command::party::execute(ctx, msg).await,
            "!swap" => command::swap::execute(ctx, msg, args).await,
            "!c" => command::collection::execute(ctx, msg, args).await,
            _ => {}
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        info!("{} is connected", ready.user.name)
    }
}

#[tokio::main]
async fn main() {
    if let Err(e) = setup_logger() {
        panic!("Could not setup logger: {}", e);
    }

    let token = env::var("DISCORD_TOKEN")
        .expect("Expected discord token in the environment");

    let mut client = Client::builder(token)
        .event_handler(Handler)
        .await
        .expect("Err create client");

    {
        let mut data = client.data.write().await;

        data.insert::<AwsS3Client>(Arc::new(AwsS3Client::new()));
        data.insert::<SakataApi>(Arc::new(SakataApi::new()));
    }

    if let Err(e) = client.start().await {
        error!("Error on starting client: {}", e)
    }
}

fn setup_logger() -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .level_for("tracing", log::LevelFilter::Error)
        .chain(std::io::stdout())
        .chain(fern::log_file(
            chrono::Utc::now().format("/tmp/sakata-bot[%Y-%m-%d].log").to_string())?
        ).apply()?;
    Ok(())
}