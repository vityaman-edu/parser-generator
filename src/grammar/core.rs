use super::production::Production;
use super::symbol::Nonterminal;

pub(super) struct Grammar {
    pub start: Nonterminal,
    pub productions: Vec<Production>,
}
