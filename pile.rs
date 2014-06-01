use std::iter::AdditiveIterator;
use std::fmt;
use prob;

pub trait Pile {
    fn total(&self) -> uint;

    fn get(&self, uint) -> uint;

    fn keys(&self) -> uint;

    fn has(&self, other: &Self) -> bool;

    fn add(&self, &Self) -> Self;
    fn sub(&self, &Self) -> Self;

    fn p(&self, draw: &Self) -> f64 {
	    let mut nT:uint = 0;
	    let mut kT:uint = 0;
	    let mut cT = 1.0f64;
   
	    let nlen = self.keys();

	    for i in range(0, nlen) {
            let nI = self.get(i);
            let kI = draw.get(i);
		    nT = nT + nI;
		    kT = kT + kI;
		    cT *= prob::c(nI, kI);
	    }

	    cT / prob::c(nT, kT)
    }
}

pub trait LandPile {
    fn spells(&self) -> uint;
    fn lands(&self) -> uint;

    fn p_lands(&self, l:uint, s:uint) -> f64 {
        let L = self.lands();
        let S = self.spells();
        prob::h(L, l, S, s)
    }
}

pub struct PileInfo {
    num_keys : uint,
    is_land  : fn(uint) -> bool
}

impl PileInfo {
    pub fn new(keys: uint, lands: fn(uint)->bool) -> PileInfo {
        PileInfo { num_keys : keys, is_land : lands }
    }
}

impl PartialEq for PileInfo {
    fn eq(&self, b : &PileInfo) -> bool {
        self.num_keys == b.num_keys &&
        (self.is_land as *u8) == (b.is_land as *u8)
    }
}

pub struct GenPile {
    e: Vec<uint>,
    info: PileInfo
}

impl GenPile {
    pub fn new(l : Vec<uint>, i : PileInfo) -> GenPile {
        GenPile { e : l, info : i }
    }

    pub fn iter(n : uint, i : PileInfo) -> GenPile {
        let sz = i.num_keys;
        let l = Vec::from_fn(sz, |idx| if idx == 0 { n } else { 0 });
        GenPile { e : l, info : i }
    }
}

impl Pile for GenPile {
    fn total(&self) -> uint { self.e.iter().map(|&p| p).sum() }

    fn keys(&self) -> uint { self.e.len() }
    
    fn get(&self, k: uint) -> uint { 
        *self.e.get(k)
    }

    fn has(&self, other: &GenPile) -> bool {
        self.e.iter().zip(other.e.iter()).all(|(&i0, &i1)| i0 >= i1)
    }

    fn add(&self, other : &GenPile) -> GenPile {
        assert!(self.info == other.info);
        GenPile::new(self.e.iter().zip(other.e.iter()).map(|(&i0, &i1)| i0+i1).collect(),
                     self.info)
    }

    fn sub(&self, other : &GenPile) -> GenPile {
        assert!(self.info == other.info);
        assert!(self.has(other));
        GenPile::new(self.e.iter().zip(other.e.iter()).map(|(&i0, &i1)| i0-i1).collect(),
                     self.info)
    }        
}

impl fmt::Show for GenPile {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {        
        try!(write!(fmt, "("));
        for (i,v) in self.e.iter().enumerate() {
            try!(write!(fmt, "{}", v));
            if i < self.e.len() - 1 {
                try!(write!(fmt, ","))
            }
        }
        write!(fmt, ")")
    }
}

impl Iterator<GenPile> for GenPile {

    fn next(&mut self) -> Option<GenPile> {
        let res = GenPile { e: self.e.clone(), info : self.info };

        if *self.e.get(0) > 0 {
            *self.e.get_mut(0) -= 1;
            *self.e.get_mut(1) += 1;
            Some(res)
        } else {
            let len = self.e.len();
            for i in range(1u, len-1) {
                if self.get(i) > 0 {
                    *self.e.get_mut(0) = self.get(i) - 1; 
                    *self.e.get_mut(i+1) += 1;
                    *self.e.get_mut(i) = 0;
                    return Some(res)
                }
            }
            if self.get(len - 1) > 0 {
                *self.e.get_mut(len-1) = 0;
                Some(res)
            } else {
                None
            }
        }
    }
}

impl LandPile for GenPile {
    fn lands(&self) -> uint { 
        self.e.iter().enumerate()
            .map(|(i, v)| if (self.info.is_land)(i) { *v } else { 0 }).sum()
    }

    fn spells(&self) -> uint { 
        self.total() - self.lands() 
    }
}

pub struct ColoredPile {
    e:[uint, ..3]
}

impl ColoredPile {
    pub fn new(c: uint, n: uint, s: uint) -> ColoredPile {
        ColoredPile{ e:[c, n, s] }
    }

    pub fn iter(n: uint) -> ColoredPile {
        ColoredPile{ e:[n, 0, 0] }
    }

    pub fn colored(&self) -> uint { self.e[0] }
}

impl Pile for ColoredPile {
    fn total(&self) -> uint { self.e.iter().fold(0, |a, &b| a + b) }

    fn keys(&self) -> uint { 3 }
    
    fn get(&self, k: uint) -> uint { 
        self.e[k]
    }

    fn has(&self, other: &ColoredPile) -> bool {
        self.e[0] >= other.e[0] && self.e[1] >= other.e[1] && self.e[2] >= other.e[2]
    }

    fn add(&self, other : &ColoredPile) -> ColoredPile {
        ColoredPile::new(self.e[0] + other.e[0], 
                         self.e[1] + other.e[1], 
                         self.e[2] + other.e[2])
    }

    fn sub(&self, other : &ColoredPile) -> ColoredPile {
        assert!(self.has(other));

        ColoredPile::new(self.e[0] - other.e[0], 
                         self.e[1] - other.e[1], 
                         self.e[2] - other.e[2])
    }        
}

impl LandPile for ColoredPile {
    fn lands(&self) -> uint { self.e[0] + self.e[1] }
    fn spells(&self) -> uint { self.total() - self.lands() }
}

impl Iterator<ColoredPile> for ColoredPile {
    fn next(&mut self) -> Option<ColoredPile> {
        let res = ColoredPile { e: self.e };
        if self.e[0] > 0 {
            self.e[0] -= 1;
            self.e[1] += 1;
            Some(res)
        } else if self.e[1] > 0 { 
            self.e[0] += self.e[1] - 1;
            self.e[2] += 1;
            self.e[1] = 0;
            Some(res)
        } else if self.e[2] > 0 {
            self.e[2] = 0;
            Some(res)
        } else {
            None
        }
    }
}

impl fmt::Show for ColoredPile {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {        
        write!(fmt, "(c:{:3}, x:{:3}, s:{:3})", 
               self.e[0], self.e[1], self.e[2])        
    }
}

#[deriving(Clone)]
pub struct DualPile {
    pub a:  uint, 
    pub b:  uint, // non-colored lands
    pub ab: uint,
    pub x:  uint,
    pub s:  uint, // spells
}

impl DualPile {
    pub fn new(a: uint, b: uint, ab: uint, x: uint, s: uint) -> DualPile {
        DualPile { a:a, b:b, ab:ab, x:x, s:s }
    }

    pub fn iter(d: uint) -> DualPile {
        DualPile { a:d, b:0, ab:0, x:0, s:0 }
    }
}

impl Pile for DualPile {
    fn total(&self) -> uint { self.a + self.b + self.ab + self.x + self.s }

    fn keys(&self) -> uint { 5 }
    
    fn get(&self, k: uint) -> uint { match k { 0 => self.a,
                                               1 => self.b,
                                               2 => self.ab, 
                                               3 => self.x,
                                               4 => self.s,
                                               _ => fail!("out of range") } }

    fn has(&self, other : &DualPile) -> bool {
        (self.a >= other.a &&
         self.b >= other.b &&
         self.ab >= other.ab &&
         self.x >= other.x &&
         self.s >= other.s)
    }

    fn add(&self, other : &DualPile) -> DualPile {
        DualPile::new(self.a + other.a, 
                      self.b + other.b, 
                      self.ab + other.ab,
                      self.x + other.x,
                      self.s + other.s)
    }

    fn sub(&self, other : &DualPile) -> DualPile {
        DualPile::new(self.a - other.a, 
                      self.b - other.b, 
                      self.ab - other.ab,
                      self.x - other.x,
                      self.s - other.s)
    }
}

impl LandPile for DualPile {
    fn spells(&self) -> uint { self.total() - self.s }
    fn lands(&self) -> uint { self.s }
}

impl Iterator<DualPile> for DualPile {

    fn next(&mut self) -> Option<DualPile> {
        let res = DualPile::new(self.a, self.b, self.ab, self.x, self.s);
        if self.a > 0 {
            self.a -= 1;
            self.b += 1;
            Some(res)
        } else if self.b > 0 { 
            // a = 0, b > 0
            self.a += self.b - 1;
            self.ab += 1;
            self.b = 0;
            Some(res)
        } else if self.ab > 0 {
            // a = 0, b = 0, ab > 0
            self.a += self.ab - 1; 
            self.x += 1;
            self.ab = 0;                
            Some(res)
        } else if self.x > 0 {
            // a = 0, b = 0, ab = 0, x > 0
            self.a += self.x - 1;
            self.s += 1;
            self.x = 0;
            Some(res)
        } else if self.s > 0 {
            self.s = 0;
            Some(res)
        } else {
            None
        }
    }
}

impl fmt::Show for DualPile {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {        
        write!(fmt, "[a:{:3}, b:{:3}, ab:{:3}, x:{:3}, l:{:3}]", 
               self.a, self.b, self.ab, self.x, self.s)        
    }
}
