use mana::Mana;

#[derive(Copy, Clone, Show, PartialEq, Eq, PartialOrd, Ord, RustcDecodable)]
pub enum Color { W, U, B, R, G, C } 

impl Color {
    pub fn source(&self) -> Mana {
        match *self {
            Color::W => Mana::w(1), 
            Color::U => Mana::u(1), 
            Color::B => Mana::b(1), 
            Color::R => Mana::r(1),
            Color::G => Mana::g(1),
            Color::C => Mana::c(1)
        }
    }    

    pub fn parse(c: &str) -> Color {
        match c {
            "White" => Color::W,
            "Blue" => Color::U,
            "Black" => Color::B,
            "Red" => Color::R,
            "Green" => Color::G,
            "Colorless" => Color::C,
            _ => panic!("Unknown color {}", c)
        }
    }
}

#[derive(Copy, Clone, Show, PartialEq, Eq, PartialOrd, Ord)]
pub enum Allied { 
    WU, UB, BR, RG, GW
}

impl Allied {
    pub fn source(&self) -> Mana {
        match *self {
            Allied::WU => Mana::w(1) + Mana::u(1), 
            Allied::UB => Mana::u(1) + Mana::b(1), 
            Allied::BR => Mana::b(1) + Mana::r(1), 
            Allied::RG => Mana::r(1) + Mana::g(1),
            Allied::GW => Mana::g(1) + Mana::w(1) 
        }
    }    
}

#[derive(Copy, Clone, Show, PartialEq, Eq, PartialOrd, Ord)]
pub enum Enemy {
    WB, UR, BG, RW, GU 
}

impl Enemy {
    pub fn source(&self) -> Mana {
        match *self {
            Enemy::WB => Mana::w(1) + Mana::b(1), 
            Enemy::UR => Mana::u(1) + Mana::r(1), 
            Enemy::BG => Mana::b(1) + Mana::g(1), 
            Enemy::RW => Mana::r(1) + Mana::w(1),
            Enemy::GU => Mana::g(1) + Mana::u(1) 
        }
    }    
}

#[derive(Copy, Clone, Show, PartialEq, Eq, PartialOrd, Ord)]
pub enum Dual { 
    A(Allied),
    E(Enemy)
}

impl Dual {
    pub fn source(&self) -> Mana {
        match *self {
            Dual::A(a) => a.source(),
            Dual::E(e) => e.source()
        }
    }    
}

#[derive(Copy, Clone, Show, PartialEq, Eq, PartialOrd, Ord)]
pub enum Shard {
    WUR, UBG, BRW, RGU, GWB
}

impl Shard {
    pub fn source(&self) -> Mana {
        match *self {
            Shard::WUR => Mana::w(1) + Mana::u(1) + Mana::r(1), 
            Shard::UBG => Mana::u(1) + Mana::b(1) + Mana::g(1), 
            Shard::BRW => Mana::b(1) + Mana::r(1) + Mana::w(1), 
            Shard::RGU => Mana::r(1) + Mana::g(1) + Mana::u(1),
            Shard::GWB => Mana::g(1) + Mana::w(1) + Mana::b(1) 
        }
    }    
}

#[derive(Copy, Clone, Show, PartialEq, Eq, PartialOrd, Ord)]
pub enum Wedge {
    WBR, URG, BGW, RWU, GUB
}

impl Wedge {
    pub fn source(&self) -> Mana {
        match *self {
            Wedge::WBR => Mana::w(1) + Mana::b(1) + Mana::r(1), 
            Wedge::URG => Mana::u(1) + Mana::r(1) + Mana::g(1), 
            Wedge::BGW => Mana::b(1) + Mana::g(1) + Mana::w(1), 
            Wedge::RWU => Mana::r(1) + Mana::w(1) + Mana::u(1),
            Wedge::GUB => Mana::g(1) + Mana::u(1) + Mana::b(1) 
        }
    }    
}
