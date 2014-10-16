use mana::Mana;

#[deriving(Clone, Show, PartialEq, Eq, PartialOrd, Ord, Encodable, Decodable)]
pub enum Color { W, U, B, R, G, C } 

impl Color {
    pub fn source(&self) -> Mana {
        match *self {
            W => Mana::w(1u), 
            U => Mana::u(1u), 
            B => Mana::b(1u), 
            R => Mana::r(1u),
            G => Mana::g(1u),
            C => Mana::c(1u)
        }
    }    
}

#[deriving(Clone, Show, PartialEq, Eq, PartialOrd, Ord)]
pub enum Allied { 
    WU, UB, BR, RG, GW
}

impl Allied {
    pub fn source(&self) -> Mana {
        match *self {
            WU => Mana::w(1) + Mana::u(1), 
            UB => Mana::u(1) + Mana::b(1), 
            BR => Mana::b(1) + Mana::r(1), 
            RG => Mana::r(1) + Mana::g(1),
            GW => Mana::g(1) + Mana::w(1) 
        }
    }    
}

#[deriving(Clone, Show, PartialEq, Eq, PartialOrd, Ord)]
pub enum Enemy {
    WB, UR, BG, RW, GU 
}

impl Enemy {
    pub fn source(&self) -> Mana {
        match *self {
            WB => Mana::w(1) + Mana::b(1), 
            UR => Mana::u(1) + Mana::r(1), 
            BG => Mana::b(1) + Mana::g(1), 
            RW => Mana::r(1) + Mana::w(1),
            GU => Mana::g(1) + Mana::u(1) 
        }
    }    
}

#[deriving(Clone, Show, PartialEq, Eq, PartialOrd, Ord)]
pub enum Dual { 
    A(Allied),
    E(Enemy)
}

impl Dual {
    pub fn source(&self) -> Mana {
        match *self {
            A(a) => a.source(),
            E(e) => e.source()
        }
    }    
}

#[deriving(Clone, Show, PartialEq, Eq, PartialOrd, Ord)]
pub enum Shard {
    WUR, UBG, BRW, RGU, GWB
}

impl Shard {
    pub fn source(&self) -> Mana {
        match *self {
            WUR => Mana::w(1) + Mana::u(1) + Mana::r(1), 
            UBG => Mana::u(1) + Mana::b(1) + Mana::g(1), 
            BRW => Mana::b(1) + Mana::r(1) + Mana::w(1), 
            RGU => Mana::r(1) + Mana::g(1) + Mana::u(1),
            GWB => Mana::g(1) + Mana::w(1) + Mana::b(1) 
        }
    }    
}

#[deriving(Clone, Show, PartialEq, Eq, PartialOrd, Ord)]
pub enum Wedge {
    WBR, URG, BGW, RWU, GUB
}

impl Wedge {
    pub fn source(&self) -> Mana {
        match *self {
            WBR => Mana::w(1) + Mana::b(1) + Mana::r(1), 
            URG => Mana::u(1) + Mana::r(1) + Mana::g(1), 
            BGW => Mana::b(1) + Mana::g(1) + Mana::w(1), 
            RWU => Mana::r(1) + Mana::w(1) + Mana::u(1),
            GUB => Mana::g(1) + Mana::u(1) + Mana::b(1) 
        }
    }    
}
