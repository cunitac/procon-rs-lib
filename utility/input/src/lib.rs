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
macro_rules! read {
    ($($arg:tt)*) => {
        $crate::try_read!($($arg)*).unwrap()
    };
}

#[macro_export]
macro_rules! try_read {
    (from $source:expr, [$type:tt; $len:expr]) => {
        (0..$len)
            .map(|_| $crate::try_read!(from $source, $type))
            .collect::<::std::option::Option<::std::vec::Vec<_>>>()
    };
    (from $source:expr, [$type:tt]) => {
        $crate::try_read!(from $source, usize)
            .and_then(|len| $crate::try_read!(from $source, [$type; len]))
    };
    (from $source:expr, ($($type:tt),* $(,)?)) => {
        $crate::try_read!(from $source, $($type),*)
    };
    (from $source:expr, $type:ty) => {
        <$type as $crate::FromSource>::from_source(&mut $source)
    };
    (from $source:expr, $($type:tt),* $(,)?) => {
        (|| Some(($($crate::try_read!(from $source, $type)?),*)))()
    };
    ($($rest:tt)*) => {
        $crate::STDIN_SOURCE.with(|stdin| $crate::try_read!(from stdin.borrow_mut(), $($rest)*))
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

pub trait FromSource {
    type Output;
    /// 読んでいる途中に `next_token` が `None` になった場合に限って `None` を返す
    fn from_source<R: Read>(source: &mut Source<R>) -> Option<Self::Output>;
}

impl<T: FromStr> FromSource for T {
    type Output = T;
    fn from_source<R: Read>(source: &mut Source<R>) -> Option<T> {
        Some(source.next_token()?.parse().ok().expect("failed to parse"))
    }
}

pub mod marker {
    use {
        super::{FromSource, Source},
        std::io::Read,
    };
    macro_rules! marker {
        ($name:ident, $output:ty, |$source:ident| $read:expr) => {
            pub enum $name {}
            impl FromSource for $name {
                type Output = $output;
                #[allow(unused_mut)]
                fn from_source<R: Read>(mut $source: &mut Source<R>) -> Option<$output> {
                    Some($read)
                }
            }
        };
    }
    marker!(Byte, u8, |s| try_read!(from s, char)? as u8);
    marker!(Bytes, Vec<u8>, |s| s.next_token()?.bytes().collect());
    marker!(Chars, Vec<char>, |s| s.next_token()?.chars().collect());
    marker!(Usize1, usize, |s| try_read!(from s, usize)? - 1);
    marker!(Isize1, isize, |s| try_read!(from s, isize)? - 1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let source = "
            42
            4.2 四二
            3 1 2 3
            b bytes chars 1 -5
            3 3 2 1
            try
        ";
        let mut source = Source::new(source.as_bytes());

        assert_eq!(read!(from source, usize), 42);
        assert_eq!(read!(from source, f64, String), (4.2, String::from("四二")));
        assert_eq!(read!(from source, [u32]), vec![1, 2, 3]);

        use super::marker::*;
        input!(
            from source,
            (byte, (bytes, (chars, usize1,)), isize1): (Byte, (Bytes, (Chars, Usize1),), Isize1),
            n: usize,
            a: [i32; n],
        );
        assert_eq!(byte, b'b');
        assert_eq!(bytes, b"bytes");
        assert_eq!(chars, vec!['c', 'h', 'a', 'r', 's']);
        assert_eq!(usize1, 0);
        assert_eq!(isize1, -6);
        assert_eq!(a, vec![3, 2, 1]);

        assert_eq!(try_read!(from source, String), Some(String::from("try")));
        assert_eq!(try_read!(from source, String), None);
    }
}
