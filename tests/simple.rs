use lexington::{Any,Lexer,Matcher,Scanner,Token,Unit,Within};

#[derive(Copy,Clone,Debug,PartialEq)]    
enum Kind {
    WhiteSpace,
    LeftBrace,
    RightBrace,
    Identifier,
    Number
}

use Kind::*;

fn scan(input: &str) -> Vec<Token<Kind>> {
    // [ \n\t]+
    let whitespace = Any([' ','\n','\t']).one_or_more();
    // [0..9]+
    let number = Within('0'..='9').one_or_more();
    // [a..zA..Z_]([0..9a..zA..Z_]*)
    let identifier_start = Within('a'..='z')
        .or(Within('A'..='Z')).or('_');
    let identifier_rest = Within('0'..='9').or(Within('a'..='z'))
        .or(Within('A'..='Z')).or('_').zero_or_more();
    let identifier = identifier_start.then(identifier_rest);
    // Construct scanner
    let scanner = Unit(whitespace,WhiteSpace)
        .or(Unit(number,Number))
        .or(Unit(identifier,Identifier))
        .or(Unit('(',LeftBrace))
        .or(Unit(')',RightBrace));
    // Construct the lexer.
    Lexer::new(input.chars(),scanner).collect()
}

#[test]
fn test_whitespace_01() {
    let tokens = scan(&"");
    assert_eq!(tokens,Vec::<Token<Kind>>::new());
}

#[test]
fn test_whitespace_02() {
    let tokens = scan(&" ");
    assert_eq!(tokens,&[(WhiteSpace,0..1)]);
}

#[test]
fn test_whitespace_03() {
    let tokens = scan(&"  ");
    assert_eq!(tokens,&[(WhiteSpace,0..2)]);
}

#[test]
fn test_whitespace_04() {
    let tokens = scan(&"\n");
    assert_eq!(tokens,&[(WhiteSpace,0..1)]);
}

#[test]
fn test_whitespace_05() {
    let tokens = scan(&"\t");
    assert_eq!(tokens,&[(WhiteSpace,0..1)]);
}

#[test]
fn test_whitespace_06() {
    let tokens = scan(&"\t\n ");
    assert_eq!(tokens,&[(WhiteSpace,0..3)]);
}

#[test]
fn test_whitespace_07() {
    let tokens = scan(&" \n\t ");
    assert_eq!(tokens,&[(WhiteSpace,0..4)]);
}

#[test]
fn test_braces_01() {
    let tokens = scan(&"()");
    assert_eq!(tokens,&[(LeftBrace,0..1),(RightBrace,1..2)]);
}

#[test]
fn test_braces_02() {
    let tokens = scan(&"(())");
    assert_eq!(tokens,&[(LeftBrace,0..1),(LeftBrace,1..2),(RightBrace,2..3),(RightBrace,3..4)]);
}

#[test]
fn test_braces_03() {
    let tokens = scan(&"( )");
    assert_eq!(tokens,&[(LeftBrace,0..1),(WhiteSpace,1..2),(RightBrace,2..3)]);
}

#[test]
fn test_braces_04() {
    let tokens = scan(&"(  )");
    assert_eq!(tokens,&[(LeftBrace,0..1),(WhiteSpace,1..3),(RightBrace,3..4)]);
}

#[test]
fn test_number_01() {
    let tokens = scan(&"0");
    assert_eq!(tokens,&[(Number,0..1)]);
}

#[test]
fn test_number_02() {
    let tokens = scan(&"9");
    assert_eq!(tokens,&[(Number,0..1)]);
}

#[test]
fn test_number_03() {
    let tokens = scan(&"12");
    assert_eq!(tokens,&[(Number,0..2)]);
}

#[test]
fn test_number_04() {
    let tokens = scan(&"234");
    assert_eq!(tokens,&[(Number,0..3)]);
}

#[test]
fn test_number_05() {
    let tokens = scan(&"123898172398123");
    assert_eq!(tokens,&[(Number,0..15)]);
}

#[test]
fn test_identifier_01() {
    let tokens = scan(&"a");
    assert_eq!(tokens,&[(Identifier,0..1)]);
}

#[test]
fn test_identifier_02() {
    let tokens = scan(&"z");
    assert_eq!(tokens,&[(Identifier,0..1)]);
}

#[test]
fn test_identifier_03() {
    let tokens = scan(&"A");
    assert_eq!(tokens,&[(Identifier,0..1)]);
}

#[test]
fn test_identifier_04() {
    let tokens = scan(&"Z");
    assert_eq!(tokens,&[(Identifier,0..1)]);
}

#[test]
fn test_identifier_05() {
    let tokens = scan(&"_");
    assert_eq!(tokens,&[(Identifier,0..1)]);
}

#[test]
fn test_identifier_06() {
    let tokens = scan(&"aa");
    assert_eq!(tokens,&[(Identifier,0..2)]);
}

#[test]
fn test_identifier_07() {
    let tokens = scan(&"bz");
    assert_eq!(tokens,&[(Identifier,0..2)]);
}

#[test]
fn test_identifier_08() {
    let tokens = scan(&"cA");
    assert_eq!(tokens,&[(Identifier,0..2)]);
}

#[test]
fn test_identifier_09() {
    let tokens = scan(&"dZ");
    assert_eq!(tokens,&[(Identifier,0..2)]);
}

#[test]
fn test_identifier_10() {
    let tokens = scan(&"e_");
    assert_eq!(tokens,&[(Identifier,0..2)]);
}

#[test]
fn test_identifier_11() {
    let tokens = scan(&"f0");
    assert_eq!(tokens,&[(Identifier,0..2)]);
}

#[test]
fn test_identifier_12() {
    let tokens = scan(&"g9");
    assert_eq!(tokens,&[(Identifier,0..2)]);
}


#[test]
fn test_identifier_13() {
    let tokens = scan(&"hello");
    assert_eq!(tokens,&[(Identifier,0..5)]);
}

#[test]
fn test_identifier_14() {
    let tokens = scan(&"HELLO");
    assert_eq!(tokens,&[(Identifier,0..5)]);
}

#[test]
fn test_identifier_15() {
    let tokens = scan(&"hEllO");
    assert_eq!(tokens,&[(Identifier,0..5)]);
}

#[test]
fn test_identifier_16() {
    let tokens = scan(&"hE110");
    assert_eq!(tokens,&[(Identifier,0..5)]);
}

#[test]
fn test_identifier_17() {
    let tokens = scan(&"hE110_w0R1d");
    assert_eq!(tokens,&[(Identifier,0..11)]);
}
