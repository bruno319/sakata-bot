use log::*;
use serenity::model::prelude::Message;
use serenity::prelude::Context;
use serenity::utils::MessageBuilder;

use crate::api::SakataApi;
use crate::embed;

pub async fn execute(ctx: Context, msg: Message) {
    let party = {
        let api = {
            let data = ctx.data.read().await;
            data.get::<SakataApi>().unwrap().clone()
        };
        api.party(msg.author.id.0).await
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