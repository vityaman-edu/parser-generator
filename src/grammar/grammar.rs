use std::collections::{HashMap, HashSet};

use super::symbol::{GrammarSymbol, Nonterminal, Terminal};

pub(super) trait Productions {
    fn terminals(&self) -> HashSet<Terminal>;
    fn nonterminals(&self) -> HashSet<Nonterminal>;
    fn productions(&self, head: Nonterminal) -> &HashSet<Vec<GrammarSymbol>>;
}

pub(super) struct BasicGrammar {
    pub(super) rules: HashMap<Nonterminal, HashSet<Vec<GrammarSymbol>>>,
}

impl Productions for BasicGrammar {
    fn terminals(&self) -> HashSet<Terminal> {
        let mut terminals = HashSet::new();
        for bodies in self.rules.values() {
            for body in bodies {
                for symbol in body {
                    if let GrammarSymbol::Terminal(t) = symbol {
                        terminals.insert(t.clone());
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
