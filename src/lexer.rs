use crate::{Token};
use crate::util::{ResetIterator};
use crate::scanner::Scanner;

/// =============================================================================
/// Lexer
/// =============================================================================

pub struct Lexer<I:Iterator,S:Scanner> {
    iter: ResetIterator<I>,
    rules: S,
    offset: usize
}

impl<I:Iterator,S:Scanner> Lexer<I,S> {
    pub fn new(iter: I, rules: S) -> Self {
        let iter = ResetIterator::new(iter);
        Self{iter,rules, offset:0}
    }
}

impl<I:Iterator,S:Scanner<Item=I::Item>> Iterator for Lexer<I,S>
where I::Item: Copy
{
    type Item = Token<S::Token>;
    
    fn next(&mut self) -> Option<Self::Item> {
        // Compute start offset
        let start = self.iter.offset();
        // See what we've got
        match self.rules.scan(&mut self.iter) {
            Some(t) => {               
                // Compute end offset
                let end = self.iter.offset();
                // Reset iterator
                self.iter.reset();                
                // Done
                Some(Token::new(t,start..end))
            }
            None => None
        }
    }
}
