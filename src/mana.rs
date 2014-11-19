use colors::Color;
use colors::Color::{W,U,B,R,G,C};

#[deriving(Show, PartialEq, Eq, Clone)]
pub struct Mana {
    pub w : uint,
    pub u : uint,
    pub b : uint,
    pub r : uint,
    pub g : uint,
    pub c : uint
}

impl Mana {
    pub fn new(w:uint, u:uint, b:uint, r:uint, g:uint, c:uint) -> Mana {
        Mana { w : w, u : u, b : b, r : r, g : g, c : c }
    }

    pub fn zero() -> Mana { 
        Mana::new(0, 0, 0, 0, 0, 0) 
    }

    pub fn w(n : uint) -> Mana { Mana::new(n, 0, 0, 0, 0, 0) }
    pub fn u(n : uint) -> Mana { Mana::new(0, n, 0, 0, 0, 0) }
    pub fn b(n : uint) -> Mana { Mana::new(0, 0, n, 0, 0, 0) }
    pub fn r(n : uint) -> Mana { Mana::new(0, 0, 0, n, 0, 0) }
    pub fn g(n : uint) -> Mana { Mana::new(0, 0, 0, 0, n, 0) }
    pub fn c(n : uint) -> Mana { Mana::new(0, 0, 0, 0, 0, n) }

    pub fn reset(&self, color: Color) -> Mana {
        match color { 
            W => Mana::new(0, self.u, self.b, self.r, self.g, self.c),
            U => Mana::new(self.w, 0, self.b, self.r, self.g, self.c),
            B => Mana::new(self.w, self.u, 0, self.r, self.g, self.c),
            R => Mana::new(self.w, self.u, self.b, 0, self.g, self.c),
            G => Mana::new(self.w, self.u, self.b, self.r, 0, self.c),
            C => Mana::new(self.w, self.u, self.b, self.r, self.g, 0),
        }
    }

    pub fn as_vec(&self) -> Vec<uint> {
        vec![self.w, self.u, self.b, self.r, self.g, self.c]
    }
    
    pub fn cmc(&self) -> uint { 
        use std::iter::AdditiveIterator;

        let v = self.as_vec();
        v.iter().map(|&x| x).sum()
    }

    pub fn show(&self) -> String { 
        format!("({:2},{:2},{:2},{:2},{:2},{:2})",
                self.w, self.u, self.b, self.r, self.g, self.c)
    }
    
    pub fn pretty(&self) -> String {
        let ns = if self.c > 0 { self.c.to_string() } else { "".to_string() };
        format!("{}{}{}{}{}{}",
                ns, 
                "W".repeat(self.w as uint),
                "U".repeat(self.u as uint),
                "B".repeat(self.b as uint),
                "R".repeat(self.r as uint),
                "G".repeat(self.g as uint))
    }

    pub fn src(&self) -> String {
        fn f(v:uint, l:char, spc:bool) -> String {
            if v > 0 {
			    format!("{}{:2u}{}", if spc { " " } else { "" }, v, l)
		    }
		    else { 
                (if spc {"    "} else {"   "}).to_string()
            }
	    }        

        let w = f(self.w, 'W', false);
        let u = f(self.u, 'U', true);
        let b = f(self.b, 'B', true);
        let r = f(self.r, 'R', true);
        let g = f(self.g, 'G', true);
        let x = f(self.c, 'X', true);

        format!("{}{}{}{}{}{}", w, u, b, r, g, x)
    }

    pub fn parse(s: &str) -> Mana {
        use regex::Regex;
        
        let re = match Regex::new(r"\{([0-9]+|W|U|B|R|G)\}") {
            Ok(r) => r,
            Err(e) => panic!("{}", e)
        };
        let mut mana = Mana::zero();
        for cap in re.captures_iter(s) {
            let m = match cap.at(1) {
                "W" => Mana::w(1),
                "U" => Mana::u(1),
                "B" => Mana::b(1),
                "R" => Mana::r(1),
                "G" => Mana::g(1),
                n => {
                    let v = from_str::<uint>(n);
                    Mana::c(v.unwrap_or(0u))
                }
            };
            mana = mana.add(&m)
        }
        mana
    }
}

// impl Zero<Mana> for Mana {
//     fn zero() -> Self { Mana([0, 0, 0, 0, 0, 0]) }
//     fn is_zero(&self) -> bool { 
//         match *self { Mana([0, 0, 0, 0, 0, 0]) => true, _ => false }
//     }
// }

impl Add<Mana, Mana> for Mana {
    fn add(&self, b: &Mana) -> Mana {
        let a = self;
        Mana::new(a.w + b.w, a.u + b.u, a.b + b.b,
                  a.r + b.r, a.g + b.g, a.c + b.c)
    }
}

impl Sub<Mana, Mana> for Mana {
    fn sub(&self, b: &Mana) -> Mana {
        let a = self;
        Mana::new(a.w - b.w, a.u - b.u, a.b - b.b,
                  a.r - b.r, a.g - b.g, a.c - b.c)
    }
}

impl Mana {
    pub fn mul(&self, rhs: uint) -> Mana {
        let a = self;
        let k = rhs;
        Mana::new(a.w * k, a.u * k, a.b * k,
                  a.r * k, a.g * k, a.c * k)
    }
}
