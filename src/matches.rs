use crate::{expr::*, symbol_table::SymbolHandle};

pub trait Matches {

    fn matches<'t>(&self, other: &'t Self) -> (bool, Vec<VariableBinding<'t>>);

}

impl Matches for Expression {

    fn matches<'t>(&self, other: &'t Self) -> (bool, Vec<VariableBinding<'t>>) {
        if self.0.len() != other.0.len() {
            return (false, vec![]);
        }
        let mut bindings = vec![];
        for (a, b) in self.0.iter().zip(other.0.iter()) {
            let (m, b) = a.matches(&b);
            if !m { return (false, vec![]); }
            bindings.extend(b);
        }
        (true, bindings)
    }

}

pub struct VariableBinding<'t> {
    pub var: SymbolHandle,
    pub expr: &'t Terminal
}

impl Matches for Terminal {

    fn matches<'t>(&self, other: &'t Self) -> (bool, Vec<VariableBinding<'t>>) {
        match self {
            // Symbols must match exactly another symbol
            Terminal::Symbol(a) => {
                if let Terminal::Symbol(b) = other {
                    (a == b, vec![])
                } else {
                    (false, vec![])
                }
            },
            // Variables match any terminal
            Terminal::Variable(v) => {
                (true, vec![VariableBinding { var: *v, expr: other }])
            },
            // Subexpressions in both parens must match
            Terminal::Parentheses(a) => {
                if let Terminal::Parentheses(b) = other {
                    a.matches(b)
                } else {
                    (false, vec![])
                }
            }
        }
    }

}
