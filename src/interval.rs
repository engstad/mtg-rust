use std::num::Int;

#[deriving(Clone)]
enum IntervalType { Open, Closed, Range, OpenClosed }

#[deriving(Clone)]
pub struct Interval<A> {
    start : A,
    end   : A,
    rtype : IntervalType
}

#[inline]
fn minmax<A:Ord>(a: A, b: A) -> (A, A) {
    if a < b { (a, b) } else { (b, a) }
}

// (a,b)
pub fn open<A:Ord>(a: A, b: A) -> Interval<A> {
    let (min, max) = minmax(a, b);
    Interval { start:min, end:max, rtype:IntervalType::Open }
}

// [a,b]
pub fn closed<A:Ord>(a: A, b: A) -> Interval<A> {
    let (min, max) = minmax(a, b);
    Interval { start:min, end:max, rtype:IntervalType::Closed }
}

// [a,b)
pub fn range<A:Ord>(a: A, b: A) -> Interval<A> {
    let (min, max) = minmax(a, b);
    Interval { start:min, end:max, rtype:IntervalType::Range }
}

// (a,b]
pub fn open_closed<A:Ord>(a: A, b: A) -> Interval<A> {
    let (min, max) = minmax(a, b);
    Interval { start:min, end:max, rtype:IntervalType::OpenClosed }
}

impl<A : Ord + Add<A,A> + Clone + Int> Interval<A> {
    pub fn iter<'a>(&'a self) -> IntervalIter<'a, A> {
        match self.rtype {
            IntervalType::Range | IntervalType::Closed => IntervalIter { state : self.start.clone(), range : self },
            IntervalType::Open | IntervalType::OpenClosed => IntervalIter { state : self.start + Int::one(), range : self }
        }
    }
}

struct IntervalIter<'a, A:'a> {
    state: A,
    range: &'a Interval<A>,
}

impl<'a, A : Ord + Add<A,A> + Clone + Int> Iterator<A> for IntervalIter<'a,A> {
    fn next(&mut self) -> Option<A> {
        match self.range.rtype {
            IntervalType::Range | IntervalType::Open => 
                if self.state < self.range.end {
                    let result = self.state.clone();
                    self.state = self.state + Int::one();
                    Some(result)
                } else {
                    None
                },
            IntervalType::Closed | IntervalType::OpenClosed =>
                if self.state <= self.range.end {
                    let result = self.state.clone();
                    self.state = self.state + Int::one();
                    Some(result)
                } else {
                    None
                }
        }        
    }
}

pub fn test() {
    assert!(open(0i, 5).iter().collect::<Vec<int>>()        == vec![  1,2,3,4  ]);
    assert!(range(0i, 5).iter().collect::<Vec<int>>()       == vec![0,1,2,3,4  ]);
    assert!(open_closed(0i, 5).iter().collect::<Vec<int>>() == vec![  1,2,3,4,5]);
    assert!(closed(0i, 5).iter().collect::<Vec<int>>()      == vec![0,1,2,3,4,5]);
}
