/// 累積和をもつ stack
pub struct StackAggregation<T, I, F> {
    accum: Vec<T>,
    #[allow(unused)]
    id: I,
    id_: T,
    prod: F,
}

impl<T, I: Fn() -> T, F: Fn(&T, &T) -> T> StackAggregation<T, I, F> {
    pub fn new(id: I, prod: F) -> Self {
        Self {
            accum: vec![],
            id_: id(),
            id,
            prod,
        }
    }
    // 空なら`id`
    pub fn prod_all(&self) -> &T {
        self.accum.last().unwrap_or(&self.id_)
    }
    // 空なら `false`
    pub fn pop(&mut self) -> bool {
        self.accum.pop().is_some()
    }
    pub fn push(&mut self, val: &T) {
        self.accum.push((self.prod)(self.prod_all(), val))
    }
}

impl<T, I: Fn() -> T, F: Fn(&T, &T) -> T> Extend<T> for StackAggregation<T, I, F> {
    fn extend<It: IntoIterator<Item = T>>(&mut self, iter: It) {
        let mut iter = iter.into_iter();
        while let Some(val) = iter.next() {
            if self.accum.len() == self.accum.capacity() {
                self.accum.reserve(iter.size_hint().0);
            }
            self.push(&val);
        }
    }
}

#[cfg(test)]
mod tests {}
