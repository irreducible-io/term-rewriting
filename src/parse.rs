use std::{fmt::Display, error::Error};
use crate::{expr::*, symbol_table::SymbolTable};

#[derive(Debug, Eq, PartialEq)]
pub enum Token {
    Symbol,
    Constant(&'static str),
}

impl Display for Token {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Symbol => write!(f, "<symbol>"),
            Token::Constant(s) => write!(f, "\"{}\"", s),
        }
    }

}

#[derive(Debug)]
pub struct ParseError {
    idx: usize,
    error: ErrorKind
}

impl Error for ParseError {}

pub type ParseResult<'s, T> = Result<(T, &'s str), ParseError>;

impl Display for ParseError {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Parse error at col {}: {}", self.idx, self.error)
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum ErrorKind {
    ReservedSymbol(&'static str),
    UnexpectedEoF,
    ExpectedToken(Token)
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorKind::ReservedSymbol(s) => write!(f, "\"{}\" is a reserved symbol", s),
            ErrorKind::UnexpectedEoF => write!(f, "Incomplete statement"),
            ErrorKind::ExpectedToken(t) => write!(f, "Expected token {}", t)
        }
    }
}

trait TryParse where Self: Sized {

    fn try_parse<'s>(s: &'s str, symbols: &mut SymbolTable) -> ParseResult<'s, Self>;

}

fn take_until<'s, F: Fn(char) -> bool>(s: &'s str, predicate: F) -> ParseResult<'s, &'s str> {
    if let Some(idx) = s.chars().position(predicate) {
        if idx == 0 {
            return Err(ParseError { idx, error: ErrorKind::ExpectedToken(Token::Symbol)})
        } else {
            return Ok((&s[0..idx], &s[idx..]))
        }
    }
    Ok((s, "")) // never matched, take the whole input
}

fn take_const<'s>(s: &'s str, c: &'static str) -> ParseResult<'s, &'s str> {
    if s.len() < c.len() {
        return Err(ParseError { idx: 0, error: ErrorKind::UnexpectedEoF })
    }
    for (i, (a, b)) in s.chars().zip(c.chars()).enumerate() {
        if a != b {
            return Err(ParseError { idx: i, error: ErrorKind::ExpectedToken(Token::Constant(c)) })
        }
    }
    Ok((c, &s[c.len()..]))
}

impl Terminal {

    fn try_parse_variable<'s>(s: &'s str, symbols: &mut SymbolTable) -> ParseResult<'s, Terminal> {
        let (_d, s) = take_const(s, "$")?;
        let (n, s) = take_until(s, |c| !c.is_alphanumeric())?;
        Ok((Terminal::Variable(symbols.handle(n)), s))
    }

    fn try_parse_symbol<'s>(s: &'s str, symbols: &mut SymbolTable) -> ParseResult<'s, Terminal> {
        let (symbol, s) = take_until(s, |c| c.is_whitespace()
                                                                      || c == '('
                                                                      || c == ')')?;
        if symbol == "->" {
            return Err(ParseError{ idx: 0, error: ErrorKind::ReservedSymbol("->") });
        }
        Ok((Terminal::Symbol(symbols.handle(symbol)), s))
    }

    fn try_parse_parens<'s>(s: &'s str, symbols: &mut SymbolTable) -> ParseResult<'s, Terminal> {
        let (_paren, s) = take_const(s, "(")?;
        let (expr, s) = Expression::try_parse(s, symbols)?;
        let (_paren, s) = take_const(s, ")")?;
        Ok((Terminal::Parentheses(expr), s))
    }

}

impl TryParse for Terminal {

    fn try_parse<'s>(s: &'s str, symbols: &mut SymbolTable) -> ParseResult<'s, Self> {
        Terminal::try_parse_variable(s, symbols)
            .or_else(|_| Terminal::try_parse_parens(s, symbols))
            .or_else(|_| Terminal::try_parse_symbol(s, symbols))
            // TODO: Map the error here to better explain what is being parsed.
    }

}

impl TryParse for Expression {

    fn try_parse<'s>(s: &'s str, symbols: &mut SymbolTable) -> ParseResult<'s, Self> {
        let mut parsed = Vec::<Terminal>::new();
        let mut rem = s;
        loop {
            if rem.is_empty() { break; }
            let (_w, s) = take_until(rem,|c| !c.is_whitespace())
                            .unwrap_or(("", rem));
            match Terminal::try_parse(s, symbols) {
                Ok((term, s)) => {
                    rem = s;
                    parsed.push(term);
                },
                Err(_) => {
                    return Ok((Expression(parsed), rem));
                }
            }
        }
        Ok((Expression(parsed), rem))
    }

}

impl Statement {

    pub fn parse<'s>(s: &'s str, symbols: &mut SymbolTable) -> Result<Statement, ParseError> {
        let (left, s) = Expression::try_parse(s, symbols)?;
        let (_w, s) = take_until(s, |c| !c.is_whitespace()).unwrap_or(("", s));
        let (_arrow, s) = take_const(s, "->")?;
        let (_w, s) = take_until(s, |c| !c.is_whitespace()).unwrap_or(("", s));
        let (right, _s) = Expression::try_parse(s, symbols)?;
        Ok(Statement::Rewrite(left, right))
    }

}
