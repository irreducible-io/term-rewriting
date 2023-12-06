use std::fmt::Display;


pub struct SymbolTable {
    symbols: Vec<String>
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct SymbolHandle {
    idx: usize,
}

impl Display for SymbolHandle {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{}>", self.idx)
    }

}

impl SymbolTable {

    pub fn new() -> Self {
        SymbolTable {
            symbols: Vec::new()
        }
    }

    pub fn handle(&mut self, s: &str) -> SymbolHandle {
        for (idx, symbol) in self.symbols.iter().enumerate() {
            if symbol == s {
                return SymbolHandle { idx }
            }
        }
        self.symbols.push(s.to_owned());
        SymbolHandle { idx: self.symbols.len()-1 }
    }

    pub fn lookup(&self, handle: SymbolHandle) -> &str {
        &self.symbols[handle.idx]
    }

}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_symbol_lookup() {
        let mut symbols = SymbolTable::new();
        let handle = symbols.handle("x");
        let val = symbols.lookup(handle);
        assert_eq!(val, "x");
    }

    #[test]
    fn test_symbol_duplicate() {
        let mut symbols =  SymbolTable::new();
        let handle1 = symbols.handle("x");
        let handle2 = symbols.handle("x");
        assert_eq!(handle1, handle2);
    }

}
