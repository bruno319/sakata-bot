use log::*;
use serenity::model::prelude::{Message, User};
use serenity::prelude::Context;
use serenity::utils::MessageBuilder;

use crate::api::SakataApi;
use crate::embed;
use crate::types::json::{Player, PlayerJoinedJson};

pub async fn execute(ctx: Context, msg: Message) {
    let player = PlayerJoinedJson {
        nickname: msg.author.name.clone(),
        discord_id: msg.author.id.0,
    };

    let player = {
        let api = {
            let data = ctx.data.read().await;
            data.get::<SakataApi>().unwrap().clone()
        };
        api.save_player(player).await
    };

    let response = match player {
        Ok(mut p) => {
            let thumbnail = msg.author.avatar_url().unwrap_or_default();
            msg.channel_id.send_message(&ctx.http, |m| {
                let content = create_join_msg(&msg.author, &p);
                m.content(content);
                m.embed(|e| embed::party(e, &mut p, &thumbnail));
                m
            }).await
        }
        Err(e) => {
            error!("{}", e);
            let err_msg = MessageBuilder::new()
                .mention(&msg.author)
                .push(format!(", could not possible invite you to server. Try again please"))
                .build();
            msg.channel_id.say(&ctx.http, err_msg).await
        }
    };

    if let Err(e) = response {
        error!("{}", e);
    };
}

fn create_join_msg(author: &User, p: &Player) -> String {
    MessageBuilder::new()
        .mention(author)
        .push(format!(" you joined to server. You start with the following cards plus {}\u{1FA99} ", p.coins))
        .push_bold_safe("Coins ")
        .push(format!("and {}\u{1F4AB} ", p.stardust))
        .push_bold_safe("Stardust")
        .build()
}