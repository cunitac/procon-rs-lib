//! ```no_run
//! use minimal_input::*;
//!
//! let stdin = std::io::stdin();
//! let mut stdin = Source::new(stdin.lock());
//! macro_rules! read { ($($arg:tt)*) => { read_from!(stdin; $($arg)*) } }
//! macro_rules! input { ($($arg:tt)*) => { input_from!(stdin; $($arg)*) } }
//!
//! let (a, b, c) = read!(u32, i32, String);
//! input!(d: u32, e: i32, f: String);
//! input!((g, h, i): (u32, i32, String));
//!
//! let x = read!([u32; 3]); // x: Vec<u32>
//! let y = read!([u32]);    // input!(len: usize, y: [u32; len]); とだいたい等価、もちろん len は見えない
//!
//! let z = read!([(([u32; 4], i32), [i32]); 36]); // 入れ子も可
//!```

use std::{
    any::type_name,
    io::BufRead,
    str::{FromStr, SplitWhitespace},
};

#[macro_export]
macro_rules! read_from {
    ($source:expr; [$type:tt; $len:expr]) => {
        (0..$len).map(|_| $crate::read_from!($source; $type))
        .collect::<::std::vec::Vec<_>>()
    };
    ($source:expr; [$type:tt]) => {{
        let len = $source.parse_next::<usize>();
        $crate::read_from!($source; [$type; len])
    }};
    ($source:expr; ($($type:tt),* $(,)?)) => {{
        ($($crate::read_from!($source; $type)),*)
    }};
    ($source:expr; $type:ty) => {
        $source.parse_next::<$type>()
    };
    ($source:expr; $($type:tt),* $(,)?) => {{
        ($($crate::read_from!($source; $type)),*)
    }};
}

#[macro_export]
macro_rules! input_from {
    ($source:expr; $($name:tt: $type:tt),* $(,)?) => {
        $(let $name = $crate::read_from!($source; $type);)*
    };
}

pub struct Source<R> {
    tokens: SplitWhitespace<'static>,
    source: R,
}

impl<R: BufRead> Source<R> {
    pub fn new(source: R) -> Self {
        Self {
            tokens: "".split_whitespace(),
            source,
        }
    }
    pub fn next_token(&mut self) -> &str {
        self.tokens.next().unwrap_or_else(|| {
            self.load();
            self.tokens.next().expect("no token")
        })
    }
    pub fn parse_next<T: FromStr>(&mut self) -> T {
        let token = self.tokens.next().expect("tokens is empty");
        token
            .parse()
            .unwrap_or_else(|_| panic!("failed to parse \"{}\" as {}", token, type_name::<T>()))
    }
    pub fn load(&mut self) {
        let mut buf = String::new();
        self.source.read_line(&mut buf).unwrap();
        let buf = Box::leak(buf.into_boxed_str());
        self.tokens = buf.split_whitespace();
    }
}
