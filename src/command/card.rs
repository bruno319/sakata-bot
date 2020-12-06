use std::path::Path;

use log::*;
use serenity::builder::CreateEmbed;
use serenity::http::AttachmentType;
use serenity::model::prelude::Message;
use serenity::prelude::Context;

use crate::api::SakataApi;
use crate::model::PlayerCard;

pub async fn execute(ctx: Context, msg: Message, api: &SakataApi) {
    let acquired_card = api.buy_common_card(msg.author.id.0).await;

    let result_send_msg = match acquired_card {
        Ok(card) => {
            msg.channel_id.send_message(&ctx.http, |m| {
                m.content(format!("{} acquired {}", msg.author.name, card.name));
                m.embed(|e| create_embed(e, &card));
                m.add_file(AttachmentType::Path(Path::new(&card.image_url)));
                m
            }).await
        }
        Err(e) => {
            msg.channel_id.say(&ctx.http, format!("Could not buy card: {}", e)).await
        }
    };

    if let Err(e) = result_send_msg {
        error!("{}", e)
    }
}

fn create_embed<'a, 'b>(embed: &'b mut CreateEmbed, card: &'a PlayerCard) -> &'b mut CreateEmbed {
    embed.color(card.rarity.get_colour());
    embed.title(&card.name);
    embed.fields(vec![
        ("Rarity", card.rarity.to_string(), false),
        ("Class", card.class.to_string(), true),
        ("Genre", card.genre.to_string(), true),
        ("Quantity", card.quantity.to_string(), true),
    ]);
    embed
}