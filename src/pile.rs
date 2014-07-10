use std::iter::AdditiveIterator;
use std::fmt;
use prob;
use standard::Keys;
use perm::MultiSubSetIterator;

//
// A Pile is like a key-value map, where the values are always uint.
//
pub trait KvMap<K : Copy, Ks : Copy + Keys<K>> {
    fn keys(&self) -> Ks;
    fn get<'a>(&'a self, K) -> uint;

    fn prob_draw(&self, draw: &Self) -> f64 {
        let (nT, kT, cT) =
            self.keys().iter()
            .map(|k| (self.get(k), draw.get(k)))
            .fold((0, 0, 1.0), 
                  |(nT, kT, cT), (nI, kI)| (nT + nI, kT + kI, cT * prob::c(nI, kI)));

        cT / prob::c(nT, kT)
    }    

    fn total(&self) -> uint {
        let ks = self.keys();
        ks.iter().map(|k| self.get(k)).sum()
    }

    fn num_keys(&self) -> uint { self.keys().size() }
    
    fn has(&self, other: &Self) -> bool {
        self.keys()
            .iter()
            .map(|k| (self.get(k), other.get(k)))
            .all(|(v0, v1)| v0 >= v1)
    }

    fn add(&self, &Self) -> Self;
    fn sub(&self, &Self) -> Self;    
}

pub trait LandPile {
    fn spells(&self) -> uint;
    fn lands(&self) -> uint;

    fn prob_land(&self, l:uint, s:uint) -> f64 {
        let ls = self.lands();
        let ss = self.spells();
        prob::h(ls, l, ss, s)
    }
}

pub struct GenPileKeys {
    num_keys : uint,
    is_land  : fn(uint) -> bool
}

impl GenPileKeys {
    pub fn new(keys: uint, lands: fn(uint)->bool) -> GenPileKeys {
        GenPileKeys { num_keys : keys, is_land : lands }
    }
}

impl Keys<uint> for GenPileKeys {
    fn size(&self) -> uint { self.num_keys }
    fn to_uint(&self, n: uint) -> uint { n }
    fn from_uint(&self, n: uint) -> uint { 
        if n < self.num_keys { n } else { fail!("out of range") } 
    }
}

impl PartialEq for GenPileKeys {
    fn eq(&self, b : &GenPileKeys) -> bool {
        self.num_keys == b.num_keys &&
        (self.is_land as *const u8) == (b.is_land as *const u8)
    }
}

pub struct GenPile<Keys> {
    e: Vec<uint>,
    k: Keys
}

impl<Ks : Keys<uint> + Copy> GenPile<Ks> {
    pub fn new(l : Vec<uint>, ks : Ks) -> GenPile<Ks> {
        GenPile { e : l, k : ks }
    }

    pub fn iter(n : uint, ks : Ks) -> GenPile<Ks> {
        let sz = ks.size();
        let l = Vec::from_fn(sz, |idx| if idx == 0 { n } else { 0 });
        GenPile { e : l, k : ks }
    }

    pub fn subsets(&self, n : uint) -> Vec<GenPile<Ks>> {
        MultiSubSetIterator::new(self.e.as_slice(), n)
            .map(|e| GenPile { e:e, k:self.k })
            .collect()
    }
}

impl KvMap<uint, GenPileKeys> for GenPile<GenPileKeys> {
    fn keys(&self) -> GenPileKeys { self.k }

    fn get(&self, k: uint) -> uint { 
        *self.e.get(k)
    }

    fn has(&self, other: &GenPile<GenPileKeys>) -> bool {
        self.e.iter().zip(other.e.iter()).all(|(&i0, &i1)| i0 >= i1)
    }

    fn add(&self, other : &GenPile<GenPileKeys>) -> GenPile<GenPileKeys> {
        assert!(self.k == other.k);
        GenPile::new(self.e.iter().zip(other.e.iter()).map(|(&i0, &i1)| i0+i1).collect(),
                     self.k)
    }

    fn sub(&self, other : &GenPile<GenPileKeys>) -> GenPile<GenPileKeys> {
        assert!(self.k == other.k);
        assert!(self.has(other));
        GenPile::new(self.e.iter().zip(other.e.iter()).map(|(&i0, &i1)| i0-i1).collect(),
                     self.k)
    }        
}

impl fmt::Show for GenPile<GenPileKeys> {
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

impl Iterator<GenPile<GenPileKeys>> for GenPile<GenPileKeys> {

    fn next(&mut self) -> Option<GenPile<GenPileKeys>> {
        let res = GenPile { e: self.e.clone(), k : self.k };

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

impl LandPile for GenPile<GenPileKeys> {
    fn lands(&self) -> uint { 
        self.e.iter().enumerate()
            .map(|(i, v)| if (self.k.is_land)(i) { *v } else { 0 }).sum()
    }

    fn spells(&self) -> uint { 
        self.total() - self.lands() 
    }
}

#[deriving(Clone, Show)]
pub enum Colored { C, N, S }
struct ColoredKeys;

impl Keys<Colored> for ColoredKeys {
    fn size(&self) -> uint { 3 }
    fn to_uint(&self, w:Colored) -> uint { w as uint }
    fn from_uint(&self, n: uint) -> Colored {
        match n { 0 => C, 1 => N, 2 => S, _ => fail!("out of range") }
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

impl KvMap<Colored, ColoredKeys> for ColoredPile {
    fn keys(&self) -> ColoredKeys { ColoredKeys }
    
    fn get(&self, k: Colored) -> uint { 
        self.e[ColoredKeys.to_uint(k)]
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

struct DualPileKeys;

impl Keys<uint> for DualPileKeys {
    fn size(&self) -> uint { 5 }
    fn to_uint(&self, c: uint) -> uint { c }
    fn from_uint(&self, n: uint) -> uint { if n < 5 { n } else { fail!("out of range") } }
}

impl DualPile {
    pub fn new(a: uint, b: uint, ab: uint, x: uint, s: uint) -> DualPile {
        DualPile { a:a, b:b, ab:ab, x:x, s:s }
    }

    pub fn iter(d: uint) -> DualPile {
        DualPile { a:d, b:0, ab:0, x:0, s:0 }
    }
}

impl KvMap<uint, DualPileKeys> for DualPile {
    fn keys(&self) -> DualPileKeys { DualPileKeys }

    fn get(&self, k: uint) -> uint { match k { 0 => self.a,
                                               1 => self.b,
                                               2 => self.ab, 
                                               3 => self.x,
                                               4 => self.s,
                                               _ => fail!("out of range") } }

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

// mset = [0,1,2,2,2,3,3]
// nums = [0,1,2,2]
pub fn mc_next(mset: &[uint], nums: &mut[uint]) -> bool {
    let n = mset.len();
    let k = nums.len();
    let mut finished = false;
    let mut changed = false;

    let mut i = k - 1;
    while !finished && !changed {
        if nums[i] < mset[i + (n - k)] { // 7-4 = 3
            // successor
            let mut j = 0;
            while mset[j] <= nums[i] { j += 1 }; // mset[j] > nums[i]
            // replace
            nums[i] = mset[j];
            if i < k - 1 {
                let mut l = i + 1;
                j += 1;
                while l < k {
                    nums[l] = mset[j];
                    l += 1;
                    j += 1
                }
            }
            changed = true;
        }
        finished = i == 0;
        i -= 1;
    }

    if !changed {
        for i in range(0, k) {
            nums[i] = mset[i]
        }
    }

    changed
}
