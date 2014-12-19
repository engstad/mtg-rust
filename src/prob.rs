//
// First probabilities
//

pub fn perc(x:f64) -> f64 { 100.0 * x }

pub fn pow(x:int, n:int, acc:i64) -> i64
{
    if n == 0 {
	    return acc;
    }
	else 
	{
		let mut res = 1_i64;
		for _ in range(0, n)
		{
			res *= x as i64;
		}
		return res;
	}
}

//
// c(n, k), number of ways of combining `k` elements out of `n`
//
// E.g.: c(3, 2) = 3, c(4, 2) = 6.
//

pub fn c(n:uint, k:uint) -> f64 {
	if /*k < 0 ||*/ k > n { 
        return 0.0;
    }
	else if k == 0 || k == n {
		return 1.0;
    }
	else {
		let k:uint = if k + k > n { n - k } else { k };

		let mut res:f64 = 1.0;

		for j in range(0u, k)
		{
			let num:f64 = (n - j).to_f64().unwrap();
			let den:f64 = (j + 1).to_f64().unwrap();
			res = (res * num) / den;
		}

		return res;
  	}
}


pub fn ch(n:uint, k:uint) -> uint {
	if /*k < 0 ||*/ k > n { 
        return 0;
    }
	else if k == 0 || k == n {
		return 1;
    }
	else {
		let k:uint = if k + k > n { n - k } else { k };

		let mut res = 1u;

		for j in range(0u, k)
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
pub fn h(n0 : uint, k0 : uint, n1 : uint, k1 : uint) -> f64
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

fn h3(num: [uint, ..3], den: [uint, ..3]) -> f64
{
    let n_t = num[0] + num[1] + num[2];
    let k_t = den[0] + den[1] + den[2];
    let c_t = c(num[0], den[0]) * c(num[1], den[1]) * c(num[2], den[2]);
    c_t / c(n_t, k_t)
}

pub fn when(cond: bool, what: || -> f64) -> f64 {
    if cond { 
        what()
    } else {
        0.0
    }
}

pub fn cond(c: bool) -> f64 {
    if c { 1.0 } else { 0.0 }
}
