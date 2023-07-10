pub(super) enum Symbol {
    Terminal(Terminal),
    Nonterminal(Nonterminal),
}

pub(super) trait SymbolConvertable {
    fn to_symbol(self) -> Symbol;
}

#[derive(Clone, Copy)]
pub(super) struct Terminal(pub u64);

impl SymbolConvertable for Terminal {
    fn to_symbol(self) -> Symbol {
        Symbol::Terminal(self)
    }
}

#[derive(Clone, Copy)]
pub(super) struct Nonterminal(pub u64);

impl SymbolConvertable for Nonterminal {
    fn to_symbol(self) -> Symbol {
        Symbol::Nonterminal(self)
    }
}
