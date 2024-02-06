use crate::util::{LookaheadIterator};

pub trait Scanner<I:Iterator> {
    type Token;
    
    fn scan(&self,input: &mut LookaheadIterator<I>) -> Option<Self::Token>;
}

impl<I:Iterator,A:Scanner<I>,B:Scanner<I,Token=A::Token>> Scanner<I> for &(A,B) {
    type Token = A::Token;
    
    fn scan(&self,input: &mut LookaheadIterator<I>) -> Option<Self::Token> {
        match self.0.scan(input) {
            Some(t) => Some(t),
            None => self.1.scan(input)
        }
    }
}
