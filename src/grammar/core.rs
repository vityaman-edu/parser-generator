use std::collections::{HashMap, HashSet};

use super::symbol::{GrammarSymbol, Nonterminal, Terminal};

pub trait Productions {
    fn terminals(&self) -> HashSet<Terminal>;
    fn nonterminals(&self) -> HashSet<Nonterminal>;
    fn productions(&self, head: Nonterminal) -> &HashSet<Vec<GrammarSymbol>>;
}

pub struct BasicGrammar {
    pub rules: HashMap<Nonterminal, HashSet<Vec<GrammarSymbol>>>,
}

impl Productions for BasicGrammar {
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

    fn productions(&self, symbol: Nonterminal) -> &HashSet<Vec<GrammarSymbol>> {
        &self.rules[&symbol]
    }
}
