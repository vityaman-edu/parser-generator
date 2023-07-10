use self::core::Grammar;
use self::production::Production;
use self::symbol::Nonterminal;
use self::symbol::SymbolConvertable;
use self::symbol::Terminal;
mod core;
mod production;
mod symbol;

pub fn test() {
    let e = Nonterminal(1);
    let a = Terminal(2);
    let eps = Terminal(3);
    let grammar = Grammar {
        start: e,
        productions: Vec::from([
            Production {
                head: e,
                body: Vec::from([a.to_symbol(), e.to_symbol()]),
            },
            Production {
                head: e,
                body: Vec::from([eps.to_symbol()]),
            },
        ]),
    };
    
}
