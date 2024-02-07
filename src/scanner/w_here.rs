use super::Scanner;
use crate::util::{LookaheadIterator};

pub struct Where<T,I:Iterator>(pub fn(&I::Item)->bool, pub T);

impl<T:Copy,I:Iterator> Scanner for Where<T,I> {
    type Token = T;
    type Iter = I;

    fn scan(&self,input: &mut LookaheadIterator<I>) -> Option<Self::Token> {
        match input.lookahead(0) {
            Some(t) if (self.0)(t) => Some(self.1),
            _ => None
        }
    }
}
