use std::str::Chars;
use lexington::{Any,Lexer,Matcher,Scanner,Token,Unit,Within};

/// A simple definition of the components of an S-expression.
#[derive(Copy,Clone,Debug,PartialEq)]    
enum Kind {
    WhiteSpace,
    LeftBrace,
    RightBrace,
    Symbol
}

/// A simple definition of an S-expression.
#[derive(Clone,Debug,PartialEq)]    
enum SExp<'a> {
    Symbol(&'a str),
    List(Vec<SExp<'a>>)
}

/// Identifiers differents kinds of error.
#[derive(Debug,PartialEq)]    
enum ParseError {
    UnexpectedEndOfFile,
    UnexpectedToken(Kind)
}

/// Construct a very simple lexer for S-expressions, and scan them to
/// produce a list of zero or more tokens.
fn lex(input: &str) -> Vec<Token<Kind>> {
    // [ \n\t]+
    let whitespace = Any([' ','\n','\t']).one_or_more();
    // [0..9a..zA..Z_]+
    let symbol = Within('0'..='9').or(Within('a'..='z'))
        .or(Within('A'..='Z')).or('_').one_or_more();
    // Construct scanner
    let scanner = Unit(whitespace,Kind::WhiteSpace)
        .or(Unit(symbol,Kind::Symbol))
        .or(Unit('(',Kind::LeftBrace))
        .or(Unit(')',Kind::RightBrace));
    // Construct the lexer.
    Lexer::new(input.chars(),scanner).collect()
}

struct Parser {
    tokens: Vec<Token<Kind>>,
    offset: usize
}

impl Parser {
    fn new(tokens: Vec<Token<Kind>>) -> Self { Self{tokens, offset:0} }
    
    /// Extract the next token from a sequence of tokens.  If none, then
    /// raise an error.
    fn lookahead(&mut self) -> Result<Token<Kind>,ParseError> {
        if self.offset == self.tokens.len() {
            Err(ParseError::UnexpectedEndOfFile)
        } else {
            let token = self.tokens[self.offset];
            Ok(token)
        }
    }

    /// Check that the next token matches an expected kind and, if so,
    /// skip over it.  Otherwise, return an error.
    fn expect(&mut self, kind: Kind) -> Result<(),ParseError> {
        // Get next token
        let next = self.lookahead()?;
        // Check whether if its expected or not.
        if next.kind == kind {
            self.offset += 1;
            Ok(())
        } else {
            Err(ParseError::UnexpectedToken(next.kind))
        }
    }
}


fn parse<'a>(parser: &mut Parser,input: &'a str) -> Result<SExp<'a>,ParseError> {
    let mut token = parser.lookahead()?;
    //
    match token.kind {
        Kind::WhiteSpace => {
            parser.expect(Kind::WhiteSpace)?;
            parse(parser,input)
        }
        Kind::Symbol => {
            parser.expect(Kind::Symbol)?;            
            Ok(SExp::Symbol(&input[token.range()]))
        }
        _ => {
            // Match '('
            parser.expect(Kind::LeftBrace)?;
            //
            let mut terms : Vec<SExp<'a>> = Vec::new();            
            // Match rest
            loop {
                let next = parser.lookahead()?;
                //
                match next.kind {
                    Kind::RightBrace => {
                        parser.expect(Kind::RightBrace)?;
                        return Ok(SExp::List(terms));
                    }
                    Kind::WhiteSpace => {
                        parser.expect(Kind::WhiteSpace)?;
                    }
                    _ => {
                        terms.push(parse(parser,input)?);                        
                    }
                }
            }
        }
    }
}

fn check_ok(input: &str, expecting: SExp) {
    // Lex into tokens and construct parser
    let mut parser = Parser::new(lex(input));    
    // Parse tokens into S-expression
    let actual = parse(&mut parser,input).unwrap();
    // Check what we got
    assert_eq!(actual,expecting);
}

fn check_err(input: &str, expecting: ParseError) {
    // Lex into tokens and construct parser
    let mut parser = Parser::new(lex(input));    
    // Parse tokens into S-expression
    let actual = parse(&mut parser,input).unwrap_err();
    // Check what we got
    assert_eq!(actual,expecting);
}

use SExp::*;

#[test]
fn lisp_01() {
    check_ok("x",Symbol("x"));
}

#[test]
fn lisp_02() {
    check_ok("901",Symbol("901"));
}

#[test]
fn lisp_03() {
    check_ok("0x01",Symbol("0x01"));
}

#[test]
fn lisp_04() {
    check_ok("_hello",Symbol("_hello"));
}

#[test]
fn lisp_05() {
    check_ok(" xyz",Symbol("xyz"));
}

#[test]
fn lisp_06() {
    check_ok("()",List(vec![]));
}

#[test]
fn lisp_07() {
    check_ok("(x)",List(vec![Symbol("x")]));
}

#[test]
fn lisp_08() {
    check_ok("( x)",List(vec![Symbol("x")]));
}

#[test]
fn lisp_09() {
    check_ok("(x )",List(vec![Symbol("x")]));
}

#[test]
fn lisp_10() {
    check_ok("(x y)",List(vec![Symbol("x"),Symbol("y")]));
}

#[test]
fn lisp_11() {
    check_ok("(())",List(vec![List(vec![])]));
}

#[test]
fn lisp_12() {
    check_ok("((x))",List(vec![List(vec![Symbol("x")])]));
}

#[test]
fn lisp_13() {
    check_ok("(x (y))",List(vec![Symbol("x"),List(vec![Symbol("y")])]));
}

#[test]
fn lisp_14() {
    check_ok("((x) y)",List(vec![List(vec![Symbol("x")]),Symbol("y")]));
}

#[test]
fn lisp_15() {
    check_err("",ParseError::UnexpectedEndOfFile);
}

#[test]
fn lisp_16() {
    check_err(" ",ParseError::UnexpectedEndOfFile);
}

#[test]
fn lisp_17() {
    check_err("(",ParseError::UnexpectedEndOfFile);
}

#[test]
fn lisp_18() {
    check_err("(",ParseError::UnexpectedEndOfFile);
}

#[test]
fn lisp_19() {
    check_err("(()",ParseError::UnexpectedEndOfFile);
}

#[test]
fn lisp_20() {
    check_err(")",ParseError::UnexpectedToken(Kind::RightBrace));
}
