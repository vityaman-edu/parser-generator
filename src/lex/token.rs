trait Token {
    fn kind(&self) -> usize;
    fn text(&self) -> String;
}

struct BasicToken {
    kind: usize,
    text: String,
}

impl Token for BasicToken {
    fn kind(&self) -> usize {
        self.kind
    }

    fn text(&self) -> String {
        self.text.clone()
    }
}
