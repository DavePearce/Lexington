use lexington::{Lexer,Token};
use lexington::util::{LookaheadIterator};
use lexington::scanner::*;
use std::str::Chars;

//type Scanner = fn(&mut LookaheadIterator<Chars>)->Option<TokenKind>;

#[derive(Copy,Clone,Debug,PartialEq)]    
enum TokenKind {
    WhiteSpace,
    LeftBrace,
    RightBrace,
    Identifier
}

use TokenKind::*;

fn scan(input: &str) -> Vec<Token<TokenKind>> {
    let whitespace = Unit(' '.many(),WhiteSpace);
    // Construct scanner
    let scanner = whitespace
        .or(Unit('(',LeftBrace))
        .or(Unit(')',RightBrace));
    // Construct the lexer.
    Lexer::new(input.chars(),scanner).collect()
}

// #[test]
// fn test_01() {
//     let tokens = scan(&"hello world");
//     assert_eq!(tokens,&[]);
// }

#[test]
fn test_02() {
    let tokens = scan(&"()");
    assert_eq!(tokens,&[(LeftBrace,0..1),(RightBrace,1..2)]);
}
