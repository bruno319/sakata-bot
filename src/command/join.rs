use log::*;
use serenity::model::prelude::Message;
use serenity::prelude::Context;
use serenity::utils::MessageBuilder;

use crate::api::SakataApi;
use crate::types::PlayerDto;

pub async fn execute(ctx: Context, msg: Message, api: &SakataApi) {
    let player = PlayerDto {
        nickname: msg.author.name.clone(),
        discord_id: msg.author.id.0,
    };

    let player = api.save_player(player).await;

    let response = match player {
        Ok(p) => MessageBuilder::new()
            .mention(&msg.author)
            .push(format!(" you joined to server. You start with {}\u{1FA99} ", p.coins))
            .push_bold_safe("Coins ")
            .push(format!("and {}\u{1F4AB} ", p.stardust))
            .push_bold_safe("Stardust ")
            .push(" to build your initial squad")
            .build(),
        Err(e) => {
            error!("{}", e);
            MessageBuilder::new()
                .mention(&msg.author)
                .push(format!(", could not possible invite you to server. Try again please"))
                .build()
        }
    };

    if let Err(e) = msg.channel_id.say(&ctx.http, response).await {
        error!("{}", e);
    };
}