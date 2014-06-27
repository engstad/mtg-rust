use std::num::One;

#[deriving(Clone)]
enum IntervalType { Open, Closed, ClosedOpen, OpenClosed }

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
    Interval { start:min, end:max, rtype:Open }
}

// [a,b]
pub fn closed<A:Ord>(a: A, b: A) -> Interval<A> {
    let (min, max) = minmax(a, b);
    Interval { start:min, end:max, rtype:Closed }
}

// [a,b)
pub fn closed_open<A:Ord>(a: A, b: A) -> Interval<A> {
    let (min, max) = minmax(a, b);
    Interval { start:min, end:max, rtype:ClosedOpen }
}

// (a,b]
pub fn open_closed<A:Ord>(a: A, b: A) -> Interval<A> {
    let (min, max) = minmax(a, b);
    Interval { start:min, end:max, rtype:OpenClosed }
}

impl<A : Ord + Add<A,A> + Clone + One> Interval<A> {
    pub fn iter<'a>(&'a self) -> IntervalIter<'a, A> {
        match self.rtype {
            ClosedOpen | Closed => IntervalIter { state : self.start.clone(), range : self },
            Open | OpenClosed => IntervalIter { state : self.start + One::one(), range : self }
        }
    }
}

struct IntervalIter<'a, A> {
    state: A,
    range: &'a Interval<A>,
}

impl<'a, A : Ord + Add<A,A> + Clone + One> Iterator<A> for IntervalIter<'a,A> {
    fn next(&mut self) -> Option<A> {
        match self.range.rtype {
            ClosedOpen | Open => 
                if self.state < self.range.end {
                    let result = self.state.clone();
                    self.state = self.state + One::one();
                    Some(result)
                } else {
                    None
                },
            Closed | OpenClosed =>
                if self.state <= self.range.end {
                    let result = self.state.clone();
                    self.state = self.state + One::one();
                    Some(result)
                } else {
                    None
                }
        }        
    }
}

pub fn test() {
    assert!(open(0i, 5).iter().collect::<Vec<int>>()        == vec![  1,2,3,4  ]);
    assert!(closed_open(0i, 5).iter().collect::<Vec<int>>() == vec![0,1,2,3,4  ]);
    assert!(open_closed(0i, 5).iter().collect::<Vec<int>>() == vec![  1,2,3,4,5]);
    assert!(closed(0i, 5).iter().collect::<Vec<int>>()      == vec![0,1,2,3,4,5]);
}
