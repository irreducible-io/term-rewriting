use crate::reduce::{RewriteRules, RewriteRule};
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
                    Statement::Rewrite(mut l, r) => {
                        let s = &mut self.symbols;
                        if r == expr!(s ?) {
                            while l.reduce_once(&self.rules) {
                                println!("\t{}", l.bind(&self.symbols));
                            }
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
