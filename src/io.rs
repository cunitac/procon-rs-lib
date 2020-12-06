use std::{io::prelude::*, iter::FromIterator};

pub struct IO<R, W> {
    i: Source<R>,
    o: W,
}

impl<R: Read, W: Write> IO<R, W> {
    pub fn new(r: R, o: W) -> Self {
        Self {
            i: Source::new(r),
            o,
        }
    }
    pub fn read<T: Readable>(&mut self) -> T {
        self.i.read::<T>()
    }
    pub fn read_collect<T: Readable, A: FromIterator<T>>(&mut self, len: usize) -> A {
        (0..len).map(|_| self.read::<T>()).collect()
    }
    pub fn read_vec<T: Readable>(&mut self, len: usize) -> Vec<T> {
        self.read_collect::<T, _>(len)
    }
    pub fn write<T: ToString>(&mut self, item: T) {
        self.o
            .write_all(item.to_string().as_bytes())
            .expect("failed to write");
    }
    pub fn writeln<T: ToString>(&mut self, item: T) {
        self.write(item);
        self.o.write_all(b"\n").expect("failed to write");
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
        while self.ptr != self.buf.len() && self.buf[self.ptr] <= 0x20 {
            debug_assert!(self.buf[self.ptr] < 0x7f, "invalid src");
            self.ptr += 1;
        }
        if self.ptr == self.buf.len() {
            self.load();
            return self.next_token();
        }
        let begin = self.ptr;
        while self.ptr != self.buf.len() && self.buf[self.ptr] > 0x20 {
            debug_assert!(self.buf[self.ptr] < 0x7f, "invalid src");
            self.ptr += 1;
        }
        unsafe { std::str::from_utf8_unchecked(&self.buf[begin..self.ptr]) }
    }
    pub fn read<T: Readable>(&mut self) -> T {
        T::read_from(self)
    }
    fn load(&mut self) {
        self.buf.clear();
        self.src.read_to_end(&mut self.buf).expect("failed to read");
        self.ptr = 0;
    }
}

pub trait Readable {
    fn read_from<R: Read>(src: &mut Source<R>) -> Self;
}

#[macro_export]
macro_rules! derive_readable {
    ($($t:ty),*) => {$(
        impl $crate::io::Readable for $t {
            fn read_from<R: ::std::io::Read>(src: &mut $crate::io::Source<R>) -> Self {
                src.next_token().parse().ok().expect("parse error")
            }
        }
    )*};
}

derive_readable!(f32, f64, i8, i16, i32, i64, i128, isize, u16, u32, u64, u128, usize, String);

// ASCII code. This does not skip whitespaces after character.
impl Readable for u8 {
    fn read_from<R: Read>(src: &mut Source<R>) -> Self {
        while src.ptr != src.buf.len() && src.buf[src.ptr] <= 0x20 {
            debug_assert!(src.buf[src.ptr] < 0x7f, "invalid src");
            src.ptr += 1;
        }
        if src.ptr == src.buf.len() {
            src.load();
            return Self::read_from(src);
        }
        src.buf[src.ptr]
    }
}

// This does not skip whitespaces after character.
impl Readable for char {
    fn read_from<R: Read>(src: &mut Source<R>) -> Self {
        while src.ptr != src.buf.len() && src.buf[src.ptr] <= 0x20 {
            debug_assert!(src.buf[src.ptr] < 0x7f, "invalid src");
            src.ptr += 1;
        }
        if src.ptr == src.buf.len() {
            src.load();
            return Self::read_from(src);
        }
        src.buf[src.ptr] as char
    }
}

impl Readable for bool {
    fn read_from<R: Read>(src: &mut Source<R>) -> Self {
        match src.read::<u8>() {
            0 => false,
            1 => true,
            _ => panic!("parse error, '0' and '1' are bool"),
        }
    }
}

macro_rules! impl_readable_tuple {
    ($t0:ident) => {};
    ($t0:ident, $($t:ident),*) => {
        impl<$t0: Readable, $($t: Readable),*> Readable for ($t0, $($t),*) {
            fn read_from<R: Read>(src: &mut Source<R>) -> Self {
                ($t0::read_from(src), $($t::read_from(src)),*)
            }
        }
        impl_readable_tuple!($($t),*);
    };
}

impl_readable_tuple!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11);

macro_rules! impl_to_string_intoiter_with_delim {
    ($name:ident, $delim:expr) => {
        pub struct $name<T>(T);
        impl<T> ToString for $name<T>
        where
            for<'a> &'a T: IntoIterator,
            for<'a> <&'a T as IntoIterator>::Item: ToString,
        {
            fn to_string(&self) -> String {
                self.0
                    .into_iter()
                    .by_ref()
                    .map(|s| s.to_string())
                    .collect::<Vec<_>>()
                    .join($delim)
            }
        }
    };
}

impl_to_string_intoiter_with_delim!(Lines, "\n");
impl_to_string_intoiter_with_delim!(Words, " ");
impl_to_string_intoiter_with_delim!(Concat, "");
