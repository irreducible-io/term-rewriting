use crate::reduce::{RewriteRules, RewriteRule, reduce};
use crate::expr::*;
use crate::symbol_table::*;
use crate::parse::*;

pub struct Repl {
    symbols: SymbolTable,
    rules: RewriteRules
}

impl Repl {

    pub fn new() -> Self {
        Repl {
            symbols: SymbolTable::new(),
            rules: RewriteRules::new()
        }
    }

    pub fn exec(&mut self, line: &str) {
        match Item::parse(line, &mut self.symbols) {
            Ok(item) => {
                println!("{}", item.bind(&self.symbols));
                match item.statement {
                    Statement::Noop => {},
                    Statement::Rewrite(l, r) => {
                        let s = &mut self.symbols;
                        if r == expr!(s ?) {
                            let out = reduce(&l, &self.rules);
                            println!("\t{}", out.bind(&self.symbols));
                        } else {
                            self.rules.add(RewriteRule::new(l, r));
                        }
                    }
                }
            },
            Err(err) => eprintln!("{}", err)
        }
    }

}
