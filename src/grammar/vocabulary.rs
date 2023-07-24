use bimap::BiMap;

use super::symbol::GrammarSymbol;

pub trait Vocabulary {
    fn symbol(&self, name: &str) -> Option<GrammarSymbol>;
    fn name(&self, symbol: GrammarSymbol) -> Option<&String>;
}

pub struct BiMapVocabulary {
    map: BiMap<GrammarSymbol, String>,
}

impl From<BiMap<GrammarSymbol, String>> for BiMapVocabulary {
    fn from(value: BiMap<GrammarSymbol, String>) -> Self {
        BiMapVocabulary { map: value }
    }
}

impl Vocabulary for BiMapVocabulary {
    fn symbol(&self, name: &str) -> Option<GrammarSymbol> {
        self.map.get_by_right(name).cloned()
    }

    fn name(&self, symbol: GrammarSymbol) -> Option<&String> {
        self.map.get_by_left(&symbol)
    }
}
