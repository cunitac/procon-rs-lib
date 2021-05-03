use std::{
    cell::RefCell,
    io::{Read, Stdin},
    str::{FromStr, SplitWhitespace},
};

thread_local!(
    #[doc(hidden)]
    pub static STDIN_SOURCE: RefCell<Source<Stdin>> = RefCell::new(Source::new(std::io::stdin()));
);

#[macro_export]
macro_rules! input {
    (from $source:expr, [$type:tt; $len:expr]) => {
        (0..$len).map(|_| $crate::input!(from $source, $type)).collect::<Vec<_>>()
    };
    (from $source:expr, [$type:tt]) => {{
        let len = $crate::input!(from $source, usize);
        $crate::input!(from $source, [$type; len])
    }};
    (from $source:expr, ($($type:tt),* $(,)?)) => {
        ($($crate::input!(from $source, $type)),*)
    };
    (from $source:expr, $type:ty) => {
        $source.read::<$type>().unwrap()
    };
    (from $source:expr, $($type:tt),* $(,)?) => {
        ($($crate::input!(from $source, $type)),*)
    };
    ($($rest:tt)*) => {
        $crate::STDIN_SOURCE.with(|stdin| $crate::input!(from stdin.borrow_mut(), $($rest)*))
    };

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
    /// バッファが空なら一度 `load` して再度試す
    pub fn next_token(&mut self) -> Option<&str> {
        self.tokens.next().or_else(|| {
            self.load();
            self.tokens.next()
        })
    }
    /// `next_token` が `None` のときに限って `None`
    pub fn read<T: FromStr>(&mut self) -> Option<T> {
        Some(self.next_token()?.parse().ok().expect("failed to parse"))
    }
    /// まだ `next_token` 等で読み出していない入力は破棄される
    pub fn load(&mut self) {
        let mut input = String::new();
        self.source.read_to_string(&mut input).unwrap();
        self.tokens = Box::leak(input.into_boxed_str()).split_whitespace();
    }
    /// バッファが空でなければ panic
    /// `load` して試すことはない
    pub fn finish(&mut self) {
        if self.tokens.next().is_some() {
            panic!("not finished")
        }
    }
}

#[cfg(test)]
mod tests {}
