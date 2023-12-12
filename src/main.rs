use std::io::{stdin, BufRead, BufReader};
use std::error::Error;
use std::env::args;
use std::fs;

use peano::repl::*;

fn main() -> Result<(), Box<dyn Error>> {
    let f = args().nth(1);
    let mut repl = Repl::new();
    if let Some(path) = f {
        println!("<LOAD> '{}'", path);
        let f = fs::File::open(path)?;
        let b = BufReader::new(f);
        for rline in b.lines() {
            let line = rline?;
            repl.exec(&line);
        }
    }
    for rline in stdin().lines() {
        let line = rline?;
        repl.exec(&line);
    }
    Ok(())
}
