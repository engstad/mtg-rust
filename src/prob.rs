use std::num::*;

//
// First probabilities
//

pub fn perc(x:f64) -> f64 { 100.0 * x }

pub fn pow_acc(x:i64, n:i64, acc:i64) -> i64
{
    if n == 0 {
	    return acc;
    }
	else 
	{
		let mut res:i64 = 1;
        for _ in (0..n)
        {
			res *= x
		}
		return res;
	}
}

//
// c(n, k), number of ways of combining `k` elements out of `n`
//
// E.g.: c(3, 2) = 3, c(4, 2) = 6.
//

pub fn c(n: u64, k: u64) -> f64 {
	if /*k < 0 ||*/ k > n { 
        return 0.0;
    }
	else if k == 0 || k == n {
		return 1.0;
    }
	else {
		let k: u64 = if k + k > n { n - k } else { k };

		let mut res:f64 = 1.0;

		for j in (0..k) {
			let num:f64 = (n - j).to_f64().unwrap();
			let den:f64 = (j + 1).to_f64().unwrap();
			res = (res * num) / den;
		}

		return res;
  	}
}


pub fn ch(n: u64, k: u64) -> u64 {
	if /*k < 0 ||*/ k > n { 
        return 0;
    }
	else if k == 0 || k == n {
		return 1;
    }
	else {
		let k: u64 = if k + k > n { n - k } else { k };

		let mut res = 1u64;

		for j in (0..k)
		{
			let num = n - j;
			let den = j + 1;
			res = (res * num) / den;
		}

		return res;
  	}
}


//
// Given n0 red balls and n1 white balls, the chance of drawing k0 red balls and k1 white balls.
//
pub fn h(n0 : u64, k0 : u64, n1 : u64, k1 : u64) -> f64
{
	//                      c(n0, k0) * c(n1, k1)
	// h(n0, k0, n1, k2) = -----------------------
	//                      c(n0 + n1, k0 + k1)
	//
	//                      n0|_k0 n1|_k1 (k0+k1)!
	//                   = ---------------------------
	//                       k0! k1! (n0+n1)|_(k0+k1)
	//
	return c(n0, k0) * c(n1, k1) / c(n0 + n1, k0 + k1);
}

fn h3(num: [u64; 3], den: [u64; 3]) -> f64
{
    let n_t = num[0] + num[1] + num[2];
    let k_t = den[0] + den[1] + den[2];
    let c_t = c(num[0], den[0]) * c(num[1], den[1]) * c(num[2], den[2]);
    c_t / c(n_t, k_t)
}

pub fn when<F>(cond: bool, what: F) -> f64 
    where F : Fn() -> f64
{
    if cond { 
        what()
    } else {
        0.0
    }
}

pub fn cond(c: bool) -> f64 {
    if c { 1.0 } else { 0.0 }
}
