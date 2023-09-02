pub trait Stream<T: Clone> {
    fn current(&mut self) -> &T;
    fn next(&mut self);
}

pub struct VectorStream<T: Clone> {
    position: usize,
    origin: Vec<T>,
}

impl<T: Clone> VectorStream<T> {
    pub fn from(vector: Vec<T>) -> VectorStream<T> {
        VectorStream {
            position: 0,
            origin: vector,
        }
    }
}

impl<T: Clone> Stream<T> for VectorStream<T> {
    fn current(&mut self) -> &T {
        &self.origin[self.position]
    }

    fn next(&mut self) {
        self.position += 1
    }
}
