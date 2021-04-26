use std::{
    io::Read,
    iter::FromIterator,
    marker::PhantomData,
    str::{FromStr, SplitWhitespace},
};

pub trait Reader<T> {
    fn next(&mut self) -> T;
    fn collect(&mut self, len: usize) -> Collect<Self, T> {
        Collect {
            source: self,
            len,
            _phantom: PhantomData,
        }
    }
}

pub struct Source<R> {
    tokens: SplitWhitespace<'static>,
    source: R,
}

impl<R: Read> Source<R> {
    pub fn new(source: R) -> Self {
        Self {
            tokens: "".split_whitespace(),
            source,
        }
    }
    pub fn next_token(&mut self) -> Option<&str> {
        self.tokens.next().or_else(|| {
            self.load();
            self.tokens.next()
        })
    }
    pub fn load(&mut self) {
        let mut input = String::new();
        self.source.read_to_string(&mut input).unwrap();
        self.tokens = Box::leak(input.into_boxed_str()).split_whitespace();
    }
}

impl<R: Read, T: FromStr> Reader<T> for Source<R> {
    fn next(&mut self) -> T {
        self.next_token().unwrap().parse().ok().unwrap()
    }
}

pub struct Collect<'a, R: ?Sized, T> {
    source: &'a mut R,
    len: usize,
    _phantom: PhantomData<T>,
}

impl<T, R: Reader<T>, A: FromIterator<T>> Reader<A> for Collect<'_, R, T> {
    fn next(&mut self) -> A {
        (0..self.len).map(|_| self.source.next()).collect()
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_collect() {
        let mut src = Source::new(&b" 1 2 3 4 5 6"[..]);
        let v: Vec<Vec<u32>> = src.collect(3).collect(2).next();
        assert_eq!(v, vec![vec![1, 2, 3], vec![4, 5, 6]]);
    }
}
