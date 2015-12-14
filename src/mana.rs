use colors::Color;
use colors::Color::{W,U,B,R,G,C};
use std::ops::{Add, Sub, Mul};
use std::iter::repeat;
use pile::sum;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Mana {
    pub w : u32,
    pub u : u32,
    pub b : u32,
    pub r : u32,
    pub g : u32,
    pub c : u32,
    pub x : u32
}

impl Mana {
    pub fn new(w:u32, u:u32, b:u32, r:u32, g:u32, c:u32, x: u32) -> Mana {
        Mana { w : w, u : u, b : b, r : r, g : g, c : c, x : x }
    }

    pub fn zero() -> Mana { 
        Mana::new(0, 0, 0, 0, 0, 0, 0) 
    }

    pub fn w(n : u32) -> Mana { Mana::new(n, 0, 0, 0, 0, 0, 0) }
    pub fn u(n : u32) -> Mana { Mana::new(0, n, 0, 0, 0, 0, 0) }
    pub fn b(n : u32) -> Mana { Mana::new(0, 0, n, 0, 0, 0, 0) }
    pub fn r(n : u32) -> Mana { Mana::new(0, 0, 0, n, 0, 0, 0) }
    pub fn g(n : u32) -> Mana { Mana::new(0, 0, 0, 0, n, 0, 0) }
    pub fn c(n : u32) -> Mana { Mana::new(0, 0, 0, 0, 0, n, 0) }
    pub fn x(n : u32) -> Mana { Mana::new(0, 0, 0, 0, 0, 0, n) }

    pub fn reset(&self, color: Color) -> Mana {
        match color { 
            W => Mana::new(0, self.u, self.b, self.r, self.g, self.c, self.x),
            U => Mana::new(self.w, 0, self.b, self.r, self.g, self.c, self.x),
            B => Mana::new(self.w, self.u, 0, self.r, self.g, self.c, self.x),
            R => Mana::new(self.w, self.u, self.b, 0, self.g, self.c, self.x),
            G => Mana::new(self.w, self.u, self.b, self.r, 0, self.c, self.x),
            C => Mana::new(self.w, self.u, self.b, self.r, self.g, 0, self.x),
            //X => Mana::new(self.w, self.u, self.b, self.r, self.g, self.c, 0),
        }
    }

    pub fn as_vec(&self) -> Vec<u32> {
        vec![self.w, self.u, self.b, self.r, self.g, self.c]
    }
    
    pub fn cmc(&self) -> u32 { 
        let v = self.as_vec();
        sum(v.iter().map(|&x| x))
    }

    pub fn show(&self) -> String { 
        format!("({:2},{:2},{:2},{:2},{:2},{:2})",
                self.w, self.u, self.b, self.r, self.g, self.c)
    }
    
    pub fn pretty(&self) -> String {
        let ns = if self.c > 0 { self.c.to_string() } else { "".to_string() };
        format!("{}{}{}{}{}{}{}",
                ns, 
                repeat('X').take(self.x as usize).collect::<String>(),
                repeat('W').take(self.w as usize).collect::<String>(),
                repeat('U').take(self.u as usize).collect::<String>(),
                repeat('B').take(self.b as usize).collect::<String>(),
                repeat('R').take(self.r as usize).collect::<String>(),
                repeat('G').take(self.g as usize).collect::<String>())
    }

    pub fn src(&self) -> String {
        fn f(v:u32, l:char, spc:bool) -> String {
            if v > 0 {
			    format!("{}{:2}{}", if spc { " " } else { "" }, v, l)
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
        let c = f(self.c, 'C', true);

        format!("{}{}{}{}{}{}", w, u, b, r, g, c)
    }

    pub fn parse(s: &str) -> Mana {
        use regex::Regex;
        
        let re = match Regex::new(r"\{([0-9]+|X|W|U|B|R|G)\}") {
            Ok(r) => r,
            Err(e) => panic!("{:?}", e)
        };
        let mut mana = Mana::zero();
        for cap in re.captures_iter(s) {
            let m = match cap.at(1) {
                Some("W") => Mana::w(1),
                Some("U") => Mana::u(1),
                Some("B") => Mana::b(1),
                Some("R") => Mana::r(1),
                Some("G") => Mana::g(1),
                Some("X") => Mana::x(1),
                Some(n) => {
                    let v = n.parse::<u32>();
                    Mana::c(v.unwrap_or(0))
                },
                None => Mana::c(0)
            };
            mana = mana.add(m)
        }
        mana
    }
}

impl Add for Mana {
    type Output = Mana;

    fn add(self, b: Mana) -> Mana {
        let a = self;
        Mana::new(a.w + b.w, a.u + b.u, a.b + b.b,
                  a.r + b.r, a.g + b.g, a.c + b.c,
                  a.x + b.x)
    }
}

impl Sub for Mana {
    type Output = Mana;

    fn sub(self, b: Mana) -> Mana {
        let a = self;
        Mana::new(a.w - b.w, a.u - b.u, a.b - b.b,
                  a.r - b.r, a.g - b.g, a.c - b.c,
                  a.x - b.x)
    }
}

impl Mul<u32> for Mana {
    type Output = Mana;

    fn mul(self, rhs: u32) -> Mana {
        let a = self;
        let k = rhs;
        Mana::new(a.w * k, a.u * k, a.b * k,
                  a.r * k, a.g * k, a.c * k,
                  a.x * k)
    }
}
