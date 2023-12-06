use std::io::stdin;
use std::error::Error;

#[macro_use]
mod expr;
mod parse;
mod unparse;
mod matches;
mod reduce;
mod repl;
mod symbol_table;
mod interpolate;

use repl::*;

fn main() -> Result<(), Box<dyn Error>> {
    let mut repl = Repl::new();
    for rline in stdin().lines() {
        let line = rline?;
        repl.exec(&line);
    }
    Ok(())
}
