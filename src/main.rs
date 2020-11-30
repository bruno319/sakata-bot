use std::env;

use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

use jikan_rs::client::Jikan;
use jikan_rs::prelude::{SearchQueryBuilder, SearchSource, SearchResultEnum};

const HELP_MESSAGE: &str = "
          Hello there, Human!

          You have summoned me. Let's see about getting you what you need.

          ? Need technical help?
          => Post in the <#366230177413595138> channel and other humans will assist you.
          
          ? Looking for the Code of Conduct?
          => Here it is: <https://opensource.facebook.com/code-of-conduct> 
          
          ? Something wrong?
          => You can flag an admin with @admin
          
          I hope that resolves your issue!
          -- Helpbot
          
          ";

struct Handler {
    jikan: Jikan,
}

impl Handler {
    fn new() -> Handler {
        Handler {
            jikan: Jikan::new()
        }
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        let mut split_msg = msg.content.split(" ");
        let cmd = split_msg.nth(0).unwrap_or_default();
        match cmd {
            "!help" => if let Err(why) = msg.channel_id.say(&ctx.http, HELP_MESSAGE).await {
                eprintln!("Error sending message: {:?}", why);
            },
            "!anime" => {
                let anime = split_msg.collect::<Vec<&str>>().join(" ");
                let result = self.jikan
                    .search(SearchQueryBuilder::new(SearchSource::Anime).name(&anime))
                    .await;
                match result {
                    Ok(result) => {
                        match result {
                            SearchResultEnum::Anime(a) => {
                                for x in a.results {
                                    if let Err(why) = msg.channel_id.say(&ctx.http, x.url).await {
                                        eprintln!("Error sending message: {:?}", why);
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
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