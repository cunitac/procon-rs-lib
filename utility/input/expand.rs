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
fn test() {
    let a = crate::STDIN_SOURCE
        .with(|stdin| <u32 as crate::FromSource>::from_source(&mut stdin.borrow_mut()).unwrap())
        .unwrap();
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
mod marker {
    pub enum Byte {}
    pub enum Bytes {}
    pub enum Usize1 {}
    pub enum Isize1 {}
}
mod tests {}
