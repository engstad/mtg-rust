use std::iter::repeat;

#[derive(Show)]
pub struct MultiSubSetIterator<'a> {
    ns : &'a [usize],
    stack : Vec<Stack>
}

#[derive(Clone, Show)]
struct Stack { 
    k : usize,       // how many left to add
    n : usize,       // how many left to chose from
    l : usize,       // where we are next
    a : Vec<usize>
}

impl<'a> MultiSubSetIterator<'a> {
    pub fn new(ns: &'a [usize], k: usize) -> MultiSubSetIterator<'a> {
        let l = ns.len();
        let n = ns.iter().fold(0, |a,&b| a+b);
        let a = repeat(0).take(ns.len()).collect::<_>();
        MultiSubSetIterator { 
            ns : ns.clone(), 
            stack : vec![Stack{k:k, l:l, n:n, a:a}]
        }
    }
}

impl<'a> Iterator for MultiSubSetIterator<'a> {
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Vec<usize>> {
        loop {
            match self.stack.pop() {
                None => return None,
                Some(top) => {
                    let k = top.k;
                    if k == 0 { 
                        return Some(top.a.clone())
                    }
                    else {
                        let l = top.l;
                        let n = top.n;
                        let t = l - 1;
                        let m = self.ns[t];
                        
                        let s = if k + m > n { k + m - n } else { 0 };
                        let e = if k < m { k } else { m };
                        
                        for i in range(s, e + 1).rev() {
                            let mut na = top.a.clone();
                            na[t] = i;
                            let new_top = Stack{ k:k-i, l:t, n:n-m, a:na };
                            self.stack.push(new_top);
                        }
                    }        
                }
            }
        }
    }
}

#[test]
pub fn test_gen() {
    for it in MultiSubSetIterator::new(vec![2,4,1], 2) {
        println!("{}", it);
    }
    
    println!("---------------------------------------------------");
}
