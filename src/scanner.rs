use super::Matcher;
use crate::util::{ResetIterator};

pub trait Scanner {
    type Item;
    type Token;
    
    fn scan<I:Iterator<Item=Self::Item>>(&self,input: &mut ResetIterator<I>) -> Option<Self::Token>;

    /// Combine two scanners together.
    fn or<Rhs:Scanner>(self, other: Rhs) -> (Self,Rhs) where Self:Sized { (self,other) }
}

impl<A:Scanner,B:Scanner<Item=A::Item,Token=A::Token>> Scanner for (A,B) {
    type Item = A::Item;
    type Token = A::Token;
    
    fn scan<I:Iterator<Item=Self::Item>>(&self,input: &mut ResetIterator<I>) -> Option<Self::Token> {
        match self.0.scan(input) {
            Some(t) => Some(t),
            None => self.1.scan(input)
        }
    }
}

/// A scanner which matches a single item with a given token.  This
/// is one of the fundamental building blocks for most scanners.
pub struct Unit<M:Matcher,T>(pub M, pub T);

impl<M:Matcher,T:Copy> Scanner for Unit<M,T> {
    type Item = M::Item;
    type Token = T;

    fn scan<I:Iterator<Item=Self::Item>>(&self,input: &mut ResetIterator<I>) -> Option<Self::Token> {
        match self.0.matches(input) {
            false => None,
            true => Some(self.1)
        }
    }
}
