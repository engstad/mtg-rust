//use hyper::Url;
//use hyper::client::Request;
use curl::http;

use serialize::json;
use mana::Mana;
use colors::Color;

#[deriving(Show, PartialEq, Eq, Clone)]
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

pub fn fetch_set(set : &str) -> Vec<Card> {
    // Creating an outgoing request.
    let loc = format!("http://mtgjson.com/json/{}.json", set);

    // let url = match Url::parse(loc.as_slice()) {
    //     Ok(url) => url,
    //     Err(e) => panic!("Invalid URL: {}", e)
    // };

    // let req = match Request::get(url) {
    //     Ok(req) => req,
    //     Err(err) => panic!("Failed to connect: {}", err)
    // };

    // let mut res = req
    //     .start().unwrap() // failure: Error writing Headers
    //     .send().unwrap(); // failure: Error reading Response head.

    // let str = res.read_to_string().unwrap();

    let resp = http::handle().get(loc.as_slice()).exec().unwrap();
    let rstr = String::from_utf8_lossy(resp.get_body());

    //println!("{}", resp);

    let json = json::from_str(rstr.as_slice());

    fn trim(s : &str) -> &str {
        let l = s.len();
        if l <= 2 { s } else { s[1..l-1] }
    }

    fn to_str(c : Option<&json::Json>) -> String {
        match c {
            Some(j) => {
                let n = j.to_string();
                trim(n[]).to_string()
            },
            None => "".to_string()
        }
    }

    fn to_str_list(card : &json::Json, what : &str) -> Vec<String> {
        let empty = vec![];
        let subtyps = match card.find(what) {
            Some(t) => t.as_list().unwrap().as_slice(),
            None => empty.as_slice()
        };
        let subtypes = subtyps
            .iter()
            .map(|t| to_str(Some(t)))
            .collect::<Vec<String>>();
        subtypes
    }

    match json {
        Ok(doc) => {
            let cards = doc
                .find("cards").unwrap()
                .as_list().unwrap();

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
                        Mana::parse(trim(mana_cost[]))
                    };

                    let colors = {
                        let cs = match card.find("colors") {
                            Some(c) => c.as_list().unwrap().iter()
                                .map(|s| Color::parse(trim(s.to_string()[]))).collect(),
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
        Err(err) => panic!("Error: {}", err)
    }
}
