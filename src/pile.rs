use std::ops::{Index, Add, Sub};
use std::fmt;
use prob;
use perm::MultiSubSetIterator;

//
// A Pile is like a key-value map
//
pub trait Pile : Add + Sub {
    type Key : Copy;

    fn all<'a>(&'a self) -> Vec<Self::Key>;
    fn get<'a>(&'a self, Self::Key) -> usize;
    fn num_keys(&self) -> usize;

    fn prob_draw(&self, draw: &Self) -> f64 {
        let (n, k, c) =
            self.all().iter()
            .map(|k| (self.get(*k), draw.get(*k)))
            .fold((0, 0, 1.0), 
                  |(n, k, c), (n_i, k_i)| 
                  (n + n_i, k + k_i, c * prob::c(n_i as u64, k_i as u64)));

        c / prob::c(n as u64, k as u64)
    }    

    fn total(&self) -> usize {
        self.all()
            .iter()
            .map(|&k| self.get(k)).sum()
    }

    fn has(&self, other: &Self) -> bool {
        self.all().iter()
            .map(|&k| (self.get(k), other.get(k)))
            .all(|(v0, v1)| v0 >= v1)
    }
}

pub trait LandPile {
    fn spells(&self) -> usize;
    fn lands(&self) -> usize;

    fn prob_land(&self, l:usize, s:usize) -> f64 {
        let ls = self.lands();
        let ss = self.spells();
        prob::h(ls as u64, l as u64, ss as u64, s as u64)
    }
}

#[derive(Copy, Clone)]
pub struct GenPileKeys {
    num_keys : usize,
    is_land  : fn(usize) -> bool
}

impl GenPileKeys {
    pub fn new(keys: usize, lands: fn(usize)->bool) -> GenPileKeys {
        GenPileKeys { num_keys : keys, is_land : lands }
    }
}

impl PartialEq for GenPileKeys {
    fn eq(&self, b : &GenPileKeys) -> bool {
        self.num_keys == b.num_keys &&
        (self.is_land as *const u8) == (b.is_land as *const u8)
    }
}

#[derive(Clone)]
pub struct GenPile {
    e: Vec<usize>,
    k: GenPileKeys
}

impl Pile for GenPile {
    type Key = usize;

    fn num_keys(&self) -> usize { self.k.num_keys }
    fn all<'a>(&'a self) -> Vec<usize> { (0 .. self.num_keys()).collect() }
    fn get(&self, k: usize) -> usize { 
        self.e[k]
    }
}

impl Add for GenPile {
    type Output = GenPile;

    fn add(self, other: Self) -> Self {
        assert!(self.k == other.k);
        GenPile::new(self.e.iter().zip(other.e.iter()).map(|(&i0, &i1)| i0+i1).collect(),
                     self.k)
    }
}

impl Sub for GenPile {
    type Output = GenPile;

    fn sub(self, other: Self) -> Self {
        assert!(self.k == other.k);
        assert!(self.has(&other));
        GenPile::new(self.e.iter().zip(other.e.iter()).map(|(&i0, &i1)| i0-i1).collect(),
                     self.k)
    }        
}

impl GenPile {
    pub fn new(l : Vec<usize>, ks : GenPileKeys) -> GenPile {
        GenPile { e : l, k : ks }
    }

    pub fn iter(&self, n : usize, ks : GenPileKeys) -> GenPile {
        let sz = self.num_keys();
        let l = (0 .. sz).map(|idx| if idx == 0 { n } else { 0 }).collect();
        GenPile { e : l, k : ks }
    }

    pub fn subsets(&self, n : usize) -> Vec<GenPile> {
        MultiSubSetIterator::new(&*self.e, n)
            .map(|e| GenPile { e:e, k:self.k })
            .collect()
    }

    fn has(&self, other: &Self) -> bool {
        self.e.iter().zip(other.e.iter()).all(|(&i0, &i1)| i0 >= i1)
    }
}

impl Index<usize> for GenPile {
    type Output = usize;

    #[inline]
    fn index<'a>(&'a self, i: usize) -> &'a usize {
        &self.e[i]
    }
}


impl fmt::Debug for GenPile {
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

impl Iterator for GenPile {
    type Item = GenPile;

    fn next(&mut self) -> Option<GenPile> {
        let res = GenPile { e: self.e.clone(), k : self.k };

        if self.e[0] > 0 {
            self.e[0] -= 1;
            self.e[1] += 1;
            Some(res)
        } else {
            let len = self.e.len();
            for i in (1 .. len-1) {
                if self.get(i) > 0 {
                    self.e[0] = self.get(i) - 1; 
                    self.e[i+1] += 1;
                    self.e[i] = 0;
                    return Some(res)
                }
            }
            if self.get(len - 1) > 0 {
                self.e[len-1] = 0;
                Some(res)
            } else {
                None
            }
        }
    }
}

impl LandPile for GenPile {
    fn lands(&self) -> usize { 
        self.e.iter().enumerate()
            .map(|(i, v)| if (self.k.is_land)(i) { *v } else { 0 }).sum()
    }

    fn spells(&self) -> usize { 
        self.total() - self.lands() 
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Colored { C, N, S }

#[derive(Copy, Clone)]
pub struct ColoredPile {
    e: [usize; 3]
}

impl ColoredPile {
    pub fn new(c: usize, n: usize, s: usize) -> ColoredPile {
        ColoredPile{ e:[c, n, s] }
    }

    pub fn iter(n: usize) -> ColoredPile {
        ColoredPile{ e:[n, 0, 0] }
    }

    pub fn colored(&self) -> usize { self.e[0] }

    fn to_usize(w:Colored) -> usize { w as usize }

    fn from_usize(n: usize) -> Option<Colored> {
        match n { 0 => Some(Colored::C), 
                  1 => Some(Colored::N), 
                  2 => Some(Colored::S), 
                  _ => None }
    }
}

impl Pile for ColoredPile {
    type Key = Colored;

    fn all(&self) -> Vec<Colored> { vec![Colored::C, Colored::N, Colored::S] }
    fn num_keys(&self) -> usize { 3 }

    fn get(&self, k: Colored) -> usize { 
        self.e[ColoredPile::to_usize(k)]
    }
}

impl Add for ColoredPile {
    type Output = ColoredPile;

    fn add(self, other: ColoredPile) -> ColoredPile {
        ColoredPile::new(self.e[0] + other.e[0], 
                         self.e[1] + other.e[1], 
                         self.e[2] + other.e[2])
    }
}

impl Sub for ColoredPile {
    type Output = ColoredPile;

    fn sub(self, other: ColoredPile) -> ColoredPile {
        assert!(self.has(&other));

        ColoredPile::new(self.e[0] - other.e[0], 
                         self.e[1] - other.e[1], 
                         self.e[2] - other.e[2])
    }        
}

impl LandPile for ColoredPile {
    fn lands(&self) -> usize { self.e[0] + self.e[1] }
    fn spells(&self) -> usize { self.total() - self.lands() }
}

impl Iterator for ColoredPile {
    type Item = ColoredPile;

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

impl fmt::Debug for ColoredPile {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {        
        write!(fmt, "(c:{:3}, x:{:3}, s:{:3})", 
               self.e[0], self.e[1], self.e[2])        
    }
}

#[derive(Copy, Clone)]
pub struct DualPile {
    pub a:  usize, 
    pub b:  usize, // non-colored lands
    pub ab: usize,
    pub x:  usize,
    pub s:  usize, // spells
}

impl DualPile {
    pub fn new(a: usize, b: usize, ab: usize, x: usize, s: usize) -> DualPile {
        DualPile { a:a, b:b, ab:ab, x:x, s:s }
    }

    pub fn iter(d: usize) -> DualPile {
        DualPile { a:d, b:0, ab:0, x:0, s:0 }
    }

    fn to_usize(&self, c: usize) -> usize { c }
    fn from_usize(&self, n: usize) -> usize { if n < 5 { n } else { panic!("out of range") } }
}

impl Pile for DualPile {
    type Key = usize;

    fn num_keys(&self) -> usize { 5 }

    fn all<'a>(&'a self) -> Vec<usize> { (0 .. self.num_keys()).collect() }

    fn get(&self, k: usize) -> usize { match k { 0 => self.a,
                                               1 => self.b,
                                               2 => self.ab, 
                                               3 => self.x,
                                               4 => self.s,
                                               _ => panic!("out of range") } }
}

impl Add for DualPile {
    type Output = DualPile;

    fn add(self, other: DualPile) -> DualPile {
        DualPile::new(self.a + other.a, 
                      self.b + other.b, 
                      self.ab + other.ab,
                      self.x + other.x,
                      self.s + other.s)
    }
}

impl Sub for DualPile {
    type Output = DualPile;

    fn sub(self, other: DualPile) -> DualPile {
        DualPile::new(self.a - other.a, 
                      self.b - other.b, 
                      self.ab - other.ab,
                      self.x - other.x,
                      self.s - other.s)
    }
}

impl LandPile for DualPile {
    fn spells(&self) -> usize { self.total() - self.s }
    fn lands(&self) -> usize { self.s }
}

impl Iterator for DualPile {
    type Item = DualPile;

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

impl fmt::Debug for DualPile {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {        
        write!(fmt, "[a:{:3}, b:{:3}, ab:{:3}, x:{:3}, l:{:3}]", 
               self.a, self.b, self.ab, self.x, self.s)        
    }
}

// mset = [0,1,2,2,2,3,3]
// nums = [0,1,2,2]
pub fn mc_next(mset: &[usize], nums: &mut[usize]) -> bool {
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
        for i in (0 .. k) {
            nums[i] = mset[i]
        }
    }

    changed
}
