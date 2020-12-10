use log::*;
use serenity::model::prelude::Message;
use serenity::prelude::Context;

use crate::api::SakataApi;
use crate::command::card::send_obtained_card;
use crate::s3::AwsS3Client;

pub async fn execute(ctx: Context, msg: Message, api: &SakataApi, s3: &AwsS3Client) {
    let obtained_card = api.buy_star_card(msg.author.id.0).await;
    match obtained_card {
        Ok(card) => {
            let image = s3.get_object(&card.image).await;
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