mod algorithm;
mod grammar;
mod symbol;
mod vocabulary;

use std::collections::HashMap;
use std::collections::HashSet;

use self::grammar::BasicGrammar;
use self::symbol::GrammarSymbol;
use self::symbol::Nonterminal;
use self::symbol::Terminal;

pub fn test() {
    // E  -> T E'
    // E' -> '+' T E'
    // E' -> ''
    // T  -> F T'
    // T' -> '*' F T'
    // T' -> ''
    // F  -> '(' E ')'
    // F  -> N

    let view = BiMap::from_iter([
        (0usize, "''"),
        (1usize, "E"),
        (2usize, "E'"),
        (3usize, "T"),
        (4usize, "T'"),
        (5usize, "F"),
        (6usize, "'+'"),
        (7usize, "'*'"),
        (8usize, "')'"),
        (9usize, "'('"),
        (10usize, "N"),
    ]);

    let id = |name| *view.get_by_right(name).unwrap();
    let name = |id| *view.get_by_left(&id).unwrap();
    let rule = |name| Nonterminal(id(name));
    let r = |name| GrammarSymbol::Nonterminal(Nonterminal(id(name)));
    let t = |name| GrammarSymbol::Terminal(Terminal(id(name)));

    let grammar = BasicGrammar {
        rules: HashMap::from([
            // E  -> T E'
            (rule("E"), HashSet::from([vec![r("T"), r("E'")]])),
            // E' -> '+' T E'
            // E' -> ''
            (
                rule("E'"),
                HashSet::from([vec![t("'+'"), r("T"), r("E'")], vec![t("''")]]),
            ),
            // T  -> F T'
            (rule("T"), HashSet::from([vec![r("F"), r("T'")]])),
            // T' -> '*' F T'
            // T' -> ''
            (
                rule("T'"),
                HashSet::from([vec![t("'*'"), r("F"), r("T'")], vec![t("''")]]),
            ),
            // F  -> '(' E ')'
            // F  -> N
            (
                rule("F"),
                HashSet::from([vec![t("'('"), r("E"), t("')'")], vec![t("N")]]),
            ),
        ]),
    };

    for (rule, first) in algorithm::first(&grammar) {
        println!(
            "FIRST({rule}) = {first:?}",
            rule = name(rule.id()),
            first = first
                .iter()
                .map(|terminal| name(terminal.0))
                .collect::<HashSet<_>>()
        )
    }
}

