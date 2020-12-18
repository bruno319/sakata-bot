use serde_repr::Deserialize_repr;
use serenity::utils::Colour;

#[derive(Deserialize_repr, PartialEq, Debug)]
#[repr(i8)]
pub enum Class {
    Unknown = -1,
    Fighter = 1,
    Magician = 2,
    Swordsman = 3,
    Ranger = 4,
    Support = 5,
    Beast = 6,
    Machinist = 7,
    Supernatural = 8,
    Scholar = 9,
    Worker = 10,
    Musician = 11,
}

#[derive(Deserialize_repr, PartialEq, Debug)]
#[repr(i8)]
pub enum Domain {
    Unknown = -1,
    Action = 1,
    Adventure = 2,
    SciFi = 3,
    Sports = 4,
    Mystery = 5,
    SliceOfLife = 6,
    Comedy = 7,
    Romance = 8,
}

#[derive(Deserialize_repr, PartialEq, Debug)]
#[repr(i8)]
pub enum Rarity {
    Unknown = -1,
    Silver = 1,
    Gold = 2,
    Epic = 3,
    Legend = 4,
}

impl Rarity {
    pub fn get_colour(&self) -> Colour {
        match self {
            Rarity::Unknown => Colour::default(),
            Rarity::Silver => Colour::LIGHTER_GREY,
            Rarity::Gold => Colour::GOLD,
            Rarity::Epic => Colour::from((204, 0, 204)),
            Rarity::Legend => Colour::from((102, 255, 224)),
        }
    }
}

impl ToString for Rarity {
    fn to_string(&self) -> String {
        match self {
            Rarity::Silver => "Silver".to_string(),
            Rarity::Gold => "Gold".to_string(),
            Rarity::Epic => "Epic".to_string(),
            Rarity::Legend => "Legend".to_string(),
            _ => "".to_string()
        }
    }
}

impl ToString for Class {
    fn to_string(&self) -> String {
        match self {
            Class::Fighter => "Fighter".to_string(),
            Class::Magician => "Magician".to_string(),
            Class::Swordsman => "Swordsman".to_string(),
            Class::Ranger => "Ranger".to_string(),
            Class::Support => "Support".to_string(),
            Class::Beast => "Beast".to_string(),
            Class::Machinist => "Machinist".to_string(),
            Class::Supernatural => "Supernatural".to_string(),
            Class::Scholar => "Scholar".to_string(),
            Class::Worker => "Worker".to_string(),
            Class::Musician => "Musician".to_string(),
            Class::Unknown => "".to_string(),
        }
    }
}

impl From<Option<&str>> for Class {
    fn from(s: Option<&str>) -> Self {
        match s.unwrap_or_default().to_lowercase().as_str() {
            "fighter" => Class::Fighter,
            "magician" => Class::Magician,
            "swordsman" => Class::Swordsman,
            "ranger" => Class::Ranger,
            "support" => Class::Support,
            "beast" => Class::Beast,
            "machinist" => Class::Machinist,
            "supernatural" => Class::Supernatural,
            "scholar" => Class::Scholar,
            "worker" => Class::Worker,
            "musician" => Class::Musician,
            _ => Class::Unknown,
        }
    }
}

impl ToString for Domain {
    fn to_string(&self) -> String {
        match self {
            Domain::Action => "Action".to_string(),
            Domain::Adventure => "Adventure".to_string(),
            Domain::SciFi => "Sci-Fi".to_string(),
            Domain::Sports => "Sports".to_string(),
            Domain::Mystery => "Mystery".to_string(),
            Domain::SliceOfLife => "Slice of Life".to_string(),
            Domain::Comedy => "Comedy".to_string(),
            Domain::Romance => "Romance".to_string(),
            Domain::Unknown => "".to_string(),
        }
    }
}

impl From<Option<&str>> for Domain {
    fn from(s: Option<&str>) -> Self {
        match s.unwrap_or_default().to_lowercase().as_str() {
            "action" => Domain::Action,
            "adventure" => Domain::Adventure,
            "sci-Fi" => Domain::SciFi,
            "sports" => Domain::Sports,
            "mystery" => Domain::Mystery,
            "slice of Life" => Domain::SliceOfLife,
            "comedy" => Domain::Comedy,
            "romance" => Domain::Romance,
            _ => Domain::Unknown,
        }
    }
}