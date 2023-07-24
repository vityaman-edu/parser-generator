pub trait Stream<T>
where
    T: Clone,
{
    fn current(&mut self) -> &T;
    fn next(&mut self);
}

pub struct VectorStream<T>
where
    T: Clone,
{
    position: usize,
    origin: Vec<T>,
}

impl<T: Clone> VectorStream<T> {
    pub fn from(vector: Vec<T>) -> VectorStream<T> {
        VectorStream { position: 0, origin: vector }
    }
}

impl<T> Stream<T> for VectorStream<T>
where
    T: Clone,
{
    fn current(&mut self) -> &T {
        &self.origin[self.position]
    }

    fn next(&mut self) {
        self.position += 1
    }
}
