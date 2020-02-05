//
// First probabilities
//

pub fn perc(x: f64) -> f64 {
    100.0 * x
}

pub fn pow(mut base: usize, mut exp: usize) -> usize {
    let mut acc = 1;

    if exp == 0 {
        return acc;
    }

    loop {
        if (exp & 1) == 1 {
            acc *= base
        }
        exp /= 2;

        if exp == 0 {
            return acc;
        }
        base = base * base
    }
}

//
// c(n, k), number of ways of combining `k` elements out of `n`
//
// E.g.: c(3, 2) = 3, c(4, 2) = 6.
//
pub fn c(n: u64, k: u64) -> f64 {
    if k > n {
        return 0.0;
    } else if k == 0 || k == n {
        return 1.0;
    } else {
        let k: u64 = if k + k > n { n - k } else { k };

        let mut res: f64 = 1.0;

        for j in 0..k {
            let num: f64 = (n - j) as f64;
            let den: f64 = (j + 1) as f64;
            res = res * num / den;
        }

        return res;
    }
}

pub fn ch(n: usize, k: usize) -> usize {
    if k > n {
        return 0;
    } else if k == 0 || k == n {
        return 1;
    } else {
        let k = if 2 * k > n { n - k } else { k };

        (0..k).fold(1usize, |c, j| {
            let num = n - j;
            let den = j + 1;
            c * num / den
        })
    }
}

//
// Given n0 red balls and n1 white balls, the chance of drawing k0 red balls and k1 white balls.
//
pub fn h(n0: u64, k0: u64, n1: u64, k1: u64) -> f64 {
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

fn h3(num: [u64; 3], den: [u64; 3]) -> f64 {
    let n_t = num[0] + num[1] + num[2];
    let k_t = den[0] + den[1] + den[2];
    let c_t = c(num[0], den[0]) * c(num[1], den[1]) * c(num[2], den[2]);
    c_t / c(n_t, k_t)
}

pub fn hyp(n: usize, f: impl Fn(usize) -> (usize, usize)) -> f64 {
    let (n_t, k_t, c_t) = (0..n).fold((0, 0, 1.0), |(n_t, k_t, c_t), i| {
        let (n, k) = f(i);
        (n_t + n, k_t + k, c_t * c(n as u64, k as u64))
    });
    c_t / c(n_t as u64, k_t as u64)
}

pub fn when<F>(cond: bool, what: F) -> f64
where
    F: Fn() -> f64,
{
    if cond {
        what()
    } else {
        0.0
    }
}

pub fn cond(c: bool) -> f64 {
    if c {
        1.0
    } else {
        0.0
    }
}
