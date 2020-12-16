use std::cmp::Reverse;

use serenity::builder::CreateEmbed;
use serenity::utils::Colour;

use crate::types::json::{Player, PlayerCard};

pub fn card<'a, 'b>(embed: &'b mut CreateEmbed, card: &'a PlayerCard, thumb: &str) -> &'b mut CreateEmbed {
    embed.color(card.rarity.get_colour());
    embed.title(format!("{}", card.name));
    embed.description(format!("{} Card", card.rarity.to_string()));
    embed.thumbnail(thumb);
    embed.attachment(&card.image);
    embed.fields(vec![
        ("Class", card.class.to_string(), true),
        ("Domain", card.domain.to_string(), true),
    ]);
    embed
}

pub fn party<'a, 'b>(embed: &'b mut CreateEmbed, player: &'a mut Player, thumb: &str) -> &'b mut CreateEmbed {
    embed.color(Colour::DARK_RED);
    embed.title(format!("{}'s Party", player.nickname));
    embed.description(format!("Overall Power **{}**\u{2694}", player.party_power));
    embed.thumbnail(thumb);
    player.party.sort_by_key(|c| Reverse(c.overall_power));
    let (cards, stats) = player.party
        .iter()
        .map(|c| {
            let n = format!("{}\n", c.name);
            let s = format!("[**{}**] {}\n", c.overall_power, c.rarity.to_string());
            (n, s)
        })
        .fold(("".to_string(), "".to_string()), |acc, i| {
            (format!("{}{}", acc.0, i.0), format!("{}{}", acc.1, i.1))
        });
    embed.field("Cards", cards, true);
    embed.field("Stats", stats, true);

    embed
}