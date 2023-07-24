use std::collections::{HashMap, HashSet};

use crate::grammar::{
    core::Grammar,
    symbol::{GrammarSymbol, Nonterminal, Terminal},
};

use super::first::FirstSet;

#[derive(Debug)]
pub struct FollowSet {
    map: HashMap<Nonterminal, HashSet<Terminal>>,
}

impl FollowSet {
    pub fn of(&self, symbol: Nonterminal) -> &HashSet<Terminal> {
        &self.map[&symbol]
    }
}

impl FollowSet {
    pub fn build(g: &impl Grammar, first: &FirstSet) -> FollowSet {
        let mut follow = HashMap::with_capacity(g.nonterminals().len());

        follow.insert(g.start(), HashSet::from([Terminal::end()]));

        loop {
            let prev = &follow.clone();

            for head in g.nonterminals() {
                for body in g.alternatives_for(head) {
                    for i in 0..body.len() {
                        if let GrammarSymbol::Nonterminal(symbol) = body[i] {
                            let first_gamma = first.of_seq(&body[(i + 1)..]);
                            let eps = Terminal::epsilon();
                            follow
                                .entry(symbol)
                                .or_insert(HashSet::new())
                                .extend(&first_gamma - &HashSet::from([eps]));
                            if first_gamma.contains(&eps) || first_gamma.is_empty() {
                                let follow_head = follow.entry(head).or_default().clone();
                                follow
                                    .entry(symbol)
                                    .or_insert(HashSet::new())
                                    .extend(follow_head.iter());
                            }
                        }
                    }
                }
            }

            let next = &follow;

            if prev == next {
                break FollowSet { map: follow };
            }
        }
    }
}
