use std::marker::PhantomData;
use super::Scanner;
use crate::util::{LookaheadIterator};

/// A scanner which matches the end of the input stream.
pub struct Eof<T,I>(pub T, pub PhantomData<I>);

impl<T:Copy,I:Iterator> Scanner for Eof<T,I> {
    type Token = T;
    type Iter = I;
    
    fn scan(&self,input: &mut LookaheadIterator<I>) -> Option<Self::Token> {
        match input.lookahead(0) {
            Some(_) => None,
            None => Some(self.0)
        }
    }
}
