use std::{
    collections::HashSet,
    io::{self, Write},
};

use crate::{
    algorithm::{FirstSet, FollowSet, IsLL1},
    grammar::{
        core::Grammar,
        symbol::{GrammarSymbol, Nonterminal, Terminal},
    },
};

pub trait ParserCodegen {
    fn emit_parser(&self, out: &mut impl Write) -> Result<(), io::Error>;
}

impl<T: Grammar> ParserCodegen for T {
    fn emit_parser(&self, o: &mut impl Write) -> Result<(), io::Error> {
        debug_assert!(self.is_ll1());

        let first_set = FirstSet::build(self);
        let follow_set = FollowSet::build(self, &first_set);
        let first = |head: Nonterminal, body: &Vec<GrammarSymbol>| {
            let end = Terminal::end();
            let epsilon = Terminal::epsilon();
            let first_head = first_set.of_seq(body);
            if first_head.contains(&epsilon) {
                &(&first_head | &follow_set.of(head)) - &HashSet::from([epsilon, end])
            } else {
                first_head
            }
        };

        for head in self.nonterminals() {
            writeln!(o, "fn parse_{nonterm}(&mut self) {{", nonterm = head.0)?;
            writeln!(o, "  match (stream.current().kind()) {{")?;
            for body in self.alternatives_for(head) {
                let condition = first(head, body)
                    .iter()
                    .map(|t| t.0.to_string())
                    .collect::<Vec<String>>()
                    .join(" | ");

                writeln!(o, "    {condition} => {{", condition = condition)?;

                for symbol in body {
                    match symbol {
                        GrammarSymbol::Terminal(term) => {
                            writeln!(o, "      assert!(stream.current().kind() == {});", term.0)?;
                            writeln!(o, "      println!(\"Terminal {})\"", term.0)?;
                            writeln!(o, "      stream.next();")?;
                        }
                        GrammarSymbol::Nonterminal(nonterm) => {
                            writeln!(o, "      self.parse_{}()", nonterm.0)?;
                        }
                    }
                }

                writeln!(o, "    }}")?;
            }
            writeln!(o, "    _ => panic!(\"Invalid Input!\")")?;
            writeln!(o, "  }}")?;

            writeln!(o, "}}")?;
            writeln!(o)?;
        }

        Ok(())
    }
}
