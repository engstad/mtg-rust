use collections::treemap::{TreeMap};
use std::iter::AdditiveIterator;

#[deriving(Clone, Show, PartialEq, Eq, PartialOrd, Ord)]
pub enum Color { W, U, B, R, G } 

trait Enum {
    fn show(&self) -> String;
    fn to_uint(&self) -> uint;
    fn from_uint(n: uint) -> Option<Self>;
}

impl Color {
    fn size() -> uint { 5 }
}

impl Enum for Color {
    fn show(&self) -> String { 
        match *self {
            W => "W", U => "U", B => "B", R => "R", G => "G"
        }.to_string()
    }

    fn to_uint(&self) -> uint { *self as uint }
    fn from_uint(n: uint) -> Option<Color> { 
        match n {
            0 => Some(W), 1 => Some(U), 2 => Some(B), 3 => Some(R), 4 => Some(G), 
            _ => None
        }
    }
}

#[deriving(Clone, Show, PartialEq, Eq, PartialOrd, Ord)]
pub enum Dual { 
    WU, UB, BR, RG, GW, 
    WB, UR, BG, RW, GU 
}

impl Dual {
    pub fn size() -> uint { 10 }
}

impl Enum for Dual {
    fn show(&self) -> String { 
        match *self {
            WU => "WU", UB => "UB", BR => "BR", RG => "RG", GW => "GW",
            WB => "WB", UR => "UR", BG => "BG", RW => "RW", GU => "GU"
        }.to_string()
    }

    fn to_uint(&self) -> uint { *self as uint }
    fn from_uint(n: uint) -> Option<Dual> { 
        match n {
            0 => Some(WU), 1 => Some(UB), 2 => Some(BR), 3 => Some(RG), 4 => Some(GW), 
            5 => Some(WB), 6 => Some(UR), 7 => Some(BG), 8 => Some(RW), 9 => Some(GU), 
            _ => None
        }
    }
}

#[deriving(Clone, Show, PartialEq, Eq, PartialOrd, Ord)]
pub enum Land {
    Basic(Color),
    Shock(Dual),
    Gate(Dual),
    Scry(Dual)
}

impl Land {
    pub fn size() -> uint { Color::size() + 3 * Dual::size() }
}

impl Enum for Land {
    fn show(&self) -> String { 
        match *self {
            Basic(c) => match c {
                W => "Plains",
                U => "Island",
                B => "Swamp",
                R => "Mountain",
                G => "Forest"
            },
            Shock(d) => match d {
                WU => "Hallowed Fountain", 
		UB => "Watery Grave", 
		BR => "Blood Crypt", 
		RG => "Stomping Ground", 
		GW => "Temple Garden", 
		WB => "Godless Shrine",
		UR => "Steam Vents",
		BG => "Overgrown Tomb",
		RW => "Sacred Foundry",
		GU => "Breeding Pool"
            },
            Gate(d) => match d {
                WU => "Azorius Guildgate",
		UB => "Dimir Guildgate",
		BR => "Rakdos Guildgate",
		RG => "Gruul Guildgate",
		GW => "Selesnya Guildgate",
		WB => "Orzhov Guildgate",
		UR => "Izzet Guildgate",
		BG => "Golgari Guildgate",
		RW => "Boros Guildgate",
		GU => "Simic Guildgate"
            },
            Scry(d) => match d {
                WU => "Temple of Enlightenment",
		UB => "Temple of Deceit",
		BR => "Temple of Malice",
		RG => "Temple of Abandon",
		GW => "Temple of Plenty",
		WB => "Temple of Silence",
		UR => "Temple of Epiphany",
		BG => "Temple of Malady",
		RW => "Temple of Triumph",
		GU => "Temple of Mystery"
            }
        }.to_string()
    }

    fn to_uint(&self) -> uint { 
        match *self {
            Basic(c) => c.to_uint(),
            Shock(d) => Color::size() + d.to_uint(),
            Gate(d)  => Color::size() + Dual::size() + d.to_uint(),
            Scry(d)  => Color::size() + 2u * Dual::size() + d.to_uint()
        }
    }

    fn from_uint(d: uint) -> Option<Land> {
        if d < Color::size() { 
            Some(Basic(Enum::from_uint(d).unwrap()))
        }
        else if d < Color::size() + Dual::size() { 
            Some(Shock(Enum::from_uint(d - Color::size()).unwrap()))
        }
        else if d < Color::size() + 2 * Dual::size() {
            Some(Gate(Enum::from_uint(d - Color::size() - Dual::size()).unwrap()))
        } 
        else if d < Color::size() + 3 * Dual::size() {
            Some(Scry(Enum::from_uint(d - Color::size() - 2 * Dual::size()).unwrap()))
        }
        else {
            None
        }
    }
}

#[deriving(Clone, Show, PartialEq, Eq, PartialOrd, Ord)]
pub enum Card {
    Land(Land),
    Spell
}

impl Card {
    pub fn size() -> uint { Land::size() + 1 }
}

impl Enum for Card {
    fn show(&self) -> String { 
        match *self {
            Land(l) => l.show(),
            Spell   => "Spell".to_string()
        }
    }

    fn to_uint(&self) -> uint {
        match *self {
            Spell   => 0u,
            Land(l) => l.to_uint() + 1u,
        }
    }

    fn from_uint(v: uint) -> Option<Card> {
        if v == 0u { Some(Spell) }
        else {
            match Enum::from_uint(v-1) {
                Some(l) => Some(Land(l)),
                None => None
            }
        }
    }
}

pub struct Mana([uint, ..6]);

impl Mana {
    pub fn zero() -> Mana { 
        Mana([0, 0, 0, 0, 0, 0]) 
    }

    pub fn w(n : uint) -> Mana { Mana([n, 0, 0, 0, 0, 0]) }
    pub fn u(n : uint) -> Mana { Mana([0, n, 0, 0, 0, 0]) }
    pub fn b(n : uint) -> Mana { Mana([0, 0, n, 0, 0, 0]) }
    pub fn r(n : uint) -> Mana { Mana([0, 0, 0, n, 0, 0]) }
    pub fn g(n : uint) -> Mana { Mana([0, 0, 0, 0, n, 0]) }
    pub fn c(n : uint) -> Mana { Mana([0, 0, 0, 0, 0, n]) }
    
    pub fn cmc(&self) -> uint { 
        let Mana(v) = *self;
        v.iter().map(|&x| x).sum()
    }

    pub fn show(&self) -> String { 
        let Mana(v) = *self;
        format!("({}, {}, {}, {}, {}, {})",
                v[0], v[1], v[2], v[3], v[4], v[5])
    }
    
    pub fn pretty(&self) -> String {
        let Mana(v) = *self;
        let ns = if v[5] > 0 { v[5].to_str() } else { "".to_string() };
        format!("{}{}{}{}{}{}",
                ns, 
                "W".repeat(v[0] as uint),
                "U".repeat(v[1] as uint),
                "B".repeat(v[2] as uint),
                "R".repeat(v[3] as uint),
                "G".repeat(v[4] as uint))
    }
}

impl Add<Mana, Mana> for Mana {
    fn add(&self, rhs: &Mana) -> Mana {
        let Mana(a) = *self;
        let Mana(b) = *rhs;
        Mana([a[0] + b[0], a[1] + b[1], a[2] + b[2],
              a[3] + b[3], a[4] + b[4], a[4] + b[5]])
    }
}

impl Sub<Mana, Mana> for Mana {
    fn sub(&self, rhs: &Mana) -> Mana {
        let Mana(a) = *self;
        let Mana(b) = *rhs;
        Mana([a[0] - b[0], a[1] - b[1], a[2] - b[2],
              a[3] - b[3], a[4] - b[4], a[4] - b[5]])
    }
}

//
// Representing a set of cards
//  - Must quickly know how many of a particular card there is
//

pub fn test()
{
    let m = Mana::b(2) + Mana::u(1) + Mana::c(2);
    
    println!("cmc({}) = {}", m.pretty(), m.cmc());
    
    let l1 = Shock(UB);
    let l2 = Scry(BR);
    
    println!("{:12} : {:20s}, id={}", l1.to_str(), l1.show(), l1.to_uint() );
    println!("{:12} : {:20s}, id={}", l2.to_str(), l2.show(), l2.to_uint() );
    println!("Cmp: {}", (Shock(UB) > Basic(U)).to_str());
    
    let mut ls = TreeMap::<Card, int>::new();
    ls.insert(Land(Shock(BR)), 4);
    ls.insert(Land(Shock(UB)), 4);
    ls.insert(Land(Shock(UR)), 4);
    ls.insert(Land(Scry(BR)), 4);
    ls.insert(Land(Scry(UB)), 4);
    ls.insert(Land(Scry(UR)), 4);
    ls.insert(Land(Basic(U)), 2);
    ls.insert(Spell, 36);
    
    {
        let key = Land(Shock(UR));
        let rem = match ls.find_mut(&key) {
            Some(v) => {
                if *v > 2 { *v -= 2; None } 
                else if *v == 2 { Some(&key) } 
                else { None }
            },                
            _ => None
        };
        match rem { Some(k) => { ls.remove(k); },
                    None => () }
    }
    
    let conc = |acc:String, (&card, &num): (&Card, &int)| -> String {
        if acc == "".to_string() {
            format!("{:2d} {:-30s} {:2}", num, card.show(), card.to_uint())
        } else {
            format!("{}\n{:2d} {:-30s} {:2}", acc, num, card.show(), card.to_uint())
        }
    };
    
    let res = ls.iter().fold("".to_string(), conc);
    println!("Res:\n{}", res);
}
