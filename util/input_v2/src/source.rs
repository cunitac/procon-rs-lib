use crate::parser::Parser;
use std::io::{BufRead, BufReader};

pub struct Source<R> {
    source: R,
    buffer: Buffer,
}

impl<R: BufRead> Source<R> {
    /// バッファが空の [`Source`] を生成する。
    pub fn new(source: R) -> Self {
        Self {
            source,
            buffer: Default::default(),
        }
    }
    pub fn load_once(&mut self) {
        let mut buffer = String::new();
        self.source.read_to_string(&mut buffer).unwrap();
        self.buffer = Buffer::new(buffer);
    }
    pub fn load(&mut self) {
        while self.buffer.is_empty() {
            let mut buffer = String::new();
            if self.source.read_line(&mut buffer).unwrap() == 0 {
                return; // EOF
            };
            self.buffer = Buffer::new(buffer);
        }
    }
    pub fn next_token(&mut self) -> &str {
        self.load();
        self.buffer.next_token().expect("No next token")
    }
    pub fn next_char(&mut self) -> char {
        self.load();
        self.buffer.next_char().expect("No next char")
    }
    pub fn parse<T, P: Parser<T>>(&mut self, parser: P) -> T {
        parser.parse(self)
    }
}

impl<'a> From<&'a str> for Source<BufReader<&'a [u8]>> {
    fn from(source: &'a str) -> Self {
        Self::new(BufReader::new(source.as_bytes()))
    }
}

#[derive(Default)]
pub(crate) struct Buffer {
    buffer: Box<str>,
    cursor: usize,
}

impl Buffer {
    pub fn new(buffer: String) -> Self {
        Self {
            buffer: buffer.into(),
            cursor: 0,
        }
    }
    pub fn is_empty(&mut self) -> bool {
        self.skip_whitespace();
        self.cursor == self.buffer.len()
    }
    pub fn next_token(&mut self) -> Option<&str> {
        self.skip_whitespace();
        let buffer = &self.buffer[self.cursor..];
        let token_len = buffer
            .find(char::is_whitespace)
            .unwrap_or_else(|| buffer.len());
        if token_len == 0 {
            None
        } else {
            self.cursor += token_len;
            Some(&buffer[..token_len])
        }
    }
    pub fn next_char(&mut self) -> Option<char> {
        self.skip_whitespace();
        let next = self.buffer[self.cursor..].chars().next()?;
        self.cursor += next.len_utf8();
        Some(next)
    }
    fn skip_whitespace(&mut self) {
        if let Some(i) = self.buffer[self.cursor..].find(|c: char| !c.is_whitespace()) {
            self.cursor += i;
        } else {
            self.cursor = self.buffer.len();
        }
    }
}
