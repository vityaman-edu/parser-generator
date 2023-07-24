use super::{FirstSet, FollowSet};
use crate::grammar::{core::Grammar, symbol::Terminal};

pub trait IsLL1 {
    fn is_ll1(&self) -> bool;
}

impl<T: Grammar> IsLL1 for T {
    fn is_ll1(&self) -> bool {
        let first = FirstSet::build(self);
        let follow = FollowSet::build(self, &first);

        for head in self.nonterminals() {
            let alts: Vec<_> = self.alternatives_for(head).iter().collect();
            for i in 0..alts.len() {
                for j in (i + 1)..alts.len() {
                    let (a, b) = &(alts[i], alts[j]);
                    let (a, b) = &(first.of_seq(a), first.of_seq(b));

                    if a.intersection(b).count() != 0 {
                        return false;
                    }

                    if b.contains(&Terminal::epsilon())
                        && b.intersection(follow.of(head)).count() != 0
                    {
                        return false;
                    }
                }
            }
        }

        true
    }
}
