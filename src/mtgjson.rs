use rustc_serialize::{json};
use crate::mana::Mana;
use crate::colors::Color;
use std::io::Error;
use url;
use reqwest;

#[derive(Debug)]
pub enum MtgError {
    General(String),
    IO(Error),
    JsonParser(json::ParserError),
    JsonDecoder(json::DecoderError),
    JsonEncoder(json::EncoderError),
    UrlError(url::ParseError),
    Reqwest(reqwest::Error)
}

impl From<Error> for MtgError {
    fn from(err: Error) -> MtgError {
        MtgError::IO(err)
    }
}

impl From<url::ParseError> for MtgError {
    fn from(err: url::ParseError) -> MtgError {
        MtgError::UrlError(err)
    }
}

impl From<reqwest::Error> for MtgError {
    fn from(err: reqwest::Error) -> MtgError {
        MtgError::Reqwest(err)
    }
}

impl From<json::ParserError> for MtgError {
    fn from(err: json::ParserError) -> MtgError {
        MtgError::JsonParser(err)
    }
}

impl From<json::DecoderError> for MtgError {
    fn from(err: json::DecoderError) -> MtgError {
        MtgError::JsonDecoder(err)
    }
}

impl From<json::EncoderError> for MtgError {
    fn from(err: json::EncoderError) -> MtgError {
        MtgError::JsonEncoder(err)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
pub enum Rarity {
    Mythic, Rare, Uncommon, Common, BasicLand, Special
}

impl Rarity {
    pub fn parse(s: &str) -> Result<Rarity, MtgError> {
        match s {
            "Mythic Rare" => Ok(Rarity::Mythic),
            "Rare" => Ok(Rarity::Rare),
            "Uncommon" => Ok(Rarity::Uncommon),
            "Common" => Ok(Rarity::Common),
            "Special" => Ok(Rarity::Special),
            "Basic Land" => Ok(Rarity::BasicLand),
            _ => Err(MtgError::General(String::from(s)))
        }
    }

    pub fn short(self) -> &'static str {
        match self {
            Rarity::Mythic => "M",
            Rarity::Rare => "R",
            Rarity::Uncommon => "U",
            Rarity::Common => "C",
            Rarity::Special => "S",
            Rarity::BasicLand => "L"
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Card {
    pub card_name   : String,
    pub mana_cost   : Mana,
    pub colors      : Vec<Color>,
    pub card_type   : String,
    pub super_types : Vec<String>,
    pub card_types  : Vec<String>,
    pub sub_types   : Vec<String>,
    pub power       : String,
    pub toughness   : String,
    pub card_text   : String,
    pub image_name  : String,
    pub expansion   : String,
    pub rarity      : Rarity
}

pub fn fetch(set: &str) -> Result<json::Json, MtgError> {
    let loc = format!("http://mtgjson.com/json/{}.json", set);
    let url = url::Url::parse(loc.as_str())?;
    let text = reqwest::blocking::get(url)?.text()?;
    let json = json::Json::from_str(text.as_str())?;
    return Ok(json);
}

pub fn fetch_set(set: &str) -> Vec<Card> {

    fn trim(s : &str) -> &str {
        let l = s.len();
        if l <= 2 { s } else { &s[1..l-1] }
    }

    fn to_str(c : Option<&json::Json>) -> String {
        match c {
            Some(j) => {
                let n = j.to_string();
                trim(&n).to_string()
            },
            None => "".to_string()
        }
    }

    fn to_str_list(card : &json::Json, what : &str) -> Vec<String> {
        let empty:Vec<json::Json> = vec![];
        let subtyps = match card.find(what) {
            Some(t) => t.as_array().unwrap(),
            None => &empty
        };
        let subtypes = subtyps
            .iter()
            .map(|t| to_str(Some(t)))
            .collect::<Vec<String>>();
        subtypes
    }

    match fetch(set) {
        Ok(doc) => {
            let cards = doc
                .find("cards").unwrap()
                .as_array().unwrap();

            cards.iter()
                .map(|card| {
                    //println!("{}", card.to_pretty_str());

                    let name = to_str(card.find("name"));
                    let typ  = to_str(card.find("type"));
                    let styps = to_str_list(card, "supertypes");
                    let typs = to_str_list(card, "types");
                    let subtypes = to_str_list(card, "subtypes");
                    let image = to_str(card.find("imageName"));
                    let text  = to_str(card.find("text"));
                    let rarity:Rarity = Rarity::parse(&*to_str(card.find("rarity"))).unwrap(); //.unwrap_or(Rarity::Special);
                    let power = to_str(card.find("power"));
                    let toughness = to_str(card.find("toughness"));

                    let cost = {
                        let mana_cost = match card.find("manaCost") {
                            Some(c) => c.to_string(), None => "".to_string()
                        };
                        Mana::parse(trim(&mana_cost))
                    };

                    let colors = {
                        let cs = match card.find("colors") {
                            Some(c) => c.as_array().unwrap().iter()
                                .map(|s| Color::parse(trim(&s.to_string()))).collect(),
                            None => vec![]
                        };
                        cs
                    };


                    let c = Card {
                        card_name   : name,
                        card_type   : typ,
                        super_types : styps,
                        card_types  : typs,
                        sub_types   : subtypes,
                        image_name  : image,
                        mana_cost   : cost,
                        power       : power,
                        toughness   : toughness,
                        card_text   : text,
                        expansion   : set.to_string(),
                        colors      : colors,
                        rarity      : rarity
                    };

                    c
                })
                .collect::<Vec<Card>>()
        },
        _ => panic!("Couldn't load")
    }
}
