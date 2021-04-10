use std::{io::Read, str::SplitWhitespace};

pub struct Scanner<R> {
    tokens: SplitWhitespace<'static>,
    source: R,
}

impl<R: Read> Scanner<R> {
    pub fn new(source: R) -> Self {
        Self {
            tokens: "".split_whitespace(),
            source,
        }
    }
    pub fn load(&mut self) {
        let mut input = String::new();
        self.source.read_to_string(&mut input).unwrap();
        self.tokens = Box::leak(input.into_boxed_str()).split_whitespace();
    }
}
