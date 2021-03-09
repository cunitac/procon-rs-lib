use {
    marker::*,
    std::{io::prelude::*, marker::PhantomData},
};

pub struct Source<R> {
    source: R,
    buffer: Vec<u8>,
    cursor: usize,
}

impl<R: Read> Source<R> {
    pub fn new(source: R) -> Self {
        Self {
            source,
            buffer: vec![],
            cursor: 0,
        }
    }
    pub fn is_empty(&self) -> bool {
        debug_assert!(self.cursor <= self.buffer.len());
        self.cursor == self.buffer.len()
    }
    /// 初期化して再読み込み、読んだ長さを返す
    pub fn load(&mut self) -> usize {
        self.buffer.clear();
        self.cursor = 0;
        self.source.read_to_end(&mut self.buffer).unwrap()
    }
    pub fn read<I: FromSource>(&mut self) -> I::Item {
        I::read_from(self).unwrap()
    }
    pub fn next_token(&mut self) -> Option<&str> {
        self.skip_while(|b| b.is_ascii_whitespace());
        if self.is_empty() {
            if self.load() == 0 {
                return None;
            }
            return self.next_token();
        }
        let start = self.cursor;
        self.skip_while(|b| !b.is_ascii_whitespace());
        // SAFETY: ぜんぶ ASCII なのは `skip_while` で検証している
        Some(unsafe { std::str::from_utf8_unchecked(&self.buffer[start..self.cursor]) })
    }
    pub fn skip_while(&mut self, mut pred: impl FnMut(u8) -> bool) {
        while self.now().map_or(false, &mut pred) {
            assert!(self.now().unwrap().is_ascii(), "not ASCII");
            self.cursor += 1;
        }
    }
    /// cursor が指している文字
    fn now(&self) -> Option<u8> {
        self.buffer.get(self.cursor).copied()
    }
}

pub trait FromSource {
    type Item;
    /// `source` が空だった場合に限って `None`。
    fn read_from<R: Read>(source: &mut Source<R>) -> Option<Self::Item>;
}

macro_rules! impl_tuple_input {
    () => {};
    ($t0:ident, $($t:ident,)*) => {
        impl<$t0: Input, $($t: Input),*> Input for ($t0, $($t),*) {
            type Item = ($t0::Item, $($t::Item),*);
            fn read_from<R: Read>(source: &mut Source<R>) -> Option<Self::Item> {
                Some(($t0::read_from(source)?, $($t::read_from(source)?),*))
            }
        }
        impl_tuple_input!($($t,)*);
    };
}

impl_tuple_input!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11,);

macro_rules! impl_primitive_input {
    ($($t:ty),* $(,)+) => {$(
        impl Input for $t {
            type Item = $t;
            fn read_from<R: Read>(source: &mut Source<R>) -> Option<$t> {
                source.next_token().map(|s| s.parse().unwrap())
            }
        }
    )*};
}

impl_primitive_input! {
    String,
    f32, f64,
    i8, i16, i32, i64, i128, isize,
    u8, u16, u32, u64, u128, usize,
}

impl FromSource for char {
    type Item = char;
    fn read_from<R: Read>(source: &mut Source<R>) -> Option<char> {
        Byte::read_from(source).map(|v| v as char)
    }
}

pub mod marker {
    use super::*;
    pub enum Usize1 {}
    impl FromSource for Usize1 {
        type Item = usize;
        fn read_from<R: Read>(source: &mut Source<R>) -> Option<usize> {
            usize::read_from(source).map(|u| u - 1)
        }
    }
    pub enum Byte {}
    impl FromSource for Byte {
        type Item = u8;
        fn read_from<R: Read>(source: &mut Source<R>) -> Option<u8> {
            source.skip_while(|b| b.is_ascii_whitespace());
            if source.is_empty() {
                source.load();
            }
            source.skip_while(|b| b.is_ascii_whitespace());
            let ret = source.now();
            source.cursor += 1;
            ret
        }
    }
    pub enum Bytes {}
    impl FromSource for Bytes {
        type Item = Vec<u8>;
        fn read_from<R: Read>(source: &mut Source<R>) -> Option<Vec<u8>> {
            source.next_token().map(|s| s.bytes().collect())
        }
    }
    pub enum Chars {}
    impl FromSource for Chars {
        type Item = Vec<char>;
        fn read_from<R: Read>(source: &mut Source<R>) -> Option<Vec<char>> {
            source.next_token().map(|s| s.chars().collect())
        }
    }
}

macro_rules! alias_input {
    ($($name:ident, $input:ty);* $(;)+) => {$(
        pub fn $name(&mut self) -> <$input as Input>::Item {
            self.read::<$input>()
        }
    )*};
}

impl<R: Read> Source<R> {
    alias_input! {
        u8, u8; u16, u16; u32, u32; u64, u64; u128, u128; usize, usize;
        i8, i8; i16, i16; i32, i32; i64, i64; i128, i128; isize, isize;
        f32, f32; f64, f64;
        string, String; char, char;
        usize1, Usize1; byte, Byte;
        bytes, Bytes; chars, Chars;
    }
}

pub struct Iter<'a, I, R> {
    source: &'a mut Source<R>,
    _phantom: PhantomData<fn() -> I>,
}

impl<'a, I: FromSource, R: Read> Iter<'a, I, R> {
    fn new(source: &'a mut Source<R>) -> Self {
        Self {
            source,
            _phantom: PhantomData,
        }
    }
}

impl<R: Read, I: FromSource> Iterator for Iter<'_, I, R> {
    type Item = I::Item;
    fn next(&mut self) -> Option<I::Item> {
        I::read_from(&mut self.source)
    }
}

impl<R: Read> Source<R> {
    pub fn iter<I: FromSource>(&mut self) -> Iter<'_, I, R> {
        Iter::new(self)
    }
    pub fn vec<I: FromSource>(&mut self, len: usize) -> Vec<I::Item> {
        self.iter::<I>().take(len).collect()
    }
}

#[cfg(test)]
mod test {
    use {super::*, std::io::Cursor};

    #[test]
    #[allow(clippy::float_cmp)]
    fn test() {
        let source = r"
            1 2 3 4 5
            6 7 8 9 10
            one two three
        ";
        let mut source = Source::new(Cursor::new(source));
        assert_eq!(source.usize(), 1);
        assert_eq!(source.isize(), 2);
        assert_eq!(source.f64(), 3.0);
        assert_eq!(source.char(), '4');
        assert_eq!(source.byte(), b'5');
        assert_eq!(source.vec::<u32>(5), vec![6, 7, 8, 9, 10]);
        assert_eq!(source.string(), "one");
        assert_eq!(source.char(), 't');
        assert_eq!(source.chars(), vec!['w', 'o']);
        assert_eq!(source.byte(), b't');
        assert_eq!(source.bytes(), b"hree");
    }
}
