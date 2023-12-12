use std::{fmt::Display, error::Error};
use crate::{expr::*, symbol_table::SymbolTable};

#[derive(Debug, Eq, PartialEq)]
pub enum Token {
    Symbol,
    Constant(&'static str),
    Eof
}

impl Display for Token {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Eof => write!(f, "<eof>"),
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

pub trait TryParse: Sized {

    fn try_parse<'s>(s: &'s str, symbols: &mut SymbolTable) -> ParseResult<'s, Self>;

    fn parse<'s>(s: &'s str, symbols: &mut SymbolTable) -> Result<Self, ParseError> {
        let (parsed, _s) = Self::try_parse(s, symbols)?;
        Ok(parsed)
    }

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

fn optionally<'s, T>(s: &'s str, res: ParseResult<'s, T>) -> ParseResult<'s, Option<T>> {
    match res {
        Ok((t, s)) => Ok((Some(t), s)),
        Err(_e) => Ok((None, s))
    }
}

impl Terminal {

    fn try_parse_variable<'s>(s: &'s str, symbols: &mut SymbolTable) -> ParseResult<'s, Terminal> {
        let (_d, s) = take_const(s, "$")?;
        let (distinct, s) = optionally(s, take_const(s, "$"))?;
        let (n, s) = take_until(s, |c| !c.is_alphanumeric())?;
        let kind = if distinct.is_some() { VariableKind::Distinct } else { VariableKind::Any };
        Ok((Terminal::Variable(symbols.handle(n), kind), s))
    }

    fn try_parse_symbol<'s>(s: &'s str, symbols: &mut SymbolTable) -> ParseResult<'s, Terminal> {
        let (symbol, s) = take_until(s, |c| c.is_whitespace()
                                                                      || c == '('
                                                                      || c == ')')?;
        if symbol == "->" {
            return Err(ParseError{ idx: 0, error: ErrorKind::ReservedSymbol("->") });
        }
        if symbol == "//" {
            return Err(ParseError { idx: 0, error: ErrorKind::ReservedSymbol("//") });
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

    fn try_parse_rewrite<'s>(s: &'s str, symbols: &mut SymbolTable) -> ParseResult<'s, Statement> {
        let (left, s) = Expression::try_parse(s, symbols)?;
        let (_w, s) = optionally(s, take_until(s, |c| !c.is_whitespace()))?;
        let (_arrow, s) = take_const(s, "->")?;
        let (_w, s) = optionally(s, take_until(s, |c| !c.is_whitespace()))?;
        let (right, s) = Expression::try_parse(s, symbols)?;
        Ok((Statement::Rewrite(left, right), s))
    }

    fn try_parse_noop<'s>(s: &'s str, _symbols: &mut SymbolTable) -> ParseResult<'s, Statement> {
        let (_w, s) = optionally(s, take_until(s, |c| !c.is_whitespace()))?;
        if s.is_empty() || s.starts_with("//") {
            Ok((Statement::Noop, s))
        } else {
            Err(ParseError { idx: 0, error: ErrorKind::ExpectedToken(Token::Eof)})
        }
    }

    

}

impl TryParse for Statement {

    fn try_parse<'s>(s: &'s str, symbols: &mut SymbolTable) -> ParseResult<'s, Self> {
        Statement::try_parse_noop(s, symbols)
            .or_else(|_| Statement::try_parse_rewrite(s, symbols))
    }

}

impl TryParse for Label {

    fn try_parse<'s>(s: &'s str, symbols: &mut SymbolTable) -> ParseResult<'s, Self> {
        let (_, s) = take_const(s, "[")?;
        let (text, s) = take_until(s, |c| c == ']')?;
        let (_, s) = take_const(s, "]")?;
        Ok((Label(symbols.handle(text)), s))
    }

}

impl TryParse for Comment {

    fn try_parse<'s>(s: &'s str, _symbols: &mut SymbolTable) -> ParseResult<'s, Self> {
        let (_, s) = take_const(s, "//")?;
        let (_w, s) = optionally(s, take_until(s, |c| !c.is_whitespace()))?;
        Ok((Comment(s.to_owned()), ""))
    }

}

impl TryParse for Item {

    fn try_parse<'s>(s: &'s str, symbols: &mut SymbolTable) -> ParseResult<'s, Self> {
        let (label, s) = optionally(s, Label::try_parse(s, symbols))?;
        let (statement, s) = Statement::try_parse(s, symbols)?;
        let (_w, s) = optionally(s, take_until(s, |c| !c.is_whitespace()))?;
        let (comment, s) = optionally(s, Comment::try_parse(s, symbols))?;
        Ok((Item { label, statement, comment }, s))
    }

}
