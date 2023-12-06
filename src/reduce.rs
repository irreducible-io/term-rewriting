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
            let (m, b) = rule.left.matches(expr);
            if m {
                matches.push((rule, b))
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

pub fn reduce<'s>(expr: &Expression, rules: &RewriteRules) -> Expression {
    let matches = rules.find_matches(expr);
    if matches.is_empty() { return expr.clone(); }
    let (rule, bindings) = &matches[0];
    return rule.right.interpolate(&bindings);
}