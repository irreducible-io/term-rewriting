use crate::reduce::{RewriteRules, RewriteRule, reduce};
use crate::expr::*;
use crate::symbol_table::*;

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
        match Statement::parse(line, &mut self.symbols) {
            Ok(statement) => {
                match statement {
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
