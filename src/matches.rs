use crate::{expr::*, symbol_table::SymbolHandle};

pub trait Matches {

    fn matches<'t>(&self, other: &'t Self, bindings: &mut Vec<VariableBinding<'t>>) -> bool;

}

impl Matches for Expression {

    fn matches<'t>(&self, other: &'t Self, bindings: &mut Vec<VariableBinding<'t>>) -> bool {
        if self.0.len() != other.0.len() {
            return false;
        }
        for (a, b) in self.0.iter().zip(other.0.iter()) {
            if !a.matches(b, bindings) {
                return false;
            }
        }
        true
    }

}

pub struct VariableBinding<'t> {
    pub var: SymbolHandle,
    pub expr: &'t Terminal
}

impl Matches for Terminal {

    fn matches<'t>(&self, other: &'t Self, bindings: &mut Vec<VariableBinding<'t>>) -> bool {
        match self {
            // Symbols must match exactly another symbol
            Terminal::Symbol(a) => {
                if let Terminal::Symbol(b) = other {
                    a == b
                } else {
                    false
                }
            },
            // Variables match any terminal
            Terminal::Variable(v) => {
                
                // Except when this variable has already been bound!
                // In this case, the other expression must *equal* the bound
                // expression exactly.
                if let Some(binding) = bindings.iter().find(|b| b.var == *v) {
                    return binding.expr == other;
                }

                // Additionally, we must verify that the other expression
                // has not already matched another variable.
                // This enforces that variables of two different names e.g. $x and $y
                // match distinct subexpressions.
                if bindings.iter().any(|b| b.expr == other) {
                    return false;
                }

                bindings.push(VariableBinding { var: *v, expr: other });
                true
            },
            // Subexpressions in both parens must match
            Terminal::Parentheses(a) => {
                if let Terminal::Parentheses(b) = other {
                    a.matches(b, bindings)
                } else {
                    false
                }
            }
        }
    }

}
