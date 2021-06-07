/*! List of symbols */

use alloc::vec::Vec;

use crate::code_symbol::CodeSymbol;

/**
 * Ordered list of code symbols
 */
pub struct CodeSymbolsList {
    m_symbols: Vec<CodeSymbol>
}

impl CodeSymbolsList {
    /**
     * Constructs an uninitialized `CodeSymbolsList`
     */
    pub const fn new_uninitialized() -> Self {
        Self { m_symbols: Vec::new() }
    }

    /**
     * Constructs a `CodeSymbolsList` reading the newline-separated list of
     * raw symbols
     */
    pub fn load_from_raw(&mut self, raw_symbols: &str) -> bool {
        self.m_symbols = raw_symbols.split('\n')
                                    .map(CodeSymbol::from_raw_line)
                                    .filter_map(|code_symbol_opt| code_symbol_opt)
                                    .collect();
        self.m_symbols.len() > 0
    }

    /**
     * Returns the `CodeSymbol` for the given virtual address
     */
    pub fn symbol_at(&self, virt_addr: usize) -> Option<&CodeSymbol> {
        for code_symbol in self.m_symbols.iter().rev() {
            if virt_addr >= code_symbol.virt_addr() {
                return Some(code_symbol);
            }
        }
        None
    }
}