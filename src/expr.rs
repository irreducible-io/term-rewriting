use crate::symbol_table::SymbolHandle;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum VariableKind {
    Any,
    Distinct
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Terminal {
    // $n $x $abc
    Variable(SymbolHandle, VariableKind),
    // 0 1 + abc x
    Symbol(SymbolHandle),
    // (a + b) / 4
    Parentheses(Expression)
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Expression(pub Vec<Terminal>);

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Statement {
    // Define a new rewrite rule
    Rewrite(Expression, Expression),
    // An empty statement that does nothing
    Noop
}

impl Statement {

    pub fn is_noop(&self) -> bool {
        match self {
            Statement::Noop => true,
            _ => false
        }
    }

}

#[derive(Debug)]
pub struct Label(pub SymbolHandle);

#[derive(Debug)]
pub struct Comment(pub String);

#[derive(Debug)]
pub struct Item {
    pub label: Option<Label>,
    pub comment: Option<Comment>,
    pub statement: Statement
}

macro_rules! term {
    ($sym:ident ( $($t:tt)* ) ) => {
        Terminal::Parentheses( expr!($sym $($t)*) )
    };
    ($sym:ident [$s:ident]) => {
        Terminal::Variable($sym.handle(stringify!($s)), VariableKind::Any)
    };
    ($sym:ident [$s:literal]) => {
        Terminal::Variable($sym.handle(&format!("{}", $s)), VariableKind::Any)
    };
    ($sym:ident [$s:tt]) => {
        Terminal::Variable($sym.handle(stringify!($s)), VariableKind::Any)
    };
    ($sym:ident $s:ident) => {
        Terminal::Symbol($sym.handle(stringify!($s)))
    };
    ($sym:ident $s:literal) => {
        Terminal::Symbol($sym.handle(&format!("{}", $s)))
    };
    ($sym:ident $s:tt) => {
        Terminal::Symbol($sym.handle(stringify!($s)))
    };
}

macro_rules! expr {
    ($sym:ident $($t:tt)* ) => {
        Expression(vec![$( term!($sym $t) ),*])
    };
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::symbol_table::SymbolTable;

    #[test]
    fn test_term_macro_var() {
        let mut symbols = SymbolTable::new();
        assert_eq!(term!(symbols [x]), Terminal::Variable(symbols.handle("x"), VariableKind::Any));
        assert_eq!(term!(symbols ["with space"]), Terminal::Variable(symbols.handle("with space"), VariableKind::Any));
    }

    #[test]
    fn test_term_macro_symbol() {
        let mut symbols = SymbolTable::new();
        assert_eq!(term!(symbols x), Terminal::Symbol(symbols.handle("x")));
        assert_eq!(term!(symbols "with space"), Terminal::Symbol(symbols.handle("with space")));
    }

    #[test]
    fn test_term_macro_paren() {
        let mut symbols = SymbolTable::new();
        assert_eq!(term!(symbols (S 0)), Terminal::Parentheses(Expression(vec![Terminal::Symbol(symbols.handle("S")), Terminal::Symbol(symbols.handle("0"))])));
    }

    #[test]
    fn test_expr_macro_empty() {
        assert_eq!(expr!(symbols), Expression(vec![]))
    }

    #[test]
    fn test_expr_macro_one() {
        let mut symbols = SymbolTable::new();
        assert_eq!(expr!(symbols one), Expression(vec![Terminal::Symbol(symbols.handle("one"))]))
    }

    #[test]
    fn test_expr_macro_many() {
        let mut symbols = SymbolTable::new();
        assert_eq!(
            expr!(symbols ([x] + [y]) / 2),
            Expression(vec![
                Terminal::Parentheses(Expression(vec![
                    Terminal::Variable(symbols.handle("x"), VariableKind::Any),
                    Terminal::Symbol(symbols.handle("+")),
                    Terminal::Variable(symbols.handle("y"), VariableKind::Any)
                ])),
                Terminal::Symbol(symbols.handle("/")),
                Terminal::Symbol(symbols.handle("2"))
            ])
        );
    }

}