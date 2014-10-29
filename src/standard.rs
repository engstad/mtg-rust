//use collections::treemap::TreeMap;
use mana::Mana;
use colors::{Color,U,W,B,R,G,C};
use serialize::json;

#[deriving(Clone, Show, PartialEq, Eq, PartialOrd, Ord, Encodable, Decodable)]
enum LandType
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

        if self.landtype == FetchLand {
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
            AlphaLand => true,
            BasicLand => true,
            ShockLand => true,
            Gates => false,
            ScryLand => false,
            RefuLand => false,
            FetchLand => true,
            FastLand => true,
            CheckLand => true,
            PainLand => true,
            WedgeLand => false,
            TappedLand => false,
            UntappedLand => true,
            StorageLand => true,
            ManLand => false,
            FilterLand => true,
            LifeLand => false
	    }	
    }    
}

//
//
pub fn parse_lands<'db>(lands: &str, db: &'db Vec<LandCardInfo>) -> Vec<(&'db LandCardInfo, uint)>
{
    lands.as_slice().split('\n').filter_map(|line| { 
        let caps:Vec<&str> = line.as_slice().trim().splitn(1, ' ').collect(); 

        if caps.len() != 2 { return None }

        let n = from_str::<uint>(caps[0]);
        let l = db.iter().find(|&nm| nm.name.as_slice() == caps[1] ||
                                     nm.short.as_slice() == caps[1]);
        
        match (n, l) {
            (Some(n), Some(l)) => Some((l, n)), 
            _ => None 
        }
    }).collect()
}

pub fn analyze(deck: &str) -> uint
{
    use std::io::File;
    
    let text = include_str!("lands.json");
    let db:Vec<LandCardInfo> = json::decode(text.as_slice()).unwrap();

    /*
    let ls2:Vec<(&LandCardInfo, uint)> = vec![(2, WBp), (2, WUf), (3, Isl), (2, Pla), (2, UBf), 
                                              (2, Swa), (4, UBt), (4, WUt), (4, WBt), (1, Urb)]
        .iter().map(|&(n, s)| (db.iter().find(|&c| c.short == s).unwrap(), n)).collect();
    */

    let deck = 
        File::open(&Path::new(deck)).unwrap().read_to_string().unwrap();

    //println!("=========================\n{}================", deck);

    let ls = parse_lands(deck.as_slice(), &db);    

    {
        use table::{Table, LStr, RStr, UInt};
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

    println!("{:2u} {:-30s} {:-5s}\n", lds.0, "Cards".to_string(), lds.1.src());
    println!("{:2u} {:-30s} {:-5s}", unt.0, "Untapped".to_string(), unt.1.src());
    println!("{:2u} {:-30s} {:-5s}\n", tap.0, "Tapped".to_string(), tap.1.src());

    let basics = ls.iter().filter(|&&(c, _)| c.landtype == BasicLand)
        .fold((0u, Mana::zero()), |(l, m), &(c, n)| { (l + n, m + c.source(&ls).mul(n)) });
    let fetches = ls.iter().filter(|&&(c, _)| c.landtype == FetchLand)
        .fold((0u, Mana::zero()), |(l, m), &(c, n)| { (l + n, m + c.source(&ls).mul(n)) });
    let scrys = ls.iter().filter(|&&(c, _)| c.landtype == ScryLand)
        .fold((0u, Mana::zero()), |(l, m), &(c, n)| { (l + n, m + c.source(&ls).mul(n)) });
    let pains = ls.iter().filter(|&&(c, _)| c.landtype == PainLand)
        .fold((0u, Mana::zero()), |(l, m), &(c, n)| { (l + n, m + c.source(&ls).mul(n)) });
    let refus = ls.iter().filter(|&&(c, _)| c.landtype == RefuLand)
        .fold((0u, Mana::zero()), |(l, m), &(c, n)| { (l + n, m + c.source(&ls).mul(n)) });
    let specs = ls.iter().filter(|&&(c, _)| c.landtype == TappedLand || c.landtype == UntappedLand)
        .fold((0u, Mana::zero()), |(l, m), &(c, n)| { (l + n, m + c.source(&ls).mul(n)) });

    println!("{:2u} {:-30s} {:-5s}", basics.0, "Basics".to_string(), basics.1.src());
    println!("{:2u} {:-30s} {:-5s}", fetches.0, "Fetch-lands".to_string(), fetches.1.src());
    println!("{:2u} {:-30s} {:-5s}", pains.0, "Pain-lands".to_string(), pains.1.src());
    println!("{:2u} {:-30s} {:-5s}", scrys.0, "Scry-lands".to_string(), scrys.1.src());
    println!("{:2u} {:-30s} {:-5s}", refus.0, "Refugee lands".to_string(), refus.1.src());
    println!("{:2u} {:-30s} {:-5s}", specs.0, "Special lands".to_string(), specs.1.src());

    return lds.0;
}
