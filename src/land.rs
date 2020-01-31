//use collections::treemap::TreeMap;
use crate::mana::Mana;
use crate::colors::Color::{self, U,W,B,R,G,C};
use rustc_serialize::json;
use std::path::Path;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, RustcDecodable)]
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

#[derive(RustcDecodable)]
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

    pub fn source(&self, deck: &Vec<(&LandCardInfo, u32)>) -> Mana {

        fn basic(c: Color) -> &'static str {
            match c {
                W => "Plains", U => "Island", B => "Swamp", R => "Mountain", G => "Forest", C => "Invalid"
            }
        }

        if self.landtype == LandType::FetchLand || self.landtype == LandType::TappedFetchLand {
            let colors = vec![U, W, B, R, G];

            colors.iter().fold(Mana::zero(), |acc, &color| {

                // To fetch a color c, we must be able to fetch a land with the following subtypes.
                if deck.iter().any(|&(tgt, n)| n > 0 &&
                                   tgt.produces.iter().any(|&tgt_clr| tgt_clr == color) && // target can produce color
                                   tgt.subtypes.iter()
                                     .any(|tgt_subtype| self.produces.iter()
                                          .any(|&src_clr| *tgt_subtype == basic(src_clr)))) {
                    acc + color.source()
                }
                else {
                    acc
                }
            })
        }
        else {
            self.produces.iter().fold(Mana::zero(), |a, &c| a + c.source())
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
pub fn parse_lands<'db>(lands: &str, db: &'db Vec<LandCardInfo>) -> Vec<(&'db LandCardInfo, u32)>
{
    lands.split('\n').filter_map(|line| {
        let line = line.trim();

        if line.len() == 0 { return None }

        let caps:Vec<&str> = line.splitn(2, ' ').collect();

        if caps.len() != 2 {
            println!("warning: No space character in line: '{}' ({} parts)", line, caps.len());
            return None
        }

        let l0 = caps[0].len();
        let n = if l0 > 1 && caps[0].chars().last().unwrap() == 'x' {
            caps[0][0..l0-1].parse::<u32>()
        } else {
            caps[0].parse::<u32>()
        };

        let l = db.iter().find(|&nm| nm.name == caps[1] ||
                                     nm.short == caps[1]);

        match (n, l) {
            (Ok(n), Some(l)) => Some((l, n)),
            (Err(_), Some(l)) => {
                println!("Could not parse count on '{}': {}", l.name, caps[0]);
                None
            },
            (Ok(_), None) => {
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

pub fn analyze(deck: &str) -> (u32, Vec<u32>)
{
    use std::fs::File;
    use std::io::Read;

    let text = include_str!("lands.json");
    let db:Vec<LandCardInfo> = json::decode(text).unwrap();

    let mut file = match File::open(&Path::new(deck)) {
        Ok(f) => f, Err(e) => { println!("Error: {}", e); return (0, vec![]); }
    };
    let mut deck = String::new();
    match file.read_to_string(&mut deck) {
       Ok(_) => (),
       Err(e) => { println!("Error: {}", e); return (0, vec![]) }
    };

    //println!("=========================\n{}================", deck);

    let mut ls : Vec<(&LandCardInfo, u32)> = parse_lands(&*deck, &db);
    ls.sort_by(|&a, &b| a.0.landtype.cmp(&b.0.landtype));

    {
        use crate::table::{Table, left, right};
        use crate::table::TableElem::{LStr, U32};
        let mut table = Table::new(1+ls.len(), 3);

        {
            table.set(0, 0, right("#"));
            table.set(0, 1, left("Land"));
            table.set(0, 2, left(""));
        }

        for (row, &(card, num)) in ls.iter().enumerate() {
            table.set(1 + row, 0, U32(num));
            table.set(1 + row, 1, LStr(card.show()));
            table.set(1 + row, 2, LStr((card.source(&ls) * num).src()));
        }

        table.print("Deck");
    }

    let lds = ls.iter()
        .fold((0u32, Mana::zero()), |(l, m), &(c, n)| { (l + n, m + c.source(&ls) * n) });
    let unt = ls.iter().filter(|&&(c, _)| c.untapped())
        .fold((0u32, Mana::zero()), |(l, m), &(c, n)| { (l + n, m + c.source(&ls) * n) });
    let tap = ls.iter().filter(|&&(c, _)| !c.untapped())
        .fold((0u32, Mana::zero()), |(l, m), &(c, n)| { (l + n, m + c.source(&ls) * n) });

    println!("{:2} {:-30} {:-5}\n", lds.0, "Cards".to_string(), lds.1.src());
    println!("{:2} {:-30} {:-5}", unt.0, "Untapped".to_string(), unt.1.src());
    println!("{:2} {:-30} {:-5}\n", tap.0, "Tapped".to_string(), tap.1.src());

    let basics = ls.iter().filter(|&&(c, _)| c.landtype == LandType::BasicLand)
        .fold((0u32, Mana::zero()), |(l, m), &(c, n)| { (l + n, m + c.source(&ls) * n) });
    let fetches = ls.iter().filter(|&&(c, _)| c.landtype == LandType::FetchLand)
        .fold((0u32, Mana::zero()), |(l, m), &(c, n)| { (l + n, m + c.source(&ls) * n) });
    let scrys = ls.iter().filter(|&&(c, _)| c.landtype == LandType::ScryLand)
        .fold((0u32, Mana::zero()), |(l, m), &(c, n)| { (l + n, m + c.source(&ls) * n) });
    let pains = ls.iter().filter(|&&(c, _)| c.landtype == LandType::PainLand)
        .fold((0u32, Mana::zero()), |(l, m), &(c, n)| { (l + n, m + c.source(&ls) * n) });
    let refus = ls.iter().filter(|&&(c, _)| c.landtype == LandType::RefuLand)
        .fold((0u32, Mana::zero()), |(l, m), &(c, n)| { (l + n, m + c.source(&ls) * n) });
    let specs = ls.iter().filter(|&&(c, _)| c.landtype == LandType::TappedLand || c.landtype == LandType::UntappedLand)
        .fold((0u32, Mana::zero()), |(l, m), &(c, n)| { (l + n, m + c.source(&ls) * n) });

    println!("{:2} {:-30} {:-5}", basics.0, "Basics".to_string(), basics.1.src());
    println!("{:2} {:-30} {:-5}", fetches.0, "Fetch-lands".to_string(), fetches.1.src());
    println!("{:2} {:-30} {:-5}", pains.0, "Pain-lands".to_string(), pains.1.src());
    println!("{:2} {:-30} {:-5}", scrys.0, "Scry-lands".to_string(), scrys.1.src());
    println!("{:2} {:-30} {:-5}", refus.0, "Refugee lands".to_string(), refus.1.src());
    println!("{:2} {:-30} {:-5}", specs.0, "Special lands".to_string(), specs.1.src());

    let mut colors = vec![];

    if lds.1.w > 0 { colors.push(lds.1.w) }
    if lds.1.u > 0 { colors.push(lds.1.u) }
    if lds.1.b > 0 { colors.push(lds.1.b) }
    if lds.1.r > 0 { colors.push(lds.1.r) }
    if lds.1.g > 0 { colors.push(lds.1.g) }

    return (lds.0, colors);
}
