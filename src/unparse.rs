use std::fmt::Display;

use crate::{expr::*, symbol_table::SymbolTable};

impl Display for Statement {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Statement::Rewrite(l, r) => write!(f, "{} -> {}", l, r)
        }
    }

}

pub struct BoundStatement<'s> {
    symbols: &'s SymbolTable,
    statement: &'s Statement
}

impl Statement {

    #[allow(dead_code)]
    pub fn bind<'s>(&'s self, symbols: &'s SymbolTable) -> BoundStatement<'s> {
        BoundStatement { symbols, statement: self }
    }

}

impl<'s> Display for BoundStatement<'s> {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.statement {
            Statement::Rewrite(l, r) => write!(f, "{} -> {}", l.bind(self.symbols), r.bind(self.symbols))
        }
    }

}

impl Display for Expression {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut first = true;
        for term in &self.0 {
            if !first {
                write!(f, " ")?;
            }
            write!(f, "{}", term)?;
            first = false;
        }
        Ok(())
    }
}

pub struct BoundExpression<'s> {
    symbols: &'s SymbolTable,
    expr: &'s Expression
}

impl Expression {

    pub fn bind<'s>(&'s self, symbols: &'s SymbolTable) -> BoundExpression<'s> {
        BoundExpression { symbols, expr: self }
    }

}

impl<'s> Display for BoundExpression<'s> {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut first = true;
        for term in &self.expr.0 {
            if !first {
                write!(f, " ")?;
            }
            write!(f, "{}", term.bind(self.symbols))?;
            first = false;
        }
        Ok(())
    }

}

impl Display for Terminal {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Terminal::Parentheses(e) => write!(f, "({})", e),
            Terminal::Symbol(s) => write!(f, "{}", s),
            Terminal::Variable(v) => write!(f, "${}", v)
        }
    }

}

impl Terminal {

    pub fn bind<'s>(&'s self, symbols: &'s SymbolTable) -> BoundTerminal {
        BoundTerminal { symbols, terminal: self }
    }

}

pub struct BoundTerminal<'s> {
    symbols: &'s SymbolTable,
    terminal: &'s Terminal
}

impl<'s> Display for BoundTerminal<'s> {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.terminal {
            Terminal::Parentheses(e) => write!(f, "({})", e.bind(self.symbols)),
            Terminal::Symbol(s) => write!(f, "{}", self.symbols.lookup(*s)),
            Terminal::Variable(v) => write!(f, "${}", self.symbols.lookup(*v))
        }
    }
}
