use crate::expr::*;
use crate::matches::*;
use crate::interpolate::*;

pub struct RewriteRules {
    rules: Vec<RewriteRule>
}

impl<'s> RewriteRules {

    pub fn new() -> Self {
        RewriteRules { rules: vec![] }
    }

    pub fn add(&mut self, rule: RewriteRule) {
        self.rules.push(rule)
    }

    pub fn find_matches<'t>(&self, expr: &'t Expression) -> Vec<(&RewriteRule, Vec<VariableBinding<'t>>)> {
        let mut matches = vec![];
        for rule in &self.rules {
            let mut bindings = vec![];
            if rule.left.matches(expr, &mut bindings) {
                matches.push((rule, bindings))
            }
        }
        matches
    }

}

pub struct RewriteRule {
    pub left: Expression,
    pub right: Expression
}

impl RewriteRule {

    pub fn new(left: Expression, right: Expression) -> Self {
        RewriteRule { left, right }
    }

}

impl Expression {

    /// Apply a single reduction step to this expression.
    /// Returns true if a rewrite was applied and false
    /// if no rewrites matched.
    pub fn reduce_once(&mut self, rules: &RewriteRules) -> bool {

        // TODO: Can this be rewritten to use an explicit stack instead of recursion?

        // For complex expressions with many subexpressions,
        // we need to apply rewrite rules to all subexpressions
        // before rewriting this expression. This means parentheses
        // but also each individual term evaluated as a single-term expression.
        if self.0.len() > 1 {
            let mut swap = None;
            for (idx, term) in &mut self.0.iter_mut().enumerate() {
                match term {
                    Terminal::Parentheses(e) => {
                        if e.reduce_once(rules) {
                            swap = Some((idx, e.clone()));
                            break;
                        }
                    },
                    Terminal::Symbol(s) => {
                        let mut expr = Expression(vec![Terminal::Symbol(*s)]);
                        if expr.reduce_once(rules) {
                            swap = Some((idx, expr));
                            break;
                        }
                    },
                    Terminal::Variable(v, k) => {
                        let mut expr = Expression(vec![Terminal::Variable(*v, *k)]);
                        if expr.reduce_once(rules) {
                            swap = Some((idx, expr));
                            break;
                        }
                    }
                }
            }
            if let Some((idx, mut expr)) = swap {
                if expr.0.len() == 1 {
                    self.0[idx] = expr.0.pop().unwrap();
                } else {
                    self.0[idx] = Terminal::Parentheses(expr);
                }
                return true;
            }
        }
        
        // If no subexpressions were simplified, we can apply
        // the rewrite rules to this expression.
        let matches = rules.find_matches(self);
        if matches.is_empty() {
            return false;
        }

        // TODO: For now we just choose the first one but we should
        // have some form of explicit precedence when multiple rules
        // might apply. Or apply all rules and branch ... ?
        let (rule, bindings) = &matches[0];
        let rewritten = rule.right.interpolate(bindings);
        self.0 = rewritten.0;

        true
    }

}
