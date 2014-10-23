//use collections::treemap::TreeMap;
use mana::Mana;
use colors::*;
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
	FilterLand
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
            FilterLand => true
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

pub fn test() -> uint
{
    use std::io::File;
    
    let p = Path::new("src/lands.json");
    let mut f = File::open(&p).unwrap();
    let text = f.read_to_string().unwrap();
    let db:Vec<LandCardInfo> = json::decode(text.as_slice()).unwrap();

    let lands = 
        File::open(&Path::new("src/deck.txt")).unwrap().read_to_string().unwrap();

    //println!("=========================\n{}================", lands);

    let ls = parse_lands(lands.as_slice(), &db);    

    for &(card, num) in ls.iter() {
        println!("{:2u} {:-30s} {:-5s}", num, card.show(), card.source(&ls).mul(num).src())
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
