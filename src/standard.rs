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

struct KeysIterator<'a, K:'a> {
    keys : &'a Keys<K>+'a,
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

impl Color {
    fn source(&self) -> Mana {
        match *self {
            W => Mana::w(1u), 
            U => Mana::u(1u), 
            B => Mana::b(1u), 
            R => Mana::r(1u),
            G => Mana::g(1u)
        }
    }    
}

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
pub enum Allied { 
    WU, UB, BR, RG, GW
}

impl Allied {
    fn source(&self) -> Mana {
        match *self {
            WU => Mana::w(1) + Mana::u(1), 
            UB => Mana::u(1) + Mana::b(1), 
            BR => Mana::b(1) + Mana::r(1), 
            RG => Mana::r(1) + Mana::g(1),
            GW => Mana::g(1) + Mana::w(1) 
        }
    }    
}


struct AlliedKeys;

impl Keys<Allied> for AlliedKeys {
    fn size(&self) -> uint { 5 }
    fn to_uint(&self, c:Allied) -> uint { c as uint }
    fn from_uint(&self, n:uint) -> Allied { 
        match n {
            0 => Some(WU), 1 => Some(UB), 2 => Some(BR), 3 => Some(RG), 4 => Some(GW), 
            _ => None
        }.unwrap()
    }    
}

#[deriving(Clone, Show, PartialEq, Eq, PartialOrd, Ord)]
pub enum Enemy {
    WB, UR, BG, RW, GU 
}

impl Enemy {
    fn source(&self) -> Mana {
        match *self {
            WB => Mana::w(1) + Mana::b(1), 
            UR => Mana::u(1) + Mana::r(1), 
            BG => Mana::b(1) + Mana::g(1), 
            RW => Mana::r(1) + Mana::w(1),
            GU => Mana::g(1) + Mana::u(1) 
        }
    }    
}

struct EnemyKeys;

impl Keys<Enemy> for EnemyKeys {
    fn size(&self) -> uint { 5 }
    fn to_uint(&self, c:Enemy) -> uint { c as uint }
    fn from_uint(&self, n:uint) -> Enemy { 
        match n {
            0 => Some(WB), 1 => Some(UR), 2 => Some(BG), 3 => Some(RW), 4 => Some(GU), 
            _ => None
        }.unwrap()
    }    
}


#[deriving(Clone, Show, PartialEq, Eq, PartialOrd, Ord)]
pub enum Dual { 
    A(Allied),
    E(Enemy)
}

impl Dual {
    fn source(&self) -> Mana {
        match *self {
            A(a) => a.source(),
            E(e) => e.source()
        }
    }    
}

struct DualKeys;

impl Keys<Dual> for DualKeys {
    fn size(&self) -> uint { 10 }
    fn to_uint(&self, c:Dual) -> uint { 
        match c { A(a) => AlliedKeys.to_uint(a), 
                  E(e) => AlliedKeys.size() + EnemyKeys.to_uint(e)
        }
    }
    fn from_uint(&self, n:uint) -> Dual { 
        match n {
            0 => Some(A(WU)), 1 => Some(A(UB)), 2 => Some(A(BR)), 3 => Some(A(RG)), 4 => Some(A(GW)), 
            5 => Some(E(WB)), 6 => Some(E(UR)), 7 => Some(E(BG)), 8 => Some(E(RW)), 9 => Some(E(GU)), 
            _ => None
        }.unwrap()
    }    
}

#[deriving(Clone, Show, PartialEq, Eq, PartialOrd, Ord)]
pub enum Shard {
    WUB, UBR, BRG, RGW, GWU 
}

impl Shard {
    fn source(&self) -> Mana {
        match *self {
            WUB => Mana::w(1) + Mana::u(1) + Mana::b(1), 
            UBR => Mana::u(1) + Mana::b(1) + Mana::r(1), 
            BRG => Mana::b(1) + Mana::r(1) + Mana::g(1), 
            RGW => Mana::r(1) + Mana::g(1) + Mana::w(1),
            GWU => Mana::g(1) + Mana::w(1) + Mana::u(1) 
        }
    }    
}

struct ShardKeys;

impl Keys<Shard> for ShardKeys {
    fn size(&self) -> uint { 5 }
    fn to_uint(&self, c:Shard) -> uint { c as uint }
    fn from_uint(&self, n:uint) -> Shard { 
        match n {
            0 => Some(WUB), 1 => Some(UBR), 2 => Some(BRG), 3 => Some(RGW), 4 => Some(GWU), 
            _ => None
        }.unwrap()
    }    
}

// WUBRG

#[deriving(Clone, Show, PartialEq, Eq, PartialOrd, Ord)]
pub enum Wedge {
    WBR, URG, BGW, RWU, GUB
}

impl Wedge {
    fn source(&self) -> Mana {
        match *self {
            WBR => Mana::w(1) + Mana::b(1) + Mana::r(1), 
            URG => Mana::u(1) + Mana::r(1) + Mana::g(1), 
            BGW => Mana::b(1) + Mana::g(1) + Mana::w(1), 
            RWU => Mana::r(1) + Mana::w(1) + Mana::u(1),
            GUB => Mana::g(1) + Mana::u(1) + Mana::b(1) 
        }
    }    
}

struct WedgeKeys;

impl Keys<Wedge> for WedgeKeys {
    fn size(&self) -> uint { 5 }
    fn to_uint(&self, c:Wedge) -> uint { c as uint }
    fn from_uint(&self, n:uint) -> Wedge { 
        match n {
            0 => Some(WBR), 1 => Some(URG), 2 => Some(BGW), 3 => Some(RWU), 4 => Some(GUB), 
            _ => None
        }.unwrap()
    }    
}


#[deriving(Clone, Show, PartialEq, Eq, PartialOrd, Ord)]
pub enum Land {
    EvolvingWild,
    Urborg,
    ManaConfluence,
    Basic(Color),
    Shock(Dual),
    Gate(Dual),
    Scry(Dual),
    Refu(Dual),
    Fetch(Dual),
    Pain(Enemy),
    Khan(Wedge)
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
                A(WU) => "Hallowed Fountain", 
		        A(UB) => "Watery Grave", 
		        A(BR) => "Blood Crypt", 
		        A(RG) => "Stomping Ground", 
		        A(GW) => "Temple Garden", 
		        E(WB) => "Godless Shrine",
		        E(UR) => "Steam Vents",
		        E(BG) => "Overgrown Tomb",
		        E(RW) => "Sacred Foundry",
		        E(GU) => "Breeding Pool"
            },
            Gate(d) => match d {
                A(WU) => "Azorius Guildgate",
		        A(UB) => "Dimir Guildgate",
		        A(BR) => "Rakdos Guildgate",
		        A(RG) => "Gruul Guildgate",
		        A(GW) => "Selesnya Guildgate",
		        E(WB) => "Orzhov Guildgate",
		        E(UR) => "Izzet Guildgate",
		        E(BG) => "Golgari Guildgate",
		        E(RW) => "Boros Guildgate",
		        E(GU) => "Simic Guildgate"
            },
            Scry(d) => match d {
                A(WU) => "Temple of Enlightenment",
		        A(UB) => "Temple of Deceit",
		        A(BR) => "Temple of Malice",
		        A(RG) => "Temple of Abandon",
		        A(GW) => "Temple of Plenty",
		        E(WB) => "Temple of Silence",
		        E(UR) => "Temple of Epiphany",
		        E(BG) => "Temple of Malady",
		        E(RW) => "Temple of Triumph",
		        E(GU) => "Temple of Mystery"
            },
            Refu(d) => match d {
                A(WU) => "Tranquil Cove",
                A(UB) => "Dismal Backwater",
                A(BR) => "Bloodfell Caves",
                A(RG) => "Rugged Highlands",
                A(GW) => "Blossoming Sands",
                
                E(WB) => "Scoured Barrens",
                E(UR) => "Swiftwater Cliffs",
                E(BG) => "Jungle Hollow",
                E(RW) => "Wind-Scarred Crag",
                E(GU) => "Thornwood Falls"
            },
            Fetch(d) => match d {
                A(WU) => "Flooded Strand",
                A(UB) => "Polluted Delta",
                A(BR) => "Bloodstrained Mire",
                A(RG) => "Wooded Foothills",
                A(GW) => "Windswept Heath",
                
                E(WB) => "Marsh Flats",
                E(UR) => "Scalding Tarn",
                E(BG) => "Verdant Catacombs",
                E(RW) => "Arid Mesa",
                E(GU) => "Misty Rainforest"
            },
            Pain(e) => match e {
                WB => "Caves of Koilos",
                UR => "Shivan Reef",
                BG => "Llanowar Wastes",
                RW => "Battlefield Forge",
                GU => "Yavimaya Coast"
            },
            Khan(w) => match w {
                WBR => "Nomad Outpost",
                URG => "Frontier Bivouac",
                BGW => "Sandsteppe Citadel",
                RWU => "Mystic Monestary",
                GUB => "Opulent Palace"
            },
            EvolvingWild =>
                "Evolving Wild",
            Urborg => 
                "Urborg, Tomb of Yawgmoth",
            ManaConfluence =>
                "Mana Confluence"

            //Frontier Bivouac
            //Mystic Monastery
            //Nomad Outpost
            //Opulent Palace
            //Sandsteppe Citadel
        }.to_string()
    }

    fn source(&self) -> Mana {
        match *self { 
            Basic(c) => c.source(),
            Shock(d) => d.source(),
            Gate(d) => d.source(),
            Scry(d) => d.source(),
            Refu(d) => d.source(),
            Fetch(d) => d.source(),
            Pain(e) => e.source(),
            Khan(w) => w.source(),
            EvolvingWild => Mana::w(1) + Mana::u(1) + Mana::b(1) + Mana::r(1) + Mana::g(1),
            ManaConfluence => Mana::w(1) + Mana::u(1) + Mana::b(1) + Mana::r(1) + Mana::g(1),
            Urborg => Mana::b(1)
        }
    }

    fn untapped(&self) -> bool { 
        match *self {
            Basic(_) => true,
            Shock(_) => true,
            Gate(_) => false,
            Scry(_) => false,
            Refu(_) => false,
            Fetch(_) => true,
            Pain(_) => true,
            Khan(_) => false,
            EvolvingWild => false,
            Urborg => true,
            ManaConfluence => true
	    }	
    }

    fn tapped(&self) -> bool { !self.untapped() }
}

struct LandKeys;

impl Keys<Land> for LandKeys {
    fn size(&self) -> uint { ColorKeys.size() + 3 * DualKeys.size() }

    fn to_uint(&self, l:Land) -> uint { 
        match l {
            EvolvingWild => 0,
            Urborg => 1,
            ManaConfluence => 2,
            Basic(c) => 3 + ColorKeys.to_uint(c),
            Shock(d) => 3 + ColorKeys.size() + DualKeys.to_uint(d),
            Gate(d)  => 3 + ColorKeys.size() + DualKeys.size() + DualKeys.to_uint(d),
            Scry(d)  => 3 + ColorKeys.size() + 2u * DualKeys.size() + DualKeys.to_uint(d),
            Refu(d)  => 3 + ColorKeys.size() + 3u * DualKeys.size() + DualKeys.to_uint(d),
            Fetch(d) => 3 + ColorKeys.size() + 4u * DualKeys.size() + DualKeys.to_uint(d),
            Pain(e)  => 3 + ColorKeys.size() + 5u * DualKeys.size() + EnemyKeys.to_uint(e),
            Khan(w)  => 3 + ColorKeys.size() + 5u * DualKeys.size() + EnemyKeys.size() + WedgeKeys.to_uint(w),
        }
    }

    fn from_uint(&self, d: uint) -> Land {
        if d == 0 { EvolvingWild }
        else if d == 1 { Urborg }
        else if d == 2 { ManaConfluence }
        else if d < 3 + ColorKeys.size() { 
            Basic(ColorKeys.from_uint(d - 2))
        }
        else if d < (3 + ColorKeys.size()) + DualKeys.size() { 
            Shock(DualKeys.from_uint(d - (3 + ColorKeys.size())))
        }
        else if d < (3 + ColorKeys.size()) + 2 * DualKeys.size() {
            Gate(DualKeys.from_uint(d - (3 + ColorKeys.size()) - DualKeys.size()))
        } 
        else if d < (3 + ColorKeys.size()) + 3 * DualKeys.size() {
            Scry(DualKeys.from_uint(d - (3 + ColorKeys.size()) - 2 * DualKeys.size()))
        }
        else if d < (3 + ColorKeys.size()) + 4 * DualKeys.size() {
            Refu(DualKeys.from_uint(d - (3 + ColorKeys.size()) - 3 * DualKeys.size()))
        }
        else if d < (3 + ColorKeys.size()) + 5 * DualKeys.size() {
            Fetch(DualKeys.from_uint(d - (3 + ColorKeys.size()) - 4 * DualKeys.size()))
        }        
        else {
            fail!("out of range")
        }
    }
}

#[deriving(Clone, Show, PartialEq, Eq, PartialOrd, Ord)]
enum LandType { Special, BasicLand, ShockLand, Gates, ScryLand, RefuLand, FetchLand, PainLand, WedgeLand }

#[deriving(Clone, Show, PartialEq, Eq, PartialOrd, Ord)]
pub enum Card {
    L(Land),
    S
}

impl Card {
    fn show(&self) -> String {
        match *self { L(l) => l.show(), S => "Spell".to_string() }
    }

    fn source(&self) -> Mana {
        match *self { L(l) => l.source(), S => Mana::zero() } 
    }

    fn untapped(&self) -> bool { 
        match *self { L(l) => l.untapped(), S => false }
    }

    fn land_type(&self) -> LandType {
        match *self { 
            L(Basic(_)) => BasicLand,
            L(Shock(_)) => ShockLand,
            L(Gate(_)) => Gates,
            L(Scry(_)) => ScryLand,
            L(Refu(_)) => RefuLand,
            L(Fetch(_)) => FetchLand,
            L(Pain(_)) => PainLand,
            L(Khan(_)) => WedgeLand,
            _ => Special
        }
    }
}

struct CardKeys;

impl Keys<Card> for CardKeys {
    fn size(&self) -> uint { 1 + 35 }
    fn to_uint(&self, c:Card) -> uint {
        match c {
            S   => 0u,
            L(l) => LandKeys.to_uint(l) + 1u,
        }
    }
    fn from_uint(&self, v:uint) -> Card { 
        if v < 1u { 
            S 
        }
        else if v < self.size() {
            L(LandKeys.from_uint(v - 1u))
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

    pub fn reset(&self, color: Color) -> Mana {
        let Mana(v) = *self;
        match color { 
            W => Mana([0, v[1], v[2], v[3], v[4], v[5]]),
            U => Mana([v[0], 0, v[2], v[3], v[4], v[5]]),
            B => Mana([v[0], v[1], 0, v[3], v[4], v[5]]),
            R => Mana([v[0], v[1], v[2], 0, v[4], v[5]]),
            G => Mana([v[0], v[1], v[2], v[3], 0, v[5]])
        }
    }
    
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
        let ns = if v[5] > 0 { v[5].to_string() } else { "".to_string() };
        format!("{}{}{}{}{}{}",
                ns, 
                "W".repeat(v[0] as uint),
                "U".repeat(v[1] as uint),
                "B".repeat(v[2] as uint),
                "R".repeat(v[3] as uint),
                "G".repeat(v[4] as uint))
    }
    
    pub fn src(&self) -> String {
        let Mana(v) = *self;
        format!("{}{}{}{}{}{}",
                if v[0] > 0 { format!("W:{:2u} ", v[0]) } else { "".to_string() }, 
                if v[1] > 0 { format!("U:{:2u} ", v[1]) } else { "".to_string() }, 
                if v[2] > 0 { format!("B:{:2u} ", v[2]) } else { "".to_string() }, 
                if v[3] > 0 { format!("R:{:2u} ", v[3]) } else { "".to_string() }, 
                if v[4] > 0 { format!("G:{:2u} ", v[4]) } else { "".to_string() }, 
                if v[5] > 0 { format!("X:{:2u} ", v[5]) } else { "".to_string() })        
    }
}

impl Add<Mana, Mana> for Mana {
    fn add(&self, rhs: &Mana) -> Mana {
        let Mana(a) = *self;
        let Mana(b) = *rhs;
        Mana([a[0] + b[0], 
              a[1] + b[1], 
              a[2] + b[2],
              a[3] + b[3], 
              a[4] + b[4], 
              a[5] + b[5]])
    }
}

impl Sub<Mana, Mana> for Mana {
    fn sub(&self, rhs: &Mana) -> Mana {
        let Mana(a) = *self;
        let Mana(b) = *rhs;
        Mana([a[0] - b[0], 
              a[1] - b[1], 
              a[2] - b[2],
              a[3] - b[3], 
              a[4] - b[4], 
              a[5] - b[5]])
    }
}

impl Mana {
    fn mul(&self, rhs: uint) -> Mana {
        let Mana(a) = *self;
        let k = rhs;
        Mana([a[0] * k, a[1] * k, a[2] * k,
              a[3] * k, a[4] * k, a[5] * k])
    }
}

//
// Representing a set of cards
//  - Must quickly know how many of a particular card there is
//

pub fn test() -> uint
{
    println!("Colors: {}", to_vec(&ColorKeys));
    println!("Duals : {}", to_vec(&DualKeys));
    //println!("Lands : {}", LandKeys.all());
    //println!("Cards : {}", CardKeys.all());

    let m = Mana::b(2) + Mana::u(1) + Mana::c(2);
    
    println!("cmc({}) = {}", m.pretty(), m.cmc());
    
    let l1 = Shock(A(UB));
    let l2 = Scry(A(BR));
    
    println!("{:12} : {:20s}, id={}", l1.to_string(), l1.show(), LandKeys.to_uint(l1) );
    println!("{:12} : {:20s}, id={}", l2.to_string(), l2.show(), LandKeys.to_uint(l2) );
    println!("Cmp: {}", (Shock(A(UB)) > Basic(U)).to_string());
    
    let mut ls = TreeMap::<Card, uint>::new();

    ls.insert(L(ManaConfluence), 2);
    //ls.insert(L(EvolvingWild), 2);
    ls.insert(L(Basic(U)), 2);
    ls.insert(L(Basic(B)), 1);
    ls.insert(L(Basic(R)), 1);

    // UR:
    ls.insert(L(Pain(UR)), 4);
    ls.insert(L(Scry(E(UR))), 1);
    //ls.insert(L(Refu(E(UR))), 1);
    
    // UB:
    ls.insert(L(Fetch(A(UB))), 4);
    ls.insert(L(Scry(A(UB))), 4);
    ls.insert(L(Refu(A(UB))), 2);
    //ls.insert(L(Khan(GUB)), 1);

    // BR:
    ls.insert(L(Fetch(A(BR))), 1);
    ls.insert(L(Scry(A(BR))), 4);
    //ls.insert(L(Refu(A(BR))), 1);

    ls.insert(L(Urborg), 1);
    
    //ls.insert(S, 36);
    //ls.insert(L(Khan(URG)), 2);            

    let conc = |acc:String, (&card, &num): (&Card, &uint)| -> String {
        if acc == "".to_string() {
            format!("{:2u} {:-30s} {:-5s}", num, card.show(), card.source().mul(num).reset(W).reset(G).src())
        } else {
            format!("{}\n{:2u} {:-30s} {:-5s}", acc, num, card.show(), card.source().mul(num).reset(W).reset(G).src())
        }
    };

    let res = ls.iter().fold("".to_string(), conc);

    let lds = ls.iter()
        .fold((0u, Mana::zero()), |(l, m), (c, n)| { (l + *n, m + c.source().mul(*n).reset(W).reset(G)) });
    let unt = ls.iter().filter(|&(c, _)| c.untapped())
        .fold((0u, Mana::zero()), |(l, m), (c, n)| { (l + *n, m + c.source().mul(*n).reset(W).reset(G)) });
    let tap = ls.iter().filter(|&(c, _)| !c.untapped())
        .fold((0u, Mana::zero()), |(l, m), (c, n)| { (l + *n, m + c.source().mul(*n).reset(W).reset(G)) });

    println!("Res:\n{}", res);
    println!("{:2u} {:-30s} {:-5s}\n", lds.0, "Cards".to_string(), lds.1.src());
    println!("{:2u} {:-30s} {:-5s}", unt.0, "Untapped".to_string(), unt.1.src());
    println!("{:2u} {:-30s} {:-5s}\n", tap.0, "Tapped".to_string(), tap.1.src());

    let basics = ls.iter().filter(|&(c, _)| c.land_type() == BasicLand)
        .fold((0u, Mana::zero()), |(l, m), (c, n)| { (l + *n, m + c.source().mul(*n).reset(W).reset(G)) });
    let fetches = ls.iter().filter(|&(c, _)| c.land_type() == FetchLand)
        .fold((0u, Mana::zero()), |(l, m), (c, n)| { (l + *n, m + c.source().mul(*n).reset(W).reset(G)) });
    let scrys = ls.iter().filter(|&(c, _)| c.land_type() == ScryLand)
        .fold((0u, Mana::zero()), |(l, m), (c, n)| { (l + *n, m + c.source().mul(*n).reset(W).reset(G)) });
    let pains = ls.iter().filter(|&(c, _)| c.land_type() == PainLand)
        .fold((0u, Mana::zero()), |(l, m), (c, n)| { (l + *n, m + c.source().mul(*n).reset(W).reset(G)) });
    let refus = ls.iter().filter(|&(c, _)| c.land_type() == RefuLand)
        .fold((0u, Mana::zero()), |(l, m), (c, n)| { (l + *n, m + c.source().mul(*n).reset(W).reset(G)) });

    println!("{:2u} {:-30s} {:-5s}", basics.0, "Basics".to_string(), basics.1.src());
    println!("{:2u} {:-30s} {:-5s}", fetches.0, "Fetch-lands".to_string(), fetches.1.src());
    println!("{:2u} {:-30s} {:-5s}", pains.0, "Pain-lands".to_string(), pains.1.src());
    println!("{:2u} {:-30s} {:-5s}", scrys.0, "Scry-lands".to_string(), scrys.1.src());
    println!("{:2u} {:-30s} {:-5s}", refus.0, "Refugee lands".to_string(), refus.1.src());

    return lds.0;
}
