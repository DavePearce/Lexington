use super::{Matcher,Scanner};
use crate::util::{LookaheadIterator};

/// A scanner which matches a single item with a given token.  This
/// is one of the fundamental building blocks for most scanners.
pub struct Unit<M:Matcher,T>(pub M, pub T);

impl<M:Matcher,T:Copy> Scanner for Unit<M,T> {
    type Item = M::Item;
    type Token = T;

    fn scan<I:Iterator<Item=Self::Item>>(&self,input: &mut LookaheadIterator<I>) -> Option<Self::Token> {
        // match self.0.matches(input) {
        //     0 => None,
        //     n => {
        //         input.skip(n);
        //         Some(self.1)
        //     }
        // }
        todo!()
    }
}
