//! Iterator の拡張

impl<I> IterExt for I where I: Iterator {}

pub trait IterExt: Iterator {
    /// 累積和のようなもの
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
    fn new(iter: I, next: Option<B>, f: F) -> Self {
        Self { iter, next, f }
    }
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
    let cum: Vec<_> = [1, 2, 3].iter().accumulate(1, |a, b| a * b).collect();
    assert_eq!(cum, vec![1, 1, 2, 6]);
}
