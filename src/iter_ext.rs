impl<I> IterExt for I where I: Iterator {}

pub trait IterExt: Iterator {
    fn accumulate<B, F>(self, init: B, f: F) -> Accumulate<Self, B, F>
    where
        F: FnMut(&B, Self::Item) -> B,
        Self: Sized,
    {
        Accumulate::new(self, Some(init), f)
    }
}

pub struct Accumulate<I, B, F> {
    iter: I,
    next: Option<B>,
    f: F,
}
impl<I, B, F> Accumulate<I, B, F> {
    fn new(iter: I, next: Option<B>, f: F) -> Self { Self { iter, next, f } }
}
impl<I, B, F> Iterator for Accumulate<I, B, F>
where
    F: FnMut(&B, I::Item) -> B,
    I: Iterator,
{
    type Item = B;
    fn next(&mut self) -> Option<B> {
        let item = self.next.take()?;
        self.next = self.iter.next().map(|x| (self.f)(&item, x));
        Some(item)
    }
}

#[test]
fn test_accumulate() {
    use itertools::Itertools;

    fn f(a: &i32, b: &i32) -> i32 { a + b }
    let cum = [1, 2, 3].iter().accumulate(1, f).collect_vec();
    assert_eq!(cum, vec![1, 1, 2, 6]);
}
