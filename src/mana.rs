use colors::*;

pub struct Mana([uint, ..6]);

impl Mana {
    pub fn zero() -> Mana { 
        Mana([0, 0, 0, 0, 0, 0]) 
    }

    pub fn new(w:uint, u:uint, b:uint, r:uint, g:uint, x:uint) -> Mana {
        Mana([w, u, b, r, g, x])
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
            G => Mana([v[0], v[1], v[2], v[3], 0, v[5]]),
            C => Mana([v[0], v[1], v[2], v[3], v[4], 0]),
        }
    }
    
    pub fn cmc(&self) -> uint { 
        use std::iter::AdditiveIterator;

        let Mana(v) = *self;
        v.iter().map(|&x| x).sum()
    }

    pub fn show(&self) -> String { 
        let Mana(v) = *self;
        format!("({:2}, {:2}, {:2}, {:2}, {:2}, {:2})",
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

        fn f(v:uint, l:char, spc:bool) -> String {
            if v > 0 {
			    format!("{}{:2u}{}", if spc { " " } else { "" }, v, l)
		    }
		    else { 
                (if spc {"    "} else {"   "}).to_string()
            }
	    }        

        let w = f(v[0], 'W', false);
        let u = f(v[1], 'U', true);
        let b = f(v[2], 'B', true);
        let r = f(v[3], 'R', true);
        let g = f(v[4], 'G', true);
        let x = f(v[5], 'X', true);

        format!("{}{}{}{}{}{}", w, u, b, r, g, x)
    }
}

// impl Zero<Mana> for Mana {
//     fn zero() -> Self { Mana([0, 0, 0, 0, 0, 0]) }
//     fn is_zero(&self) -> bool { 
//         match *self { Mana([0, 0, 0, 0, 0, 0]) => true, _ => false }
//     }
// }

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
    pub fn mul(&self, rhs: uint) -> Mana {
        let Mana(a) = *self;
        let k = rhs;
        Mana([a[0] * k, a[1] * k, a[2] * k,
              a[3] * k, a[4] * k, a[5] * k])
    }
}
