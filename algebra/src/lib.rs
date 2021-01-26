use std::default::Default;
use std::marker::PhantomData;

pub trait Monoid {
    type Item;
    fn e(&self) -> Self::Item;
    fn f(&self, a: &Self::Item, b: &Self::Item) -> Self::Item;
}

impl<T, E: Fn() -> T, F: Fn(&T, &T) -> T> Monoid for (E, F) {
    type Item = T;
    fn e(&self) -> T {
        self.0()
    }
    fn f(&self, a: &T, b: &T) -> T {
        self.1(a, b)
    }
}

#[derive(Default, Copy, Clone, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct Max<T>(PhantomData<fn() -> T>);
impl<T> Max<T> {
    pub fn new() -> Self {
        Self(PhantomData)
    }
}
impl Monoid for Max<u32> {
    type Item = u32;
    fn e(&self) -> u32 {
        0
    }
    fn f(&self, a: &u32, b: &u32) -> u32 {
        *a.max(b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        struct S<M: Monoid>(M);
        let m1 = Max::<u32>::new();
        let m2 = S((|| 0, |&a, &b| a + b));
    }
}
