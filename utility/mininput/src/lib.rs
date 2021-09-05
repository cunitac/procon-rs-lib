use std::{
    cell::RefCell,
    fmt::Debug,
    io::{BufRead, BufReader, Stdin},
    str::{FromStr, SplitWhitespace},
};
#[macro_export]
macro_rules! read {
    (from $source:expr, [$type:tt; $len:expr]) => {
        (0..$len)
            .map(|_| $crate::read!(from $source, $type))
            .collect::<::std::vec::Vec<_>>()
    };
    (from $source:expr, [$type:tt]) => {{
        let len = $crate::read!(from $source, usize);
        $crate::read!(from $source, [$type; len])
    }};
    (from $source:expr, ($($type:tt),* $(,)?)) => {
        ($($crate::read!(from $source, $type)),*)
    };
    (from $source:expr, $type:ty) => {
        $source.read::<$type, _>()
    };
    ($($rest:tt)*) => {
        $crate::STDIN_SOURCE.with(|stdin| {
            let mut source = stdin.borrow_mut();
            $crate::read!(from source, $($rest)*)
        })
    };
}
#[macro_export]
macro_rules! input {
    (from $source:expr, $($name:tt: $type:tt),* $(,)?) => {
        $(let $name = $crate::read!(from $source, $type);)*
    };
    ($($name:tt: $type:tt),* $(,)?) => {
        $(let $name = $crate::read!($type);)*
    };
}
thread_local!(
    pub static STDIN_SOURCE: RefCell<Source> = RefCell::new(Source::new());
);
pub struct Source {
    tokens: SplitWhitespace<'static>,
    source: BufReader<Stdin>,
}
#[allow(clippy::new_without_default)]
impl Source {
    pub fn new() -> Self {
        Self {
            tokens: "".split_whitespace(),
            source: BufReader::new(std::io::stdin()),
        }
    }
    pub fn read<T: FromStr<Err = E>, E: Debug>(&mut self) -> T {
        self.tokens
            .next()
            .unwrap_or_else(|| {
                let mut buf = String::new();
                self.source.read_line(&mut buf).unwrap();
                self.tokens = Box::leak(buf.into_boxed_str()).split_whitespace();
                self.tokens.next().unwrap()
            })
            .parse()
            .unwrap()
    }
}
