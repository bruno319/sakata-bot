use std::str::{FromStr, SplitWhitespace};

use log::*;
use serenity::model::prelude::Message;
use serenity::prelude::Context;
use serenity::utils::MessageBuilder;

use crate::api::SakataApi;
use crate::embed;
use crate::types::json::SwapCardsJson;

pub async fn execute(ctx: Context, msg: Message, args: SplitWhitespace<'_>) {
    let swap_cards = valid_cards(args);
    let swap_cards = match swap_cards {
        Ok(sc) => sc,
        Err(e) => {
            let err_msg = MessageBuilder::new()
                .mention(&msg.author)
                .push(e)
                .build();
            msg.channel_id.say(&ctx.http, err_msg).await;
            return;
        }
    };
    let party = {
        let api = {
            let data = ctx.data.read().await;
            data.get::<SakataApi>().unwrap().clone()
        };
        api.swap_cards(msg.author.id.0, swap_cards).await
    };

    let response = match party {
        Ok(mut p) => {
            msg.channel_id.send_message(&ctx.http, |m| {
                let nick = msg.author.name.clone();
                let thumb = msg.author.avatar_url().unwrap_or_default();

                m.embed(|e| embed::party(e, &mut p, &nick, &thumb));
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

fn valid_cards(mut args: SplitWhitespace) -> Result<SwapCardsJson, String> {
    let card_in = args.next().ok_or(" You must informe two cards")?;
    let card_out = args.next().ok_or(" You must informe two cards")?;

    let card_in = i32::from_str(card_in).map_err(|_| " You need provide the card id")?;
    let card_out = i32::from_str(card_out).map_err(|_| " You need provide the card id")?;

    Ok(SwapCardsJson {
        card_in,
        card_out,
    })
}