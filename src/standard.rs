//use collections::treemap::TreeMap;
use mana::Mana;
use colors::Color;
use colors::Color::{U,W,B,R,G,C};
use serialize::json;

#[deriving(Copy, Clone, Show, PartialEq, Eq, PartialOrd, Ord, Encodable, Decodable)]
pub enum LandType
{
	BasicLand,
	AlphaLand,
	TappedLand,
	UntappedLand,
	ShockLand,
	FastLand,
	Gates,
	ScryLand,
	RefuLand,
	FetchLand,
    TappedFetchLand,
	PainLand,
	WedgeLand,
	CheckLand,
	ManLand,
	StorageLand,
	FilterLand, 
    LifeLand
}

#[deriving(Encodable, Decodable)]
pub struct LandCardInfo {
    pub name : String,
    pub short : String,
    pub cardtype : String,
    pub subtypes : Vec<String>,
    pub landtype : LandType,
    pub produces : Vec<Color>
}

impl LandCardInfo {
    pub fn show(&self) -> String { 
        format!("{}", self.name)
    }

    pub fn source(&self, deck: &Vec<(&LandCardInfo, uint)>) -> Mana {

        fn basic(c: Color) -> &'static str {
            match c {
                W => "Plains", U => "Island", B => "Swamp", R => "Mountain", G => "Forest", C => "Invalid"
            }
        }

        if self.landtype == LandType::FetchLand || self.landtype == LandType::TappedFetchLand {
            let colors = vec![U, W, B, R, G];

            colors.iter().fold(Mana::new(0, 0, 0, 0, 0, 0), |acc, &color| {

                // To fetch a color c, we must be able to fetch a land with the following subtypes.
                if deck.iter().any(|&(tgt, n)| n > 0 && 
                                   tgt.produces.iter().any(|&tgt_clr| tgt_clr == color) && // target can produce color
                                   tgt.subtypes.iter()
                                     .any(|tgt_subtype| self.produces.iter()
                                          .any(|&src_clr| tgt_subtype.as_slice() == basic(src_clr)))) {
                    acc + color.source()
                }
                else {
                    acc 
                }
            })
        }
        else {
            self.produces.iter().fold(Mana::new(0, 0, 0, 0, 0, 0), |a, &c| a + c.source())
        }
    }

    fn untapped(&self) -> bool { 
        match self.landtype {
            LandType::AlphaLand => true,
            LandType::BasicLand => true,
            LandType::ShockLand => true,
            LandType::Gates => false,
            LandType::ScryLand => false,
            LandType::RefuLand => false,
            LandType::FetchLand => true,
            LandType::FastLand => true,
            LandType::CheckLand => true,
            LandType::PainLand => true,
            LandType::WedgeLand => false,
            LandType::TappedLand => false,
            LandType::TappedFetchLand => false,
            LandType::UntappedLand => true,
            LandType::StorageLand => true,
            LandType::ManLand => false,
            LandType::FilterLand => true,
            LandType::LifeLand => false
	    }	
    }    
}

//
//
pub fn parse_lands<'db>(lands: &str, db: &'db Vec<LandCardInfo>) -> Vec<(&'db LandCardInfo, uint)>
{
    lands.split('\n').filter_map(|line| { 
        let caps:Vec<&str> = line.trim().splitn(1, ' ').collect(); 

        if caps.len() != 2 { return None }

        let l0 = caps[0].len();
        let n = if l0 > 1 && caps[0].chars().last().unwrap() == 'x' {
            from_str::<uint>(caps[0][0..l0-1])
        } else {
            from_str::<uint>(caps[0])
        };

        let l = db.iter().find(|&nm| nm.name == caps[1] ||
                                     nm.short == caps[1]);
        
        match (n, l) {
            (Some(n), Some(l)) => Some((l, n)), 
            (None, Some(l)) => {
                println!("Could not parse count on '{}': {}", l.name, caps[0]);
                None
            },
            (Some(_), None) => {
                println!("Could not find card named '{}' in database", caps[1]);
                None
            },
            _ => {
                println!("Invalid line: {} {}", caps[0], caps[1]);
                None 
            }
        }
    }).collect()
}

pub fn analyze(deck: &str) -> uint
{
    use std::io::File;
    
    let text = include_str!("lands.json");
    let db:Vec<LandCardInfo> = json::decode(text).unwrap();

    let mut file = match File::open(&Path::new(deck)) {
        Ok(f) => f, Err(e) => { println!("Error: {}", e); return 0; }
    };
    let deck = file.read_to_string().unwrap_or("".to_string());

    //println!("=========================\n{}================", deck);

    let ls = parse_lands(deck.as_slice(), &db);    

    {
        use table::Table;
        use table::TableElem::{LStr, RStr, UInt};
        let mut table = Table::new(1+ls.len(), 3);

        {
            table.set(0, 0, RStr("#".to_string()));
            table.set(0, 1, LStr("Land".to_string()));
            table.set(0, 2, LStr("".to_string()));
        }

        for (row, &(card, num)) in ls.iter().enumerate() {
            table.set(1 + row, 0, UInt(num));
            table.set(1 + row, 1, LStr(card.show()));
            table.set(1 + row, 2, LStr(card.source(&ls).mul(num).src()));
        }

        table.print("Deck");
    }

    let lds = ls.iter()
        .fold((0u, Mana::zero()), |(l, m), &(c, n)| { (l + n, m + c.source(&ls).mul(n)) });
    let unt = ls.iter().filter(|&&(c, _)| c.untapped())
        .fold((0u, Mana::zero()), |(l, m), &(c, n)| { (l + n, m + c.source(&ls).mul(n)) });
    let tap = ls.iter().filter(|&&(c, _)| !c.untapped())
        .fold((0u, Mana::zero()), |(l, m), &(c, n)| { (l + n, m + c.source(&ls).mul(n)) });

    println!("{:2} {:-30} {:-5}\n", lds.0, "Cards".to_string(), lds.1.src());
    println!("{:2} {:-30} {:-5}", unt.0, "Untapped".to_string(), unt.1.src());
    println!("{:2} {:-30} {:-5}\n", tap.0, "Tapped".to_string(), tap.1.src());

    let basics = ls.iter().filter(|&&(c, _)| c.landtype == LandType::BasicLand)
        .fold((0u, Mana::zero()), |(l, m), &(c, n)| { (l + n, m + c.source(&ls).mul(n)) });
    let fetches = ls.iter().filter(|&&(c, _)| c.landtype == LandType::FetchLand)
        .fold((0u, Mana::zero()), |(l, m), &(c, n)| { (l + n, m + c.source(&ls).mul(n)) });
    let scrys = ls.iter().filter(|&&(c, _)| c.landtype == LandType::ScryLand)
        .fold((0u, Mana::zero()), |(l, m), &(c, n)| { (l + n, m + c.source(&ls).mul(n)) });
    let pains = ls.iter().filter(|&&(c, _)| c.landtype == LandType::PainLand)
        .fold((0u, Mana::zero()), |(l, m), &(c, n)| { (l + n, m + c.source(&ls).mul(n)) });
    let refus = ls.iter().filter(|&&(c, _)| c.landtype == LandType::RefuLand)
        .fold((0u, Mana::zero()), |(l, m), &(c, n)| { (l + n, m + c.source(&ls).mul(n)) });
    let specs = ls.iter().filter(|&&(c, _)| c.landtype == LandType::TappedLand || c.landtype == LandType::UntappedLand)
        .fold((0u, Mana::zero()), |(l, m), &(c, n)| { (l + n, m + c.source(&ls).mul(n)) });

    println!("{:2} {:-30} {:-5}", basics.0, "Basics".to_string(), basics.1.src());
    println!("{:2} {:-30} {:-5}", fetches.0, "Fetch-lands".to_string(), fetches.1.src());
    println!("{:2} {:-30} {:-5}", pains.0, "Pain-lands".to_string(), pains.1.src());
    println!("{:2} {:-30} {:-5}", scrys.0, "Scry-lands".to_string(), scrys.1.src());
    println!("{:2} {:-30} {:-5}", refus.0, "Refugee lands".to_string(), refus.1.src());
    println!("{:2} {:-30} {:-5}", specs.0, "Special lands".to_string(), specs.1.src());

    return lds.0;
}
