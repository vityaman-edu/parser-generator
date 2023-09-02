use std::marker::PhantomData;

use bimap::BiMap;
use map_macro::{hash_map, hash_set};
use pargen::codegen::ll1::ParserCodegen;
use pargen::grammar::core::BasicGrammar;
use pargen::grammar::symbol::GrammarSymbol;
use pargen::grammar::symbol::Nonterminal;
use pargen::grammar::symbol::Terminal;
use pargen::lex::stream::Stream;
use pargen::lex::stream::VectorStream;
use pargen::lex::token::{BasicToken, Token};

#[test]
pub fn ll1_arithmetic_expression_grammar_parser() {
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

    grammar.emit_parser(&mut std::io::stdout()).unwrap();

    let mut parser = GeneratedParser::reading(VectorStream::from(vec![
        BasicToken::from(id("N"), "5"),
        BasicToken::from(id("'+'"), "+"),
        BasicToken::from(id("N"), "10"),
        BasicToken::from(id("'*'"), "*"),
        BasicToken::from(id("N"), "100"),
        BasicToken::eof(),
    ]));

    parser.parse_1();
}

struct GeneratedParser<S, T>
where
    T: Token + Clone,
    S: Stream<T>,
{
    stream: S,
    __phantom: PhantomData<T>,
}

impl<S, T> GeneratedParser<S, T>
where
    T: Token + Clone,
    S: Stream<T>,
{
    fn reading(stream: S) -> GeneratedParser<S, T> {
        GeneratedParser {
            stream,
            __phantom: PhantomData,
        }
    }

    fn parse_5(&mut self) {
        match self.stream.current().kind() {
            9 => {
                assert!(self.stream.current().kind() == 9);
                println!("Terminal 9");
                self.stream.next();
                self.parse_1();
                assert!(self.stream.current().kind() == 8);
                println!("Terminal 8");
                self.stream.next();
            }
            10 => {
                assert!(self.stream.current().kind() == 10);
                println!("Terminal 10");
                self.stream.next();
            }
            usize::MAX => println!("Terminal <EOF>"),
            _ => panic!("Invalid Input!"),
        }
    }

    fn parse_1(&mut self) {
        match self.stream.current().kind() {
            10 | 9 => {
                self.parse_3();
                self.parse_2();
            }
            usize::MAX => println!("Terminal <EOF>"),
            _ => panic!("Invalid Input!"),
        }
    }

    fn parse_2(&mut self) {
        match self.stream.current().kind() {
            6 => {
                assert!(self.stream.current().kind() == 6);
                println!("Terminal 6");
                self.stream.next();
                self.parse_3();
                self.parse_2();
            }
            8 => {
                // Skip an epsilon grammar symbol
            }
            usize::MAX => println!("Terminal <EOF>"),
            _ => panic!("Invalid Input!"),
        }
    }

    fn parse_4(&mut self) {
        match self.stream.current().kind() {
            7 => {
                assert!(self.stream.current().kind() == 7);
                println!("Terminal 7");
                self.stream.next();
                self.parse_5();
                self.parse_4();
            }
            6 | 8 => {
                // Skip an epsilon grammar symbol
            }
            usize::MAX => println!("Terminal <EOF>"),
            _ => panic!("Invalid Input!"),
        }
    }

    fn parse_3(&mut self) {
        match self.stream.current().kind() {
            10 | 9 => {
                self.parse_5();
                self.parse_4();
            }
            usize::MAX => println!("Terminal <EOF>"),
            _ => panic!("Invalid Input!"),
        }
    }
}
