#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2018::*;
#[macro_use]
extern crate std;
use std::{
    cell::RefCell,
    io::{Read, Stdin},
    str::{FromStr, SplitWhitespace},
};
#[doc(hidden)]
pub const STDIN_SOURCE: ::std::thread::LocalKey<RefCell<Source<Stdin>>> = {
    #[inline]
    fn __init() -> RefCell<Source<Stdin>> {
        RefCell::new(Source::new(std::io::stdin()))
    }
    unsafe fn __getit() -> ::std::option::Option<&'static RefCell<Source<Stdin>>> {
        #[thread_local]
        #[cfg(all(
            target_thread_local,
            not(all(target_arch = "wasm32", not(target_feature = "atomics"))),
        ))]
        static __KEY: ::std::thread::__FastLocalKeyInner<RefCell<Source<Stdin>>> =
            ::std::thread::__FastLocalKeyInner::new();
        #[allow(unused_unsafe)]
        unsafe {
            __KEY.get(__init)
        }
    }
    unsafe { ::std::thread::LocalKey::new(__getit) }
};
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
            {
                ::std::rt::begin_panic("not finished")
            }
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
    pub enum Byte {}
    impl FromSource for Byte {
        type Output = u8;
        #[allow(unused_mut)]
        fn from_source<R: Read>(mut s: &mut Source<R>) -> Option<u8> {
            Some(<char as crate::FromSource>::from_source(&mut s)? as u8)
        }
    }
    pub enum Bytes {}
    impl FromSource for Bytes {
        type Output = Vec<u8>;
        #[allow(unused_mut)]
        fn from_source<R: Read>(mut s: &mut Source<R>) -> Option<Vec<u8>> {
            Some(s.next_token()?.bytes().collect())
        }
    }
    pub enum Chars {}
    impl FromSource for Chars {
        type Output = Vec<char>;
        #[allow(unused_mut)]
        fn from_source<R: Read>(mut s: &mut Source<R>) -> Option<Vec<char>> {
            Some(s.next_token()?.chars().collect())
        }
    }
    pub enum Usize1 {}
    impl FromSource for Usize1 {
        type Output = usize;
        #[allow(unused_mut)]
        fn from_source<R: Read>(mut s: &mut Source<R>) -> Option<usize> {
            Some(<usize as crate::FromSource>::from_source(&mut s)? - 1)
        }
    }
    pub enum Isize1 {}
    impl FromSource for Isize1 {
        type Output = isize;
        #[allow(unused_mut)]
        fn from_source<R: Read>(mut s: &mut Source<R>) -> Option<isize> {
            Some(<isize as crate::FromSource>::from_source(&mut s)? - 1)
        }
    }
}
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
    {
        match (
            &<usize as crate::FromSource>::from_source(&mut source).unwrap(),
            &42,
        ) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        }
    };
    {
        match (
            &(|| {
                Some((
                    <f64 as crate::FromSource>::from_source(&mut source)?,
                    <String as crate::FromSource>::from_source(&mut source)?,
                ))
            })()
            .unwrap(),
            &(4.2, String::from("四二")),
        ) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        }
    };
    {
        match (
            &<usize as crate::FromSource>::from_source(&mut source)
                .and_then(|len| {
                    (0..len)
                        .map(|_| <u32 as crate::FromSource>::from_source(&mut source))
                        .collect::<::std::option::Option<::std::vec::Vec<_>>>()
                })
                .unwrap(),
            &<[_]>::into_vec(box [1, 2, 3]),
        ) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        }
    };
    use marker::*;
    let (byte, (bytes, (chars, usize1)), isize1) = (|| {
        Some((
            <Byte as crate::FromSource>::from_source(&mut source)?,
            (|| {
                Some((
                    <Bytes as crate::FromSource>::from_source(&mut source)?,
                    (|| {
                        Some((
                            <Chars as crate::FromSource>::from_source(&mut source)?,
                            <Usize1 as crate::FromSource>::from_source(&mut source)?,
                        ))
                    })()?,
                ))
            })()?,
            <Isize1 as crate::FromSource>::from_source(&mut source)?,
        ))
    })()
    .unwrap();
    let n = <usize as crate::FromSource>::from_source(&mut source).unwrap();
    let a = (0..n)
        .map(|_| <i32 as crate::FromSource>::from_source(&mut source))
        .collect::<::std::option::Option<::std::vec::Vec<_>>>()
        .unwrap();
    {
        match (&byte, &b'b') {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        }
    };
    {
        match (&bytes, &b"bytes") {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        }
    };
    {
        match (&chars, &<[_]>::into_vec(box ['c', 'h', 'a', 'r', 's'])) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        }
    };
    {
        match (&usize1, &0) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        }
    };
    {
        match (&isize1, &-6) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        }
    };
    {
        match (&a, &<[_]>::into_vec(box [3, 2, 1])) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        }
    };
    {
        match (
            &<String as crate::FromSource>::from_source(&mut source),
            &Some(String::from("try")),
        ) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        }
    };
    {
        match (
            &<String as crate::FromSource>::from_source(&mut source),
            &None,
        ) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        }
    };
}
