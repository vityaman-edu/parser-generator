use super::super::grammar::symbol::Terminal;

pub trait Token {
    fn kind(&self) -> usize;
    fn text(&self) -> String;
}

#[derive(Clone)]
pub struct BasicToken {
    kind: usize,
    text: String,
}

impl BasicToken {
    pub fn from(kind: usize, text: &str) -> BasicToken {
        BasicToken {
            kind: kind,
            text: text.to_owned()
        }
    }

    pub fn eof() -> BasicToken {
        BasicToken::from(Terminal::end().0, "<EOF>")
    }
}

impl Token for BasicToken {
    fn kind(&self) -> usize {
        self.kind
    }

    fn text(&self) -> String {
        self.text.clone()
    }
}
