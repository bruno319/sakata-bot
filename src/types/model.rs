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
    Rogue = 5,
    Support = 6,
    Beast = 7,
    Machinist = 8,
    Supernatural = 9,
    Scholar = 10,
    Worker = 11,
    Musician = 12,
}

#[derive(Deserialize_repr, PartialEq, Debug)]
#[repr(i8)]
pub enum Domain {
    Unknown = -1,
    Action = 1,
    Adventure = 2,
    Fantasy = 3,
    SciFi = 4,
    Sports = 5,
    Mystery = 6,
    SliceOfLife = 7,
    Comedy = 8,
    Romance = 9,
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

    pub fn emoji(&self) -> String {
        match self {
            Rarity::Silver => "\u{26AA}",
            Rarity::Gold => "\u{1F7E1}",
            Rarity::Epic => "\u{1F7E3}",
            Rarity::Legend => "\u{1F535}",
            Rarity::Unknown => "\u{26AB}"
        }.to_string()
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

impl Class {
    pub fn abbrev(&self) -> String {
        match self {
            Class::Fighter => "FIG",
            Class::Magician => "MAG",
            Class::Swordsman => "SWO",
            Class::Ranger => "RNG",
            Class::Rogue => "ROG",
            Class::Support => "SUP",
            Class::Beast => "BEA",
            Class::Machinist => "MCH",
            Class::Supernatural => "SPR",
            Class::Scholar => "SCH",
            Class::Worker => "WRK",
            Class::Musician => "MUS",
            Class::Unknown => ""
        }.to_string()
    }
}

impl ToString for Class {
    fn to_string(&self) -> String {
        match self {
            Class::Fighter => "Fighter".to_string(),
            Class::Magician => "Magician".to_string(),
            Class::Swordsman => "Swordsman".to_string(),
            Class::Ranger => "Ranger".to_string(),
            Class::Rogue => "Rogue".to_string(),
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
            "rogue" => Class::Rogue,
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

impl Domain {
    pub fn abbrev(&self) -> String {
        match self {
            Domain::Action => "ACT",
            Domain::Adventure => "ADV",
            Domain::Fantasy => "FAN",
            Domain::SciFi => "SCF",
            Domain::Sports => "SPO",
            Domain::Mystery => "MYS",
            Domain::SliceOfLife => "SOL",
            Domain::Comedy => "CMD",
            Domain::Romance => "ROM",
            Domain::Unknown => "",
        }.to_string()
    }
}

impl ToString for Domain {
    fn to_string(&self) -> String {
        match self {
            Domain::Action => "Action".to_string(),
            Domain::Adventure => "Adventure".to_string(),
            Domain::Fantasy => "Fantasy".to_string(),
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
            "sci-fi" | "scifi" => Domain::SciFi,
            "sports" => Domain::Sports,
            "mystery" => Domain::Mystery,
            "slice of life" | "sliceoflife" | "slice" | "life" => Domain::SliceOfLife,
            "comedy" => Domain::Comedy,
            "romance" => Domain::Romance,
            _ => Domain::Unknown,
        }
    }
}