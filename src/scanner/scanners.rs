use super::Scanner;
use crate::util::{LookaheadIterator};

/// A scanner which matching a single item with a given token.
pub struct UnitScanner<T,I:Iterator>(pub I::Item, pub T);

impl<T:Copy,I:Iterator> Scanner<I> for UnitScanner<T,I>
where I::Item: PartialEq {
    type Token = T;

    fn scan(&self,input: &mut LookaheadIterator<I>) -> Option<Self::Token> {
        if input.lookahead(0) == Some(&self.0) {
            input.skip(1);
            Some(self.1)
        } else {
            None
        }        
    }
}

pub struct BlockScanner<T,I:Iterator>(pub fn(&I::Item)->bool, pub T);

impl<T:Copy,I:Iterator> Scanner<I> for BlockScanner<T,I>
where I::Item: PartialEq {
    type Token = T;

    fn scan(&self,input: &mut LookaheadIterator<I>) -> Option<Self::Token> {
        // Search forward
        let n = input.slice_while(self.0).len();
        // Take items
        input.skip(n);
        // Determine Result
        if n > 0 { Some(self.1) } else { None }        
    }
}
