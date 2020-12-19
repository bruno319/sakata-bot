use std::str::SplitWhitespace;

use log::*;
use serenity::model::prelude::Message;
use serenity::prelude::Context;
use serenity::utils::MessageBuilder;

use crate::api::SakataApi;
use crate::embed;
use crate::types::model::{Class, Domain};

pub async fn execute(ctx: Context, msg: Message, args: SplitWhitespace<'_>) {
    let query = valid_query(args);

    let cards = {
        let api = {
            let data = ctx.data.read().await;
            data.get::<SakataApi>().unwrap().clone()
        };
        api.query_cards(msg.author.id.0, query).await
    };

    let response = match cards {
        Ok(mut c) => {
            msg.channel_id.send_message(&ctx.http, |m| {
                let nick = msg.author.name.clone();
                let thumb = msg.author.avatar_url().unwrap_or_default();

                m.embed(|e| embed::card_list(e, &mut c, &nick, &thumb));
                m
            }).await
        }
        Err(e) => {
            error!("{}", e);
            let err_msg = MessageBuilder::new()
                .mention(&msg.author)
                .push(format!(", something happens. Try again please"))
                .build();
            msg.channel_id.say(&ctx.http, err_msg).await
        }
    };

    if let Err(e) = response {
        error!("{}", e);
    };
}

fn valid_query(args: SplitWhitespace) -> Vec<(String, String)> {
    let mut query_params = Vec::with_capacity(3);
    for a in args {
        let mut qu_split = a.split(':');
        let key = qu_split.nth(0)
            .unwrap_or_default()
            .trim()
            .to_lowercase();

        if key.eq("class") {
            let class = Class::from(qu_split.nth(0));
            if class != Class::Unknown {
                query_params.push(("class".to_string(), (class as i8).to_string()))
            }
        }

        if key.eq("domain") {
            let domain = Domain::from(qu_split.nth(0));
            if domain != Domain::Unknown {
                query_params.push(("domain".to_string(), (domain as i8).to_string()))
            }
        }
    }

    query_params.push(("page".to_string(), "1".to_string()));

    query_params
}