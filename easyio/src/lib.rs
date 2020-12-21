use std::{
    io::{prelude::*, BufWriter},
    marker::PhantomData,
};

pub struct IO<R: Read, W: Write> {
    src: Source<R>,
    buf: BufWriter<W>,
}

impl IO<std::io::Stdin, std::io::Stdout> {
    pub fn stdio() -> Self {
        Self::new(std::io::stdin(), std::io::stdout())
    }
}
impl<R: Read, W: Write> IO<R, W> {
    /// `BufReader`、`BufWriter`を渡す必要はない
    pub fn new(r: R, o: W) -> Self {
        Self {
            src: Source::new(r),
            buf: BufWriter::new(o),
        }
    }
    pub fn i<I: Input>(&mut self) -> I::Item {
        self.src.i::<I>()
    }
    pub fn iiter<I: Input>(&mut self) -> IIter<R, I> {
        self.src.iiter()
    }
    pub fn o<O: Output>(&mut self, item: O) {
        self.buf
            .write_all(item.as_string().as_bytes())
            .expect("failed to write");
    }
    pub fn oln<O: Output>(&mut self, item: O) {
        self.o(item);
        self.buf.write_all(b"\n").expect("failed to write");
    }
}

pub struct Source<R> {
    src: R,
    buf: Vec<u8>,
    ptr: usize,
}

impl<R: Read> Source<R> {
    pub fn new(src: R) -> Self {
        Self {
            src,
            buf: Vec::new(),
            ptr: 0,
        }
    }
    pub fn next_token(&mut self) -> &str {
        self.skip_while(|b| b.is_ascii_whitespace());
        self.take_while(|b| !b.is_ascii_whitespace())
    }
    pub fn i<I: Input>(&mut self) -> I::Item {
        I::read_from(self)
    }
    pub fn iiter<I: Input>(&mut self) -> IIter<R, I> {
        IIter::new(self)
    }
    pub fn load(&mut self) {
        self.ptr = 0;
        self.buf.clear();
        self.src.read_to_end(&mut self.buf).expect("failed to read");
    }
    /// スキップして空になったらロードしてスキップ、それでも空ならパニック
    pub fn skip_while<P: FnMut(u8) -> bool>(&mut self, mut pred: P) {
        while self.ptr != self.buf.len() && pred(self.buf[self.ptr]) {
            debug_assert!(self.buf[self.ptr].is_ascii(), "invalid src; not ASCII");
            self.ptr += 1;
        }
        if self.is_empty() {
            self.load();
            while self.ptr != self.buf.len() && pred(self.buf[self.ptr]) {
                debug_assert!(self.buf[self.ptr].is_ascii(), "invalid src; not ASCII");
                self.ptr += 1;
            }
            assert!(!self.is_empty(), "source is empty");
        }
    }
    /// 空ならパニックする、ロードはしない
    pub fn take_while<P: FnMut(u8) -> bool>(&mut self, mut pred: P) -> &str {
        assert!(!self.is_empty(), "source is empty");
        let begin = self.ptr;
        while self.ptr != self.buf.len() && pred(self.buf[self.ptr]) {
            debug_assert!(self.buf[self.ptr].is_ascii(), "invalid src; not ASCII");
            self.ptr += 1;
        }
        unsafe { std::str::from_utf8_unchecked(&self.buf[begin..self.ptr]) }
    }
    pub fn is_empty(&self) -> bool {
        self.ptr == self.buf.len()
    }
}

/// `Iterator<Item=I::Item>`。空になったら`None`を返す。
pub struct IIter<'a, R, I>(&'a mut Source<R>, PhantomData<dyn Fn() -> I>);
impl<'a, R, I> IIter<'a, R, I> {
    fn new(src: &'a mut Source<R>) -> Self {
        Self(src, PhantomData)
    }
}
impl<R: Read, I: Input> Iterator for IIter<'_, R, I> {
    type Item = I::Item;
    fn next(&mut self) -> Option<I::Item> {
        self.skip_while(|c| c.is_ascii_whitespace());
        if self.0.is_empty() {
            None
        } else {
            Some(self.0.i::<I>())
        }
    }
}

pub trait Input {
    type Item;
    fn read_from<R: Read>(src: &mut Source<R>) -> Self::Item;
}

macro_rules! derive_input {
    ($($t:ty),*) => {$(
        impl Input for $t {
            type Item = $t;
            fn read_from<R: Read>(src: &mut Source<R>) -> Self {
                src.next_token().parse().ok().expect("parse error")
            }
        }
    )*};
}

derive_input!(f32, f64, i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, String);

pub enum Byte {}
impl Input for Byte {
    type Item = u8;
    fn read_from<R: Read>(src: &mut Source<R>) -> u8 {
        src.skip_while(|b| b.is_ascii_whitespace());
        src.ptr += 1;
        src.buf[src.ptr - 1]
    }
}

impl Input for char {
    type Item = char;
    fn read_from<R: Read>(src: &mut Source<R>) -> Self {
        src.i::<Byte>() as char
    }
}

impl Input for bool {
    type Item = bool;
    fn read_from<R: Read>(src: &mut Source<R>) -> Self {
        match src.i::<u8>() {
            b'0' => false,
            b'1' => true,
            _ => panic!("parse error, '0' and '1' are bool"),
        }
    }
}

macro_rules! impl_input_tuple {
    ($t0:ident) => {};
    ($t0:ident, $($t:ident),*) => {
        impl<$t0: Input, $($t: Input),*> Input for ($t0, $($t),*) {
            type Item = ($t0::Item, $($t::Item),*);
            fn read_from<R: Read>(src: &mut Source<R>) -> Self::Item {
                (src.i::<$t0>(), $(src.i::<$t>()),*)
            }
        }
        impl_input_tuple!($($t),*);
    };
}

impl_input_tuple!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11);

/// `i::<I>`を、直後が`'\n'`か`'\r'`か入力終了になるまでして、`Vec`で返す
pub enum VecLn<I> {
    Phantom(PhantomData<fn() -> I>),
}
impl<I: Input> Input for VecLn<I> {
    type Item = Vec<I::Item>;
    fn read_from<R: Read>(src: &mut Source<R>) -> Vec<I::Item> {
        std::iter::from_fn(|| {
            if src.is_empty() || src.buf[src.ptr] == b'\n' || src.buf[src.ptr] == b'\r' {
                None
            } else {
                Some(src.i::<I>())
            }
        })
        .collect()
    }
}

/// `i::<usize>`で長さを読んでから`i_vec::<I>`
pub enum VecN<I> {
    Phantom(PhantomData<dyn Fn() -> I>),
}
impl<I: Input> Input for VecN<I> {
    type Item = Vec<I::Item>;
    fn read_from<R: Read>(src: &mut Source<R>) -> Vec<I::Item> {
        let len = src.i::<usize>();
        src.iiter::<I>().take(len).collect()
    }
}

pub trait Output {
    fn as_string(self) -> String;
}

impl<T: ToString> Output for T {
    fn as_string(self) -> String {
        self.to_string()
    }
}

macro_rules! impl_writable_intoiter_with_delim {
    ($name:ident, $delim:expr) => {
        pub struct $name<T>(pub T);
        impl<T> Output for $name<T>
        where
            T: IntoIterator,
            <T as IntoIterator>::Item: Output,
        {
            fn as_string(self) -> String {
                self.0
                    .into_iter()
                    .by_ref()
                    .map(|s| s.as_string())
                    .collect::<Vec<_>>()
                    .join($delim)
            }
        }
    };
}

impl_writable_intoiter_with_delim!(Lines, "\n");
impl_writable_intoiter_with_delim!(Words, " ");
impl_writable_intoiter_with_delim!(Concat, "");

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_input() {
        let input = &b" 0 -1   2 string cde1\n3 1 2 3 \r 4 5 6"[..];
        let mut output = Vec::new();
        let mut io = IO::new(input, &mut output);
        assert_eq!(io.i::<i32>(), 0);
        assert_eq!(io.i::<i64>(), -1);
        assert_eq!(io.i::<Byte>(), b'2');
        assert_eq!(io.i::<String>(), String::from("string"));
        assert_eq!(io.i::<char>(), 'c');
        assert_eq!(
            io.iiter::<char>().take(2).collect::<Vec<_>>(),
            vec!['d', 'e']
        );
        assert_eq!(io.i::<i32>(), 1);
        assert_eq!(io.i::<VecN<i64>>(), vec![1, 2, 3]);
        assert_eq!(io.i::<VecLn<i64>>(), vec![4, 5, 6]);
        let a = vec![vec![1], vec![2, 3], vec![4, 5, 6]];
        io.o(Lines(a.iter().map(|row| Words(row))));
        std::mem::drop(io);
        assert_eq!(&output, b"1\n2 3\n4 5 6");
    }
}
