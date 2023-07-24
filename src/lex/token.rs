trait Token {
    fn id(&self) -> usize;
    fn text(&self) -> String;
}

struct BasicToken {
    id: usize,
    text: String,
}

impl Token for BasicToken {
    fn id(&self) -> usize {
        self.id
    }

    fn text(&self) -> String {
        self.text.clone()
    }
}
