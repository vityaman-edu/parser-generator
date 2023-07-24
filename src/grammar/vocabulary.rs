use bimap::BiMap;

use super::symbol::{GrammarSymbol, Nonterminal, Terminal};

pub(super) trait Vocabulary {
    fn symbol(&self, name: &str) -> Option<GrammarSymbol>;
    fn name(&self, symbol: GrammarSymbol) -> Option<&String>;

    fn t(&self, name: &str) -> Terminal {
        match self.symbol(name).unwrap() {
            GrammarSymbol::Terminal(symbol) => symbol,
            _ => panic!("Symbol {name} does not exist in vocab", name = name),
        }
    }

    fn n(&self, name: &str) -> Nonterminal {
        match self.symbol(name).unwrap() {
            GrammarSymbol::Nonterminal(symbol) => symbol,
            _ => panic!("Symbol {name} does not exist in vocab", name = name),
        }
    }
}

pub(super) struct BiMapVocabulary {
    map: BiMap<GrammarSymbol, String>,
}

impl Vocabulary for BiMapVocabulary {
    fn symbol(&self, name: &str) -> Option<GrammarSymbol> {
        self.map.get_by_right(name)
    }

    fn name(&self, symbol: GrammarSymbol) -> Option<&String> {
        todo!()
    }
}
