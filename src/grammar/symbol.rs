#[allow(clippy::derived_hash_with_manual_eq)]
#[derive(Debug, Clone, Copy, Eq, Hash)]
pub enum GrammarSymbol {
    Terminal(Terminal),
    Nonterminal(Nonterminal),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Terminal(pub usize);

impl Terminal {
    pub fn epsilon() -> Terminal {
        Terminal(0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Nonterminal(pub usize);

impl From<Terminal> for GrammarSymbol {
    fn from(value: Terminal) -> Self {
        GrammarSymbol::Terminal(value)
    }
}

impl From<Nonterminal> for GrammarSymbol {
    fn from(value: Nonterminal) -> Self {
        GrammarSymbol::Nonterminal(value)
    }
}

impl PartialEq for GrammarSymbol {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Terminal(l), Self::Terminal(r)) => l == r,
            (Self::Nonterminal(l), Self::Nonterminal(r)) => l == r,
            (Self::Terminal(l), Self::Nonterminal(r)) if l.0 == r.0 => panic!(
                "Found Terminal and Nonterminal pair with equal id = {id}",
                id = l.0
            ),
            (Self::Nonterminal(l), Self::Terminal(r)) if l.0 == r.0 => panic!(
                "Found Nonterminal and Terminal pair with equal id = {id}",
                id = l.0
            ),
            _ => false,
        }
    }
}
