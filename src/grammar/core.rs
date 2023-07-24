use std::collections::{HashMap, HashSet};

use super::symbol::{GrammarSymbol, Nonterminal, Terminal};

// TODO: add getter for all productions for convenience
pub trait Grammar {
    fn start(&self) -> Nonterminal;
    fn terminals(&self) -> HashSet<Terminal>;
    fn nonterminals(&self) -> HashSet<Nonterminal>;
    fn alternatives_for(&self, head: Nonterminal) -> &HashSet<Vec<GrammarSymbol>>;
}

pub struct BasicGrammar {
    pub start: Nonterminal,
    pub rules: HashMap<Nonterminal, HashSet<Vec<GrammarSymbol>>>,
}

impl Grammar for BasicGrammar {
    fn start(&self) -> Nonterminal {
        self.start
    }

    fn terminals(&self) -> HashSet<Terminal> {
        let mut terminals = HashSet::new();
        for bodies in self.rules.values() {
            for body in bodies {
                for symbol in body {
                    if let GrammarSymbol::Terminal(t) = symbol {
                        terminals.insert(*t);
                    }
                }
            }
        }
        terminals
    }

    fn nonterminals(&self) -> HashSet<Nonterminal> {
        self.rules.keys().cloned().collect()
    }

    fn alternatives_for(&self, symbol: Nonterminal) -> &HashSet<Vec<GrammarSymbol>> {
        &self.rules[&symbol]
    }
}
