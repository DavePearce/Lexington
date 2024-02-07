use crate::util::{LookaheadIterator};

pub trait Scanner {
    type Item;
    type Token;
    
    fn scan<I:Iterator<Item=Self::Item>>(&self,input: &mut LookaheadIterator<I>) -> Option<Self::Token>;

    /// Combine two scanners together.
    fn or<Rhs:Scanner>(self, other: Rhs) -> (Self,Rhs) where Self:Sized { (self,other) }
}

impl<A:Scanner,B:Scanner<Item=A::Item,Token=A::Token>> Scanner for (A,B) {
    type Item = A::Item;
    type Token = A::Token;
    
    fn scan<I:Iterator<Item=Self::Item>>(&self,input: &mut LookaheadIterator<I>) -> Option<Self::Token> {
        match self.0.scan(input) {
            Some(t) => Some(t),
            None => self.1.scan(input)
        }
    }
}
