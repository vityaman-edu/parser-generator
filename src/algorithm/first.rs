use std::collections::{HashMap, HashSet};

use crate::grammar::core::Grammar;
use crate::grammar::symbol::{GrammarSymbol, Nonterminal, Terminal};

#[derive(Debug)]
pub struct FirstSet {
    map: HashMap<Nonterminal, HashSet<Terminal>>,
}

impl FirstSet {
    pub fn of_symbol(&self, symbol: GrammarSymbol) -> HashSet<Terminal> {
        match symbol {
            GrammarSymbol::Nonterminal(symbol) => self.map[&symbol].clone(),
            GrammarSymbol::Terminal(_) => todo!(),
        }
    }

    pub fn of_seq(&self, seq: &[GrammarSymbol]) -> HashSet<Terminal> {
        let epsilon = Terminal::epsilon();
        match seq {
            [] => HashSet::new(),
            [GrammarSymbol::Terminal(head)] if *head == epsilon => HashSet::from([epsilon]),
            [GrammarSymbol::Terminal(head), ..] => HashSet::from([*head]),
            [head @ GrammarSymbol::Nonterminal(_), ..] => self.of_symbol(*head),
        }
    }
}

impl FirstSet {
    pub fn build(g: &impl Grammar) -> FirstSet {
        let mut first = HashMap::with_capacity(g.nonterminals().len());

        loop {
            let prev = &first.clone();

            for &head in g.nonterminals().iter() {
                for body in g.alternatives_for(head) {
                    let computed = { compute_first(body, &first) };
                    first.entry(head).or_insert(HashSet::new()).extend(computed);
                }
            }

            let next = &first;

            if prev == next {
                break FirstSet { map: first };
            }
        }
    }
}

fn compute_first(
    body: &[GrammarSymbol],
    first: &HashMap<Nonterminal, HashSet<Terminal>>,
) -> HashSet<Terminal> {
    let epsilon = Terminal::epsilon();
    match body {
        [] => HashSet::new(),
        [GrammarSymbol::Terminal(head)] if *head == epsilon => HashSet::from([epsilon]),
        [GrammarSymbol::Terminal(head), ..] => HashSet::from([*head]),
        [GrammarSymbol::Nonterminal(head), tail @ ..] => {
            let first_head = first.get(head).cloned().unwrap_or_default();
            let addition = if first_head.contains(&epsilon) {
                compute_first(tail, first)
            } else {
                HashSet::new()
            };
            &(&first_head - &[epsilon].into()) | &addition
        }
    }
}
