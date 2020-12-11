use log::*;
use serenity::model::prelude::Message;
use serenity::prelude::Context;

use crate::api::SakataApi;
use crate::command::card::send_obtained_card;
use crate::s3::AwsS3Client;

pub async fn execute(ctx: Context, msg: Message) {
    let obtained_card = {
        let api = {
            let data = ctx.data.read().await;
            data.get::<SakataApi>().unwrap().clone()
        };
        api.buy_star_card(msg.author.id.0).await
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