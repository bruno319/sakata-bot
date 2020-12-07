use serenity::model::prelude::Message;
use serenity::prelude::Context;

use crate::api::SakataApi;
use crate::command::card::send_obtained_card;

pub async fn execute(ctx: Context, msg: Message, api: &SakataApi) {
    let obtained_card = api.buy_star_card(msg.author.id.0).await;
    send_obtained_card(&ctx, msg, obtained_card).await
}