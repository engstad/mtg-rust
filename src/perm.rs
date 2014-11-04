#[deriving(Show)]
pub struct MultiSubSetIterator<'a> {
    ns : &'a [uint],
    stack : Vec<Stack>
}

#[deriving(Clone, Show)]
struct Stack { 
    k : uint,       // how many left to add
    n : uint,       // how many left to chose from
    l : uint,       // where we are next
    a : Vec<uint>
}

impl<'a> MultiSubSetIterator<'a> {
    pub fn new(ns: &'a [uint], k: uint) -> MultiSubSetIterator<'a> {
        let l = ns.len();
        let n = ns.iter().fold(0, |a,&b| a+b);
        let a = Vec::from_elem(ns.len(), 0u);
        MultiSubSetIterator { 
            ns : ns.clone(), 
            stack : Vec::from_elem(1, Stack{k:k, l:l, n:n, a:a})               
        }
    }
}

impl<'a> Iterator<Vec<uint>> for MultiSubSetIterator<'a> {
    fn next(&mut self) -> Option<Vec<uint>> {
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
    for it in MultiSubSetIterator::new(vec![2,4,1].as_slice(), 2) {
        println!("{}", it);
    }
    
    println!("---------------------------------------------------");
}
