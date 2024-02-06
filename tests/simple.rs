use lexington::{Lexer,Token};
use lexington::util::{LookaheadIterator};
use lexington::scanner::{Scanner,BlockScanner,UnitScanner};
use std::str::Chars;

//type Scanner = fn(&mut LookaheadIterator<Chars>)->Option<TokenKind>;

#[derive(Copy,Clone,Debug,PartialEq)]    
enum TokenKind {
    Whitespace,
    LeftBrace,
    RightBrace,
    Identifier,
    EndOfFile
}

use TokenKind::*;

fn scan(input: &str) -> Vec<Token<TokenKind>> {
    Lexer::new(input.chars(),&(
        BlockScanner(|c:&char| c.is_whitespace(),Whitespace),
        //UnitScanner('(',LeftBrace),
        UnitScanner(')',RightBrace))).collect()
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
