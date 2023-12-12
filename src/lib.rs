
use expr::{Statement, Expression};
use parse::TryParse;
use reduce::{RewriteRules, RewriteRule};
use symbol_table::SymbolTable;
use wasm_bindgen::prelude::*;

#[macro_use]
pub mod expr;
pub mod parse;
pub mod unparse;
pub mod matches;
pub mod reduce;
pub mod repl;
pub mod symbol_table;
pub mod interpolate;

#[wasm_bindgen] extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn error(s: &str);
}

#[wasm_bindgen]
pub struct TrsHandle {
    symbols: SymbolTable,
    rules: RewriteRules,
}

#[wasm_bindgen]
pub fn trs_init(src: &str) -> TrsHandle {
    let mut symbols = SymbolTable::new();
    let mut rules = RewriteRules::new();
    for line in src.lines() {
        let result = Statement::parse(line, &mut symbols);
        match result {
            Ok(statement) => {
                match statement {
                    Statement::Noop => {},
                    Statement::Rewrite(l, r) => {
                        rules.add(RewriteRule { left: l, right: r })
                    }
                }
            }
            Err(e) => {
                error(&format!("{}", e))
            }
        }
    }
    TrsHandle { symbols, rules }
}

#[wasm_bindgen]
pub fn trs_reduce_once(s: &str, trs: &mut TrsHandle) -> String {
    let result = Expression::parse(s, &mut trs.symbols);
    match result {
        Ok(mut expr) => {
            expr.reduce_once(&mut trs.rules);
            format!("{}", expr.bind(&trs.symbols))
        },
        Err(e) => {
            error(&format!("{}", e));
            "<ERR>".to_owned()
        }
    }
}
