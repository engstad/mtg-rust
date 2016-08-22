
use hyper::Client;
use hyper::header::Connection;
use std::result::{Result};
use rustc_serialize::{json};
use mana::Mana;
use colors::Color;
use std::io::Read;

#[derive(Debug)]
pub enum Error {
    General,
    IO(::std::io::Error),
    JsonParser(json::ParserError),
    JsonDecoder(json::DecoderError),
    JsonEncoder(json::EncoderError)
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
    pub rarity      : String
}

/*    
    // Creating an outgoing request.

    let <resp = match http::handle().get(loc).exec() {
        Ok(r) => r, Err(_) => return vec![]
    };

    if resp.get_code() != 200 { return vec![] }

    let rstr = String::from_utf8_lossy(resp.get_body());
*/


pub fn fetch_response(set: &str) -> Option<String> {
    use std::io::{Read};
    
    let loc = format!("http://mtgjson.com/json/{}.json", set);

    // Create a client.
    let client = Client::new();

    // Creating an outgoing request.
    let mut res = client.get(&*loc)
        // set a header
        .header(Connection::close())
        // let 'er go!
        .send().unwrap();

    // Read the Response.
    let mut response = String::new();

    let status = res.read_to_string(&mut response);

    return match status { Ok(_) => Some(response), _ => None }
}

pub fn fetch(set: &str) -> Result<json::Json, Error> {
    let json_str = match fetch_response(set) {
        Some(s) => s, None => return Err(Error::General)
    };
    
    let json = json::Json::from_str(&*json_str);

    return match json { Ok(s) => Ok(s),
                        Err(e) => Err(Error::JsonParser(e)) }
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
        let empty = vec![];
        let subtyps = match card.find(what) {
            Some(t) => t.as_array().unwrap().as_slice(),
            None => empty.as_slice()
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
                    let rarity = to_str(card.find("rarity"));
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
        Err(err) => panic!("Error: {:?}", err)
    }
}

pub fn fetch_img(set: &str, img: &str) -> Vec<u8>
{
    let loc = format!("http://mtgimage.com/set/{}/{}.jpg", set, img);

    // Create a client.
    let client = Client::new();

    // Creating an outgoing request.
    let mut res = client.get(&*loc)
        // set a header
        .header(Connection::close())
        // let 'er go!
        .send().unwrap();

    // Read the Response.
    let mut ret = Vec::new();
    res.read_to_end(&mut ret).unwrap();
    ret
}
