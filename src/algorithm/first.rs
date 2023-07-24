use std::collections::{HashMap, HashSet};

use crate::grammar::core::Productions;
use crate::grammar::symbol::{GrammarSymbol, Nonterminal, Terminal};

pub fn first(grammar: &impl Productions) -> HashMap<Nonterminal, HashSet<Terminal>> {
    let terminals = grammar.terminals();
    let nonterminals = grammar.nonterminals();

    let capacity = terminals.len() + nonterminals.len();
    let mut first = HashMap::with_capacity(capacity);

    loop {
        let mut changed = false;

        for &head in nonterminals.iter() {
            for body in grammar.productions(head) {
                let prev = first.entry(head).or_insert(HashSet::new()).clone();

                let computed = { compute_first(body, &first) };
                first.entry(head).or_insert(HashSet::new()).extend(computed);

                let next = first[&head].clone();

                changed |= prev != next
            }
        }

        if !changed {
            break first;
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
