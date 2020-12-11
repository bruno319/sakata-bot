use bytes::{Buf, Bytes};
use log::*;
use serenity::builder::CreateEmbed;
use serenity::model::prelude::Message;
use serenity::prelude::Context;

use crate::api::SakataApi;
use crate::s3::AwsS3Client;
use crate::types::PlayerCard;

pub async fn execute(ctx: Context, msg: Message) {
    let obtained_card = {
        let api = {
            let data = ctx.data.read().await;
            data.get::<SakataApi>().unwrap().clone()
        };
        api.buy_common_card(msg.author.id.0).await
    };
    match obtained_card {
        Ok(card) => {
            let image = {
                let s3 = {
                    let data = ctx.data.read().await;
                    data.get::<AwsS3Client>().unwrap().clone()
                };
                s3.get_object(&card.image).await
            };
            send_obtained_card(&ctx, msg, card, image.unwrap()).await
        }
        Err(e) => {
            if let Err(e) = msg.channel_id
                .say(&ctx.http, format!("Could not buy card: {}", e))
                .await
            {
                error!("{}", e);
            }
        }
    }
}

pub async fn send_obtained_card(ctx: &Context, msg: Message, card: PlayerCard, image: Bytes) {
    let attachment = (image.bytes(), card.image.as_str());
    let thumbnail = msg.author.avatar_url().unwrap_or_default();
    let result_send_msg = msg.channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| create_embed(e, &card, &thumbnail));
            m.add_file(attachment);
            m
        }).await;

    if let Err(e) = result_send_msg {
        error!("{}", e)
    }
}

fn create_embed<'a, 'b>(embed: &'b mut CreateEmbed, card: &'a PlayerCard, thumb: &str) -> &'b mut CreateEmbed {
    embed.color(card.rarity.get_colour());
    embed.title(&card.name);
    embed.thumbnail(thumb);
    embed.attachment(&card.image);
    embed.fields(vec![
        ("Rarity", card.rarity.to_string(), false),
        ("Class", card.class.to_string(), true),
        ("Genre", card.genre.to_string(), true),
    ]);
    embed
}