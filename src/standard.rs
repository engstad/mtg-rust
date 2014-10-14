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

// WUBRG

#[deriving(Clone, Show, PartialEq, Eq, PartialOrd, Ord)]
pub enum Shard {
    WUR, UBG, BRW, RGU, GWB
}

impl Shard {
    fn source(&self) -> Mana {
        match *self {
            WUR => Mana::w(1) + Mana::u(1) + Mana::r(1), 
            UBG => Mana::u(1) + Mana::b(1) + Mana::g(1), 
            BRW => Mana::b(1) + Mana::r(1) + Mana::w(1), 
            RGU => Mana::r(1) + Mana::g(1) + Mana::u(1),
            GWB => Mana::g(1) + Mana::w(1) + Mana::b(1) 
        }
    }    
}

struct ShardKeys;

impl Keys<Shard> for ShardKeys {
    fn size(&self) -> uint { 5 }
    fn to_uint(&self, c:Shard) -> uint { c as uint }
    fn from_uint(&self, n:uint) -> Shard { 
        match n {
            0 => Some(WUR), 1 => Some(UBG), 2 => Some(BRW), 3 => Some(RGU), 4 => Some(GWB), 
            _ => None
        }.unwrap()
    }    
}


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

pub struct LandCardInfo {
    pub name : &'static str,
    pub cardtype : &'static str,
    pub subtypes : &'static [&'static str],
    pub landtype : LandType,
    pub produces : &'static [Color]
}

static LANDS : &'static [LandCardInfo] = [
    LandCardInfo { name : "Plains",              cardtype : "Basic",    subtypes : ["Plains"],    landtype : BasicLand, produces : [W] },
    LandCardInfo { name : "Island",              cardtype : "Basic",    subtypes : ["Island"],    landtype : BasicLand, produces : [U] },
    LandCardInfo { name : "Swamp",               cardtype : "Basic",    subtypes : ["Swamp"],     landtype : BasicLand, produces : [B] },
    LandCardInfo { name : "Mountain",            cardtype : "Basic",    subtypes : ["Mountain"],  landtype : BasicLand, produces : [R] },
    LandCardInfo { name : "Forest",              cardtype : "Basic",    subtypes : ["Forest"],    landtype : BasicLand, produces : [G] },

    LandCardInfo { name : "Hallowed Fountain",   cardtype : "Land",     subtypes : ["Plains", "Island"],   landtype : ShockLand, produces : [W, U] },
	LandCardInfo { name : "Watery Grave",        cardtype : "Land",     subtypes : ["Island", "Swamp"],    landtype : ShockLand, produces : [U, B] },
	LandCardInfo { name : "Blood Crypt",         cardtype : "Land",     subtypes : ["Swamp", "Mountain"],  landtype : ShockLand, produces : [B, R] },
	LandCardInfo { name : "Stomping Ground",     cardtype : "Land",     subtypes : ["Mountain", "Forest"], landtype : ShockLand, produces : [R, G] },
	LandCardInfo { name : "Temple Garden",       cardtype : "Land",     subtypes : ["Forest", "Plains"],   landtype : ShockLand, produces : [G, W] },
                                                                                                                                                    
	LandCardInfo { name : "Godless Shrine",      cardtype : "Land",     subtypes : ["Plains", "Swamp"],    landtype : ShockLand, produces : [W, B] },
	LandCardInfo { name : "Steam Vents",         cardtype : "Land",     subtypes : ["Island", "Mountain"], landtype : ShockLand, produces : [U, R] },
	LandCardInfo { name : "Overgrown Tomb",      cardtype : "Land",     subtypes : ["Swamp", "Forest"],    landtype : ShockLand, produces : [B, G] },
	LandCardInfo { name : "Sacred Foundry",      cardtype : "Land",     subtypes : ["Mountain", "Plains"], landtype : ShockLand, produces : [R, W] },
	LandCardInfo { name : "Breeding Pool",       cardtype : "Land",     subtypes : ["Forest", "Island"],   landtype : ShockLand, produces : [G, U] },

    LandCardInfo { name : "Azorius Guildgate",   cardtype : "Land",     subtypes : ["Gate"],   landtype : Gates, produces : [W, U] },
	LandCardInfo { name : "Dimir Guildgate",     cardtype : "Land",     subtypes : ["Gate"],   landtype : Gates, produces : [U, B] },
	LandCardInfo { name : "Rakdos Guildgate",    cardtype : "Land",     subtypes : ["Gate"],   landtype : Gates, produces : [B, R] },
	LandCardInfo { name : "Gruul Guildgate",     cardtype : "Land",     subtypes : ["Gate"],   landtype : Gates, produces : [R, G] },
	LandCardInfo { name : "Selesnya Guildgate",  cardtype : "Land",     subtypes : ["Gate"],   landtype : Gates, produces : [G, W] },
                                                                                                                                                    
	LandCardInfo { name : "Orzhov Guildgate",    cardtype : "Land",     subtypes : ["Gate"],   landtype : Gates, produces : [W, B] },
	LandCardInfo { name : "Izzet Guildgate",     cardtype : "Land",     subtypes : ["Gate"],   landtype : Gates, produces : [U, R] },
	LandCardInfo { name : "Golgari Guildgate",   cardtype : "Land",     subtypes : ["Gate"],   landtype : Gates, produces : [B, G] },
	LandCardInfo { name : "Boros Guildgate",     cardtype : "Land",     subtypes : ["Gate"],   landtype : Gates, produces : [R, W] },
	LandCardInfo { name : "Simic Guildgate",     cardtype : "Land",     subtypes : ["Gate"],   landtype : Gates, produces : [G, U] },

    LandCardInfo { name : "Temple of Enlightenment", cardtype : "Land", subtypes : [],   landtype : ScryLand, produces : [W, U] },
	LandCardInfo { name : "Temple of Deceit",        cardtype : "Land", subtypes : [],   landtype : ScryLand, produces : [U, B] },
	LandCardInfo { name : "Temple of Malice",        cardtype : "Land", subtypes : [],   landtype : ScryLand, produces : [B, R] },
	LandCardInfo { name : "Temple of Abandon",       cardtype : "Land", subtypes : [],   landtype : ScryLand, produces : [R, G] },
	LandCardInfo { name : "Temple of Plenty",        cardtype : "Land", subtypes : [],   landtype : ScryLand, produces : [G, W] },
                                                                                                                                                    
	LandCardInfo { name : "Temple of Silence",   cardtype : "Land",     subtypes : [],   landtype : ScryLand, produces : [W, B] },
	LandCardInfo { name : "Temple of Epiphany",  cardtype : "Land",     subtypes : [],   landtype : ScryLand, produces : [U, R] },
	LandCardInfo { name : "Temple of Malady",    cardtype : "Land",     subtypes : [],   landtype : ScryLand, produces : [B, G] },
	LandCardInfo { name : "Temple of Triumph",   cardtype : "Land",     subtypes : [],   landtype : ScryLand, produces : [R, W] },
	LandCardInfo { name : "Temple of Mystery",   cardtype : "Land",     subtypes : [],   landtype : ScryLand, produces : [G, U] },

    LandCardInfo { name : "Flooded Strand",      cardtype : "Land", subtypes : [],   landtype : FetchLand, produces : [W, U] },
	LandCardInfo { name : "Polluted Delta",      cardtype : "Land", subtypes : [],   landtype : FetchLand, produces : [U, B] },
	LandCardInfo { name : "Bloodstained Mire",  cardtype : "Land", subtypes : [],   landtype : FetchLand, produces : [B, R] },
	LandCardInfo { name : "Wooded Foothills",    cardtype : "Land", subtypes : [],   landtype : FetchLand, produces : [R, G] },
	LandCardInfo { name : "Windswept Heath",     cardtype : "Land", subtypes : [],   landtype : FetchLand, produces : [G, W] },

	LandCardInfo { name : "Caves of Koilos",     cardtype : "Land",     subtypes : [],   landtype : PainLand, produces : [W, B] },
	LandCardInfo { name : "Shivan Reef",         cardtype : "Land",     subtypes : [],   landtype : PainLand, produces : [U, R] },
	LandCardInfo { name : "Llanowar Wastes",     cardtype : "Land",     subtypes : [],   landtype : PainLand, produces : [B, G] },
	LandCardInfo { name : "Battlefield Forge",   cardtype : "Land",     subtypes : [],   landtype : PainLand, produces : [R, W] },
	LandCardInfo { name : "Yavimaya Coast",      cardtype : "Land",     subtypes : [],   landtype : PainLand, produces : [G, U] },

    LandCardInfo { name : "Nomad Outpost",       cardtype : "Land",     subtypes : [],   landtype : WedgeLand, produces : [W, B, R] },
    LandCardInfo { name : "Frontier Bivouac",    cardtype : "Land",     subtypes : [],   landtype : WedgeLand, produces : [U, R, G] },
    LandCardInfo { name : "Sandsteppe Citadel",  cardtype : "Land",     subtypes : [],   landtype : WedgeLand, produces : [B, G, W] },
    LandCardInfo { name : "Mystic Monestary",    cardtype : "Land",     subtypes : [],   landtype : WedgeLand, produces : [R, W, U] },
    LandCardInfo { name : "Opulent Palace",      cardtype : "Land",     subtypes : [],   landtype : WedgeLand, produces : [G, U, B] },

    LandCardInfo { name : "Evolving Wild",       cardtype : "Land", subtypes : [], landtype : SpecialLand, produces : [W, U, B, R, G] },
    LandCardInfo { name : "Urborg, Tomb of Yawgmoth", cardtype : "Land", subtypes : [], landtype: SpecialLand, produces : [B] },
    LandCardInfo { name : "Mana Confluence", cardtype : "Land", subtypes : [], landtype : SpecialLand, produces : [W, U, B, R, G] },
];

#[deriving(Clone, Show, PartialEq, Eq, PartialOrd, Ord)]
pub enum SpecType {
    EvolvingWild,
    Urborg,
    ManaConfluence
}

#[deriving(Clone, Show, PartialEq, Eq, PartialOrd, Ord)]
pub enum Land {
    Special(SpecType),
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
                A(BR) => "Bloodstained Mire",
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
            Special(EvolvingWild) =>
                "Evolving Wild",
            Special(Urborg) => 
                "Urborg, Tomb of Yawgmoth",
            Special(ManaConfluence) =>
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
            Special(EvolvingWild) => Mana::w(1) + Mana::u(1) + Mana::b(1) + Mana::r(1) + Mana::g(1),
            Special(ManaConfluence) => Mana::w(1) + Mana::u(1) + Mana::b(1) + Mana::r(1) + Mana::g(1),
            Special(Urborg) => Mana::b(1)
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
            Special(EvolvingWild) => false,
            Special(Urborg) => true,
            Special(ManaConfluence) => true
	    }	
    }

    fn tapped(&self) -> bool { !self.untapped() }

    fn from_string(s: &str) -> Option<Land> {
        for i in range(0u, LANDS.len()) { 
            let lc = LANDS[i];
            if lc.name == s {
                return match lc.landtype {
                    BasicLand => 
                        Some(match lc.produces {
                            [c] => Basic(c),
                            _ => fail!("Uh")
                        }),
                    ScryLand =>
                        Some(Scry(match lc.produces {
                            [W, U] => A(WU),
                            [U, B] => A(UB),
                            [B, R] => A(BR),
                            [R, G] => A(RG),
                            [G, W] => A(GW),
                            [W, B] => E(WB),
                            [U, R] => E(UR),
                            [B, G] => E(BG),
                            [R, W] => E(RW),
                            [G, U] => E(GU),
                            _ => fail!("oops")
                        })),
                    FetchLand =>
                        Some(Fetch(match lc.produces {
                            [W, U] => A(WU),
                            [U, B] => A(UB),
                            [B, R] => A(BR),
                            [R, G] => A(RG),
                            [G, W] => A(GW),
                            [W, B] => E(WB),
                            [U, R] => E(UR),
                            [B, G] => E(BG),
                            [R, W] => E(RW),
                            [G, U] => E(GU),
                            _ => fail!("oops")
                        })),
                    PainLand => 
                        Some(Pain(match lc.produces {
                            [W, B] => WB,
                            [U, R] => UR,
                            [B, G] => BG,
                            [R, W] => RW,
                            [G, U] => GU,
                            _ => fail!("oops")
                        })),
                    WedgeLand =>
                        Some(Khan(match lc.produces {
                            [W, B, R] => WBR,
                            [U, R, G] => URG,
                            [B, G, W] => BGW,
                            [R, W, U] => RWU,
                            [G, U, B] => GUB,
                            _ => fail!("oops")
                        })),

                    SpecialLand =>
                        match lc.name {
                            "Urborg, Tomb of Yawgmoth" => Some(Special(Urborg)),
                            "Evolving Wild" => Some(Special(EvolvingWild)),
                            "Mana Confluence" => Some(Special(ManaConfluence)),
                            _ => None
                        },
                    _ => None
                }
            }
        }
        return None
    }
}

struct LandKeys;

impl Keys<Land> for LandKeys {
    fn size(&self) -> uint { ColorKeys.size() + 3 * DualKeys.size() }

    fn to_uint(&self, l:Land) -> uint { 
        match l {
            Special(EvolvingWild) => 0,
            Special(Urborg) => 1,
            Special(ManaConfluence) => 2,
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
        if d == 0 { Special(EvolvingWild) }
        else if d == 1 { Special(Urborg) }
        else if d == 2 { Special(ManaConfluence) }
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
enum LandType { SpecialLand, BasicLand, ShockLand, Gates, ScryLand, RefuLand, FetchLand, PainLand, WedgeLand }

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
            _ => SpecialLand
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
//
pub fn test_parsing() -> Vec<(Land, int)>
{
    //use regex::Regex;

    /*
    let lands = r#"2 Caves of Koilos               
                   2 Flooded Strand                
                   3 Island                        
                   2 Plains                        
                   2 Polluted Delta                
                   2 Swamp                         
                   4 Temple of Deceit              
                   4 Temple of Enlightenment       
                   4 Temple of Silence             
                   1 Urborg, Tomb of Yawgmoth"#;
    */
    let lands = r#"4 Bloodstained Mire
                   2 Mana Confluence
                   3 Mountain
                   4 Nomad Outpost
                   1 Plains
                   3 Swamp
                   4 Temple of Silence
                   4 Temple of Triumph
                   1 Urborg, Tomb of Yawgmoth"#;


    lands.as_slice().split('\n').map(|line| { 
        //println!("line = '{}'", line.);
        let caps:Vec<&str> = line.as_slice().trim().splitn(1, ' ').collect(); 
        let n = from_str::<int>(caps[0]).unwrap();

        (Land::from_string(caps[1]).unwrap(), n)

        //println!("{:d} {:s}", n, match Land::from_string(caps[1]) {
        //Some(l) => l.show(), None => format!("{}", "??")
        //})            
    }).collect()
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

    if false {
        ls.insert(L(Special(ManaConfluence)), 2);
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
        
        ls.insert(L(Special(Urborg)), 1);
    }
    else if false {
        // 26: (10, 18, 16, 10) 

        // --: ( 4,  0,  4,  4)
        // --: ( 4,  4,  0,  4)
        // ----------------
        // 18: ( 2, 14, 12,  2)
        // 12: ( 1, 12, 10,  1) 6 basics [24/12]
        
        // Mana Confluence = 4.0 sources per card
        // Tri-lands = 3.0 sources per card
        // 4 Evolving Wilds + 4 Basics = (4*4+4)/8 = 2.5 sources per card
        // Duals = 2.0 sources per card
        // 4 Fetchland + 4 Basics = 12/8 = 1.5 sources per card

        // Base UB, splashing white (Utter End, Narset) & red.
        ls.insert(L(Khan(WBR)), 4);
        ls.insert(L(Khan(RWU)), 4);

        ls.insert(L(Basic(W)), 1);
        ls.insert(L(Basic(U)), 2);
        ls.insert(L(Basic(B)), 2);
        ls.insert(L(Special(Urborg)), 1);
        ls.insert(L(Special(ManaConfluence)), 1);

        ls.insert(L(Fetch(A(UB))), 4);
        ls.insert(L(Fetch(A(WU))), 1);
        ls.insert(L(Scry(A(UB))), 3);
        ls.insert(L(Refu(A(UB))), 1);
        ls.insert(L(Pain(UR)), 2);

    }
    else {
        let ps = test_parsing();

        for &(l, n) in ps.iter() {
            ls.insert(L(l), n as uint);
        }
    }
        
        
    
    //ls.insert(S, 36);
    //ls.insert(L(Khan(URG)), 2);            

    let conc = |acc:String, (&card, &num): (&Card, &uint)| -> String {
        if acc == "".to_string() {
            format!("{:2u} {:-30s} {:-5s}", num, card.show(), card.source().mul(num).src())
        } else {
            format!("{}\n{:2u} {:-30s} {:-5s}", acc, num, card.show(), card.source().mul(num).src())
        }
    };

    let res = ls.iter().fold("".to_string(), conc);

    let lds = ls.iter()
        .fold((0u, Mana::zero()), |(l, m), (c, n)| { (l + *n, m + c.source().mul(*n)) });
    let unt = ls.iter().filter(|&(c, _)| c.untapped())
        .fold((0u, Mana::zero()), |(l, m), (c, n)| { (l + *n, m + c.source().mul(*n)) });
    let tap = ls.iter().filter(|&(c, _)| !c.untapped())
        .fold((0u, Mana::zero()), |(l, m), (c, n)| { (l + *n, m + c.source().mul(*n)) });

    println!("Res:\n{}", res);
    println!("{:2u} {:-30s} {:-5s}\n", lds.0, "Cards".to_string(), lds.1.src());
    println!("{:2u} {:-30s} {:-5s}", unt.0, "Untapped".to_string(), unt.1.src());
    println!("{:2u} {:-30s} {:-5s}\n", tap.0, "Tapped".to_string(), tap.1.src());

    let basics = ls.iter().filter(|&(c, _)| c.land_type() == BasicLand)
        .fold((0u, Mana::zero()), |(l, m), (c, n)| { (l + *n, m + c.source().mul(*n)) });
    let fetches = ls.iter().filter(|&(c, _)| c.land_type() == FetchLand)
        .fold((0u, Mana::zero()), |(l, m), (c, n)| { (l + *n, m + c.source().mul(*n)) });
    let scrys = ls.iter().filter(|&(c, _)| c.land_type() == ScryLand)
        .fold((0u, Mana::zero()), |(l, m), (c, n)| { (l + *n, m + c.source().mul(*n)) });
    let pains = ls.iter().filter(|&(c, _)| c.land_type() == PainLand)
        .fold((0u, Mana::zero()), |(l, m), (c, n)| { (l + *n, m + c.source().mul(*n)) });
    let refus = ls.iter().filter(|&(c, _)| c.land_type() == RefuLand)
        .fold((0u, Mana::zero()), |(l, m), (c, n)| { (l + *n, m + c.source().mul(*n)) });

    println!("{:2u} {:-30s} {:-5s}", basics.0, "Basics".to_string(), basics.1.src());
    println!("{:2u} {:-30s} {:-5s}", fetches.0, "Fetch-lands".to_string(), fetches.1.src());
    println!("{:2u} {:-30s} {:-5s}", pains.0, "Pain-lands".to_string(), pains.1.src());
    println!("{:2u} {:-30s} {:-5s}", scrys.0, "Scry-lands".to_string(), scrys.1.src());
    println!("{:2u} {:-30s} {:-5s}", refus.0, "Refugee lands".to_string(), refus.1.src());

    return lds.0;
}
