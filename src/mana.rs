use crate::colors::Color;
use crate::colors::Color::{W,U,B,R,G,C};
use std::ops::{Add, Sub, Mul};
use std::iter::repeat;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Mana {
    pub w: u32,
    pub u: u32,
    pub b: u32,
    pub r: u32,
    pub g: u32,
    pub c: u32, // colorless
    pub n: u32, // generic
    pub x: u32  // X i.e. some not determined value
}

impl Mana {
    pub fn new(w:u32, u:u32, b:u32, r:u32, g:u32, c:u32, n: u32, x: u32) -> Mana {
        Mana { w: w, u: u, b: b, r: r, g: g, c: c, n: n, x: x }
    }

    pub fn zero() -> Mana {
        Mana::new(0, 0, 0, 0, 0, 0, 0, 0)
    }

    pub fn w(n: u32) -> Mana { Mana::new(n, 0, 0, 0, 0, 0, 0, 0) }
    pub fn u(n: u32) -> Mana { Mana::new(0, n, 0, 0, 0, 0, 0, 0) }
    pub fn b(n: u32) -> Mana { Mana::new(0, 0, n, 0, 0, 0, 0, 0) }
    pub fn r(n: u32) -> Mana { Mana::new(0, 0, 0, n, 0, 0, 0, 0) }
    pub fn g(n: u32) -> Mana { Mana::new(0, 0, 0, 0, n, 0, 0, 0) }
    pub fn c(n: u32) -> Mana { Mana::new(0, 0, 0, 0, 0, n, 0, 0) }
    pub fn n(n: u32) -> Mana { Mana::new(0, 0, 0, 0, 0, 0, n, 0) }
    pub fn x(n: u32) -> Mana { Mana::new(0, 0, 0, 0, 0, 0, 0, n) }

    pub fn reset(&self, color: Color) -> Mana {
        match color {
            W => Mana::new(0, self.u, self.b, self.r, self.g, self.c, self.n, self.x),
            U => Mana::new(self.w, 0, self.b, self.r, self.g, self.c, self.n, self.x),
            B => Mana::new(self.w, self.u, 0, self.r, self.g, self.c, self.n, self.x),
            R => Mana::new(self.w, self.u, self.b, 0, self.g, self.c, self.n, self.x),
            G => Mana::new(self.w, self.u, self.b, self.r, 0, self.c, self.n, self.x),
            C => Mana::new(self.w, self.u, self.b, self.r, self.g, 0, self.n, self.x),
            //X => Mana::new(self.w, self.u, self.b, self.r, self.g, self.c, 0, self.x),
        }
    }

    pub fn as_vec(&self) -> Vec<u32> {
        vec![self.w, self.u, self.b, self.r, self.g, self.c, self.n, self.x]
    }

    pub fn cmc(&self) -> u32 {
        self.w + self.u + self.b + self.r + self.g + self.c + self.n
    }

    pub fn show(&self) -> String {
        format!("({:2},{:2},{:2},{:2},{:2},{:2},{:2},{:2})",
                self.w, self.u, self.b, self.r, self.g, self.c, self.n, self.x)
    }

    pub fn pretty(&self) -> String {
        let ns = if self.n > 0 { self.n.to_string() } else { "".to_string() };
        format!("{}{}{}{}{}{}{}{}",
                repeat('X').take(self.x as usize).collect::<String>(),
                ns,
                repeat('W').take(self.w as usize).collect::<String>(),
                repeat('U').take(self.u as usize).collect::<String>(),
                repeat('B').take(self.b as usize).collect::<String>(),
                repeat('R').take(self.r as usize).collect::<String>(),
                repeat('G').take(self.g as usize).collect::<String>(),
                repeat("C").take(self.c as usize).collect::<String>()
                )
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
        let n = f(self.n, 'N', true);

        format!("{}{}{}{}{}{}{}", w, u, b, r, g, c, n)
    }

    pub fn parse(s: &str) -> Mana {
        use regex::Regex;

        let re = Regex::new(r"\{([0-9]+|X|W|U|B|R|G|C)\}").unwrap();
        // Want to panic!() here if regex is wrong

        let mut mana = Mana::zero();
        for cap in re.captures_iter(s) {
            let m = match &cap[1] {
                "W" => Mana::w(1),
                "U" => Mana::u(1),
                "B" => Mana::b(1),
                "R" => Mana::r(1),
                "G" => Mana::g(1),
                "C" => Mana::c(1),
                "X" => Mana::x(1),
                n => {
                    let v = n.parse::<u32>();
                    Mana::n(v.unwrap_or(0))
                }
            };
            mana = mana.add(m)
        }

        //println!("{:20} => {:20} :: {:20}", s, mana.show(), mana.pretty());

        mana
    }
}

impl Add for Mana {
    type Output = Mana;

    fn add(self, b: Mana) -> Mana {
        let a = self;
        Mana::new(a.w + b.w, a.u + b.u, a.b + b.b,
                  a.r + b.r, a.g + b.g, a.c + b.c,
                  a.n + b.n, a.x + b.x)
    }
}

impl Sub for Mana {
    type Output = Mana;

    fn sub(self, b: Mana) -> Mana {
        let a = self;
        Mana::new(a.w - b.w, a.u - b.u, a.b - b.b,
                  a.r - b.r, a.g - b.g, a.c - b.c,
                  a.n - b.n, a.x + b.x)
    }
}

impl Mul<u32> for Mana {
    type Output = Mana;

    fn mul(self, rhs: u32) -> Mana {
        let a = self;
        let k = rhs;
        Mana::new(a.w * k, a.u * k, a.b * k,
                  a.r * k, a.g * k, a.c * k,
                  a.n * k, a.x * k)
    }
}
