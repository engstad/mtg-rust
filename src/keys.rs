
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

