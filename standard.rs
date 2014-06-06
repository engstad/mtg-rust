use collections::treemap::{TreeMap};
use std::iter::AdditiveIterator;

pub trait Keys<K> {
    fn size(&self) -> uint;
    fn to_uint(&self, K) -> uint;
    fn from_uint(&self, n: uint) -> K;
    fn iter<'a>(&'a self) -> KeysIterator<'a, K> {
        KeysIterator { keys : self, idx : 0 }
    }
}

struct KeysIterator<'a, K> {
    keys : &'a Keys<K>,
    idx : uint
}

impl<'a,K> Iterator<K> for KeysIterator<'a, K> {
    fn next(&mut self) -> Option<K> {
        let i = self.idx;
        self.idx += 1;
        if i < self.keys.size() {
            Some(self.keys.from_uint(i))
        }
        else {
            None
        }
    }
}

fn to_vec<K>(k : &Keys<K>) -> Vec<K> {
    k.iter().collect::<Vec<K>>()
}

#[deriving(Clone, Show, PartialEq, Eq, PartialOrd, Ord)]
pub enum Color { W, U, B, R, G } 

struct ColorKeys;

impl Keys<Color> for ColorKeys {
    fn size(&self) -> uint { 5 }
    fn to_uint(&self, c:Color) -> uint { c as uint }
    fn from_uint(&self, n:uint) -> Color { 
        match n {
            0 => W, 1 => U, 2 => B, 3 => R, 4 => G, 
            _ => fail!("out of range")
        }
    }
}

#[deriving(Clone, Show, PartialEq, Eq, PartialOrd, Ord)]
pub enum Dual { 
    WU, UB, BR, RG, GW, 
    WB, UR, BG, RW, GU 
}

struct DualKeys;

impl Keys<Dual> for DualKeys {
    fn size(&self) -> uint { 10 }
    fn to_uint(&self, c:Dual) -> uint { c as uint }
    fn from_uint(&self, n:uint) -> Dual { 
        match n {
            0 => Some(WU), 1 => Some(UB), 2 => Some(BR), 3 => Some(RG), 4 => Some(GW), 
            5 => Some(WB), 6 => Some(UR), 7 => Some(BG), 8 => Some(RW), 9 => Some(GU), 
            _ => None
        }.unwrap()
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
}

struct LandKeys;

impl Keys<Land> for LandKeys {
    fn size(&self) -> uint { ColorKeys.size() + 3 * DualKeys.size() }

    fn to_uint(&self, l:Land) -> uint { 
        match l {
            Basic(c) => ColorKeys.to_uint(c),
            Shock(d) => ColorKeys.size() + DualKeys.to_uint(d),
            Gate(d)  => ColorKeys.size() + DualKeys.size() + DualKeys.to_uint(d),
            Scry(d)  => ColorKeys.size() + 2u * DualKeys.size() + DualKeys.to_uint(d)
        }
    }

    fn from_uint(&self, d: uint) -> Land {
        if d < ColorKeys.size() { 
            Basic(ColorKeys.from_uint(d))
        }
        else if d < ColorKeys.size() + DualKeys.size() { 
            Shock(DualKeys.from_uint(d - ColorKeys.size()))
        }
        else if d < ColorKeys.size() + 2 * DualKeys.size() {
            Gate(DualKeys.from_uint(d - ColorKeys.size() - DualKeys.size()))
        } 
        else if d < ColorKeys.size() + 3 * DualKeys.size() {
            Scry(DualKeys.from_uint(d - ColorKeys.size() - 2 * DualKeys.size()))
        }
        else {
            fail!("out of range")
        }
    }
}



#[deriving(Clone, Show, PartialEq, Eq, PartialOrd, Ord)]
pub enum Card {
    Land(Land),
    Spell
}

impl Card {
    fn show(&self) -> String {
        match *self { Land(l) => l.show(), _ => "Spell".to_string() }
    }
}

struct CardKeys;

impl Keys<Card> for CardKeys {
    fn size(&self) -> uint { 1 + 35 }
    fn to_uint(&self, c:Card) -> uint {
        match c {
            Spell   => 0u,
            Land(l) => LandKeys.to_uint(l) + 1u,
        }
    }
    fn from_uint(&self, v:uint) -> Card { 
        if v < 1u { 
            Spell 
        }
        else if v < self.size() {
            Land(LandKeys.from_uint(v - 1u))
        }
        else {
            fail!("out of range")
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
    println!("Colors: {}", to_vec(&ColorKeys));
    println!("Duals : {}", to_vec(&DualKeys));
    //println!("Lands : {}", LandKeys.all());
    //println!("Cards : {}", CardKeys.all());

    let m = Mana::b(2) + Mana::u(1) + Mana::c(2);
    
    println!("cmc({}) = {}", m.pretty(), m.cmc());
    
    let l1 = Shock(UB);
    let l2 = Scry(BR);
    
    println!("{:12} : {:20s}, id={}", l1.to_str(), l1.show(), LandKeys.to_uint(l1) );
    println!("{:12} : {:20s}, id={}", l2.to_str(), l2.show(), LandKeys.to_uint(l2) );
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
            format!("{:2d} {:-30s}", num, card.show())
        } else {
            format!("{}\n{:2d} {:-30s}", acc, num, card.show())
        }
    };
    
    let res = ls.iter().fold("".to_string(), conc);
    println!("Res:\n{}", res);
}
