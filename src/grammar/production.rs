use super::symbol::Nonterminal;
use super::symbol::Symbol;

pub(super) struct Production {
    pub head: Nonterminal,
    pub body: Vec<Symbol>,
}
