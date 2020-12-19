use std::cmp::Reverse;

use serenity::builder::CreateEmbed;
use serenity::utils::Colour;

use crate::types::json::{Party, PlayerCard};

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

pub fn party<'a, 'b>(embed: &'b mut CreateEmbed, party: &'a mut Party, owner: &str, thumb: &str) -> &'b mut CreateEmbed {
    embed.color(Colour::DARK_RED);
    embed.title(format!("{}'s Party", owner));
    embed.description(format!("Overall Power **{}**\u{2694}", party.power));
    embed.thumbnail(thumb);

    party.cards.sort_by_key(|c| Reverse(c.overall_power));
    let (cards, stats, ids) = mount_card_list(&party.cards);
    embed.field("Cards", cards, true);
    embed.field("Stats", stats, true);
    embed.field("ID", ids, true);

    embed
}

pub fn card_list<'a, 'b>(embed: &'b mut CreateEmbed, cards: &'a Vec<PlayerCard>, owner: &str, thumb: &str) -> &'b mut CreateEmbed {
    embed.color(Colour::DARK_GREY);
    embed.title(format!("{}'s Card Results", owner));
    embed.thumbnail(thumb);

    let (cards, stats, ids) = mount_card_list(cards);
    embed.field("Cards", cards.clone(), true);
    embed.field("Stats", stats, true);
    embed.field("ID", ids, true);

    embed
}

fn mount_card_list(cards: &Vec<PlayerCard>) -> (String, String, String) {
    cards.iter()
        .map(|c| {
            let n = format!("{}\n", c.name);
            let s = format!(
                "[**{}**] {} **{}**-**{}**\n",
                c.overall_power,
                c.rarity.emoji(),
                c.class.abbrev(),
                c.domain.abbrev()
            );
            let id = format!("{}\n", c.mal_id);
            (n, s, id)
        })
        .fold(("".to_string(), "".to_string(), "".to_string()), |acc, i| {
            (format!("{}{}", acc.0, i.0), format!("{}{}", acc.1, i.1), format!("{}{}", acc.2, i.2))
        })
}