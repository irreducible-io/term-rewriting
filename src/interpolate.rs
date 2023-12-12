use crate::matches::VariableBinding;
use crate::expr::*;

pub trait Interpolate {
    fn interpolate<'t>(&self, bindings: &[VariableBinding<'t>]) -> Self;
}

impl Interpolate for Terminal {

    fn interpolate<'t>(&self, bindings: &[VariableBinding<'t>]) -> Self {
        match self {
            Terminal::Parentheses(e) => Terminal::Parentheses(e.interpolate(bindings)),
            Terminal::Symbol(s) => Terminal::Symbol(*s),
            Terminal::Variable(v, k) => {
                for b in bindings {
                    if b.var == *v {
                        return b.expr.clone()
                    }
                }
                Terminal::Variable(*v, *k)
            }
        }
    }

}

impl Interpolate for Expression {

    fn interpolate<'t>(&self, bindings: &[VariableBinding<'t>]) -> Self {
        let mut interpolated = vec![];
        for term in &self.0 {
            interpolated.push(term.interpolate(bindings))
        }
        // We may need to remove unnecessary parens after a transplant...
        if interpolated.len() == 1 {
            if let Terminal::Parentheses(e) = &interpolated[0] {
                return e.clone()
            }
        }
        Expression(interpolated)
    }

}
