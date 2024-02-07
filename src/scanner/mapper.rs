use std::marker::PhantomData;
use super::Scanner;
use crate::util::{LookaheadIterator};

pub struct Mapping<T,I,F>
where I:Iterator, F:Fn(&I::Item)->Option<T> {
    matcher: F,
    _token: PhantomData<T>,
    _iter: PhantomData<I>
}

impl<T,I,F> Mapping<T,I,F>
where I:Iterator, F:Fn(&I::Item)->Option<T> {
    pub fn new(matcher: F) -> Self {
        Self{matcher,_token: PhantomData, _iter: PhantomData}
    }
}

impl<T,I:Iterator,F> Scanner for Mapping<T,I,F>
where F:Fn(&I::Item)->Option<T> {
    type Token = T;
    type Iter = I;

    fn scan(&self,input: &mut LookaheadIterator<I>) -> Option<Self::Token> {
        match input.lookahead(0) {
            None => None,
            Some(t) => (self.matcher)(t)
        }
    }
}
