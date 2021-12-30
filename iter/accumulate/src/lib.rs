impl<I: Iterator> IterExt for I {}
pub trait IterExt: Iterator + Sized {
    /// ```
    /// use accumulate::IterExt;
    ///
    /// let c = [1, 2, 3].iter().accumulate(0, |&a, &b| a + b).collect::<Vec<_>>();
    /// assert_eq!(c, vec![0, 0 + 1, 0 + 1 + 2, 0 + 1 + 2 + 3]);
    /// ```
    fn accumulate<B, F: FnMut(&B, Self::Item) -> B>(self, init: B, f: F) -> Accumulate<Self, B, F> {
        Accumulate {
            iter: self,
            next: Some(init),
            f,
        }
    }
}

pub struct Accumulate<I, B, F> {
    iter: I,
    next: Option<B>,
    f: F,
}

impl<I: Iterator, B, F: FnMut(&B, I::Item) -> B> Iterator for Accumulate<I, B, F> {
    type Item = B;
    fn next(&mut self) -> Option<B> {
        let next = self
            .iter
            .next()
            .map(|x| (self.f)(self.next.as_ref().unwrap(), x));
        std::mem::replace(&mut self.next, next)
    }
}
