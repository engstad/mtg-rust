// Crate ID
//#![crate_id = "standard"]
// Specify the output type
//#![crate_type = "lib"]

use std::rc::Rc;


#[deriving(Clone, Show, PartialEq, TotalEq, PartialOrd, TotalOrd)]
pub enum Color { W, U, B, R, G } 

impl Color {
    pub fn show(&self) -> String { 
        match *self {
            W => "W", U => "U", B => "B", R => "R", G => "G"
        }.to_string()
    }

    pub fn to_uint(&self) -> uint { *self as uint }
    pub fn from_uint(n: uint) -> Color { 
        match n {
            0 => W, 1 => U, 2 => B, 3 => R, 4 => G, _ => fail!("wrong color")
        }
    }
    pub fn size() -> uint { 5 }
}

#[deriving(Clone, Show, PartialEq, TotalEq, PartialOrd, TotalOrd)]
pub enum Dual  { WU, UB, BR, RG, GW, 
             WB, UR, BG, RW, GU }

impl Dual {
    pub fn show(&self) -> String { 
        match *self {
            WU => "WU", UB => "UB", BR => "BR", RG => "RG", GW => "GW",
            WB => "WB", UR => "UR", BG => "BG", RW => "RW", GU => "GU"
        }.to_string()
    }

    pub fn to_uint(&self) -> uint { *self as uint }
    pub fn from_uint(n: uint) -> Dual { 
        match n {
            0 => WU, 1 => UB, 2 => BR, 3 => RG, 4 => GW, 
            5 => WB, 6 => UR, 7 => BG, 8 => RW, 9 => GU, 
            _ => fail!("wrong color")
        }
    }
    pub fn size() -> uint { 10 }
}

pub struct Mana([int, ..6]);

impl Mana {
    pub fn zero() -> Mana { 
        Mana([0, 0, 0, 0, 0, 0]) 
    }

    pub fn w(n : int) -> Mana { Mana([n, 0, 0, 0, 0, 0]) }
    pub fn u(n : int) -> Mana { Mana([0, n, 0, 0, 0, 0]) }
    pub fn b(n : int) -> Mana { Mana([0, 0, n, 0, 0, 0]) }
    pub fn r(n : int) -> Mana { Mana([0, 0, 0, n, 0, 0]) }
    pub fn g(n : int) -> Mana { Mana([0, 0, 0, 0, n, 0]) }
    pub fn c(n : int) -> Mana { Mana([0, 0, 0, 0, 0, n]) }
    
    pub fn cmc(&self) -> int { 
        let Mana(v) = *self;
        v.iter().fold(0, |a, &b| a + b) 
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

#[deriving(Clone, Show, PartialEq, TotalEq, PartialOrd, TotalOrd)]
pub enum Land {
    Basic(Color),
    Shock(Dual),
    Gate(Dual),
    Scry(Dual)
}

impl Land {
    pub fn show(&self) -> String { 
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

    pub fn to_uint(&self) -> uint { match *self {
            Basic(c) => c.to_uint(),
            Shock(d) => Color::size() + d.to_uint(),
            Gate(d)  => Color::size() + Dual::size() + d.to_uint(),
            Scry(d)  => Color::size() + 2u * Dual::size() + d.to_uint()
        }
    }
    pub fn from_uint(d: uint) -> Land {
        if d < Color::size() { 
            Basic(Color::from_uint(d)) 
        }
        else if d < Color::size() + Dual::size() { 
            Shock(Dual::from_uint(d - Color::size()))
        }
        else if d < Color::size() + 2 * Dual::size() {
            Gate(Dual::from_uint(d - Color::size() - Dual::size()))
        } 
        else if d < Color::size() + 3 * Dual::size() {
            Scry(Dual::from_uint(d - Color::size() - 2 * Dual::size()))
        }
        else {
            fail!("Out of range")
        }
    }

    pub fn size() -> uint { Color::size() + 3 * Dual::size() }
}

#[deriving(Clone, Show, PartialEq, TotalEq, PartialOrd, TotalOrd)]
pub enum Card {
    Land(Land),
    Spell
}

impl Card {
    pub fn show(&self) -> String { 
        match *self {
            Land(l) => l.show(),
            Spell   => "Spell".to_string()
        }
    }

    pub fn to_uint(&self) -> uint {
        match *self {
            Spell   => 0u,
            Land(l) => l.to_uint() + 1u,
        }
    }

    pub fn from_uint(v: uint) -> Card {
        if v == 0u { Spell }
        else { Land(Land::from_uint(v - 1u)) }
    }

    pub fn size() -> uint { Land::size() + 1 }
}




type Map = Option<Rc<Node>>;

struct Node {
    key   : Card,
    value : int, 
    left  : Map,
    right : Map
}

pub fn lookup(m: Map, k:Card) -> Option<int> {
    match m {
        None =>               { None }
        Some(n) => {
            let key = n.key;
            let val = n.value;
            let lft = n.left.clone();
            let rgt = n.right.clone();
            
            if k == key { 
                Some(val) 
            }
            else if k < key { 
                lookup(lft, k) 
            }
            else { 
                lookup(rgt, k) 
            }
        }
    }
}

pub fn update(m: Map, k:Card, f: |Option<int>| -> int) -> Map {
    match m {
        None    => 
            { Some(Rc::new(Node{key:k, value:f(None), left:None, right:None})) }
        Some(n) => {
            let key = n.key;
            let val = n.value;
            if k == key { 
                Some(Rc::new(Node{key:k, value:f(Some(val)), 
                                  left:n.left.clone(), right:n.right.clone()}))
            } else if k < key {
                let l = update(n.left.clone(), k, f);
                Some(Rc::new(Node{key:key, value:val, 
                                  left: l, right: n.right.clone()}))
            } else {
                let r = update(n.right.clone(), k, f);
                Some(Rc::new(Node{key:key, value:val, 
                                  left: n.left.clone(), right: r}))
            }
        }
    }
}                

pub fn insert(m: Map, k: Card, n : int) -> Map {
    update(m, k, |oi:Option<int>| { match oi { None => n, Some(v) => v + n }})
}

pub fn remove(m: Map, k: Card, n : int) -> Map {
    if n > 0 {
        update(m, k, |oi:Option<int>| { 
                match oi { 
                    None => { fail!("impossible"); }
                    Some(v) => { 
                        if v > n { v - n } else { fail!("impossible"); }
                    }
                }
            })
    }
    else {
        m
    }
}

pub fn fold<A>(m: Map, acc : A, f: fn (A, Card, int) -> A) -> A {
    match m {
        None => { acc }
        Some(n) => {
            let key = n.key;
            let val = n.value;
            let lft = n.left.clone();
            let rgt = n.right.clone();
            
            let acc = fold(lft, acc, f);
            let acc = f(acc, key, val);
            let acc = fold(rgt, acc, f);
            acc
        }
    }
}

pub fn iter<B>(m: Map, f: fn (Card, int) -> B) {
    match m {
        None => { }
        Some(n) => {
            let key = n.key;
            let val = n.value;
            let lft = n.left.clone();
            let rgt = n.right.clone();
            
            iter(lft, f);
            f(key, val);
            iter(rgt, f)
        }
    }
}


//
// Representing a set of cards
//  - Must quickly know how many of a particular card there is
//
struct Cards(Map);

pub fn test()
{
	let m = Mana::b(2) + Mana::u(1) + Mana::c(2);
    
    println!("cmc({}) = {}", m.pretty(), m.cmc());
    
    let l1 = Shock(UB);
    let l2 = Scry(BR);
    
    println!("{:12} : {:20s}, id={}", l1.to_str(), l1.show(), l1.to_uint() );
    println!("{:12} : {:20s}, id={}", l2.to_str(), l2.show(), l2.to_uint() );
    println!("Cmp: {}", (Shock(UB) > Basic(U)).to_str());
    
    let ls = None;
    let ls = insert(ls, Land(Shock(BR)), 4);
    let ls = insert(ls, Land(Shock(UB)), 4);
    let ls = insert(ls, Land(Shock(UR)), 4);
    let ls = insert(ls, Land(Scry(BR)), 4);
    let ls = insert(ls, Land(Scry(UB)), 4);
    let ls = insert(ls, Land(Scry(UR)), 4);
    let ls = insert(ls, Land(Basic(U)), 2);
    let ls = insert(ls, Spell, 36);
    
    let ls = remove(ls, Land(Shock(UR)), 2);
    
    fn conc(acc:String, card:Card, num:int) -> String {
        if acc == "".to_string() {
            format!("{:2d} {:-30s} {:2}", num, card.show(), card.to_uint())
        } else {
            format!("{}\n{:2d} {:-30s} {:2}", acc, num, card.show(), card.to_uint())
        }
    };
    
    let res = fold(ls, "".to_string(), conc);
    println!("Res:\n{}", res);
}
