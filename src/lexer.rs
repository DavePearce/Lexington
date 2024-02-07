use crate::{Token};
use crate::util::{LookaheadIterator};
use crate::scanner::Scanner;

/// =============================================================================
/// Lexer
/// =============================================================================

pub struct Lexer<I:Iterator,S:Scanner> {
    input: LookaheadIterator<I>,
    rules: S    
}

impl<I:Iterator,S:Scanner> Lexer<I,S> {
    pub fn new(iter: I, rules: S) -> Self {
        let input = LookaheadIterator::new(iter);
        Self{input,rules}
    }
}

impl<I:Iterator,S:Scanner<Item=I::Item>> Iterator for Lexer<I,S> {
    type Item = Token<S::Token>;
    
    fn next(&mut self) -> Option<Self::Item> {
        // Compute start offset
        let start = self.input.offset();
        // See what we've got
        match self.rules.scan(&mut self.input) {
            Some(t) => {
                // Compute end offset
                let end = self.input.offset();
                // Done
                Some(Token::new(t,start..end))
            }
            None => None
        }
    }
}
