use bimap::BiMap;
use map_macro::{hash_map, hash_set};
use pargen::algorithm::IsLL1;
use std::collections::HashSet;

use pargen::algorithm::FirstSet;
use pargen::algorithm::FollowSet;
use pargen::grammar::core::BasicGrammar;
use pargen::grammar::symbol::GrammarSymbol;
use pargen::grammar::symbol::Nonterminal;
use pargen::grammar::symbol::Terminal;

#[test]
pub fn arithmetic_expression_grammar() {
    // E -> E + T
    // E -> T
    // T -> T * F
    // T -> F
    // F -> ( E )
    // F -> N

    let view = BiMap::from_iter([
        (0usize, "''"),
        (1usize, "E"),
        (3usize, "T"),
        (5usize, "F"),
        (6usize, "+"),
        (7usize, "*"),
        (8usize, ")"),
        (9usize, "("),
        (10usize, "N"),
        (usize::MAX, "$"),
    ]);

    let id = |name| *view.get_by_right(name).unwrap();
    let name = |id| *view.get_by_left(&id).unwrap();
    let rule = |name| Nonterminal(id(name));
    let r = |name| GrammarSymbol::Nonterminal(Nonterminal(id(name)));
    let t = |name| GrammarSymbol::Terminal(Terminal(id(name)));

    let grammar = BasicGrammar {
        start: rule("E"),
        rules: hash_map! {
                rule("E") => hash_set! {
                    vec![r("E"), t("+"), r("T")],
                    vec![r("T")],
                },
                rule("T") => hash_set! {
                    vec![r("T"), t("*"), r("F")],
                    vec![r("F")],
                },
                rule("F") => hash_set! {
                    vec![t("("), r("E"), t(")")],
                    vec![t("N")],
                },
        },
    };

    assert!(!grammar.is_ll1());

    let first_set = FirstSet::build(&grammar);
    let first = |rule_name| {
        first_set
            .of_symbol(rule(rule_name).into())
            .iter()
            .map(|terminal| name(terminal.0))
            .collect::<HashSet<_>>()
    };
    assert_eq!(first("E"), hash_set! {"(", "N" });
    assert_eq!(first("T"), hash_set! { "(", "N" });
    assert_eq!(first("F"), hash_set! { "(", "N" });

    let follow_set = FollowSet::build(&grammar, &first_set);
    let follow = |rule_name| {
        follow_set
            .of(rule(rule_name))
            .iter()
            .map(|terminal| name(terminal.0))
            .collect::<HashSet<_>>()
    };
    assert_eq!(follow("E"), hash_set! { "$", "+", ")" });
    assert_eq!(follow("T"), hash_set! { "$", "+", "*", ")" });
    assert_eq!(follow("F"), hash_set! { "$", "+", "*", ")" });
}

#[test]
pub fn ll1_arithmetic_expression_grammar() {
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
        (usize::MAX, "$"),
    ]);

    let id = |name| *view.get_by_right(name).unwrap();
    let name = |id| *view.get_by_left(&id).unwrap();
    let rule = |name| Nonterminal(id(name));
    let r = |name| GrammarSymbol::Nonterminal(Nonterminal(id(name)));
    let t = |name| GrammarSymbol::Terminal(Terminal(id(name)));

    let grammar = BasicGrammar {
        start: rule("E"),
        rules: hash_map! {
                rule("E") => hash_set! {
                    vec![r("T"), r("E'")],
                },
                rule("E'") => hash_set! {
                    vec![t("'+'"), r("T"), r("E'")],
                    vec![t("''")],
                },
                rule("T") => hash_set! {
                    vec![r("F"), r("T'")],
                },
                rule("T'") => hash_set! {
                    vec![t("'*'"), r("F"), r("T'")],
                    vec![t("''")],
                },
                rule("F") => hash_set! {
                    vec![t("'('"), r("E"), t("')'")],
                    vec![t("N")],
                },
        },
    };

    assert!(grammar.is_ll1());

    let first_set = FirstSet::build(&grammar);
    let first = |rule_name| {
        first_set
            .of_symbol(rule(rule_name).into())
            .iter()
            .map(|terminal| name(terminal.0))
            .collect::<HashSet<_>>()
    };
    assert_eq!(first("E"), hash_set! {"'('", "N" });
    assert_eq!(first("E'"), hash_set! { "'+'", "''" });
    assert_eq!(first("T"), hash_set! { "'('", "N" });
    assert_eq!(first("T'"), hash_set! { "'*'", "''" });
    assert_eq!(first("F"), hash_set! { "'('", "N" });

    let follow_set = FollowSet::build(&grammar, &first_set);
    let follow = |rule_name| {
        follow_set
            .of(rule(rule_name))
            .iter()
            .map(|terminal| name(terminal.0))
            .collect::<HashSet<_>>()
    };
    assert_eq!(follow("E"), hash_set! { "$", "')'" });
    assert_eq!(follow("E'"), hash_set! { "$", "')'" });
    assert_eq!(follow("T"), hash_set! { "$", "')'", "'+'" });
    assert_eq!(follow("T'"), hash_set! { "$", "')'", "'+'" });
    assert_eq!(follow("F"), hash_set! { "$", "')'", "'+'", "'*'" });
}
