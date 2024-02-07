use lexington::{Lexer,Matcher,Scanner,Token,Unit};

#[derive(Copy,Clone,Debug,PartialEq)]    
enum Kind {
    WhiteSpace,
    LeftBrace,
    RightBrace,
    Identifier
}

use Kind::*;

fn scan(input: &str) -> Vec<Token<Kind>> {
    let whitespace = Unit(' '.many(),WhiteSpace);
    // Construct scanner
    let scanner = whitespace
        .or(Unit('(',LeftBrace))
        .or(Unit(')',RightBrace));
    // Construct the lexer.
    Lexer::new(input.chars(),scanner).collect()
}

#[test]
fn test_01() {
    let tokens = scan(&"");
    assert_eq!(tokens,Vec::<Token<Kind>>::new());
}

#[test]
fn test_02() {
    let tokens = scan(&" ");
    assert_eq!(tokens,&[(WhiteSpace,0..1)]);
}

#[test]
fn test_03() {
    let tokens = scan(&"  ");
    assert_eq!(tokens,&[(WhiteSpace,0..2)]);
}

#[test]
fn test_04() {
    let tokens = scan(&"()");
    assert_eq!(tokens,&[(LeftBrace,0..1),(RightBrace,1..2)]);
}

#[test]
fn test_05() {
    let tokens = scan(&"( )");
    assert_eq!(tokens,&[(LeftBrace,0..1),(WhiteSpace,1..2),(RightBrace,2..3)]);
}

#[test]
fn test_06() {
    let tokens = scan(&"(  )");
    assert_eq!(tokens,&[(LeftBrace,0..1),(WhiteSpace,1..3),(RightBrace,3..4)]);
}
