use crate::source::Source;
use std::fmt::Debug;
use std::io::BufRead;
use std::ops::Sub;
use std::str::FromStr;

/// [`Source`] からのパースの方法を定める。
pub trait Parser<T> {
    /// 値をパースする。
    /// # Panics
    /// パースに失敗したとき、パニックする。
    fn parse<R: BufRead>(&self, source: &mut Source<R>) -> T;
}

impl<T, P: Parser<T>> Parser<T> for &P {
    fn parse<R: BufRead>(&self, source: &mut Source<R>) -> T {
        (*self).parse(source)
    }
}

impl<T0, T1, P0, P1> Parser<(T0, T1)> for (P0, P1)
where
    P0: Parser<T0>,
    P1: Parser<T1>,
{
    fn parse<R: BufRead>(&self, source: &mut Source<R>) -> (T0, T1) {
        (self.0.parse(source), self.1.parse(source))
    }
}

pub struct Just;

impl<T> Parser<T> for Just
where
    T: FromStr,
    T::Err: Debug,
{
    fn parse<R: BufRead>(&self, source: &mut Source<R>) -> T {
        source.next_token().parse().unwrap()
    }
}

pub struct Tuple;

macro_rules! impl_parser_for_tuple {
    ($($T:ident, )*) => {
        impl<$($T,)*> Parser<($($T,)*)> for Tuple
        where $(Just: Parser<$T>, )*
        {
            fn parse<R: BufRead>(&self, source: &mut Source<R>) -> ($($T,)*) {
                ($(source.parse::<$T, _>(Just), )*)
            }
        }
    };
}

impl_parser_for_tuple!(T0, T1,);

pub struct Base<U>(pub U);

impl<T, U> Parser<T> for Base<U>
where
    Just: Parser<T>,
    T: Sub<U, Output = T>,
    U: Copy,
{
    fn parse<R: BufRead>(&self, source: &mut Source<R>) -> T {
        source.parse(Just) - self.0
    }
}
